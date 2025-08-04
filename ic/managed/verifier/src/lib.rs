use crate::state::CONFIG;
use candid::{CandidType, Principal};
use ic_cdk::storage;
use utils::init_canister;
use verity_ic::{
    crypto::{
        config::{ensure_sufficient_cycles, Config, Environment},
        ecdsa::{self, ECDSAPublicKeyReply, PublicKeyReply},
        ethereum::{self, sign_message},
        merkle::generate_merkle_tree,
    },
    owner,
    verify::types::{PayloadBatch, PresentationBatch},
};
use verity_verify_tls::verify;

pub mod state;
pub mod utils;

#[derive(CandidType)]
pub struct VerificationResponse {
    pub payload_batches: Vec<PayloadBatch>,
    pub root: String,
    pub signature: String,
}

/// Initializes the canister with an optional environment configuration
#[ic_cdk::init]
fn init(env_opt: Option<Environment>) {
    init_canister(env_opt);
}

#[ic_cdk::update]
fn reinitialize(env_opt: Option<Environment>) {
    owner::only_owner();
    init_canister(env_opt);
}

/// A simple test function that returns "Ping"
#[ic_cdk::query]
fn ping() -> String {
    format!("Ping")
}

/// Asynchronously verifies a vector of PresentationBatches; intended for canister calls
#[ic_cdk::query]
async fn verify_async(
    presentation_batches: Vec<PresentationBatch>,
) -> Result<Vec<PayloadBatch>, String> {
    // Convert from Candid type
    let presentation_batches: Vec<verity_verify_tls::PresentationBatch> =
        presentation_batches.into_iter().map(|b| b.into()).collect();

    let (_, payload_batches) = verify(presentation_batches).map_err(|e| e.to_string())?;

    // Convert into Candid type
    let payload_batches: Vec<PayloadBatch> =
        payload_batches.into_iter().map(|b| b.into()).collect();

    Ok(payload_batches)
}

/// Asynchronously verifies proof requests; intended for direct user calls
/// Returns a detailed verification response
#[ic_cdk::update]
async fn verify_direct(
    presentation_batches: Vec<PresentationBatch>,
) -> Result<VerificationResponse, String> {
    let config = crate::CONFIG.with(|c| c.borrow().clone());
    ensure_sufficient_cycles(config.sign_cycles)?;

    // Convert from Candid type
    let presentation_batches: Vec<verity_verify_tls::PresentationBatch> =
        presentation_batches.into_iter().map(|b| b.into()).collect();

    let (_, payload_batches) = verify(presentation_batches).map_err(|e| e.to_string())?;

    let merkle_tree = generate_merkle_tree(&payload_batches);
    let merkle_root = merkle_tree.root().ok_or("NOT ENOUGH LEAVES")?;

    let signature = sign_message(&merkle_root.to_vec(), &config)
        .await?
        .signature_hex;

    // Convert into Candid type
    let payload_batches: Vec<PayloadBatch> =
        payload_batches.into_iter().map(|b| b.into()).collect();

    Ok(VerificationResponse {
        payload_batches,
        root: hex::encode(merkle_root),
        signature,
    })
}

/// Retrieves the public key of the canister
#[ic_cdk::update]
async fn public_key() -> PublicKeyReply {
    let config = crate::CONFIG.with(|c| c.borrow().clone());
    ensure_sufficient_cycles(config.sign_cycles).unwrap();

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

// Enable Candid export
ic_cdk::export_candid!();
