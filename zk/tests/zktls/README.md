# Verity zkTLS Demo

## Prerequisites

- [dfx](https://internetcomputer.org/docs/current/developer-docs/developer-tools/cli-tools/cli-reference/dfx-parent)
- [Verity CLI](https://github.com/usherlabs/verity) - to get access to the `verity` CLI, contact us at [Discord](https://go.usher.so/discord)
- [Rust](https://www.rust-lang.org/tools/install)

## Step 1: Start Verity Verifier

This means that we start a local IC (Internet Computer) node with Verity Verifier deployed.

To do this, we run:

1. Run `dfx start --clean`
2. Run `dfx deploy`

## Step 2: Start Verity

This involves operating a local Notary, and Prover.

1. Run `verity notary start --config ./config/notary.yaml`
2. Run `verity prover start`

## Step 3: Run the ZK host

From within the `zk/tests/zktls` directory, run:

```shell
  cargo run
```
