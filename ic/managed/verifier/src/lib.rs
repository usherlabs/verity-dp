use crate::state::CONFIG;
use candid::Principal;
use ic_cdk::storage;
use proof::{verify_and_sign_proof_requests, verify_proof_requests, DirectVerificationResponse};
use utils::init_canister;
use verity_dp_ic::{
    crypto::{
        config::{Config, Environment},
        ecdsa::{self, ECDSAPublicKeyReply, PublicKeyReply},
        ethereum,
    },
    verify::types::ProofResponse,
};

pub mod merkle;
pub mod proof;
pub mod state;
pub mod utils;

/// Initialize the canister
#[ic_cdk::init]
fn init(env_opt: Option<Environment>) {
    init_canister(env_opt);
}

/// Test function
#[ic_cdk::query]
fn ping() -> String {
    format!("Ping")
}

/// Verifies the proof; To be called by a canister
#[ic_cdk::update]
async fn verify_proof_async(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Vec<ProofResponse> {
    let verification_response = verify_proof_requests(proof_requests, notary_pub_key);

    verification_response
}

/// Verifies a proof; To be called by a user directly
/// returns a response back containing details of the verification
#[ic_cdk::update]
async fn verify_proof_direct(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Result<DirectVerificationResponse, String> {
    verify_and_sign_proof_requests(proof_requests, notary_pub_key).await
}

/// Get the public key of this canister
#[ic_cdk::update]
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

// --------------------------- upgrade hooks ------------------------- //
/// This function(hook) would be called before a contract is deleted or updated
/// Back up the state variables
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let cloned_config = CONFIG.with(|rc| rc.borrow().clone());
    storage::stable_save((cloned_config,)).unwrap()
}

/// This function(hook) would be called when a contract is restored/upgraded
/// Back up the state variables
#[ic_cdk::post_upgrade]
async fn post_upgrade() {
    let (old_config,): (Config,) = storage::stable_restore().unwrap();

    let env_opt = Some(old_config.env);
    init_canister(env_opt);
}
// --------------------------- upgrade hooks ------------------------- //
