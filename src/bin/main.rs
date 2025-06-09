// 📂 src/bin/main.rs
// 🚀 Entry point da Fire Wallet CLI — simples, robusto e institucional

use clap::Parser;
use colored::Colorize;

use fire_wallet_cli::cli::parser::Cli;
use fire_wallet_cli::cli::handler::handle_cli;
use fire_wallet_cli::FireError;

fn main() {
    // Parse CLI e executa
    if let Err(e) = run() {
        eprintln!("{} {}", "❌ Erro:".red(), e.to_string().dimmed());
        std::process::exit(1);
    }
}

/// 🔁 Função principal que executa a CLI
fn run() -> Result<(), FireError> {
    let cli = Cli::parse();
    handle_cli(cli)
}
