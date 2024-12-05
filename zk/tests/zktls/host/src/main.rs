// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{ VERITY_ZK_TESTS_ZKTLS_GUEST_ELF, VERITY_ZK_TESTS_ZKTLS_GUEST_ID };
use risc0_zkvm::{ default_prover, ExecutorEnv };
use verity_client::client::{ VerityClient, VerityClientConfig };
use verity_remote_verify::{ ic::{ Verifier, DEFAULT_IC_GATEWAY_LOCAL }, config::Config };
// use verity_dp_zk_host::generate_groth16_proof;

pub const DEFAULT_PROVER_URL: &str = "http://127.0.0.1:8080";

/// Proof that a transcript of communications took place between a Prover and Server.
#[derive(Debug, Serialize, Deserialize)]
pub struct TlsProof {
	/// Proof of the TLS handshake, server identity, and commitments to the transcript.
	pub session: String,
	/// Proof regarding the contents of the transcript.
	pub substrings: String,
}

/// Session information from TLS proof session sub-proof
#[derive(Debug, Serialize, Deserialize)]
pub struct TlsSessionProof {
	/// Session header information from TLS proof session sub-proof
	pub header: String,
}

/// The input parameters for the zk_circuit
///
/// Contains the details needed for proof verification
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZkInputParam {
	/// Session header information
	pub session_header: String,
	/// Proof of substrings
	pub substrings: String,
	/// Remote verifier's ECDSA public key
	pub remote_verifier_public_key: String,
	/// Notary Public Key
	pub notary_public_key: String,
}

#[tokio::main()]
async fn main() -> Result<(), reqwest::Error> {
	// Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
	tracing_subscriber
		::fmt()
		.with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
		.init();

	// Entry point of the program
	println!("Hello Verity zkTLS Demo!");

	// Generate a TLS attestation using Verity Client
	// Pepare the TLS proof for the guest
	// Call the zkVM guest for zkSNARK
	// Verify the SNARK

	println!("Proving a GET request using VerityClient...");

	let config = VerityClientConfig {
		prover_url: String::from(DEFAULT_PROVER_URL),
	};

	let client = VerityClient::new(config);

	let result = client
		.get("https://jsonplaceholder.typicode.com/posts/98")
		.redact(String::from("res:body:dolor"))
		.send().await;

	let response = match result {
		Ok(response) => response,
		Err(e) => {
			println!("Error: {}", e);
			return Ok(());
		}
	};

	let json: serde_json::Value = response.subject.json().await.unwrap();
	println!("json: {:#?}", json);
	println!("response.proof.len(): {:#?}", response.proof.len());

	// Get the Notary information from the Prover
	let notaryinfo = client.get_notary_info().await;
	println!("notaryinfo: {:#?}", notaryinfo);

	let notary_pub_key = match notaryinfo {
		Ok(notaryinfo) => notaryinfo.public_key,
		Err(e) => {
			println!("Error: {}", e);
			return Ok(());
		}
	};

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

	// Totally optional to verify proof on host too.
	let verified_by_host = verify_proof(&response.proof, &notary_pub_key).unwrap();

	// Perform the partial remote verification against dcentralised compute

	// 1. Create a config file by specifying the params
	let rv_identity_path = read_to_string("../fixtures/identity.pem").unwrap(); // ? To optain this identity.pem, use `dfx identity export`
	let rv_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai".to_string();
	// TODO: This should eventually be abstracted away from the user...
	let rv_config = Config::new(DEFAULT_IC_GATEWAY_LOCAL.to_string(), rv_identity_path, rv_id);

	// 2. Create verifier from a config file
	let remote_verifier = Verifier::from_config(&rv_config).await.unwrap();

	// 3. Extract our the public/private sub-proofs
	let TlsProof {
		session, // Public session and handshake data from TLS proof
		substrings, // Private transcript request/response data from TLS proof
	} = serde_json::from_str(&response.proof).unwrap();

	// 4. Verify a proof and get the response
	let verified_by_remote = remote_verifier.verify_proof(
		// You can verify multiple proofs at once
		vec![session],
		notary_pub_key
	).await;

	// Now we have a proof of remote verification... We can use this to verify the private transcript data within the zkVM
	// ? The reason to split the proofs is becuase the crypto primitives used for session verification are not compatible zkVM and/or dramatically increase ZK proving times.

	// Start with the remote verifier's ECDSA public key
	let remote_verifier_public_key = remote_verifier.get_public_key().await.unwrap();

	// To do this, we need to seralize the data we pass to the zkVM
	let TlsSession { session_header } = serde_json::from_str(&session).unwrap();
	let input = serde_json
		::to_string(
			&(ZkInputParam {
				session_header,
				substrings,
				remote_verifier_public_key,
				notary_public_key,
			})
		)
		.unwrap();
	let input: &[u8] = input.as_bytes();

	let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

	// Obtain the default prover.
	let prover = default_prover();

	// Proof information by proving the specified ELF binary.
	// This struct contains the receipt along with statistics about execution of the guest
	println!("Proving...");

	let prove_info = prover.prove(env, VERITY_ZK_TESTS_ZKTLS_GUEST_ELF).unwrap();

	// extract the receipt.
	let receipt = prove_info.receipt;

	let verified_by_guest: (String, String) = receipt.journal.decode().unwrap();

	// Assert that the proof verification within the zkVM matches the proof verification by the host
	// assert_eq!(verified_by_guest, verified_by_host);

	// The receipt was verified at the end of proving, but the below code is an
	// example of how someone else could verify this receipt.
	receipt.verify(VERITY_ZK_TESTS_ZKTLS_GUEST_ID).unwrap();

	println!("STARK proof generated and verified!");

	Ok(())
}
