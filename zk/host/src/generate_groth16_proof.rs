use alloy_sol_types::SolValue;
use anyhow::Context;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::{ default_prover, ExecutorEnv, ProverOpts, VerifierContext };

/// Generates a Groth16 proof given the input data for the zk circuit.
/// Returns a tuple (seal, journal_output) where:
/// - `seal` is the encoded proof representation
/// - `journal_output` is the output of the zk proving process
pub fn generate_groth16_proof(input: &[u8], guest_elf: &[u8]) -> (Vec<u8>, Vec<u8>) {
	// Begin the proving process
	let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
	let receipt = default_prover()
		.prove_with_ctx(env, &VerifierContext::default(), guest_elf, &ProverOpts::groth16())
		.unwrap().receipt;

	// Encode the seal using the Groth16 encoding
	let seal = groth16::encode(receipt.inner.groth16().unwrap().seal.clone()).unwrap();

	// Extract and decode the journal from the receipt
	let journal = receipt.journal.bytes.clone();
	let journal_output = <Vec<u8>>
		::abi_decode(&journal, true)
		.context("decoding journal data")
		.unwrap();

	(seal, journal_output)
}
