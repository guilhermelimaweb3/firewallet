use sha2::{Sha256, Digest};
use bs58;

fn main() {
    let mut resultados = vec![];

    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let mut payload = vec![a, b];
            payload.extend(vec![0u8; 20]);

            let checksum = Sha256::digest(Sha256::digest(&payload));
            payload.extend(&checksum[..4]);

            let addr = bs58::encode(&payload).into_string();

            if addr.starts_with("f1") {
                resultados.push(((a, b), addr));
            }
        }
    }

    // Ordena por proximidade visual com "f1r3" ou branding
    resultados.sort_by_key(|(_, addr)| {
        let score = if addr.starts_with("f1r3") {
            0
        } else if addr.starts_with("f1r") {
            1
        } else if addr.starts_with("f1") {
            2
        } else {
            9
        };
        (score, addr.clone())
    });

    println!("🔥 Melhores prefixos FireChain:");
    for ((a, b), addr) in resultados.iter().take(10) {
        println!("🧠 Prefixo: [0x{:02X}, 0x{:02X}] → {}", a, b, addr);
    }
}
