// 📂 src/core/wallet.rs
// 🔐 Módulo de geração de chaves e identidade FireChain

use crate::core::address::derive_fire_address_from_pubkey;
use crate::FireError;

use secp256k1::Secp256k1;
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use rand::RngCore;
use hex;
use std::fs::File;
use std::io::Write;

use aes_gcm::{
    Aes256Gcm, KeyInit,
    aead::{Aead, generic_array::GenericArray}
};

use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;

use serde::{Serialize, Deserialize};

/// 🧠 Estrutura principal de uma carteira FireChain gerada localmente
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub fingerprint: String,
    pub fire_address: String,
}

/// 📦 Dados exportáveis da carteira que serão criptografados
#[derive(Serialize, Deserialize)]
struct WalletExport {
    private_key: String,
    public_key: String,
    fire_address: String,
}

impl Wallet {
    pub fn generate() -> Result<Self, FireError> {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let priv_hex = hex::encode(secret_key.secret_bytes());
        let pub_hex = hex::encode(public_key.serialize_uncompressed());

        let mut hasher = Sha256::new();
        hasher.update(&public_key.serialize_uncompressed());
        let result = hasher.finalize();
        let fingerprint = hex::encode(&result[..8]).to_uppercase();

        let fire_address = derive_fire_address_from_pubkey(&public_key.serialize_uncompressed())?;

        Ok(Self {
            private_key: priv_hex,
            public_key: pub_hex,
            fingerprint,
            fire_address,
        })
    }

    pub fn generate_and_save(password: &str) -> Result<Self, FireError> {
        let wallet = Wallet::generate()?;

        let export = WalletExport {
            private_key: wallet.private_key.clone(),
            public_key: wallet.public_key.clone(),
            fire_address: wallet.fire_address.clone(),
        };

        let serialized = bincode::serialize(&export).map_err(|_| FireError::WalletEncryptionError)?;

        let mut salt = [0u8; 16];
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce);

        let argon2 = Argon2::default();
        let salt_str = SaltString::encode_b64(&salt).map_err(|_| FireError::WalletEncryptionError)?;
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_str)
            .map_err(|_| FireError::WalletEncryptionError)?;
        let hash_bytes = password_hash.hash.ok_or(FireError::WalletEncryptionError)?;
        let derived_key = hash_bytes.as_bytes();

        let key = GenericArray::from_slice(derived_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(&nonce);

        let ciphertext = cipher.encrypt(nonce, serialized.as_ref())
            .map_err(|_| FireError::WalletEncryptionError)?;

        let mut file_data = vec![];
        file_data.extend_from_slice(b"FIREWLT1");
        file_data.extend_from_slice(&salt);
        file_data.extend_from_slice(&nonce);
        file_data.extend_from_slice(&ciphertext);

        let filename = format!("{}.wallet", wallet.fingerprint);
        let mut file = File::create(&filename).map_err(|_| FireError::WalletWriteError)?;
        file.write_all(&file_data).map_err(|_| FireError::WalletWriteError)?;

        Ok(wallet)
    }

    pub fn load_and_decrypt(file_path: &str, password: &str) -> Result<Self, FireError> {
        use std::fs;

        let data = fs::read(file_path).map_err(|_| FireError::WalletFileNotFound)?;

        if data.len() < 36 || &data[..8] != b"FIREWLT1" {
            return Err(FireError::WalletFormatInvalid);
        }

        let salt = &data[8..24];
        let nonce = &data[24..36];
        let ciphertext = &data[36..];

        let salt_str = SaltString::encode_b64(salt).map_err(|_| FireError::WalletDecryptionError)?;
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_str)
            .map_err(|_| FireError::WalletDecryptionError)?;
        let hash_bytes = password_hash.hash.ok_or(FireError::WalletDecryptionError)?;
        let derived_key = hash_bytes.as_bytes();

        let key = GenericArray::from_slice(derived_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(nonce);

        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| FireError::WalletDecryptionError)?;

        let export: WalletExport = bincode::deserialize(&decrypted)
            .map_err(|_| FireError::WalletDecryptionError)?;

        let pub_bytes = hex::decode(&export.public_key).map_err(|_| FireError::WalletDecryptionError)?;
        let mut hasher = Sha256::new();
        hasher.update(&pub_bytes);
        let result = hasher.finalize();
        let fingerprint = hex::encode(&result[..8]).to_uppercase();

        Ok(Wallet {
            private_key: export.private_key,
            public_key: export.public_key,
            fire_address: export.fire_address,
            fingerprint,
        })
    }
}