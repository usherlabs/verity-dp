//! Encodings Precompute
//!
//! ## Generation
//!
//! During the verification of the `EncodingProof` within the `Transcript`,
//! a bottleneck is the generation of the encodings. To optimize the transcript verification,
//! the process of generating the encodings can be separated from the `Transcript` verification process.
//!
//! ## Verification
//! This custom verification method accepts precomputed encodings as a parameter,
//! enabling optimized verification by eliminating the need to compute encodings on the fly.
//! This approach leverages the provided encodings for a more efficient verification process.

use std::collections::HashMap;

use mpz_garble_core::{encoding_state::Full, EncodedValue};

use crate::{
    presentation::Presentation,
    transcript::{
        encoding::{new_encoder, Encoder, EncodingProofError, ErrorKind},
        Direction, Idx, PartialTranscript, TranscriptProofError,
    },
    CryptoProvider,
};

/// The hashmap from direction and index to the precomputed encodings
pub type EncodingsMapType = HashMap<(Direction, Idx), Vec<EncodedValue<Full>>>;

// pub fn generate_encodings() -> {}
/// Generates encodings for a given commitment_seed and openings
///
/// # Arguments
///
/// * `partial_openings` - The partial openings i.e openings without the blinder
/// * `commitment_seed` - The seed to be used to generate an encoder
pub fn generate(
    partial_opening: &Vec<(Direction, Idx)>,
    commitment_seed: &Vec<u8>,
) -> Result<EncodingsMapType, Box<dyn std::error::Error>> {
    let mut byte_encodings: EncodingsMapType = EncodingsMapType::default();
    let seed: [u8; 32] = commitment_seed.clone().try_into().map_err(|_| {
        EncodingProofError::new(ErrorKind::Commitment, "encoding seed not 32 bytes")
    })?;

    let encoder = new_encoder(seed);

    for (direction, seq_idx) in partial_opening {
        let generated_encodings = encoder.generate_encoded_bytes(direction.clone(), &seq_idx);
        byte_encodings.insert((direction.clone(), seq_idx.clone()), generated_encodings);
    }

    return Ok(byte_encodings);
}

/// Verifies a provided presentation along with the precomputed encodings provided
///
/// # Arguments
///
/// * `presentation` - The presentation which is to be verified
/// * `precomputed_encodings` - The precomputed encodings to be used in the verification process  
pub fn verify(
    presentation: &Presentation,
    precomputed_encodings: Option<&EncodingsMapType>,
) -> Result<PartialTranscript, TranscriptProofError> {
    // use the encodings to verify the presentation's transcript in the 'ZKVM'
    let provider = CryptoProvider::default();
    let attestation_body = presentation
        .get_attestation()
        .get_attestation_bodyproof()
        .body;

    let transcript_proof = presentation.get_transcript().unwrap();
    let partial_transcript = transcript_proof.verify_with_provider_with_precompute(
        &provider,
        &attestation_body,
        precomputed_encodings,
    );

    partial_transcript
}

///
pub fn verify_raw(
    presentation: &Presentation,
) -> Result<PartialTranscript, TranscriptProofError> {
    // use the encodings to verify the presentation's transcript in the 'ZKVM'
    let provider = CryptoProvider::default();
    let attestation_body = presentation
        .get_attestation()
        .get_attestation_bodyproof()
        .body;

    let transcript_proof = presentation.get_transcript().unwrap();
    let partial_transcript = transcript_proof.verify_with_provider(
        &provider,
        &attestation_body,
    );

    partial_transcript
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::EncodingPayload;
    use std::fs;

    const EXPECTED_LEN_SENT: usize = 211;
    const EXPECTED_LEN_RECEIVED: usize = 1612;

    #[test]
    fn test_encodings_precompute() {
        // HOST
        // get the partial openings  and commitment seed on the 'host'
        let bytes_input: Vec<u8> =
            fs::read("src/fixtures_data/data/example.presentation.tlsn").unwrap();
        let presentation: Presentation = bincode::deserialize(&bytes_input).unwrap();

        let EncodingPayload {
            partial_openings,
            commitment_seed,
        } = presentation.get_encodings_compute_payload();

        // IC
        // generate the encodings on the 'internet computer'
        // `partial_openings` and `commitment_seed`` by sending them to the IC
        // for generation of the encodings
        let encodings_precompute = generate(&partial_openings, &commitment_seed).unwrap();

        // ZKVM
        // use the encodings to verify the presentation's transcript in the 'ZKVM'
        let partial_transcript_response =
            verify(&presentation, Some(&encodings_precompute)).unwrap();

        assert_eq!(partial_transcript_response.len_sent(), EXPECTED_LEN_SENT);
        assert_eq!(
            partial_transcript_response.len_received(),
            EXPECTED_LEN_RECEIVED
        );
    }
}
