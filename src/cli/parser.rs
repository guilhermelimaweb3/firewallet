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
  $ fire-wallet-cli sign --file f1ABCDEF.wallet --password "senhaForteAqui" --message "meu payload"
  $ fire-wallet-cli verify --file f1ABCDEF.wallet --password "senhaForteAqui" --message "meu payload" --signature "base64..."

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

    /// ✍️ Assina um payload com a chave privada da carteira
    Sign(SignArgs),

    /// ✅ Verifica a assinatura de um payload com a chave pública da carteira
    Verify(VerifyArgs),
}

/// 🧾 Argumentos do comando `new`
#[derive(Args, Debug)]
pub struct NewWalletArgs {
    #[arg(short, long, help = "Senha de proteção (usada na criptografia da carteira)", required = true)]
    pub password: String,
}

/// 🔍 Argumentos do comando `open`
#[derive(Args, Debug)]
pub struct OpenWalletArgs {
    #[arg(short, long, help = "Caminho do arquivo .wallet", required = true)]
    pub file: String,

    #[arg(short, long, help = "Senha usada na criptografia", required = true)]
    pub password: String,

    #[arg(long, help = "Exibir a chave privada na saída")]
    pub show_private: bool,
}

/// ✍️ Argumentos do comando `sign`
#[derive(Args, Debug)]
pub struct SignArgs {
    #[arg(short, long, help = "Caminho do arquivo .wallet", required = true)]
    pub file: String,

    #[arg(short, long, help = "Senha usada na criptografia", required = true)]
    pub password: String,

    #[arg(short, long, help = "Payload a ser assinado", required = true)]
    pub message: String,
}

/// ✅ Argumentos do comando `verify`
#[derive(Args, Debug)]
pub struct VerifyArgs {
    #[arg(short, long, help = "Caminho do arquivo .wallet", required = true)]
    pub file: String,

    #[arg(short, long, help = "Senha usada na criptografia", required = true)]
    pub password: String,

    #[arg(short, long, help = "Payload original que foi assinado", required = true)]
    pub message: String,

    #[arg(short, long, help = "Assinatura Base64 gerada anteriormente", required = true)]
    pub signature: String,
}
