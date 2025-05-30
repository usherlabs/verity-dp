// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.

use std::{ env, fs::read_to_string };

// use chrono::Local;
use methods::{ ZKVM_GUEST_ELF, ZKVM_GUEST_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv };
use verity_verify_tls::verify_proof;

#[test]
fn host_works() {
	env::set_var("RISC0_DEV_MODE", "1");

	// TODO: This profiling mechanism causes the proof generation to fail.
	// let profile_file = format!("../profile/{}.pb", Local::now().format("%Y-%m-%d_%H-%M-%S"));
	// env::set_var("RISC0_PPROF_OUT", profile_file);

	// Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
	tracing_subscriber
		::fmt()
		.with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
		.init();

	// An executor environment describes the configurations for the zkVM
	// including program inputs.
	// An default ExecutorEnv can be created like so:
	// `let env = ExecutorEnv::builder().build().unwrap();`
	// However, this `env` does not have any inputs.
	//
	// To add guest input to the executor environment, use
	// ExecutorEnvBuilder::write().
	// To access this method, you'll need to use ExecutorEnv::builder(), which
	// creates an ExecutorEnvBuilder. When you're done adding input, call
	// ExecutorEnvBuilder::build().

	let proof = read_to_string("../fixture/proof.json").unwrap();
	let notary_pub_key = read_to_string("../fixture/notary/notary.pub").unwrap();

	let verified_by_host = verify_proof(&proof, &notary_pub_key).unwrap();

	let env = ExecutorEnv::builder().write(&proof).unwrap().build().unwrap();

	// Obtain the default prover.
	let prover = default_prover();

	// Proof information by proving the specified ELF binary.
	// This struct contains the receipt along with statistics about execution of the guest
	println!("Proving...");
	println!("Proof size: {:?}", proof.len());
	println!("Notary Pub Key size: {:?}", notary_pub_key.len());
	println!("ELF size: {:?}", ZKVM_GUEST_ELF.len());
	println!("--------------------------------");
	let prove_info = prover.prove(env, ZKVM_GUEST_ELF).unwrap();

	// extract the receipt.
	let receipt = prove_info.receipt;

	let verified_by_guest: (String, String) = receipt.journal.decode().unwrap();

	assert_eq!(verified_by_guest, verified_by_host);

	// The receipt was verified at the end of proving, but the below code is an
	// example of how someone else could verify this receipt.
	receipt.verify(ZKVM_GUEST_ID).unwrap();
}
