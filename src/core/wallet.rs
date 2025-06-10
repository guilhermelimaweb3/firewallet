// 📂 src/core/wallet.rs
// 🔐 Módulo de geração de chaves e identidade FireChain

use crate::core::address::derive_fire_address_from_pubkey;
use crate::FireError;

use secp256k1::Secp256k1;
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;
use hex;

/// 🧠 Estrutura principal de uma carteira FireChain gerada localmente
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub fingerprint: String,
    pub fire_address: String,
}

impl Wallet {
    /// 🔁 Gera uma nova carteira com segurança criptográfica
    pub fn generate() -> Result<Self, FireError> {
        // 🧪 Setup do contexto da biblioteca secp256k1
        let secp = Secp256k1::new();

        // 🔐 Geração de chave privada aleatória com fonte segura
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        // 🔒 Serialização das chaves
        let priv_hex = hex::encode(secret_key.secret_bytes());
        let pub_hex = hex::encode(public_key.serialize_uncompressed());

        // 🆔 Fingerprint: primeiros 8 bytes do SHA256 da chave pública
        let mut hasher = Sha256::new();
        hasher.update(&public_key.serialize_uncompressed());
        let result = hasher.finalize();
        let fingerprint = hex::encode(&result[..8]).to_uppercase(); // 🔐 Agora 8 bytes (16 caracteres hex)

        // 🔥 Derivação do endereço FireChain (f1r3)
        let fire_address = derive_fire_address_from_pubkey(&public_key.serialize_uncompressed())?;

        Ok(Self {
            private_key: priv_hex,
            public_key: pub_hex,
            fingerprint,
            fire_address,
        })
    }
}
