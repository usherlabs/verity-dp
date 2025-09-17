use mpz_circuits::types::ValueType;
use mpz_core::serialize::CanonicalSerialize;
use mpz_garble_core::{encoding_state::Full, ChaChaEncoder, EncodedValue};

use crate::tlsn_core::transcript::{Direction, Subsequence, RX_TRANSCRIPT_ID, TX_TRANSCRIPT_ID};

pub(crate) fn new_encoder(seed: [u8; 32]) -> impl Encoder {
    ChaChaEncoder::new(seed)
}

/// A transcript encoder.
pub(crate) trait Encoder {
    /// Returns the encoding for the given subsequence of the transcript.
    ///
    /// # Arguments
    ///
    /// * `seq` - The subsequence to encode.
    fn encode_subsequence(&self, direction: Direction, seq: &Subsequence) -> Vec<u8>;

    /// Returns all possible encoding values for the given subsequence/direction of the transcript.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction of the commitment.
    /// * `seq_idx` - The index of the subsequence.
    fn generate_encoded_bytes(
        &self,
        direction: Direction,
        sub: &Subsequence,
    ) -> Vec<EncodedValue<Full>>;

    /// Returns the selected encodings for a give subsequence, index and the pre computed encodings
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction of the commitment.
    /// * `seq` - The commited sequence.
    /// * `precomputed_encodings` - The pre computed encodings.
    fn encode_subsequence_with_precompute(
        &self,
        direction: Direction,
        seq: &Subsequence,
        precomputed_encodings: Vec<EncodedValue<Full>>,
    ) -> Vec<u8>;
}

impl Encoder for ChaChaEncoder {
    fn generate_encoded_bytes(
        &self,
        direction: Direction,
        seq: &Subsequence,
    ) -> Vec<EncodedValue<Full>> {
        let id = match direction {
            Direction::Sent => TX_TRANSCRIPT_ID,
            Direction::Received => RX_TRANSCRIPT_ID,
        };

        let mut encoding: Vec<EncodedValue<Full>> = vec![];
        for byte_id in seq.index().iter() {
            let id_hash = mpz_core::utils::blake3(format!("{}/{}", id, byte_id).as_bytes());
            let id = u64::from_be_bytes(id_hash[..8].try_into().unwrap());

            let encoded_values = <ChaChaEncoder as mpz_garble_core::Encoder>::encode_by_type(
                self,
                id,
                &ValueType::U8,
            );

            encoding.push(encoded_values);
        }

        encoding
    }

    fn encode_subsequence_with_precompute(
        &self,
        _direction: Direction,
        seq: &Subsequence,
        precomputed_encodings: Vec<EncodedValue<Full>>,
    ) -> Vec<u8> {
        let mut num_opened = 0;
        let mut encoding = Vec::with_capacity(seq.len().expect("subsequence has no data") * 16);

        for (_byte_id, &byte) in seq
            .index()
            .iter()
            .zip(seq.data().expect("subsequence has no data"))
        {
            let encoded_values = precomputed_encodings[num_opened].clone();
            let selected_encoding_value = encoded_values
                .select(byte)
                .expect("encoding is a byte encoding")
                .to_bytes();

            encoding.extend(selected_encoding_value);

            num_opened += 1;
        }

        encoding
    }

    fn encode_subsequence(&self, direction: Direction, seq: &Subsequence) -> Vec<u8> {
        let id = match direction {
            Direction::Sent => TX_TRANSCRIPT_ID,
            Direction::Received => RX_TRANSCRIPT_ID,
        };

        let mut encoding = Vec::with_capacity(seq.len().expect("subsequence has no data") * 16);
        for (byte_id, &byte) in seq
            .index()
            .iter()
            .zip(seq.data().expect("subsequence has no data"))
        {
            let id_hash = mpz_core::utils::blake3(format!("{}/{}", id, byte_id).as_bytes());
            let id = u64::from_be_bytes(id_hash[..8].try_into().unwrap());

            let encoded_values = <ChaChaEncoder as mpz_garble_core::Encoder>::encode_by_type(
                self,
                id,
                &ValueType::U8,
            );

            let selected_encoding_value = encoded_values
                .select(byte)
                .expect("encoding is a byte encoding")
                .to_bytes();

            encoding.extend(selected_encoding_value)
        }

        encoding
    }
}
