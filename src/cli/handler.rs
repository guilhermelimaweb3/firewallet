// 📂 src/cli/handler.rs
// 🔁 Handler principal para comandos da CLI — UX institucional FireChain

use crate::cli::parser::{Cli, Command, NewWalletArgs, OpenWalletArgs};
use crate::core::wallet::Wallet;
use crate::FireError;

use colored::*;

/// 🎛️ Dispatcher principal da CLI
pub fn handle_cli(cli: Cli) -> Result<(), FireError> {
    match cli.command {
        // 🔁 Geração de nova carteira segura
        Command::New(args) => handle_new(args),

        // 🔓 Leitura de carteira existente
        Command::Open(args) => handle_open(args),
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

/// 🔓 Executa o comando `open` com descriptografia de uma carteira existente
fn handle_open(args: OpenWalletArgs) -> Result<(), FireError> {
    print_banner();

    // 📂 Caminho do arquivo `.wallet`
    let file_path = args.file;
    let password = args.password;
    let show_private = args.show_private;

    // 🔐 Tenta carregar e descriptografar a carteira
    let wallet = Wallet::load_and_decrypt(&file_path, &password)?;

    // ✅ Impressão institucional com ou sem chave privada
    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧠 Chave Pública:".bold(), wallet.public_key.blue());

    if show_private {
        println!("{} {}", "🔓 Chave Privada:".bold(), wallet.private_key.red());
    } else {
        println!("{} {}", "🔒 Chave Privada:".bold(), "[oculta]".dimmed());
    }

    println!("{} {}\n", "🔥 Endereço FireChain:".bold(), wallet.fire_address.green());

    println!(
        "{} {}\n",
        "✅ Status:".bold(),
        "Carteira descriptografada com sucesso.".green()
    );

    Ok(())
}

/// 🎨 Banner institucional com UX visual e branding
fn print_banner() {
    println!("\n{}", "🔥 Fire Wallet CLI".bold().red());
    println!("{}", "🔐 Carteira FireChain local e segura — Geração instantânea com endereço f1r3...\n".yellow());
}
