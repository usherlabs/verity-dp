# Verity Data Processor (VDP)

_by [Usher Labs](https://www.usher.so)_

For comprehensive documentation on the architecture of Verity, please refer to the [Verity Documentation](https://docs.usher.so/verity/introduction).

## History

The Verity Data Processor (VDP) is an evolution of Usher Labs' Cross-chain Asset Manager Protocol (CCAMP), initially developed for the Internet Computer (IC). To learn more about CCAMP, please refer to the original [README](./ic/canisters/asset_manager/README.md).

This evolution involves extracting the logic within CCAMP that enables the IC as a cross-chain orchestrator and co-processor, and modularising it into libraries and components that form the framework. To enhance data sourcing and indexing throughput for the IC, the VDP leverages a highly concurrent zkTLS protocol.

## Verity: A zkTLS Protocol

Verity is distinguished by its:

1. Server-side only design, tailored for business-centric processes and applications.
2. High-frequency and concurrent proof generation capabilities.
3. Ability to produce zkProofs that can be verified within any on-chain environment.
4. Maintenance of data privacy and security while enabling verifiable data processing.

In essence, **Verity functions as a zkRollup for TLS attestations**.

This repository contains the framework for the data processing component of the Verity Protocol.

## VDP Framework

The framework is divided into the following modules:

1. `ic`: Contains libraries and pre-built cross-chain asset manager canisters for the Internet Computer.
2. `rs`: Houses Rust libraries that abstract logic for engaging Verity, and ensuring verifiability in ZK data processing.
   - `verity-client`: A Rust SDK for interfacing with a Verity Prover. Usher Labs manages a Verity Prover to streamline zkTLS proof generation, otherwise you can run your own. For more information, please [contact us via Discord](https://go.usher.so/discord).
   - `remote-verify`: Used for sending TLS proofs/attestations to the IC for **partial** or **full** verification.
     - Partial verification is necessary when leveraging the zkVM, maintaining high performance by partially verifying proofs in a replicated compute platform to minimise ZK proving times.
     - Full verification is only necessary when the data processed is public, which is rare as the zkVM is designed to maintain data privacy and roll up various TLS attestations into a single succinct proof.
   - `local-verify`: Completes the partial verification performed by the IC within the zkVM guest environment.
3. `evm`: Smart Contracts templates for integrating various chains in an app-specific multi-chain protocol powered by the IC and Verity.
4. `zk`: Utilities and tests supporting zkVM usage.

### Verity Client

The Verity Client abstracts logic associated with interfacing with a Verity Prover. The Verity Prover operates as a proxy, receiving HTTP request parameters and executing these requests with assistance from the rest of the Verity protocol to generate a TLS attestation. The proof generation mechanism within the Prover is asynchronous, and the client is responsible for subscribing to proofs generated by the Prover until the proof is generated.

You can find the Verity Client integrated into our early zkTLS programs. A notable example is the [IC-ADC](https://github.com/usherlabs/ic-adc), an asset data oracle for the Internet Computer. In this program, you can review how to:

1. [Instantiate the Client](https://github.com/usherlabs/ic-adc/blob/8f28678abff9d8c098d878b83ebcc8615d442f35/orchestrator/src/helpers/verity.rs#L7)
2. [Execute a Request Using the Client](https://github.com/usherlabs/ic-adc/blob/8f28678abff9d8c098d878b83ebcc8615d442f35/orchestrator/src/handlers/price/sources/pyth.rs#L58)

Additionally, you can review our [X (Twitter) NFTs](https://github.com/usherlabs/x-twitter-nfts) prediction market mini-game to learn how Verity can guarantee the accuracy of data from the X API for use in on-chain dynamics.

Finally, you can review direct examples of the Verity Client in the [Verity Client Examples](./rs/verity-client/examples).

Currently, the Verity Client language support is as follows:

- [x] Rust
- [ ] Typescript (Coming Soon...)

> **Note:** We are actively working on expanding language support. Stay tuned for updates via [X (Twitter)](https://x.com/usher_web3) or [Discord](https://go.usher.so/discord).

### zkTLS Demo

zkTLS acts as a cryptographic primitive that transforms a comprehensive data pipeline into a succinct proof. The Verity DP framework empowers you to develop your own verifiable data pipelines, allowing data to be sourced from any location, processed, and utilised on a blockchain in a trust-minimised manner. This significantly enhances the integrity of data-driven on-chain financial markets and liquidity management.

These verifiable data pipelines result in zkTLS proofs (using either SNARK or STARK) that can be verified across major blockchains.

For a full implementation of zkTLS, please visit the [zkTLS demo directory](./zk/tests/zktls).

## Framework SDK Reference

For detailed documentation on the Verity Data Processor Framework SDK, please refer to the SDK Reference (coming soon).

## Why Internet Computer?

The Internet Computer (IC) is a blockchain-first, decentralised compute environment that supports verifiable data portability. It can process data and produce a proof that an honest majority of actors attest to the same outcome. This data processing can be a WASM or WASI binary, offering significant flexibility.

The downside, as with many blockchains, is that processed data is assumed to be public. Within Verity, the IC powers our decentralised Verifier component.

For those interested in working with the IC, we recommend reviewing their documentation [here](https://internetcomputer.org/docs).

You can use the Verity DP alongside the IC to source arbitrary data cost-effectively without compromising high concurrency and frequency. Verity aims to reduce the cost of data sourcing for IC applications by approximately 60-80% compared to their default Oracle mechanism.

## Why zkVM?

Generally applications should opt to use the zkVM for data processing, as it maintains data privacy and provides a rollup of TLS proofs/attestations into a single succinct proof. The Verity DP includes libraries that abstract complexities in managing cryptographic operations between the host and guest to ensure full data verifiability.

In essence, Verity acts as an Oracle for the zkVM.

[RiscZero](https://www.risczero.com/) is the most production-grade zkVM, designed for CPUs with smaller workloads. It is the zkVM of choice for Verity, ensuring stateless Rust development to a limit, providing data processing capabilities, and a portable succinct proof format for verification across various environments, whether on- or off-chain.

## Contributing

If you're interested in contributing, please follow the guidelines outlined in our [CONTRIBUTORS.md](./CONTRIBUTORS.md) file. Your contributions are greatly appreciated and help improve the project for everyone.

## License

This repository is licensed under the terms specified in the LICENSE file located in the base directory.

If a subdirectory contains its own LICENSE file, the license specified in that subdirectory's LICENSE file will override the base directory license for the contents of that subdirectory.
This hierarchical licensing ensures that specific components or modules within the repository can have different licensing terms as needed.

For any questions regarding the licensing terms, please refer to the respective LICENSE files or contact Usher Labs directly.

## Contact

If you have any questions or need further assistance, feel free to reach out via [Discord](https://go.usher.so/discord).
