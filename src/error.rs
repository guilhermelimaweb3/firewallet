// 📂 src/error.rs
// ⚠️ Tratamento centralizado de erros Fire Wallet CLI

use std::fmt;

/// 🔥 Enum de erro principal do projeto
#[derive(Debug)]
pub enum FireError {
    Io(std::io::Error),

    /// ❌ Falha ao derivar o endereço a partir da chave pública
    InvalidPublicKey,

    /// ❌ Mensagem customizada genérica (fallback)
    Custom(String),

    /// 🔐 Erro durante o processo de criptografia da carteira
    WalletEncryptionError,

    /// 💾 Erro ao tentar salvar o arquivo `.wallet` no disco
    WalletWriteError,

    /// 🔓 Falha ao descriptografar o conteúdo da carteira `.wallet`
    WalletDecryptionError,

    /// 🧩 Arquivo `.wallet` possui formato inválido ou prefixo corrompido
    WalletFormatInvalid,

    /// 📁 Arquivo `.wallet` não encontrado no caminho fornecido
    WalletFileNotFound,
}

impl fmt::Display for FireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FireError::Io(e) => write!(f, "Erro de I/O: {}", e),
            FireError::InvalidPublicKey => write!(f, "❌ Chave pública inválida para derivação de endereço."),
            FireError::Custom(msg) => write!(f, "{}", msg),

            FireError::WalletEncryptionError => write!(
                f,
                "🔐 Erro ao criptografar a carteira. Isso pode indicar falha interna no algoritmo ou dados inválidos."
            ),

            FireError::WalletWriteError => write!(
                f,
                "💾 Erro ao salvar o arquivo `.wallet`. Verifique permissões de escrita, espaço em disco ou diretório atual."
            ),

            FireError::WalletDecryptionError => write!(
                f,
                "🔓 Falha na descriptografia. A senha pode estar incorreta ou o arquivo `.wallet` foi alterado/corrompido."
            ),

            FireError::WalletFormatInvalid => write!(
                f,
                "🧩 O arquivo `.wallet` possui formato inválido. Prefixo `FIREWLT1` ausente ou estrutura interna inconsistente."
            ),

            FireError::WalletFileNotFound => write!(
                f,
                "📁 Arquivo `.wallet` não encontrado. Verifique se o caminho está correto e se o arquivo possui extensão `.wallet`."
            ),
        }
    }
}

impl std::error::Error for FireError {}

impl From<std::io::Error> for FireError {
    fn from(err: std::io::Error) -> Self {
        FireError::Io(err)
    }
}

impl From<String> for FireError {
    fn from(err: String) -> Self {
        FireError::Custom(err)
    }
}
