// 📂 src/lib.rs
// 🔁 Orquestrador central e único ponto de organização modular FireChain

// 🧠 Módulos da interface de linha de comando
pub mod cli {
    pub mod parser;   // 🎛️ CLI UX: parsing de comandos e flags
    pub mod handler;  // 🔁 Dispatcher dos comandos e execução lógica
}

// 🔐 Módulos do núcleo criptográfico e geração de carteiras
pub mod core {
    pub mod address;  // ↪️ Derivação do endereço institucional "f1..."
    pub mod wallet;   // 🔑 Geração e criptografia da carteira
}

// ⚠️ Tratamento de erros institucionais
pub mod error;

// 📦 Exposição direta da enum de erro principal
pub use error::FireError;
