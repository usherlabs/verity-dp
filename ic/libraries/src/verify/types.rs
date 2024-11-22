use std::{fmt::Display, vec};

use anyhow::Result;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

/// The response from the managed verifier canister
pub type VerificationCanisterResponse = Result<VerificationResponse, String>;
/// The response from the IC-ADC canister to the requesting canister
pub type ADCResponse = Result<Response, ErrorResponse>;

/// A representation of all the different price sources of the proofs
#[derive(Debug, Clone, CandidType, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum ProofTypes {
    Pyth(String),
    Redstone(String),
}

impl ProofTypes {
    /// converts a `ProofType` to a string
    pub fn to_string(&self) -> String {
        match self {
            ProofTypes::Pyth(value) => format!("{}", value),
            ProofTypes::Redstone(value) => format!("{}", value),
        }
    }
}

/// a struct which would be used to
/// communicate data requested by the ADC
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Request {
    /// the id of this request
    pub id: String,
    /// the principal of the canister which originated this request
    pub owner: Principal,
    /// a vector of strings representing the currency pair e.b ["BTC", "BTC/USDT"]
    pub pairs: Vec<String>,
    // add other proprties about the price here
    pub opts: RequestOpts,
}

/// This is the response which will be sent out by the orchestrator
/// it will be received by both the ic-adc and the requesting canister
#[derive(Deserialize, Serialize, Clone, Debug, CandidType)]
pub struct Response {
    /// the id of this request
    pub id: String,
    /// the principal of the canister which originated this request
    pub owner: Principal,
    /// the pairs to be processed, currently these are currency pairs but they will eventually be proofs
    pub pairs: Vec<CurrencyPair>,
    /// when we `convert` a request to a response, the price/proof information is not fetched yet
    /// this property indicates if the metadata information about this request has been succesfully fetched
    /// and is ready to be sent to the canister
    pub processed: bool,
}

/// A detailed error response to the id-adc and the requesting canister
/// in the event of an error
#[derive(Deserialize, Serialize, Clone, Debug, CandidType)]
pub struct ErrorResponse {
    /// the id of this request
    pub id: String,
    /// the principal of the canister which originated this request
    pub owner: Principal,
    /// A text describing the error message
    pub message: String,
}

/// a struct representing a currency pair
#[derive(Deserialize, Serialize, Clone, Debug, CandidType)]
pub struct CurrencyPair {
    /// the base currency
    pub base: Token,
    /// the quote currency
    pub quote: Option<Token>,
    /// if there is an error getting the proofs of this currency pair
    pub error: Option<String>,
    /// price derived from this currencyc pair
    pub price: Option<f64>,
    /// a string representation of the price pair "USDT/BTC"
    pub repr: String,
}

/// A representation of a token a price request is being made for
#[derive(Deserialize, Serialize, Clone, Debug, CandidType, PartialEq)]
pub struct Token {
    /// The ticker of the token e.g USDT, ETH, SOL
    pub ticker: String,
    /// The array proofs for this ticker from various sources or `ProofTypes`
    pub proofs: Option<Vec<ProofTypes>>,
}

/// A data structure used to represent the items being requested from the orchestrator
/// for now it is limited to only the 'price'
#[derive(Deserialize, Serialize, Clone, Debug, CandidType)]
pub struct RequestOpts {
    /// Boolean flag to indicate if to fetch price data for the associated/requested currency
    pub price: bool,
}

/// The response from the managed verifier canister
/// containing the proofs and a merkle root built from the proofs
/// and the canister's ECDSA signature of the merkle root
#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct VerificationResponse {
    /// A vector of string proofs wrapped in a `ProofResponse` to know the source of the proof
    pub results: Vec<ProofResponse>,
    /// The hex encoded merkle root
    pub root: String,
    /// The ECDSA signature of the merkle root
    pub signature: String,
}

/// A Proof verified on the managed verifier could either be a SessionProof and FullProof
#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum ProofResponse {
    SessionProof(String),
    FullProof(String),
}

// implementations for structs above
impl ProofResponse {
    /// Parse the HTTP response and extract the JSON response body
    pub fn get_http_response_body(&self) -> String {
        match self {
            ProofResponse::FullProof(text) => {
                let http_parts: Vec<&str> = text.split("\n\n").collect();
                let response = http_parts[1].to_string();

                let response_parts: Vec<&str> = response.split("\r\n\r\n").collect();
                let http_body = response_parts[1].to_string();

                http_body
            }
            ProofResponse::SessionProof(_) => panic!("cannot get http response for session proof"),
        }
    }
}

impl Request {
    pub fn new(
        id: String,
        owner: Principal,
        string_currency_pair: String,
        opts: RequestOpts,
    ) -> Self {
        let pairs_list: Vec<String> = string_currency_pair
            .split(',')
            .map(|s| s.trim().to_ascii_uppercase().to_string()) // Remove leading/trailing whitespace from each substring
            .collect();

        Self {
            id,
            owner,
            pairs: pairs_list,
            opts,
        }
    }
}

impl ErrorResponse {
    pub fn new(id: String, owner: Principal, message: String) -> Self {
        Self { id, owner, message }
    }
}

impl From<Request> for Response {
    fn from(request: Request) -> Self {
        let pairs: Vec<CurrencyPair> = request
            .pairs
            .iter()
            .filter_map(|pair_string| CurrencyPair::try_from(pair_string.to_owned()).ok())
            .collect();

        Self {
            id: request.id,
            owner: request.owner,
            pairs,
            processed: false,
        }
    }
}

impl Token {
    pub fn new(ticker: String) -> Self {
        Token {
            ticker,
            proofs: None,
        }
    }
}

impl TryFrom<String> for CurrencyPair {
    type Error = anyhow::Error;

    fn try_from(currency_pair: String) -> Result<Self, Self::Error> {
        //TODO: Do we need to have a whitelist of currencies we support and fail if base not included
        let currency_pair = currency_pair.to_uppercase();
        let base_and_quote: Vec<&str> = currency_pair.split('/').collect();

        if base_and_quote.len() > 2 {
            anyhow::bail!("invalid currency_pair")
        }

        let base = Token::new(base_and_quote[0].to_string());
        let quote = if base_and_quote.len() == 2 {
            Some(Token::new(base_and_quote[1].to_string()))
        } else {
            None
        };

        Ok(Self {
            base,
            quote,
            error: None,
            price: None,
            repr: currency_pair,
        })
    }
}

impl Display for CurrencyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get the base
        let base = self.base.ticker.clone();
        // get the quote of it exists otherwise return empty string
        let quote = if self.quote.is_some() {
            format!("/{}", self.quote.clone().unwrap().ticker)
        } else {
            format!("")
        };

        write!(f, "{}{}", base, quote)
    }
}

mod tests {
    use super::CurrencyPair;

    #[test]
    fn test_currency_pair_with_base() {
        let pair_string = "BTC/ETH";
        let pair = CurrencyPair::try_from(String::from(pair_string)).unwrap();

        assert_eq!(pair.base.ticker, "BTC");
        assert_eq!(pair.clone().quote.unwrap().ticker, "ETH");
        assert_eq!(pair.repr, pair_string);
        assert_eq!(pair.clone().to_string(), pair_string);
    }

    #[test]
    fn test_currency_pair_without_base() {
        let pair_string = "BTC";
        let pair = CurrencyPair::try_from(String::from(pair_string)).unwrap();

        assert_eq!(pair.base.ticker, "BTC");
        assert_eq!(pair.quote, None);
        assert_eq!(pair.repr, pair_string);
    }
}
