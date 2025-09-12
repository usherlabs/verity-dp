use candid::Principal;
use ic_cdk::storage;
use verity_ic::{
    crypto::{
        config::{ensure_sufficient_cycles, Config, Environment},
        ecdsa::{self, ECDSAPublicKeyReply, PublicKeyReply, VerifyReceiptReply},
        ethereum::{self, sign_message},
    },
    owner,
};

use crate::{state::CONFIG, utils::init_canister};

pub mod state;
pub mod utils;

const ZKVM_IMAGE_ID: [u32; 8] = [
    1587164378, 3897773235, 1946063679, 3490198458, 3764178775, 283742902, 3039206025, 1480104475,
];

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

#[ic_cdk::update]
async fn verify_receipt(receipt: Vec<u8>) -> Result<VerifyReceiptReply, String> {
    let config = CONFIG.with(|c| c.borrow().clone());
    ensure_sufficient_cycles(config.sign_cycles)?;

    let data =
        verity_ic::verify::verify_receipt(receipt, ZKVM_IMAGE_ID).map_err(|e| e.to_string())?;

    let signature = sign_message(&data, &config)
        .await
        .map_err(|e| e.to_string())?
        .signature_hex;

    Ok(VerifyReceiptReply { data, signature })
}

/// Retrieves the public key of the canister
#[ic_cdk::update]
async fn public_key() -> PublicKeyReply {
    let config = CONFIG.with(|c| c.borrow().clone());
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
