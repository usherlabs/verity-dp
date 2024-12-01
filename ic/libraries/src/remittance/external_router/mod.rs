use crate::remittance::types::{DataModel, Event, RemittanceSubscriber, Subscriber};
use candid::{CandidType, Principal};
use ic_cdk::{api::call::RejectionCode, id};
use serde_derive::Deserialize;
use serde_json::Value;
use std::{cell::RefCell, collections::BTreeMap};

thread_local! {
    pub static REMITTANCE_CANISTER: RefCell<Option<RemittanceSubscriber>> = RefCell::default();
}

pub mod permissions;
pub type SubscriberStore = BTreeMap<Principal, Subscriber>;
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub balance: u64,
}

/// Set a value for the remittance canister registered
pub fn set_remittance_canister(remittance_principal: Principal) {
    REMITTANCE_CANISTER.with(|rc| {
        let _ = rc.borrow_mut().insert(RemittanceSubscriber {
            canister_principal: remittance_principal,
            subscribed: false,
        });
    })
}

/// Get the remittance canister registered with the DC canister
pub fn get_remittance_canister() -> RemittanceSubscriber {
    // confirm at least one remittance canister is subscribed to this pdc
    REMITTANCE_CANISTER
        .with(|rc| rc.borrow().clone())
        .expect("REMITTANCE_CANISTER_NOT_INITIALIZED")
}

/// Subscribe to the DC canister
///
/// This function is going to be called externally
/// by the remittance canister that wants to be able to get data from this canister
/// the remittance canister will call this function that exists the DC canister
pub fn subscribe() {
    // verify if this remittance canister has been whitelisted
    // set the subscribed value to true if its the same, otherwise panic
    let subscriber_principal_id = ic_cdk::caller();
    let whitelisted_remittance_canister = REMITTANCE_CANISTER
        .with(|rc| rc.borrow().clone())
        .expect("REMITTANCE_CANISTER_NOT_INITIALIZED");

    if whitelisted_remittance_canister.canister_principal != subscriber_principal_id {
        panic!("REMITTANCE_CANISTER_NOT_WHITELISTED")
    };

    REMITTANCE_CANISTER.with(|rc| {
        let _ = rc.borrow_mut().insert(RemittanceSubscriber {
            canister_principal: subscriber_principal_id,
            subscribed: true,
        });
    });
}

/// Checks is the provided canister principal is already whitelisted by this DC canister
pub fn is_subscribed(canister_principal: Principal) -> bool {
    let whitelisted_remittance_canister = REMITTANCE_CANISTER
        .with(|rc| rc.borrow().clone())
        .expect("REMITTANCE_CANISTER_NOT_INITIALIZED");

    return whitelisted_remittance_canister.canister_principal == canister_principal
        && return whitelisted_remittance_canister.canister_principal == canister_principal
            && whitelisted_remittance_canister.subscribed;
}

/// publish an array of DC state changes to the remittance canister
pub async fn publish_dc_json_to_remittance(json_data: String) -> Result<(), String> {
    // the string provided should be an array of events
    // the same format the ccamp uses to fetch events from logstore
    // probably for some reason were missed by the poller or for some other reason we need a manual provision of events
    // schema
    // NOTE: a DC canister can only perform `BalanceAdjusted` adjust operations
    // [{
    //     "event_name": "BalanceAdjusted",
    //     "canister_id": "bkyz2-fmaaa-aaaaa-qaaaq-cai",
    //     "account": "0x9C81E8F60a9B8743678F1b6Ae893Cc72c6Bc6840",
    //     "amount": 100000,
    //     "chain": "ethereum:5",
    //     "token": "0xB24a30A3971e4d9bf771BDc81435c25EA69A445c"
    // }]

    // Parse the string of data into serde_json::Value.
    let json_event: Value =
        serde_json::from_str(&json_data[..]).expect("JSON_DESERIALIZATION_FAILED");
    // Make sure the top-level JSON is an array
    let update_succesful = if let Value::Array(events) = json_event {
        let mut parsed_events: Vec<DataModel> = Vec::new();

        for event in events {
            // parse the json object gotten back into an "'Event' struct"
            let json_event: Event = serde_json::from_value(event).unwrap();
            // convert each "event" object into a data model and send it to the remittance canister
            let parsed_event: DataModel = json_event.into();
            // send this info over to the remittance canister in order to modify the balances
            parsed_events.push(parsed_event);
        }

        let dc_canister = id();
        let response = broadcast_to_remittance(&parsed_events, dc_canister)
            .or(Err("PUBLISH_FAILED".to_string()));

        response
    } else {
        Err("ERROR_PARSING_EVENT_INTO_DATAMODEL".to_string())
    };

    update_succesful
}

/// publish an array of PDC state changes to the remittance canister
pub async fn publish_pdc_json_to_remittance(json_data: String) -> Result<(), String> {
    // the string provided should be an array of events
    // the same format the ccamp uses to fetch events from logstore
    // probably for some reason were missed by the poller or for some other reason we need a manual provision of events
    // schema
    // [{
    //     "event_name": "FundsDeposited",
    //     "canister_id": "bkyz2-fmaaa-aaaaa-qaaaq-cai",
    //     "account": "0x9C81E8F60a9B8743678F1b6Ae893Cc72c6Bc6840",
    //     "amount": 100000,
    //     "chain": "ethereum:5",
    //     "token": "0xB24a30A3971e4d9bf771BDc81435c25EA69A445c"
    // }]

    // Parse the string of data into serde_json::Value.
    let json_event: Value =
        serde_json::from_str(&json_data[..]).expect("JSON_DESERIALIZATION_FAILED");
    // Make sure the top-level JSON is an array
    let update_succesful = if let Value::Array(events) = json_event {
        for event in events {
            // parse the json object gotten back into an "'Event' struct"
            let json_event: Event = serde_json::from_value(event).unwrap();
            // parse the canister_id which is a string into a principal
            let dc_canister: Principal = (&json_event.canister_id[..]).try_into().unwrap();
            // convert each "event" object into a data model and send it to the remittance canister
            let parsed_event: DataModel = json_event.into();
            // send this info over to the remittance canister in order to modify the balances
            let _ = broadcast_to_remittance(&vec![parsed_event], dc_canister);
        }
        Ok(())
    } else {
        Err("ERROR_PARSING_EVENT_INTO_DATAMODEL".to_string())
    };

    update_succesful
}

// we would use this method to publish data to the remittance model
// so when we have some new data, we would publish it to the remittance model
pub fn broadcast_to_remittance(
    events: &Vec<DataModel>,
    dc_canister: Principal,
) -> Result<(), RejectionCode> {
    let whitelisted_remittance_canister = get_remittance_canister();
    if !whitelisted_remittance_canister.subscribed {
        panic!("REMITTANCE_CANISTER_NOT_INITIALIZED")
    }

    let remittance_response: Result<(), RejectionCode> = ic_cdk::notify(
        whitelisted_remittance_canister.canister_principal,
        "update_remittance",
        (&events, dc_canister),
    );

    remittance_response
}
