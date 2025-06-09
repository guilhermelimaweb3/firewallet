<p align="center">
  <img src="assets/banner.png" alt="Fire Wallet CLI Banner" />
</p>

<h1 align="center">🔥 Fire Wallet CLI</h1>
<p align="center">
  <strong>Geração local, segura e instantânea de carteiras FireChain no terminal.</strong><br />
  <em>Clean code. Zero dependências externas. 100% auditável.</em>
</p>

<p align="center">
  <!-- 🔥 Nome + versão -->
  <img src="https://img.shields.io/badge/Fire%20Wallet%20CLI-v0.1.0-orange?style=flat-square&logo=firefox-browser&logoColor=white" alt="Versão" />

  <!-- 🔐 Licença personalizada -->
  <img src="https://img.shields.io/badge/Licença-FireChain%20Dual-red?style=flat-square&logo=scale&logoColor=white" alt="Licença FireChain Dual" />

  <!-- 🔒 Criptografia avançada -->
  <img src="https://img.shields.io/badge/Criptografia-SECP256k1%20%7C%20SHA256%20%7C%20RIPEMD160-blue?style=flat-square&logo=keycdn&logoColor=white" alt="Criptografia avançada" />

  <!-- ⚙️ Geração Local -->
  <img src="https://img.shields.io/badge/100%25%20Offline-Sem%20rede%20ou%20cache-6A5ACD?style=flat-square&logo=wifi-off&logoColor=white" alt="Offline" />

  <!-- 🧼 Clean Code -->
  <img src="https://img.shields.io/badge/Clean%20Code-✓-green?style=flat-square&logo=codefactor&logoColor=white" alt="Código limpo" />

  <!-- 🔥 Endereço FireChain -->
  <img src="https://img.shields.io/badge/f1r3%20address-Suportado-F28500?style=flat-square&logo=flame&logoColor=white" alt="FireChain Address" />

  <!-- 🛡️ Proteção institucional -->
  <img src="https://img.shields.io/badge/Proteção%20Institucional-Zero%20vazamento-critical?style=flat-square&logo=shield&logoColor=white" alt="Proteção total" />
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

Distribuído sob a **Licença FireChain Dual**:

- ✅ Uso gratuito e pessoal para estudos, autocustódia e aprendizado
- 💼 Uso comercial **mediante autorização e contrato**

[Leia os termos completos aqui →](./LICENSE.md)

---

👤 Desenvolvido por [Guilherme Lima](https://www.linkedin.com/in/guilhermelimadev-web3/)  
📦 Repositório oficial: [github.com/guilhermelimaweb3/firewallet](https://github.com/guilhermelimaweb3/firewallet.git)
