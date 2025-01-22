# Verity Data Processor (VDP)

_by [Usher Labs](https://www.usher.so)_

**For comprehensive documentation on **Verity**, please refer to the [Verity Documentation](https://docs.verity.usher.so/).**

## History on this repository

The Verity Data Processor (VDP) is an evolution of Usher Labs' Cross-chain Asset Manager Protocol (CCAMP), initially developed for the Internet Computer (IC). To learn more about CCAMP, please refer to the original [README](./ic/canisters/asset_manager/README.md).

This evolution involves extracting the logic within CCAMP that enables the IC as a cross-chain orchestrator and co-processor, and modularising it into libraries and components that form the framework. To enhance data sourcing and indexing throughput for the IC, the VDP leverages a highly concurrent zkTLS protocol.

## VDP Framework

**Refer to the [Verity Documentation](https://docs.verity.usher.so/build/vdp) for a comprehensive overview of the VDP framework.**

The VDP Framework is divided into the following modules:

- `ic`: Contains libraries and pre-built cross-chain asset manager canisters for the Internet Computer.
- `rs`: Houses Rust libraries that abstract logic for engaging Verity, and ensuring verifiability in ZK data processing.
  - `verity-client`: A Rust SDK for interfacing with a Verity Prover. Usher Labs manages a Verity Prover to streamline zkTLS proof generation, otherwise you can run your own. For more information, please contact us via Discord.
  - `verify-remote`: Used for sending TLS proofs/attestations to the IC for partial or full verification.  
    *Only required if using ZK VDPE, or preparing TLS proofs for verification and data processing directly on destination chain.*
    - **Partial verification** is necessary when leveraging the zkVM, maintaining high performance by partially verifying proofs in a replicated compute platform to minimise ZK proving times.
    - **Full verification** is only necessary when the data processed is public, which is rare as the zkVM is designed to maintain data privacy and roll up various TLS attestations into a single succinct proof.
  - `verify-local`: Performs TLS proof verification by combining remote verification of public facets, with private facets of the TLS proof. Designs specifically for zkVM guest environment.
- `evm`: Smart Contracts templates for integrating various chains in an app-specific multi-chain protocol powered by the IC and Verity.
- `examples`: Examples of how to use the Verity Data Processor Framework to generate zkTLS proofs.
- `zk`: Utilities and tests supporting zkVM usage.

## SDK Reference

For detailed documentation on the Verity Data Processor Framework SDK, please refer to the [SDK Reference](https://usherlabs.github.io/verity-dp/).

### zkTLS Demo

zkTLS acts as a cryptographic primitive that transforms a comprehensive data pipeline into a succinct proof. The Verity DP framework empowers you to develop your own verifiable data pipelines, allowing data to be sourced from any location, processed, and utilised on a blockchain in a trust-minimised manner. This significantly enhances the integrity of data-driven on-chain financial markets and liquidity management.

These verifiable data pipelines result in zkTLS proofs (using either SNARK or STARK) that can be verified across major blockchains.

For a full implementation of zkTLS, please visit the [zkTLS demo directory](./examples/zktls).

## Contributing

If you're interested in contributing, please follow the guidelines outlined in our [CONTRIBUTORS.md](./CONTRIBUTORS.md) file. Your contributions are greatly appreciated and help improve the project for everyone.

## License

This repository is licensed under the terms specified in the LICENSE file located in the base directory.

If a subdirectory contains its own LICENSE file, the license specified in that subdirectory's LICENSE file will override the base directory license for the contents of that subdirectory.
This hierarchical licensing ensures that specific components or modules within the repository can have different licensing terms as needed.

For any questions regarding the licensing terms, please refer to the respective LICENSE files or contact Usher Labs directly.

## Contact

If you have any questions or need further assistance, feel free to reach out via [Discord](https://go.usher.so/discord).
