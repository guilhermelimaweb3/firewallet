// 📂 src/cli/handler.rs
// 🔁 Handler principal para comandos da CLI — UX institucional FireChain

use crate::cli::parser::{Cli, Command};
use crate::core::wallet::Wallet;
use crate::FireError;

use colored::*;

/// 🎛️ Dispatcher principal da CLI
pub fn handle_cli(cli: Cli) -> Result<(), FireError> {
    match cli.command {
        Command::New => handle_new(),
    }
}

/// 🧠 Executa o comando `new` com geração completa de carteira
fn handle_new() -> Result<(), FireError> {
    print_banner();

    let wallet = Wallet::generate()?;

    println!("\n{} {}", "🆔 Fingerprint:".bold(), wallet.fingerprint.yellow());
    println!("{} {}", "🧠 Chave Pública:".bold(), wallet.public_key.blue());
    println!("{} {}", "🔒 Chave Privada:".bold(), wallet.private_key.red());
    println!("{} {}\n", "🔥 Endereço FireChain:".bold(), wallet.fire_address.green());

    Ok(())
}

/// 🎨 Banner institucional com UX visual e branding
fn print_banner() {
    println!("\n{}", "🔥 Fire Wallet CLI".bold().red());
    println!("{}", "🔐 Carteira FireChain local e segura — Geração instantânea com endereço f1r3...\n".yellow());
}
