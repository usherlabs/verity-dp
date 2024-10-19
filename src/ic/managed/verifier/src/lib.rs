use candid::Principal;
use ic_cdk::{api::call::RejectionCode, caller};
use ic_cdk_macros::*;
use proof::{verify_proof_requests, VerificationResponse};
use utils::init_canister;
use verity_dp_ic::{
    crypto::{
        config::Environment,
        ecdsa::{self, ECDSAPublicKeyReply, PublicKeyReply},
        ethereum,
    },
    remittance::state::CONFIG,
};

pub mod merkle;
pub mod proof;
pub mod utils;

#[init]
fn init(env_opt: Option<Environment>) {
    init_canister(env_opt);
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
async fn verify_proof_async(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Result<(), RejectionCode> {
    let verification_response = verify_proof_requests(proof_requests, notary_pub_key).await;


    let calling_canister = caller();
    let canister_response: Result<(), RejectionCode> = ic_cdk::notify(
        calling_canister,
        "recieve_proof_verification_response",
        (&verification_response,),
    );

    canister_response
}

#[update]
async fn verify_proof_direct(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Result<VerificationResponse, String> {
    verify_proof_requests(proof_requests, notary_pub_key).await
}

#[update]
async fn public_key() -> PublicKeyReply {
    let config = crate::CONFIG.with(|c| c.borrow().clone());

    let request = ecdsa::ECDSAPublicKey {
        canister_id: None,
        derivation_path: vec![],
        key_id: config.key.to_key_id(),
    };

    let (res,): (ECDSAPublicKeyReply,) = ic_cdk::call(
        Principal::management_canister(),
        "ecdsa_public_key",
        (request,),
    )
    .await
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED {}", e.1))
    .unwrap();

    let address =
        ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");

    PublicKeyReply {
        sec1_pk: hex::encode(res.public_key),
        etherum_pk: address,
    }
}
