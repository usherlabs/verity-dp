//!
//! The 'verify' submodule contains logic relevant to the ic verification of tlsn proofs
//! as well as relevant types involved in using the orchestrator to request data from the ic-adc
//!

use candid::Principal;
use types::ProofResponse;

pub mod types;

/// This 'verify_proofs' method is to be called by a canister
/// it would use this method to publish data to be verified to the verifying canister
/// so when we have some new data, we would publish it to the remittance model
pub async fn verify_proofs_onchain(
    proofs: Vec<String>,
    notary_public_key: String,
    verifier_principal: Principal,
) -> Vec<ProofResponse> {
    let verification_method_name = "verify_proof_async";

    let (verification_response,): (Vec<ProofResponse>,) = ic_cdk::call(
        verifier_principal,
        verification_method_name,
        (proofs, notary_public_key),
    )
    .await
    .unwrap();

    verification_response
}
