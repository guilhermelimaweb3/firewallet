
# 🔐 Formato `.wallet` — Fire Wallet CLI

> 🧠 **Documento oficial** do Fire Wallet CLI descrevendo a estrutura binária criptografada dos arquivos `.wallet`.  
> 🔒 Arquitetura de segurança: **AES-GCM + Argon2id**  
> 🔍 Compatível com padrões de segurança modernos e proteção contra ataques offline.

---

## 📦 Estrutura Binária: `<fingerprint>.wallet`

Cada arquivo `.wallet` contém 4 seções ordenadas de forma precisa:

| Offset  | Tamanho (bytes) | Campo         | Descrição                                                                 |
|---------|------------------|---------------|---------------------------------------------------------------------------|
| `0`     | 8                | `MAGIC`       | Prefixo fixo: `FIREWLT1` (para validação de formato)                      |
| `8`     | 16               | `SALT`        | Salt randômico para derivação Argon2id                                    |
| `24`    | 12               | `NONCE`       | Nonce aleatório para operação AES-GCM                                     |
| `36`    | variável         | `CIPHERTEXT`  | Dados criptografados da carteira (privkey, pubkey, address)               |

---

## 🧪 Criptografia Interna

### 🔑 Derivação de Chave

- Algoritmo: **Argon2id**
- Parâmetros padrão: [Rust `argon2::Argon2::default()`]
- Salt gerado por `OsRng`
- Entrada: senha fornecida via `--password`
- Resultado: chave simétrica de 32 bytes (`AES-256`)

### 🔐 Criptografia

- Algoritmo: **AES-256-GCM**
- Nonce de 96 bits (12 bytes)
- CIPHERTEXT é a serialização `bincode` dos seguintes dados:

```rust
struct WalletExport {
    private_key: String,
    public_key: String,
    fire_address: String,
}
```

---

## 📤 Exemplo de Serialização (antes da criptografia)

```json
{
  "private_key": "fdb2...38a7",
  "public_key": "04b3...0419",
  "fire_address": "f1guD6NXmQaJdw9bEyfnYDpGdxQD1aafxpP"
}
```

---

## 🚨 Validações no Momento de Leitura

| Validação             | Critério                        | Ação em caso de falha                  |
|-----------------------|----------------------------------|----------------------------------------|
| Magic Header          | Deve ser `b"FIREWLT1"`          | `FireError::InvalidWalletFormat`       |
| Tamanho Mínimo        | ≥ 36 bytes                      | `FireError::InvalidWalletFormat`       |
| AES-GCM Decrypt       | Falha na descriptografia        | `FireError::WalletDecryptionError`     |
| Senha Errada          | Falha derivação + decrypt       | `FireError::WalletDecryptionError`     |

---

## 🛡️ Segurança

✅ Arquivos `.wallet` são seguros contra:

- Acesso não autorizado mesmo com o arquivo em mãos  
- Ataques offline de dicionário (graças ao Argon2id)  
- Modificações manuais (AES-GCM autentica)  

❗ **Sem a senha correta, a chave privada permanece inacessível.**

---

## 🧾 Exemplo prático (em shell)

```bash
# Geração da carteira
fire-wallet-cli new --password "minhaSenhaUltraSegura123!"

# Arquivo gerado
B1F3A7D9E6FA42.wallet

# Leitura segura com a mesma senha
fire-wallet-cli open --file B1F3A7D9E6FA42.wallet --password "minhaSenhaUltraSegura123!"
```

---

## 🛠️ Futuro (assinatura opcional 🔏)

🚧 O padrão `.wallet` poderá futuramente conter uma **assinatura digital** embutida:
- Tipo `ECDSA/secp256k1`
- Campo opcional ao final do binário
- Proteção contra alterações ou clonagem do conteúdo

---

## 🧠 Conclusão

O formato `.wallet` da Fire Wallet CLI é:

- 🔐 **Seguro**
- 📦 **Padronizado**
- 🚀 **Leve**
- 💡 **Auditável**

Tudo isso com a mesma confiança e engenharia de ponta do ecossistema **FireChain 🔥**.
