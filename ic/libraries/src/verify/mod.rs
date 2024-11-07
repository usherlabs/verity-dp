use candid::Principal;
use ic_cdk::api::call::RejectionCode;
use types::CanisterVerificationResponse;

pub mod types;

/// This method the first of a two way process of configuring you canister for verifying proofs
/// it is to be called by a canister
/// it would use this method to publish data to be verified to the verifying canister
/// so when we have some new data, we would publish it to the remittance model
pub fn verify_proofs(
    proofs: Vec<String>,
    notary_public_key: String,
    verifier_principal: Principal,
) -> Result<(), RejectionCode> {
    let verification_method_name = "verify_proof_async";

    let verification_response: Result<(), RejectionCode> = ic_cdk::notify(
        verifier_principal,
        verification_method_name,
        (proofs, notary_public_key),
    );

    verification_response
}

/// This method is the second and last of the process of configuring you canister for verifying proofs
/// when the proofs sent above have been succesfully verified or an error occured
/// the response is sent back to the canister who made the first call
/// to a method `recieve_proof_verification_response` with a parameter
/// so this method needs to be present and available on the receiving canister for the verifying canister to respond to
pub async fn recieve_proof_verification_response(
    verification_result: CanisterVerificationResponse,
) {
    let verification_response = verification_result.unwrap();
    // perform some operations here
    println!("{:?}", verification_response);
}
