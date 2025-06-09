// 📂 src/error.rs
// ⚠️ Tratamento centralizado de erros Fire Wallet CLI

use std::fmt;

/// 🔥 Enum de erro principal do projeto
#[derive(Debug)]
pub enum FireError {
    Io(std::io::Error),
    InvalidPublicKey,
    Custom(String),
}

impl fmt::Display for FireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FireError::Io(e) => write!(f, "Erro de I/O: {}", e),
            FireError::InvalidPublicKey => write!(f, "Chave pública inválida para derivação de endereço."),
            FireError::Custom(msg) => write!(f, "{}", msg),
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
