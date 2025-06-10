<p align="center">
  <img src="assets/banner.png" alt="Fire Wallet CLI Banner" />
</p>

<h1 align="center">🔥 Fire Wallet CLI</h1>
<p align="center">
  <strong>Geração segura, offline e determinística de carteiras FireChain com suporte a assinatura digital ECDSA.</strong><br />
  <em>Clean code. Zero REST. 100% auditável.</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Fire%20Wallet%20CLI-v0.1.0-orange?style=for-the-badge&logo=firefox-browser&logoColor=white" />
  <img src="https://img.shields.io/badge/Prefixo%20FireChain-f1-blue?style=for-the-badge&logo=flame&logoColor=white" />
  <img src="https://img.shields.io/badge/Fingerprint-8B%20SHA256-green?style=for-the-badge&logo=fingerprint&logoColor=white" />
  <img src="https://img.shields.io/badge/Criptografia-SECP256k1%7CSHA256%7CRIPEMD160-critical?style=for-the-badge&logo=keycdn&logoColor=white" />
  <img src="https://img.shields.io/badge/Assinaturas-ECDSA%20%7C%20Base64-4CAF50?style=for-the-badge&logo=signature&logoColor=white" />
  <img src="https://img.shields.io/badge/Offline-100%25%20Sem%20REST-6A5ACD?style=for-the-badge&logo=wifi-off&logoColor=white" />
  <img src="https://img.shields.io/badge/Auditoria-100%25%20local-9B59B6?style=for-the-badge&logo=vercel&logoColor=white" />
  <img src="https://img.shields.io/badge/Determinismo-Garantido-3B82F6?style=for-the-badge&logo=sync&logoColor=white" />
</p>

---

## 🚀 O QUE É — FIRE WALLET CLI

Utilitário minimalista, 100% local, da FireChain para:

- 🔐 Geração de carteiras com endereço `f1...`
- ✍️ Assinatura digital de mensagens com chave privada FireChain
- ✅ Verificação de autenticidade com chave pública

Sem rede. Sem REST. Sem cache.

---

## 🔐 FUNCIONALIDADES

- Geração determinística e auditável de carteira
- Export `.wallet` criptografado com AES-GCM e Argon2
- Assinatura digital com ECDSA (SECP256k1 + SHA256)
- Verificação da integridade/autenticidade de mensagens
- CLI com feedback visual e foco em UX institucional

---

## 🧠 COMO O ENDEREÇO `f1...` É GERADO?

> Inspirado em blockchains como Bitcoin, com design FireChain.

1. Chave pública → `SHA256` → `RIPEMD160`
2. Prefixo institucional `[0x15, 0x67]` → codificado como `f1...`
3. Checksum duplo → 4 bytes de segurança
4. Base58Check final

🧠 Totalmente determinístico e validável.

---

## 🛠️ COMO USAR — FIRE WALLET CLI

### 1. 🔧 Gerar uma nova carteira

```bash
fire-wallet-cli new --password "senhaForte"
```

**Saída:**
- Fingerprint
- Chave pública (uncompressed)
- Endereço FireChain `f1...`
- Arquivo `.wallet` protegido

### 2. 🔓 Abrir uma carteira existente

```bash
fire-wallet-cli open --file f1ABCDEF.wallet --password "senhaForte"
```

Para exibir a chave privada:

```bash
fire-wallet-cli open --file f1ABCDEF.wallet --password "senhaForte" --show-private
```

### 3. ✍️ Assinar uma mensagem com sua chave FireChain

```bash
fire-wallet-cli sign --file f1ABCDEF.wallet --password "senhaForte" --message "teste de assinatura"
```

**Retorna:**
- Mensagem
- Chave pública
- Assinatura ECDSA codificada em Base64

### 4. ✅ Verificar a assinatura de uma mensagem

```bash
fire-wallet-cli verify --file f1ABCDEF.wallet --password "senhaForte"   --message "teste de assinatura"   --signature "MEUCIQC2...gQ=="
```

**Validação 100% local** — sem rede, sem REST.

---

## 🔐 SEGURANÇA INSTITUCIONAL

| Elemento              | Padrão FireChain                    |
|-----------------------|--------------------------------------|
| 🔒 Chave Privada      | SECP256k1 (formato raw 32 bytes)    |
| 🔁 Criptografia        | AES-GCM 256-bit + Argon2 (sal + nonce) |
| ✍️ Assinaturas         | ECDSA com SHA256 + Base64           |
| 🔍 Verificação local   | Sim, via chave pública uncompressed |
| ☁️ Conexão externa     | ❌ Nenhuma. 100% offline             |

---

## 📦 COMPARATIVO DE FUNCIONALIDADES

| Recurso                    | Fire Wallet CLI | MetaMask | Trust Wallet |
|----------------------------|-----------------|----------|--------------|
| 📡 Funciona offline        | ✅               | ❌       | ❌           |
| 🔗 Endereço institucional  | `f1...` exclusivo| `0x...`  | `0x...`      |
| 🧠 Fingerprint visual      | ✅ SHA256 (8B)   | ❌       | ❌           |
| 🔐 Assinatura digital      | ✅ Base64 ECDSA  | ✅ (via RPC) | ✅ (via RPC) |
| 🔁 Determinismo            | ✅               | ❌       | ❌           |
| 🔒 Autocustódia real       | ✅               | Parcial  | Parcial      |

---

## 📂 ESTRUTURA DE PASTAS

```txt
firewallet/
├── assets/              # Banner institucional
├── src/
│   ├── bin/             # Entrypoint CLI
│   ├── cli/             # Parsing CLI
│   ├── core/            # Geração, assinatura e endereço
│   ├── error.rs         # Enum centralizado de erros
│   └── lib.rs
├── tools/               # Scanner institucional de prefixo
├── Cargo.toml           # Manifesto do projeto
└── README.md            # Este documento
```

---

## 📦 INSTALAÇÃO

```bash
git clone https://github.com/firechainmainnet/fire-wallet-cli.git
cd fire-wallet-cli
cargo build --release
```

---

## 💡 DICAS

- 🌍 Para tornar global no Windows:
```powershell
copy .	arget
eleaseire-wallet-cli.exe C:\Tools$env:PATH += ";C:\Tools"
```

- ✨ Futuro: suporte a assinatura de arquivos (`--file`), export JSON (`--json`) e múltiplos formatos de saída.

---

## 📄 LICENÇA

Distribuído sob **Licença FireChain Dual**:

- ✅ Pessoal e educacional: liberado
- 💼 Comercial: requer autorização FireChain

---

👤 Desenvolvido por [Guilherme Lima](https://www.linkedin.com/in/guilhermelimadev-web3/)  
📦 Repositório oficial: [github.com/firechainmainnet/fire-wallet-cli](https://github.com/firechainmainnet/fire-wallet-cli)