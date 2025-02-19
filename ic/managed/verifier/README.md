# Verity Managed `verifier`

To learn about the Verity Verifier in detail, please refer to the [official documentation](https://docs.verity.usher.so/), specifically the [Verity Verifier](https://docs.verity.usher.so/build/verifier) section.

**The following is from the Docs as of *23rd of January 2025*.**

## Local Deployment

**Disclaimer:** Deployment of the canister (ic/managed/verifier) to the Mainnet is not advised. This is to ensure compliance with licensing agreements and to maintain security compatibility with the wider Verity Network. We recommend using our Managed Verifier and interfacing over XNET `async` call or wallet-to-IC update `direct` call for optimal security and performance.

### Prerequisites

1. **Ensure Rust is configured for for `wasm32-wasip1` target.**

```bash
rustup target add wasm32-wasip1
```

2. Install `wasi2ic`

```bash
cargo install wasi2ic
```

3. Install `binaryen`
   1. Download [Binaryen](https://github.com/WebAssembly/binaryen/releases) from the releases page.
   2. Extract the files - `tar -xzf binaryen-version.tar.gz`
   3. Move to a directory - `sudo mv binaryen-version/bin/wasm-opt /usr/local/bin/`
   4. `wasm-opt --version`

### Deployment

To deploy the canister locally, follow these steps:

1. `dfx start --clean`
2. `dfx deploy`

### Test

1. `yarn prep`
2. `yarn test`

### Caveats

#### `clang` dependency

**On macOS:** If you are experiencing issues during `cargo build` where the `ring` library fails to compile, this is typically due to `clang` not being found.

To resolve this, you can install `clang` using Homebrew. 

```bash
brew install clang llvm
```

Alternatively, you can set the following environment variables:

```bash
export WASI_SDK_PATH=/usr/local/wasi-sdk-25.0
export CC_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang"
```

#### `etherum_pk`

The `etherum_pk` field in the `PublicKeyReply` struct is the Ethereum address derived from the Sec1 public key. This is done using the `get_address_from_public_key` function in the `ethereum` module.

```rust
let address = ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");
```

*It should be spelled `ethereum_pk` and not `etherum_pk`.*
