# Verity Managed `verifier`

To learn about the Verity Verifier in detail, please refer to the [official documentation](https://docs.verity.usher.so/), specifically the [Verity Verifier](https://docs.verity.usher.so/build/verifier) section.

**The following is from the Docs as of *23rd of January 2025*.**

## Local Deployment

**Disclaimer:** Deployment of the canister (ic/managed/verifier) to the Mainnet is not advised. This is to ensure compliance with licensing agreements and to maintain security compatibility with the wider Verity Network. We recommend using our Managed Verifier and interfacing over XNET `async` call or wallet-to-IC update `direct` call for optimal security and performance.

### Prerequisites

**Ensure Rust is configured for for `wasm32-wasip1` target.**

```bash
rustup target add wasm32-wasip1
```

To deploy the canister locally, follow these steps:

1. `dfx start --clean`
2. `dfx deploy`
