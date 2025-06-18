use serde::{Deserialize, Serialize};

use verity_verify_private_transcript::{
    encodings_precompute::EncodingsMapType, presentation::Presentation,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ZKInput {
    pub presentation: Presentation,
    pub encodings: Option<EncodingsMapType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZKOutput {
    pub sent: String,
    pub received: String,    
}
