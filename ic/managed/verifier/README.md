# Verity Managed *General-purpose MPC-TLS* Verifier

For detailed information about the Verity Verifier, see the [official documentation](https://docs.verity.usher.so/), specifically the [Verity Verifier](https://docs.verity.usher.so/build/verifier) section.

**The following content reflects the documentation as at 23 January 2025.**

## Use the Verifier as a Dependency in Your Canister

The Verifier can now be used as a dependency in other canisters.

Update your `dfx.json` to include:

```json
{
  "canisters": {
    "<your_canister>": {
      .....
      "dependencies": [
        "verity_verifier"
      ]
    },
    "verity_verifier": {
      "type": "pull",
      "id": "yf57k-fyaaa-aaaaj-azw2a-cai"
    }
  }
}

``` 

## Local Deployment

**Disclaimer:** Deployment of this canister (`ic/managed/verifier`) to the mainnet is not recommended. This is to ensure compliance with licence agreements and to maintain compatibility and security across the broader Verity Network. We recommend using our managed *General-purpose MPC-TLS* Verifier and interfacing via inter‑canister `async` calls or wallet‑to‑IC `direct` calls for optimal security and performance.

### Prerequisites

1. **Ensure Rust is configured for the `wasm32-wasip1` target.**

```bash
rustup target add wasm32-wasip1
```

2. Install `wasi2ic`, `candid-extractor`, and `ic-wasm`:

```bash
cargo install wasi2ic candid-extractor
cargo install ic-wasm --version 0.3.5
```

### Deployment

To deploy the canister locally, follow these steps:

1. `dfx start --clean`
2. `dfx deploy`

### Testing

1. `pnpm prep`
2. `pnpm test --run`

### Performance Benchmarks

We have benchmarked the following functions to provide insight into their performance:

#### `verify_proof_async` and `verify_proof_async_batch`

- **Execution time:** Constant, regardless of input size (≈ 2,100 ms).
- **DFX cycle cost:** Approximately 550–720 cycles per byte of TLS data.

#### `verify_proof_direct` and `verify_proof_direct_batch`

- **Execution time:** Approximately linear; about 3× the execution time of `verify_proof_async`, plus signing time.
- **DFX cycle cost:** Roughly the same as `verify_proof_async` and `verify_proof_async_batch`.

### Caveats

#### Clang dependency

**On macOS:** If you encounter issues during `cargo build` where the `ring` library fails to compile, it is typically because `clang` is not found.

To resolve this:

1. Install `clang` using Homebrew.

```bash
brew install llvm
```

2. Ensure `clang` is on your `PATH`:

```bash
echo 'PATH="$(brew --prefix llvm)/bin${PATH:+:${PATH}}"; export PATH;' >> ~/.zshrc
```

#### `etherum_pk` (typo)

The `etherum_pk` field in the `PublicKeyReply` struct is the Ethereum address derived from the SEC1 public key. It is obtained using the `get_address_from_public_key` function in the `ethereum` module.

```rust
let address = ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");
```

Note: The correct field name is `ethereum_pk`, not `etherum_pk`.
