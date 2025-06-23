use verity_verify_private_transcript::{
    encodings_precompute::{verify, verify_raw, EncodingsMapType},
    presentation::Presentation,
    transcript::PartialTranscript,
};

pub fn verify_proof(
    presentation_string: &String,
    notary_pub_key: &Vec<u8>,
    encodings: Option<Vec<u8>>,
) -> Result<(String, String), String> {
    let presentation: Presentation = serde_json::from_str(&presentation_string).unwrap();

    if notary_pub_key != &presentation.verifying_key().data {
        return Err("Failed to verify notary public key".into());
    }

    let encodings = encodings.map(|e| bincode::deserialize::<EncodingsMapType>(&e).unwrap());

    let mut transcript: PartialTranscript = if encodings.is_some() {
        verify(&presentation, encodings.as_ref()).unwrap()
    } else {
        verify_raw(&presentation).unwrap()
    };

    transcript.set_unauthed(b'X');

    Ok((
        String::from_utf8(transcript.sent_unsafe().to_vec()).unwrap(),
        String::from_utf8(transcript.received_unsafe().to_vec()).unwrap(),
    ))
}
