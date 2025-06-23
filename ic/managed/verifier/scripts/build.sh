#!/bin/bash
set -ex

# sudo apt-get install binaryen
# cargo install wasi2ic

# Install candid-extractor to generating Candid files for Rust canisters
# https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/generating-candid
# cargo install candid-extractor

# cargo build --target wasm32-unknown-unknown --release -p ic_af --locked
# wasi2ic ./target/wasm32-unknown-unknown/release/ic_af.wasm ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm
# wasm-opt -Os -o ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm \
#         ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm

# export RUSTFLAGS=$RUSTFLAGS' -C target-feature=+simd128'
cargo build --target wasm32-wasip1 --release -p verity-ic-verifier
candid-extractor ../../../target/wasm32-wasip1/release/verity_ic_verifier.wasm > verity_verifier.did
wasi2ic ../../../target/wasm32-wasip1/release/verity_ic_verifier.wasm ../../../target/wasm32-wasip1/release/verity_ic_verifier_ic.wasm
wasm-opt -Os -o ../../../target/wasm32-wasip1/release/verity_ic_verifier_ic.wasm \
        ../../../target/wasm32-wasip1/release/verity_ic_verifier_ic.wasm
