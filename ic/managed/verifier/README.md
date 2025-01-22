# Verity Managed `verifier`

To learn about the Verity Verifier in detail, please refer to the [official documentation](https://docs.verity.usher.so/), specifically the [Verity Verifier](https://docs.verity.usher.so/build/verifier) section.

**The following is from the Docs as of *23rd of January 2025*.**

---

## Interface

The canister exposes the following methods as defined in the Candid interface:

```plaintext
service : {
  "greet" : (text) -> (text) query;
  "verify_proof_direct" : (proof_requests : vec text, notary_pub_key : text) -> (DirectVerificationResult);
  "verify_proof_async" : (proof_requests : vec text, notary_pub_key : text) -> (ProofVerificationResponse);
  "public_key" : () -> (record { sec1_pk : text; etherum_pk : text });
};
```

## Methods

### `greet`

- **Signature**: `(text) -> (text) query`
- **Purpose**: A simple query method that returns a greeting message. This is typically used for testing connectivity and basic functionality.

### `verify_proof_direct`

- **Signature**: `(proof_requests: vec text, notary_pub_key: text) -> (DirectVerificationResult)`
- **Purpose**: This method is used for direct verification of proof requests. It returns a detailed verification response, including a tECDSA signature over the Merkle Root, which is essential for verification in foreign chains or off-chain environments like zkVMs.

### `verify_proof_async`

- **Signature**: `(proof_requests: vec text, notary_pub_key: text) -> (ProofVerificationResponse)`
- **Purpose**: This method is used for asynchronous verification of proof requests. It is intended for verifications that exist within the confines of the Internet Computer (IC).

### `public_key`

- **Signature**: `() -> (record { sec1_pk: text; etherum_pk: text })`
- **Purpose**: Retrieves the public key of the Canister, which includes both the SEC1 and Ethereum public keys.

## Async vs Direct Verification

### Async Verification

Async verification is optimised for IC internal usage. It:
- Performs proof verification
- Returns results directly
- No additional signatures or Merkle trees
- Efficient for Canister-to-Canister communication

### Direct Verification

Direct verification adds extra security layers for external verification:
- Performs proof verification
- Generates Merkle tree of results
- Signs the Merkle root using tECDSA
- Enables verification in foreign chains or zkVMs
- Provides cryptographic proof of verification that can be validated outside the IC

## TLS Proof Verification Process

The Smart Contract (Canister) verifies TLS proofs either partially or fully, depending on the type of proof submitted. The verification process involves:

### Session Proofs

Session proofs verify the TLS session establishment:
- Validates handshake parameters
- Verifies server identity
- Confirms session key establishment
- Validates the session proof against notary's signature
- Returns a hash of the verified proof

### Full Proofs

Full proofs verify complete TLS sessions:
- Validates the full TLS proof including:
  - Session establishment
  - HTTP request data
  - HTTP response data
- Verifies data integrity
- Confirms server responses
- Enables verification of specific HTTP interactions

## Merkle Tree Generation

**For direct verification**, results are organised in a Merkle tree:
- Each proof response is hashed using SHA-256
- Hashes become leaves in the Merkle tree
- Tree root is computed
- Root is signed using the canister's tECDSA key
- Both root and signature are returned for external verification

## Security Considerations

1. Always validate the notary's public key before verification
2. For cross-chain verification, ensure the IC Verifier Canister's public key is properly registered
3. When using direct verification, validate both the Merkle proof via `verify-local` module, and the tECDSA signature
4. Ensure proof requests are properly formatted JSON strings
5. Handle potential errors in proof verification gracefully

## Technical Implementation Details

The verification process utilises several key components:

1. **Proof Parsing**:
   - JSON proofs are validated for required fields
   - Proofs are categorised as Session or Full proofs
   - Invalid proofs are rejected early in the process

2. **Cryptographic Operations**:
   - SHA-256 for hashing proof responses
   - tECDSA for signing Merkle roots
   - Merkle tree construction for batch verification

3. **State Management**:
   - Canister maintains its configuration state
   - ECDSA key management through IC system APIs
   - Thread-local storage for configuration

## Error Handling

The canister provides detailed error messages for common failure scenarios:

- Invalid proof format
- Invalid notary signature
- Failed TLS verification
- Invalid handshake parameters
- ECDSA signing failures

## Performance Considerations

1. **Batch Processing**:
   - Multiple proofs can be verified in a single call
   - Merkle tree construction optimises for batch verification

2. **Resource Usage**:
   - Proof verification is computationally intensive
   - Consider batch sizes carefully
   - Async verification has lower overhead

3. **Network Interaction**:
   - Direct verification requires additional cycles for ECDSA operations
   - Plan for slightly longer execution times with direct verification

## Example `verify_proof_async` Integration

The following integration is adopted from the [IC-ADC repository](https://github.com/usherlabs/ic-adc). It demonstrates how to use the `verify_proof_async` method for proof verification within the Internet Computer (IC) environment. The example includes defining the Candid interface, implementing the async verification call, and handling the verification response.

In your Rust code, you can use the `ic_cdk::call` function to make an async call to the Verifier Canister. Here's how you can do it:

```rust
pub async fn request_proof_verification(
    stringified_proofs: &Vec<String>,
    notary_pubkey: &String,
) -> Vec<ProofResponse> {
    let verifier_canister = state::get_verifier_canister().unwrap();

    // make a request to the managed verifier canister
    // to get a response which would contain the verified/decrypted proofs sent
    let (response,): (Vec<ProofResponse>,) = ic_cdk::call(
        verifier_canister,
        "verify_proof_async",
        (stringified_proofs, notary_pubkey),
    )
    .await
    .unwrap();

    response
```

In this snippet, the `request_proof_verification` function makes an async call to the Verifier Canister using the `verify_proof_async` method. It sends the proofs and the notary public key, and awaits the response.

The response is a vector of `ProofResponse` objects, which contain the verified proofs.
The async verification process does not utilize a Merkle tree, as it is intended to operate independently within the IC environment.

## Example `verify_proof_direct` Integration

The following integration is adopted from the example zkTLS flow in the Verity DP repository. It demonstrates how the `verify_proof_direct` interface is used under the hood by the [`verify-remote`](https://github.com/usherlabs/verity-dp/tree/main/rs/verify-remote) module to precompute over public facets of the TLS proof before sending the remote proof and the private facets of the TLS proof to a zkVM for verification via the [`verify-local`](https://github.com/usherlabs/verity-dp/tree/main/rs/verify-remote). In this dynamic, the Verifier will respond with a tECDSA signed Merkle Root hash representing each of the verifications and precomputes performed so that the zkVM can verify that the TLS proofs were prepared in part by a set of honest actors facilitated by the Internet Computer.

In your Rust code, you can use the `ic_cdk::call` function to make a direct call to the Verifier Canister. Here's how you can do it:

```rust
    let rv_identity_path = "fixtures/identity.pem";
    let rv_id = DEFAULT_VERITY_VERIFIER_ID.to_string();
    let rv_config = Config::new(
        DEFAULT_IC_GATEWAY_LOCAL.to_string(),
        rv_identity_path.to_string(),
        rv_id,
    );

    // 2. Create verifier from a config file
    let remote_verifier = Verifier::from_config(&rv_config).await.unwrap();

    // 3. Extract our the public/private sub-proofs
    let proof_value: serde_json::Value = serde_json::from_str(&response.proof).unwrap();
    let session = proof_value["session"].to_string();

    // 4. Verify a proof and get the response
    let verified_by_remote = remote_verifier
        .verify_proof(
            // You can verify multiple proofs at once
            vec![session],
            notary_pub_key,
        )
        .await
        .unwrap();

    // Assuming `verified_by_remote` is of type `VerifierResponse` and has a field `results`
    // which is a vector of some type that has a method `get_content()`.
    let leaves: Vec<String> = verified_by_remote
        .results
        .iter()
        .map(|proof_response| proof_response.get_content())
        .collect();

    // Create a `RemoteVerificationProof` instance
    let remote_verifier_proof = RemoteVerificationProof {
        results: leaves,
        root: verified_by_remote.root.clone(),
        signature: verified_by_remote.signature.clone(),
    };

    println!("\nverified_by_remote: {:#?}", remote_verifier_proof);

    // Now we have a proof of remote verification... We can use this to verify the private transcript data within the zkVM
    // ? The reason to split the proofs is becuase the crypto primitives used for session verification are not compatible zkVM and/or dramatically increase ZK proving times.

    // Start with the remote verifier's ECDSA public key
    let remote_verifier_public_key = remote_verifier.get_public_key().await.unwrap();

    // To do this, we need to seralize the data we pass to the zkVM
    let input = serde_json::to_string(
        &(ZkInputParam {
            tls_proof: response.proof.clone(),
            remote_verifier_proof: serde_json::to_string(&remote_verifier_proof).unwrap(),
            remote_verifier_public_key,
        }),
    )
    .unwrap();
    let input: &[u8] = input.as_bytes();

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
```
