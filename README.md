<p align="center">
  <img src="assets/banner.png" alt="Fire Wallet CLI Banner" />
</p>

<h1 align="center">🔥 Fire Wallet CLI</h1>
<p align="center">
  <strong>Geração local, segura e instantânea de carteiras FireChain no terminal.</strong><br />
  <em>Clean code. Zero dependências externas. 100% auditável.</em>
</p>

<p align="center">
  <a href="https://crates.io/crates/fire-wallet-cli">
    <img src="https://img.shields.io/crates/v/fire-wallet-cli?style=for-the-badge&color=firebrick" alt="Crate version" />
  </a>
  <a href="https://github.com/guilhermelimaweb3/firewallet/blob/main/LICENSE.md">
    <img src="https://img.shields.io/badge/license-FireChain-red?style=for-the-badge" alt="License: FireChain" />
  </a>
  <img src="https://img.shields.io/badge/clean%20code-✓-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/f1r3%20address-supported-orange?style=for-the-badge" />
</p>

---

## 🚀 O que é?

O **Fire Wallet CLI** é um utilitário minimalista que permite:

✅ Gerar uma nova chave privada/pública  
✅ Calcular o **fingerprint institucional** (hash visual da chave)  
✅ Derivar um endereço FireChain no formato `f1r3...` com segurança local

**Tudo localmente. Nada é salvo. Nada é enviado.**

---

## 🔧 Como instalar

```bash
# Clone o repositório
git clone https://github.com/guilhermelimaweb3/firewallet.git
cd firewallet

# Build otimizado (recomendado)
cargo build --release
```

> Requer: Rust 1.70+ instalado

---

## 🧪 Como usar

### 🧬 Gerar uma nova carteira FireChain

```bash
fire-wallet-cli new
```

**Saída esperada:**

```
🔥 Fire Wallet CLI
🔐 Carteira FireChain local e segura — Geração instantânea com endereço f1r3...

🆔 Fingerprint: 60D7EBE61FB3
🧠 Chave Pública: 04a17d...
🔒 Chave Privada: 2eacb6...
🔥 Endereço FireChain: f1r3sU...
```

---

## 🧠 Estrutura de pastas

```txt
firewallet/
├── src/
│   ├── bin/             # 🧭 Entrypoint CLI
│   │   └── main.rs
│   ├── cli/             # 🎛️ Parser e Handler
│   │   ├── parser.rs
│   │   └── handler.rs
│   ├── core/            # 🔐 Geração de chaves e endereços
│   │   ├── address.rs
│   │   └── wallet.rs
│   ├── error.rs         # ⚠️ Enum central de erros
│   └── lib.rs
├── Cargo.toml
├── LICENSE.md
└── README.md
```

---

## 🔐 Segurança

- ✅ Todas as chaves são geradas localmente com `secp256k1` e `OsRng`
- ✅ Sem uso de APIs, sem rede, sem cache
- ✅ Fingerprint hash para auditoria humana visual
- ✅ Endereço FireChain derivado com checksum + Base58

---

## 💡 Design Institucional FireChain

| Princípio | Aplicação |
|-----------|-----------|
| 🧼 Clean Code | Módulos organizados, funções puras |
| 🔁 Reusabilidade | `Wallet::generate()` pode ser chamado de qualquer lugar |
| 🔒 Criptografia Forte | `secp256k1` + SHA256 + RIPEMD160 |
| 🧠 Identidade clara | `fingerprint` em SHA256/hex |

---

## 📄 Licença 🔐

Este projeto é distribuído sob a **Licença FireChain Pessoal**:

- ✅ **Permitido**: uso pessoal, educacional, estudos, pesquisas e auditorias.
- ❌ **Proibido**: uso comercial, revenda, distribuição lucrativa, incorporação em produtos pagos ou monetização de qualquer forma.

> Leia o arquivo [LICENSE.md](./LICENSE.md) para os termos completos.

---

👤 Desenvolvido por [Guilherme Lima](https://www.linkedin.com/in/guilhermelimadev-web3/)  
📦 Repositório oficial: [github.com/guilhermelimaweb3/firewallet](https://github.com/guilhermelimaweb3/firewallet.git)
