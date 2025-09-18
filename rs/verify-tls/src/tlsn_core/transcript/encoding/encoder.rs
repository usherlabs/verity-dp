use mpz_circuits::types::ValueType;
use mpz_core::serialize::CanonicalSerialize;
use mpz_garble_core::ChaChaEncoder;

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
}

impl Encoder for ChaChaEncoder {
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
