// 📂 src/core/address.rs
// 🔥 Algoritmo de derivação de endereços FireChain (f1r3...)

use crate::FireError;
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;
use bs58;

/// 🔐 Tamanho mínimo esperado da chave pública
const EXPECTED_PUBKEY_LEN: usize = 33; // ou 65, dependendo do formato

/// 🔥 Prefixo da FireChain Mainnet (equivalente ao 0x00 no Bitcoin)
const FIRECHAIN_PREFIX: u8 = 0x00;

/// 🧠 Deriva um endereço FireChain (f1r3...) a partir da chave pública
///
/// Processo:
/// 1. SHA256 da chave pública
/// 2. RIPEMD160 do resultado
/// 3. Adição do prefixo FireChain
/// 4. Checksum (SHA256 dupla, 4 bytes)
/// 5. Codificação Base58
pub fn derive_fire_address_from_pubkey(pubkey_bytes: &[u8]) -> Result<String, FireError> {
    // 🔒 Validação mínima da chave
    if pubkey_bytes.len() < EXPECTED_PUBKEY_LEN {
        return Err(FireError::InvalidPublicKey);
    }

    // 🔁 SHA256 → RIPEMD160
    let sha256 = Sha256::digest(pubkey_bytes);
    let ripemd = Ripemd160::digest(sha256);

    // 🧱 Construção do payload com prefixo
    let mut payload = vec![FIRECHAIN_PREFIX];
    payload.extend(&ripemd);

    // ✅ Gerar checksum com double-SHA256
    let checksum = Sha256::digest(Sha256::digest(&payload));
    let checksum_slice = &checksum[..4];

    // 📦 Monta o endereço completo
    payload.extend(checksum_slice);

    // 🔁 Codificação em Base58
    Ok(bs58::encode(payload).into_string())
}
