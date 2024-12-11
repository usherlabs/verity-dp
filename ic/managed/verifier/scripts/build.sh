#!/bin/bash
set -ex

# sudo apt-get install binaryen
# cargo install wasi2ic

# cargo build --target wasm32-unknown-unknown --release -p ic_af --locked
# wasi2ic ./target/wasm32-unknown-unknown/release/ic_af.wasm ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm
# wasm-opt -Os -o ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm \
#         ./target/wasm32-unknown-unknown/release/ic_af-ic.wasm

# export RUSTFLAGS=$RUSTFLAGS' -C target-feature=+simd128'
cargo build --target wasm32-wasi --release -p verity-ic-verifier
wasi2ic ../../../target/wasm32-wasi/release/verity_ic_verifier.wasm ../../../target/wasm32-wasi/release/verity_ic_verifier_ic.wasm
wasm-opt -Os -o ../../../target/wasm32-wasi/release/verity_ic_verifier_ic.wasm \
        ../../../target/wasm32-wasi/release/verity_ic_verifier_ic.wasm