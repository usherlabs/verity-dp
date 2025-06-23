use std::io::Read;

use risc0_zkvm::guest::env;
use verity_dp_zk_shared::{ZKInput, ZKOutput};
use verity_verify_private_transcript::{
    encodings_precompute::{verify, verify_raw},
    transcript::PartialTranscript,
};

fn main() {
    let mut zk_input_bytes: Vec<u8> = vec![];
    env::stdin().read_to_end(&mut zk_input_bytes).unwrap();

    let ZKInput {
        presentation,
        encodings,
    } = bincode::deserialize(&zk_input_bytes).unwrap();

    let mut partial_transcript: PartialTranscript = if encodings.is_some() {
        verify(&presentation, encodings.as_ref()).unwrap()
    } else {
        verify_raw(&presentation).unwrap()
    };

    partial_transcript.set_unauthed(b'X');

    let zk_output = ZKOutput {
        sent: String::from_utf8(partial_transcript.sent_unsafe().to_vec()).unwrap(),
        received: String::from_utf8(partial_transcript.received_unsafe().to_vec()).unwrap(),
    };

    let zk_output_bytes = bincode::serialize(&zk_output).unwrap();
    env::commit(&zk_output_bytes);
}
