# Verity Ts-client

A TypeScript client for interacting with a verifiable proxy that provides cryptographic proofs for HTTP requests and responses, including Server-Sent Events (SSE).

---

## ✨ Features

- Verify HTTP responses with cryptographic proofs
- Seamless integration with any REST API
- Built-in SSE proof event subscription
- TypeScript-first

---

## 📦 Installation

```bash
npm install @usherlabs/verity-client
```

---

## 🛠️ Usage

### Initialize the Client

```ts
import { VerityClient } from "@usherlabs/verity-client";

const client = new VerityClient({ prover_url: "http://localhost:8080" });
```

---

### 🔐 Get Notary Info

```ts
const notary_info = await client.get_notary_info();

console.log(notary_info.publicKey); // PEM-encoded public key
```

---

### 🌐 Make a Verified Request

```ts
const response = await client
  .get("https://jsonplaceholder.typicode.com/posts", {
    headers: {
      "X-TEST": "SUPER_HUMAN",
    },
  })
  .redact("res:body:userId");

console.log(response.proof); // cryptographic proof
console.log(response.status); // HTTP status
```


---

## 🛡 License

MIT © 2025 — UsherLabs
