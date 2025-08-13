# Verity Managed *General Purpose MPC-TLS* Verifier

To learn about the Verity Verifier in detail, please refer to the [official documentation](https://docs.verity.usher.so/), specifically the [Verity Verifier](https://docs.verity.usher.so/build/verifier) section.

**The following is from the documentation as of *23 January 2025*.**

## Local Deployment

**Disclaimer:** Deployment of the canister (`ic/managed/verifier`) to mainnet is not recommended. This is to ensure compliance with licence agreements and to maintain compatibility and security with the broader Verity Network. We recommend using our Managed *General Purpose MPC-TLS* Verifier and interfacing via an inter-canister `async` calls or a wallet‑to‑IC `direct` calls for optimal security and performance.

### Prerequisites

1. **Ensure Rust is configured for the `wasm32-wasip1` target.**

```bash
rustup target add wasm32-wasip1
```

2. Install `wasi2ic`:

```bash
cargo install wasi2ic
```

3. Install `binaryen`:

   With Homebrew:

   ```bash
   brew install binaryen
   ```

   From the releases page:
   1. Download [Binaryen](https://github.com/WebAssembly/binaryen/releases) from the releases page.
   2. Extract the files: `tar -xzf binaryen-version.tar.gz`.
   3. Move the binary to your PATH: `sudo mv binaryen-version/bin/wasm-opt /usr/local/bin/`.
   4. Verify the installation: `wasm-opt --version`.

### Deployment

To deploy the canister locally, follow these steps:

1. `dfx start --clean`
2. `dfx deploy`

### Testing

1. `pnpm prep`
2. `pnpm test --run`

### Performance benchmarks

We have benchmarked the following functions to provide insight into their performance:

#### `verify_proof_async` and `verify_proof_async_batch`

- **Execution time:** Constant, regardless of input size (~2100 ms).
- **DFX cycle cost:** Approximately 550–720 cycles per byte of TLS data.

#### `verify_proof_direct` and `verify_proof_direct_batch`

- **Execution time:** Linear; approximately 3× the execution time of `verify_proof_async` plus signing time (L).
- **DFX cycle cost:** Roughly the same as `verify_proof_async` and `verify_proof_async_batch`.

### Caveats

#### `clang` dependency

**On macOS:** If you experience issues during `cargo build` where the `ring` library fails to compile, this is typically because `clang` is not found.

To resolve this:

1. Install `clang` using Homebrew.

```bash
brew install llvm
```

2. Ensure `clang` is on your `PATH`:

```bash
echo 'PATH="$(brew --prefix llvm)/bin${PATH:+:${PATH}}"; export PATH;' >> ~/.zshrc
```

#### `etherum_pk`

The `etherum_pk` field in the `PublicKeyReply` struct is the Ethereum address derived from the SEC1 public key. This is obtained using the `get_address_from_public_key` function in the `ethereum` module.

```rust
let address = ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");
```

*It should be spelled `ethereum_pk`, not `etherum_pk`.*
