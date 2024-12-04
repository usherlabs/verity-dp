use verity_client::client::{ VerityClient, VerityClientConfig };

// pub const DEFAULT_PROVER_URL: &str = "http://127.0.0.1:8080";
pub const DEFAULT_PROVER_URL: &str = "https://prover.verity.usher.so";

fn main() {
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

	Ok(())
}
