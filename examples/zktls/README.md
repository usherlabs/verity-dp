# Verity zkTLS Demo

zkTLS acts as a cryptographic primitive that transforms a comprehensive data pipeline into a succinct proof. The Verity DP framework empowers you to develop your own verifiable data pipelines, allowing data to be sourced from any location, processed, and utilised on a blockchain in a trust-minimised manner. This significantly enhances the integrity of data-driven on-chain financial markets and liquidity management.

These verifiable data pipelines result in zkTLS proofs (using either SNARK or STARK) that can be verified across major blockchains.

## Prerequisites

- [dfx](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-parent)
- [Verity CLI](https://github.com/usherlabs/verity) - to get access to the `verity` CLI, contact us at [Discord](https://go.usher.so/discord)
- [Rust](https://www.rust-lang.org/tools/install)

## Step 1: Start Verity Verifier

This means that we start a local IC (Internet Computer) node with Verity Verifier deployed.

To do this, we run:

1. `cd` into the `ic/managed/verifier` directory
2. Run `dfx start --clean`
3. Run `dfx deploy`

## Step 2: Start Verity

This involves operating a local Notary, and Prover.

1. Run `verity notary start --config ./config/notary.yaml`
2. Run `verity prover start`

## Step 3: Run the ZK host

From within the `examples/zktls` directory, run:

```shell
  cargo run
```
