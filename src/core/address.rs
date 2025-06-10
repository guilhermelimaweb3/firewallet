// 📂 src/core/address.rs
// 🔥 Derivação institucional de endereços FireChain — padrão fixo e instantâneo ("f1...")

use crate::FireError;
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;
use bs58;

/// 🔐 Comprimento mínimo válido da chave pública SECP256k1
/// Suporte a chaves comprimidas (33 bytes) ou não (65 bytes)
const EXPECTED_PUBKEY_LEN: usize = 33;

/// 🔥 Prefixo institucional fixo da FireChain (Mainnet)
/// Definido após análise criptográfica e branding visual
/// Este prefixo binário [0x15, 0x67] garante que todos os endereços
/// comecem com "f1" ao serem codificados em Base58Check
const FIRECHAIN_PREFIX: [u8; 2] = [0x15, 0x67];

/// 🧠 Deriva um endereço FireChain a partir de uma chave pública SECP256k1
///
/// 🔐 Processo institucional (semelhante ao Bitcoin, mas com prefixo FireChain):
///
/// 1. SHA256 da chave pública (bytes)
/// 2. RIPEMD160 do resultado anterior
/// 3. Adição do prefixo institucional fixo [0x15, 0x67]
/// 4. Geração do checksum (primeiros 4 bytes de SHA256(SHA256(payload)))
/// 5. Codificação em Base58 (com checksum) → endereço final
pub fn derive_fire_address_from_pubkey(pubkey_bytes: &[u8]) -> Result<String, FireError> {
    // 🔒 Valida se a chave pública tem o tamanho mínimo esperado
    if pubkey_bytes.len() < EXPECTED_PUBKEY_LEN {
        return Err(FireError::InvalidPublicKey);
    }

    // 🔁 Aplica SHA256 na chave pública
    let sha256 = Sha256::digest(pubkey_bytes);

    // 🔁 Aplica RIPEMD160 no resultado do SHA256
    let ripemd = Ripemd160::digest(sha256);

    // 🧱 Cria o payload inicial com o prefixo FireChain seguido do hash160
    let mut payload = FIRECHAIN_PREFIX.to_vec();
    payload.extend(&ripemd);

    // ✅ Calcula o checksum institucional com double-SHA256 (4 bytes)
    let checksum = Sha256::digest(Sha256::digest(&payload));
    let checksum_slice = &checksum[..4];

    // 📦 Monta o payload final: prefixo + hash160 + checksum
    payload.extend(checksum_slice);

    // 🔁 Codifica tudo em Base58 para obter o endereço FireChain
    Ok(bs58::encode(payload).into_string())
}
