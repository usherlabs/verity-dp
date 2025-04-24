use crate::state::CONFIG;
use candid::Principal;
use ic_cdk::storage;
use proof::{
    verify_and_sign_proof_requests, verify_and_sign_proof_requests_batch, verify_proof_requests,
    verify_proof_requests_batch, DirectVerificationResponse, ProofBatch,
};
use utils::init_canister;
use verity_ic::{
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
const MIN_BALANCE: u128 = 100_000_000_000; // 0.1 T Cycles

fn ensure_sufficient_cycles() -> Result<(), String> {
    let balance = ic_cdk::api::canister_balance128();
    if balance < MIN_BALANCE {
        Err(format!(
            "Insufficient cycles: have {}, need at least {}",
            balance, MIN_BALANCE
        ))
    } else {
        Ok(())
    }
}

/// Initializes the canister with an optional environment configuration
#[ic_cdk::init]
fn init(env_opt: Option<Environment>) {
    init_canister(env_opt);
}

/// A simple test function that returns "Ping"
#[ic_cdk::query]
fn ping() -> String {
    format!("Ping")
}

/// Asynchronously verifies proof requests; intended for canister calls
#[ic_cdk::query]
async fn verify_proof_async(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Vec<ProofResponse> {
    let verification_response = verify_proof_requests(proof_requests, notary_pub_key);
    verification_response
}

/// Asynchronously verifies batch proof requests; intended for canister calls
#[ic_cdk::query]
async fn verify_proof_async_batch(batches: Vec<ProofBatch>) -> Vec<ProofResponse> {
    let verification_responses = verify_proof_requests_batch(batches);
    verification_responses
}

/// Asynchronously verifies proof requests; intended for direct user calls
/// Returns a detailed verification response
#[ic_cdk::update]
async fn verify_proof_direct(
    proof_requests: Vec<String>,
    notary_pub_key: String,
) -> Result<DirectVerificationResponse, String> {
    ensure_sufficient_cycles()?;
    verify_and_sign_proof_requests(proof_requests, notary_pub_key).await
}

/// Asynchronously verifies proof requests; intended for direct user calls
/// Returns a detailed verification response
#[ic_cdk::update]
async fn verify_proof_direct_batch(
    batches: Vec<ProofBatch>,
) -> Result<DirectVerificationResponse, String> {
    ensure_sufficient_cycles()?;
    verify_and_sign_proof_requests_batch(batches).await
}

/// Retrieves the public key of the canister
#[ic_cdk::update]
async fn public_key() -> PublicKeyReply {
    ensure_sufficient_cycles().unwrap();
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
    .map_err(|e| format!("ECDSA_PUBLIC_KEY_FAILED: {}\t,Error_code:{:?}", e.1, e.0))
    .unwrap();

    let address =
        ethereum::get_address_from_public_key(res.public_key.clone()).expect("INVALID_PUBLIC_KEY");

    PublicKeyReply {
        sec1_pk: hex::encode(res.public_key),
        etherum_pk: address,
    }
}

// --------------------------- upgrade hooks ------------------------- //
/// Hook called before the contract is deleted or updated
/// Backs up the state variables
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let cloned_config = CONFIG.with(|rc| rc.borrow().clone());
    storage::stable_save((cloned_config,)).unwrap()
}

/// Hook called when the contract is restored or upgraded
/// Restores the state variables
#[ic_cdk::post_upgrade]
async fn post_upgrade() {
    let (old_config,): (Config,) = storage::stable_restore().unwrap();

    let env_opt = Some(old_config.env);
    init_canister(env_opt);
}
// --------------------------- upgrade hooks ------------------------- //
