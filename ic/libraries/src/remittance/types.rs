use candid::{CandidType, Principal};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
};

pub type AvailableBalanceStore = HashMap<(Wallet, Chain, Wallet, Principal), Account>;
pub type WithheldBalanceStore = HashMap<(Wallet, Chain, Wallet, Principal, u64), WithheldAccount>;
pub type WithheldAmountsStore = HashMap<(Wallet, Chain, Wallet, Principal), Vec<u64>>;
pub type RemittanceRecieptsStore = HashMap<(Principal, u64), RemittanceReciept>;
pub type CanisterBalanceStore = HashMap<(Wallet, Chain, Principal), Account>;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    pub balance: u64,
}
impl Default for Account {
    fn default() -> Self {
        return Self { balance: 0 };
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WithheldAccount {
    pub balance: u64,
    pub signature: String,
    pub nonce: u64,
}
impl Default for WithheldAccount {
    fn default() -> Self {
        return Self {
            balance: 0,
            signature: String::from(""),
            nonce: 0,
        };
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct RemittanceReply {
    pub hash: String,
    pub signature: String,
    pub nonce: u64,
    pub amount: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RemittanceReciept {
    pub token: String,
    pub chain: String,
    pub amount: u64,
    pub account: String,
    pub timestamp: u64,
}
impl Default for RemittanceReciept {
    fn default() -> Self {
        return Self {
            amount: 0,
            timestamp: 0,
            token: String::from(""),
            chain: String::from(""),
            account: String::from(""),
        };
    }
}

#[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Hash, Eq)]
pub struct Wallet {
    pub address: Vec<u8>,
}
impl TryFrom<String> for Wallet {
    type Error = String;
    fn try_from(address: String) -> Result<Self, Self::Error> {
        let starts_from: usize;
        if address.starts_with("0x") {
            starts_from = 2;
        } else {
            starts_from = 0;
        }

        let result = (starts_from..address.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&address[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();

        if result.len() != 20 {
            Err(String::from("INVALID_ADDRESSS_LENGTH"))
        } else {
            Ok(Self { address: result })
        }
    }
}
impl Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_address = self
            .address
            .iter()
            .map(|r| format!("{:02x}", r))
            .collect::<Vec<String>>()
            .join("")
            .to_string();

        write!(f, "0x{}", string_address)
    }
}

#[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Hash, Eq)]
pub enum Action {
    Adjust,
    Deposit,
    Withdraw,
    CancelWithdraw,
}
impl TryFrom<String> for Action {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match &value[..] {
            "WithdrawCanceled" => Ok(Self::CancelWithdraw),
            "FundsDeposited" => Ok(Self::Deposit),
            "FundsWithdrawn" => Ok(Self::Withdraw),
            "BalanceAdjusted" => Ok(Self::Adjust),
            _ => Err("INVALID_ACTION".to_string()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Hash, Eq)]
pub struct DataModel {
    pub token: Wallet,
    pub chain: Chain,
    pub amount: i64,
    pub account: Wallet,
    pub action: Action,
}
impl Display for DataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token:{}; chain:{}; amount:{};account:{};action:{:?}",
            self.token, self.chain, self.amount, self.account, self.action
        )
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Subscriber {
    pub topic: String,
}

#[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Hash, Eq)]
pub enum Chain {
    Ethereum1,
    Ethereum5,
    Polygon137,
    Icp,
}
impl TryFrom<String> for Chain {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let details: Vec<&str> = value.split(':').collect();

        let chain_name = details[0];
        let chain_id = details[1];

        let lowercase_chain_name = &chain_name.to_lowercase()[..];
        match (lowercase_chain_name, chain_id) {
            ("ethereum", "1") => Ok(Chain::Ethereum1),
            ("ethereum", "5") => Ok(Chain::Ethereum5),
            ("ethereum", "137") => Ok(Chain::Polygon137),
            ("icp", _) => Ok(Chain::Icp),
            _ => Err(String::from("INVALID CHAIN")),
        }
    }
}
impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ethereum1 => write!(f, "ethereum:1"),
            Self::Ethereum5 => write!(f, "ethereum:5"),
            Self::Polygon137 => write!(f, "polygon:137"),
            Self::Icp => write!(f, "icp"),
        }
    }
}
impl Chain {
    pub fn get_chain_details(&self) -> (String, String) {
        let chain_string = self.to_string();
        let chain_details: Vec<&str> = chain_string.split(":").collect();

        (
            String::from(chain_details[0]),
            String::from(*chain_details.get(1).unwrap_or(&"")),
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub event_name: String,
    pub canister_id: String,
    pub account: String,
    pub amount: i64,
    pub chain: String,
    pub token: String,
}

impl Into<DataModel> for Event {
    fn into(self) -> DataModel {
        DataModel {
            token: self.token.try_into().unwrap(),
            chain: self.chain.try_into().unwrap(),
            amount: self.amount as i64,
            account: self.account.try_into().unwrap(),
            action: self.event_name.try_into().unwrap(),
        }
    }
}
pub type SubscriberStore = BTreeMap<Principal, Subscriber>;

#[derive(Clone, Debug, Deserialize, CandidType, PartialEq, Hash, Eq)]
pub struct RemittanceSubscriber {
    pub canister_principal: Principal,
    pub subscribed: bool,
}
