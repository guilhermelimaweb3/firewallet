// 📂 src/cli/parser.rs
// 🎛️ Parser de linha de comando com UX institucional FireChain

use clap::{Parser, Subcommand};

/// 🔥 Fire Wallet CLI — Utilitário minimalista para gerar carteiras FireChain
#[derive(Parser, Debug)]
#[command(
    name = "fire-wallet-cli",
    version,
    about = "🔥 Fire Wallet CLI — Geração local e segura de carteiras FireChain",
    long_about = r#"
🔥 Fire Wallet CLI — Geração local e segura de carteiras FireChain

Este utilitário gera uma nova identidade FireChain com:

- 🔒 Chave Privada
- 🧠 Chave Pública
- 🆔 Fingerprint
- 🔥 Endereço FireChain (f1r3...)

📦 Exemplo de uso:

  $ fire-wallet-cli new

Sem armazenamento, sem transmissão — tudo acontece localmente, com clean code e criptografia forte.
"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// 📜 Subcomandos disponíveis
#[derive(Subcommand, Debug)]
pub enum Command {
    /// 🧠 Gera uma nova carteira completa (privkey, pubkey, fingerprint, f1r3...)
    New,
}
