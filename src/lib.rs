// 📂 src/lib.rs
// 🔁 Orquestrador central e único ponto de organização modular

pub mod cli {
    pub mod parser;
    pub mod handler;
}

pub mod core {
    pub mod address;
    pub mod wallet;
}

pub mod error;
pub use error::FireError;
