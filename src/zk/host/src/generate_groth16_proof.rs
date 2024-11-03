use alloy_sol_types::SolValue;
use anyhow::Context;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use tlsn_substrings_verifier::ZkInputParam;

/// Given an input representing the input data to the zk circuit.
/// genenrates an output ( seal, journal_output )
/// Which is a representation of the proof(seal) and the output of the zk proving process(journal_output)
pub fn generate_groth16_proof(zk_inputs: ZkInputParam, guest_elf: &[u8]) -> (Vec<u8>, Vec<u8>) {
    // serialize the inputs to bytes to pass to the remote prover
    let input = serde_json::to_string(&zk_inputs).unwrap();
    let input: &[u8] = input.as_bytes();

    // begin the proving process
    let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            guest_elf,
            &ProverOpts::groth16(),
        )
        .unwrap()
        .receipt;

    // // Encode the seal with the selector.
    let seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();

    // // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();
    let journal_output = <Vec<u8>>::abi_decode(&journal, true)
        .context("decoding journal data")
        .unwrap();

    (seal, journal_output)
}
