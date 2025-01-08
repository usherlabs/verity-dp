//! Encodings Precompute
//!
//! ## Generation
//!
//! During the verification of the `EncodingProof` within the `Transcript`
//! A bottle neck is the generation of the encodings. Thus in order to optimise the transcript verification
//! Thus the process of generating the encodings can be seperated from the `Transcript` verification process
//!
//! ## Verification
//! This is a custom verification method which takes in the precomputed encodings as a parameter
//! and uses it to perform an optimized verification since it doesnt have to compute the encodings on the fly
//! as it is provided as a parameter

use crate::{
    presentation::Presentation,
    transcript::{
        encoding::{new_encoder, Encoder, EncodingProofError, ErrorKind, PartialOpening},
        PartialTranscript, TranscriptProofError,
    },
    CryptoProvider,
};

/// Generates encodings for a given commitment_seed and openings
///
/// # Arguments
///
/// * `partial_openings` - The partial openings i.e openings without the blinder
/// * `commitment_seed` - The seed to be used to generate an encoder
pub fn generate(
    partial_opening: &Vec<PartialOpening>,
    commitment_seed: &Vec<u8>,
) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let seed: [u8; 32] = commitment_seed.clone().try_into().map_err(|_| {
        EncodingProofError::new(ErrorKind::Commitment, "encoding seed not 32 bytes")
    })?;
    let mut computed_encodings = vec![];

    // iterate through all the partial openings and generate a corresponging encoding
    for PartialOpening { direction, seq } in partial_opening {
        let encoder = new_encoder(seed);
        let expected_encoding = encoder.encode_subsequence(direction.to_owned(), seq);
        computed_encodings.push(expected_encoding);
    }

    Ok(computed_encodings)
}

/// Verifies a provided presentation along with the precomputed encodings provided
///
/// # Arguments
///
/// * `presentation` - The presentation which is to be verified
/// * `precomputed_encodings` - The precomputed encodings to be used in the verification process  
pub fn verify(
    presentation: &Presentation,
    precomputed_encodings: Option<&Vec<Vec<u8>>>,
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
        let bytes_input: Vec<u8> = fs::read("src/fixtures_data/data/example.presentation.tlsn").unwrap();
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
        let partial_transcript_response = verify(
                &presentation,
                Some(&encodings_precompute),
            )
            .unwrap();

        assert_eq!(partial_transcript_response.len_sent(), EXPECTED_LEN_SENT);
        assert_eq!(
            partial_transcript_response.len_received(),
            EXPECTED_LEN_RECEIVED
        );
    }
}
