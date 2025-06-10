// 📂 src/cli/handler.rs

use crate::cli::parser::{
    Cli, Command, NewWalletArgs, OpenWalletArgs, SignArgs, VerifyArgs,
};
use crate::core::wallet::Wallet;
use crate::FireError;

use colored::*;
use secp256k1::{Message, Secp256k1, SecretKey, PublicKey};
use secp256k1::ecdsa::Signature;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest}; // ✅ Importa o tipo E o trait corretamente
use hex;

/// 🎛️ Dispatcher principal da CLI
pub fn handle_cli(cli: Cli) -> Result<(), FireError> {
    match cli.command {
        Command::New(args) => handle_new(args),
        Command::Open(args) => handle_open(args),
        Command::Sign(args) => handle_sign(args),
        Command::Verify(args) => handle_verify(args),
    }
}

/// 🧠 Executa o comando `new` com geração completa de carteira
/// 🔐 A carteira é criptografada com a senha informada e salva como <fingerprint>.wallet
fn handle_new(args: NewWalletArgs) -> Result<(), FireError> {
    print_banner();

    let password = args.password;

    let wallet = Wallet::generate_and_save(&password)?;

    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧠 Chave Pública:".bold(), wallet.public_key.blue());
    println!("{} {}", "🔒 Chave Privada:".bold(), "[protegida]".red());
    println!("{} {}\n", "🔥 Endereço FireChain:".bold(), wallet.fire_address.green());

    println!(
        "{} {}\n",
        "💾 Arquivo salvo como:".bold(),
        format!("{}.wallet", wallet.fingerprint).cyan()
    );
    println!(
        "{} {}\n",
        "🛡️ Protegido com:".bold(),
        "Criptografia AES-GCM + Argon2".purple()
    );

    Ok(())
}

/// 🔓 Executa o comando `open` e exibe dados da carteira descriptografada
fn handle_open(args: OpenWalletArgs) -> Result<(), FireError> {
    print_banner();

    let wallet = Wallet::load_and_decrypt(&args.file, &args.password)?;

    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧠 Chave Pública:".bold(), wallet.public_key.blue());

    if args.show_private {
        println!("{} {}", "🔓 Chave Privada:".bold(), wallet.private_key.red());
    } else {
        println!("{} {}", "🔒 Chave Privada:".bold(), "[oculta]".red());
    }

    println!("{} {}\n", "🔥 Endereço FireChain:".bold(), wallet.fire_address.green());
    println!("{} {}\n", "✅ Status:".bold(), "Carteira descriptografada com sucesso.".green());

    Ok(())
}

/// ✍️ Assina uma mensagem com a chave privada da carteira
fn handle_sign(args: SignArgs) -> Result<(), FireError> {
    print_banner();

    let wallet = Wallet::load_and_decrypt(&args.file, &args.password)?;

    let secp = Secp256k1::new();
    let secret_key_bytes = hex::decode(&wallet.private_key).map_err(|_| FireError::WalletDecryptionError)?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes).map_err(|_| FireError::WalletDecryptionError)?;

    let mut hasher = Sha256::new();
    hasher.update(args.message.as_bytes());
    let digest = hasher.finalize();
    let msg = Message::from_slice(&digest).map_err(|_| FireError::Custom("Hash inválido.".into()))?;

    let sig = secp.sign_ecdsa(&msg, &secret_key);
    let sig_der = sig.serialize_der();
    let signature_b64 = general_purpose::STANDARD.encode(sig_der);

    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧾 Mensagem:".bold(), args.message.dimmed());
    println!("{} {}\n", "✍️ Assinatura:".bold(), signature_b64.green());

    Ok(())
}

/// ✅ Verifica se uma assinatura é válida para uma mensagem e carteira
fn handle_verify(args: VerifyArgs) -> Result<(), FireError> {
    print_banner();

    let wallet = Wallet::load_and_decrypt(&args.file, &args.password)?;

    let secp = Secp256k1::new();

    let mut hasher = Sha256::new();
    hasher.update(args.message.as_bytes());
    let digest = hasher.finalize();
    let msg = Message::from_slice(&digest).map_err(|_| FireError::Custom("Hash de mensagem inválido.".into()))?;

    let pub_bytes = hex::decode(&wallet.public_key).map_err(|_| FireError::WalletDecryptionError)?;
    let pubkey = PublicKey::from_slice(&pub_bytes).map_err(|_| FireError::WalletDecryptionError)?;

    let sig_bytes = general_purpose::STANDARD
        .decode(&args.signature)
        .map_err(|_| FireError::Custom("Assinatura inválida. Verifique se está em Base64.".into()))?;
    let signature = Signature::from_der(&sig_bytes)
        .map_err(|_| FireError::Custom("Assinatura inválida ou corrompida.".into()))?;

    let is_valid = secp.verify_ecdsa(&msg, &signature, &pubkey).is_ok();

    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧾 Mensagem:".bold(), args.message.dimmed());
    println!("{} {}", "🔍 Assinatura:".bold(), "[verificando]".blue());

    if is_valid {
        println!("\n{} {}\n", "✅ Assinatura válida!".bold().green(), "Mensagem autêntica.".dimmed());
    } else {
        println!("\n{} {}\n", "❌ Assinatura inválida!".bold().red(), "A mensagem foi adulterada ou a chave não confere.".dimmed());
    }

    Ok(())
}

/// 🎨 Banner institucional com UX visual e branding
fn print_banner() {
    println!("\n{}", "🔥 Fire Wallet CLI".bold().red());
    println!("{}", "🔐 Carteira FireChain local e segura — Geração instantânea com endereço f1r3...\n".yellow());
}
