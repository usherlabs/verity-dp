use std::io::Read;

use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

use verity_verify_private_transcript::{
    encodings_precompute::{verify, verify_raw, EncodingsMapType},
    presentation::Presentation,
    transcript::PartialTranscript,
};

#[derive(Debug, Serialize, Deserialize)]
struct ZKInput {
    presentation: Presentation,
    encodings: Option<EncodingsMapType>,
}

fn main() {
    // read the input
    let mut input_bytes: Vec<u8> = vec![];
    env::stdin().read_to_end(&mut input_bytes).unwrap();

    let ZKInput {
        presentation,
        encodings,
    } = bincode::deserialize(&input_bytes).unwrap();
    // env::log(&format!("Processing value: {:?}", presentation));

    let partial_transcript_response: PartialTranscript = if encodings.is_some() {
        verify(&presentation, encodings.as_ref()).unwrap()
    } else {
        verify_raw(&presentation).unwrap()
    };

    // write public output to the journal
    env::commit(&10);
}
