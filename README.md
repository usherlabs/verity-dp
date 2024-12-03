# Verity Data Processor (VDP)

_by [Usher Labs](https://www.usher.so)_

For detailed documentation on the architecture of Verity, please refer to the [Verity Documentation](https://docs.usher.so/verity/introduction).

## History

Verity Data Processor (VDP) is an evolution of Usher Labs' Cross-chain Asset Manager Protocol (CCAMP) originally developed for the Internet Computer (IC).  
To learn more about the CCAMP, please refer to the original [README](./ic/canisters/asset_manager/README.md).

The evolution essentially involves extrapolating the logic within the CCAMP that enables the IC as a viable cross-chain orchestrator and co-processor, and modularising the logic into libraries and components that comprise the framework.
To enhance the throughput of data sourcing and indexing for the IC, the VDP opts to leverage a highly concurrent zkTLS protocol for sourcing data.

## Verity: A zkTLS protocol

It's differentiated in that it's:

1. Server-side only, and therefore designed for business centric processes and applications
2. Designed for high frequency nad concurrent proof generation
3. Results in zkProofs that can verify within any on-chain environment
4. Maintains data privacy and security while still enabling verifiable data processing.

In summary, you can think of **Verity as a zkRollup for TLS attestations**.

This repository contains the framework for the data processing component of the Verity Protocol.

## VDP Framework

The framework is separated into the following modules:

1. `ic`: This folder contains all of the libraries and pre-built cross-chain asset manager canisters for the Internet Computer.
2. `rs`: This folder contains the Rust libraries abstract logic away from the guaranteeing verifiability across the zkVM host and guest environments
   1. `verity-client`: Verity Client is a Rust SDK for interfacing with a Verity Prover. Usher Labs is manages a Verity Prover minimising streamlining zkTLS proof generation. To learn more, please [get in touch via Discord](https://go.usher.so/discord).
   2. `remote-verify`: This library is used for sending TLS proofs/attestations to the IC for **partial** OR **full** verification.
      - Partial verification is necessary when leveraging the zkVM. The zkTLS protocol maintain high performance by partially verifying proofs in a replicated compute platform to minimise ZK proving times.
      - Full verification is ONLY necessary when the data processed is public, and there is no need for using the zkVM. This is highly unlikely for most cases as the zkVM is designed to not only maintain data privacy, but also rollup various TLS attestations into a single succinct proof of post-processing.
   3. `local-verify`: This library is used to complete the partial verification performed by the IC, but within the zkVM guest environment.
3. `evm`: These Smart Contracts are templates for integrating various chains in an app-specific multi-chain protocol powered by the IC and Verity.
4. `zk`: These libraries are utilities that can support zkVM usage.

## Framework SDK Reference

For detailed documentation on the Verity Data Processor Framework SDK, please refer to the ~SDK Reference~ (_Coming soon..._).

## Why Internet Computer?

The Internet Computer (IC) is blockchain first, but more improtantly a replicated decentralised compute environment that supports verifiable data portability.
In essence, the IC can process data and then produce a proof that some honest majority of actors all attest to the same outcome of this data processing.
Furthermore, this data processing can be a WASM, or WASI binary - which allows for quite alot of flexibility.

The only downside, like with many other blockchains and decentralised compute platforms, is that data processed is assumed to be public.
Within Verity, we leverage the IC to power our decentralised Verifier component.

If you're interested in working with the IC, we advise reviewing their documentation here: https://internetcomputer.org/docs.

You can use the Verity DP alongside the IC to source arbitrary data with all the cost-efficiencies without hampering high concurrency and frequency.
Verity aims to cust the cost of data sourcing for IC applications by approximately 60 - 80% as opposed to their default Oracle mechanism.

## Why zkVM?

For applications that must maintain data privacy, the Verity DP opts to use a zkVM for data processing.
Verity DP includes libraries that help abstract what should take place on the host and guest, to ensure full verifiability of data.

In essence, Verity is an Oracle for the zkVM.

[RiscZero](https://www.risczero.com/) is the most production grade zkVM, and is designed to work on CPUs for smaller workloads as opposed to it's counterparts.
RiscZero zkVM is the choice for Verity.
Furthermore, RiscZero ensures stateless Rust development to a limit, providing data processing capabilities and a portable succinct proof format for verification across various environments, whether on- or off-chain.
