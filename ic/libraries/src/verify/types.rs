use candid::CandidType;
use serde::Deserialize;
use verity_verify_tls::NotaryPubKey;

#[derive(CandidType, Deserialize)]
pub struct PresentationBatch {
    pub notary_pub_key: NotaryPubKey,
    pub presentations: Vec<String>,
}

impl Into<verity_verify_tls::PresentationBatch> for PresentationBatch {
    fn into(self) -> verity_verify_tls::PresentationBatch {
        verity_verify_tls::PresentationBatch {
            notary_pub_key: self.notary_pub_key,
            presentations: self
                .presentations
                .iter()
                .map(|s| serde_json::from_str(&s).unwrap())
                .collect(),
        }
    }
}

#[derive(CandidType)]
pub struct Payload {
    // pub host: String,
    pub sent: String,
    pub received: String,
}

impl From<verity_verify_tls::Payload> for Payload {
    fn from(value: verity_verify_tls::Payload) -> Self {
        Self {
            sent: value.sent,
            received: value.received,
        }
    }
}

#[derive(CandidType)]
pub struct PayloadBatch {
    pub notary_pub_key: NotaryPubKey,
    pub payloads: Vec<Payload>,
}

impl From<verity_verify_tls::PayloadBatch> for PayloadBatch {
    fn from(value: verity_verify_tls::PayloadBatch) -> Self {
        Self {
            notary_pub_key: value.notary_pub_key,
            payloads: value.payloads.into_iter().map(|p| p.into()).collect(),
        }
    }
}
