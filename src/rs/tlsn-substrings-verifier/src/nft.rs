//! NFT/TWEET data types.

use serde::{Deserialize, Serialize};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

use crate::ZkInputParam;



/// The tweet structure gotten from the API
///
/// Containing the details about a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    /// data
    pub data: Vec<Data>,
    /// users info
    pub includes: Includes,
}

/// The data substructure of a tweet
///
/// Containing the details about a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    /// date created
    pub created_at: String,
    /// id
    pub id: String,
    /// public_metrics
    pub public_metrics: PublicMetrics,
    /// edit_history_tweet_ids
    pub edit_history_tweet_ids: Vec<String>,
    /// author_id of tweet
    pub author_id: String,
    /// tweet text
    pub text: String,
}

/// The PublicMetrics substructure of a tweet
///
/// Containing the details about a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct PublicMetrics {
    /// pub retweet_count
    pub retweet_count: u32,
    /// pub reply_count
    pub reply_count: u32,
    /// pub like_count
    pub like_count: u32,
    /// pub quote_count
    pub quote_count: u32,
    /// pub bookmark_count
    pub bookmark_count: u32,
    /// pub impression_count
    pub impression_count: u32,
}

/// The Includes substructure of a tweet
///
/// Containing the details about a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct Includes {
    /// users
    pub users: Vec<User>,
}

/// The User substructure of a tweet
///
/// Containing the details about a tweet
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    /// pub id: String,
    pub id: String,
    /// pub username: Option<String>
    pub username: Option<String>, // Optional because it's not present in every user object
    /// pub name: String,
    pub name: String,
    /// pub created_at: String,
    pub created_at: String,
}

/// Obtain the request and response from the ZKInputParam
pub fn get_http_payload(zk_params: ZkInputParam) -> (String, String) {
    let (mut sent, mut recv) = zk_params.substrings.verify(&zk_params.header).unwrap();
    
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    let request = String::from_utf8(sent.data().to_vec()).unwrap();
    let response = String::from_utf8(recv.data().to_vec()).unwrap();

    (request, response)
}

/// generate the nft payload
pub fn generate_tweet_nft_payload(response_http_string: String) -> (TokenMetadata, String) {

    let lines: Vec<&str> = response_http_string.split("\n").collect();
    // the json string is the last line in the http payload
    let json_tweet = lines.last().unwrap().to_owned();

    // get the tweet and the public metric to be stringified
    let tweet: Tweet = serde_json::from_str(json_tweet).unwrap();
    let tweet_data = tweet.data.get(0).unwrap();
    let public_metric_string = serde_json::to_string(&tweet_data.public_metrics).unwrap();

    // generate a token metadata
    let token_metadata = TokenMetadata { 
        title: Some(tweet_data.id.clone()), // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
        description: Some(tweet_data.text.clone()), // free-form description
        extra: Some(public_metric_string), // anything extra the NFT wants to store on-chain. Can be stringified JSON.
        media: None, // URL to associated media, preferably to decentralized, content-addressed storage
        media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
        copies: None, // number of copies of this set of metadata in existence when token was minted.
        issued_at: None, // ISO 8601 datetime when token was issued or minted
        expires_at: None, // ISO 8601 datetime when token expires
        starts_at: None, // ISO 8601 datetime when token starts being valid
        updated_at: None, // ISO 8601 datetime when token was last updated
        reference: None, // URL to an off-chain JSON file with more info.
        reference_hash: None, 
    };
    
    let stringified_token_metadata = serde_json::to_string(&token_metadata).unwrap();

    (token_metadata, stringified_token_metadata)
}
