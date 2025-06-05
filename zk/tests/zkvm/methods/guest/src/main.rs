use std::io::Read;

use risc0_zkvm::guest::env;
use verity_verify_private_transcript::{presentation::Presentation, CryptoProvider};

fn main() {
    let mut presentation_bytes: Vec<u8> = vec![];
    env::stdin().read_to_end(&mut presentation_bytes).unwrap();

    let presentation: Presentation = bincode::deserialize(&presentation_bytes).unwrap();

    let mut transcript = presentation
        .verify(&CryptoProvider::default())
        .unwrap()
        .transcript
        .unwrap();

    transcript.set_unauthed(b'X');

    let sent = String::from_utf8(transcript.sent_unsafe().to_vec()).unwrap();
    let received = String::from_utf8(transcript.received_unsafe().to_vec()).unwrap();

    env::commit(&sent);
    env::commit(&received);
}
