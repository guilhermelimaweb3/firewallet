// 📂 src/cli/parser.rs
// 🎛️ Parser de linha de comando com UX institucional FireChain

use clap::{Parser, Subcommand, Args};

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

📦 Exemplos de uso:

  $ fire-wallet-cli new --password "senhaForteAqui"
  $ fire-wallet-cli open --file f1ABCDEF.wallet --password "senhaForteAqui" --show-private

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
    /// 🧠 Gera uma nova carteira completa (privkey, pubkey, fingerprint, f1r3...) + salva criptografada
    New(NewWalletArgs),

    /// 🔓 Abre e descriptografa uma carteira `.wallet` existente
    Open(OpenWalletArgs),
}

/// 🧾 Argumentos obrigatórios do comando `new`
#[derive(Args, Debug)]
pub struct NewWalletArgs {
    /// 🔐 Senha para criptografar a carteira (obrigatória)
    #[arg(short, long, help = "Senha de proteção (usada na criptografia da carteira)", required = true)]
    pub password: String,
}

/// 🧾 Argumentos do comando `open`
#[derive(Args, Debug)]
pub struct OpenWalletArgs {
    /// 📂 Caminho para o arquivo `.wallet` criptografado
    #[arg(short, long, help = "Arquivo .wallet a ser lido e descriptografado", required = true)]
    pub file: String,

    /// 🔐 Senha usada para descriptografar os dados
    #[arg(short, long, help = "Senha utilizada para descriptografar o arquivo", required = true)]
    pub password: String,

    /// 👁️ Exibir a chave privada descriptografada (opcional e sensível)
    #[arg(long, help = "Exibir a chave privada após descriptografar (use com cautela)")]
    pub show_private: bool,
}
