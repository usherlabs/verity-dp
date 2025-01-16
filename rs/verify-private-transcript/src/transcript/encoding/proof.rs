use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    connection::TranscriptLength,
    hash::{Blinded, Blinder, HashAlgorithmExt, HashProviderError},
    merkle::{MerkleError, MerkleProof},
    transcript::{
        encoding::{
            new_encoder, tree::EncodingLeaf, Encoder, EncodingCommitment, MAX_TOTAL_COMMITTED_DATA,
        },
        Direction, PartialTranscript, Subsequence,
    },
    CryptoProvider,
};

/// An opening of a leaf in the encoding tree.
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Opening {
    pub(super) direction: Direction,
    pub(super) seq: Subsequence,
    pub(super) blinder: Blinder,
}

/// An opening of a leaf in the encoding tree.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialOpening {
    pub direction: Direction,
    pub seq: Subsequence,
}

opaque_debug::implement!(Opening);

/// An encoding proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodingProof {
    pub(super) inclusion_proof: MerkleProof,
    pub(super) openings: HashMap<usize, Opening>,
}

impl EncodingProof {
    /// Verifies the proof against the commitment.
    ///
    /// Returns the partial sent and received transcripts, respectively.
    ///
    /// # Arguments
    ///
    /// * `transcript_length` - The length of the transcript.
    /// * `commitment` - The encoding commitment to verify against.
    pub fn verify_with_provider(
        self,
        provider: &CryptoProvider,
        transcript_length: &TranscriptLength,
        commitment: &EncodingCommitment,
    ) -> Result<PartialTranscript, EncodingProofError> {
        let hasher = provider.hash.get(&commitment.root.alg)?;

        let seed: [u8; 32] = commitment.seed.clone().try_into().map_err(|_| {
            EncodingProofError::new(ErrorKind::Commitment, "encoding seed not 32 bytes")
        })?;

        let encoder = new_encoder(seed);
        let Self {
            inclusion_proof,
            openings,
        } = self;
        let (sent_len, recv_len) = (
            transcript_length.sent as usize,
            transcript_length.received as usize,
        );

        let mut leaves = Vec::with_capacity(openings.len());
        let mut transcript = PartialTranscript::new(sent_len, recv_len);
        let mut total_opened = 0u128;
        for (
            id,
            Opening {
                direction,
                seq,
                blinder,
            },
        ) in openings
        {
            // Make sure the amount of data being proved is bounded.
            total_opened += seq.len() as u128;
            if total_opened > MAX_TOTAL_COMMITTED_DATA as u128 {
                return Err(EncodingProofError::new(
                    ErrorKind::Proof,
                    "exceeded maximum allowed data",
                ))?;
            }

            // Make sure the ranges are within the bounds of the transcript
            let transcript_len = match direction {
                Direction::Sent => sent_len,
                Direction::Received => recv_len,
            };

            if seq.index().end() > transcript_len {
                return Err(EncodingProofError::new(
                    ErrorKind::Proof,
                    format!(
                        "index out of bounds of the transcript ({}): {} > {}",
                        direction,
                        seq.index().end(),
                        transcript_len
                    ),
                ));
            }

            let expected_encoding = encoder.encode_subsequence(direction, &seq);
            let expected_leaf =
                Blinded::new_with_blinder(EncodingLeaf::new(expected_encoding), blinder);

            // Compute the expected hash of the commitment to make sure it is
            // present in the merkle tree.
            leaves.push((id, hasher.hash_canonical(&expected_leaf)));

            // Union the authenticated subsequence into the transcript.
            transcript.union_subsequence(direction, &seq);
        }

        // Verify that the expected hashes are present in the merkle tree.
        //
        // This proves the Prover committed to the purported data prior to the encoder
        // seed being revealed. Ergo, if the encodings are authentic then the purported
        // data is authentic.
        inclusion_proof.verify(hasher, &commitment.root, leaves)?;

        Ok(transcript)
    }


    /// Verify with provider while providing a precompute
    ///
    /// Returns the partial sent and received transcripts, respectively.
    ///
    /// # Arguments
    ///
    /// * `transcript_length` - The length of the transcript.
    /// * `commitment` - The encoding commitment to verify against.
    /// * `encodings_precompute` - The precomputed encodings.
    pub fn verify_with_provider_with_precompute(
        self,
        provider: &CryptoProvider,
        transcript_length: &TranscriptLength,
        commitment: &EncodingCommitment,
        encodings_precompute: &Vec<Vec<u8>>
    ) -> Result<PartialTranscript, EncodingProofError> {
        let hasher = provider.hash.get(&commitment.root.alg)?;

        let Self {
            inclusion_proof,
            openings,
        } = self;
        let (sent_len, recv_len) = (
            transcript_length.sent as usize,
            transcript_length.received as usize,
        );

        if openings.len() != encodings_precompute.len() {
            return Err(EncodingProofError::new(
                ErrorKind::Encoding,
                "invalid encoding length",
            ))?;
        }

        let mut num_opened = 0;
        let mut leaves = Vec::with_capacity(openings.len());
        let mut transcript = PartialTranscript::new(sent_len, recv_len);
        let mut total_opened = 0u128;
    
        for (
            id,
            Opening {
                direction,
                seq,
                blinder,
            },
        ) in openings
        {
            // make sure the number
            // Make sure the amount of data being proved is bounded.
            total_opened += seq.len() as u128;
            if total_opened > MAX_TOTAL_COMMITTED_DATA as u128 {
                return Err(EncodingProofError::new(
                    ErrorKind::Proof,
                    "exceeded maximum allowed data",
                ))?;
            }

            // Make sure the ranges are within the bounds of the transcript
            let transcript_len = match direction {
                Direction::Sent => sent_len,
                Direction::Received => recv_len,
            };

            if seq.index().end() > transcript_len {
                return Err(EncodingProofError::new(
                    ErrorKind::Proof,
                    format!(
                        "index out of bounds of the transcript ({}): {} > {}",
                        direction,
                        seq.index().end(),
                        transcript_len
                    ),
                ));
            }

            let expected_encoding = encodings_precompute[num_opened].clone();
            let expected_leaf =
                Blinded::new_with_blinder(EncodingLeaf::new(expected_encoding), blinder);

            // Compute the expected hash of the commitment to make sure it is
            // present in the merkle tree.
            leaves.push((id, hasher.hash_canonical(&expected_leaf)));

            // Union the authenticated subsequence into the transcript.
            transcript.union_subsequence(direction, &seq);

            // increment the opened count
            num_opened += 1;
        }

        // Verify that the expected hashes are present in the merkle tree.
        //
        // This proves the Prover committed to the purported data prior to the encoder
        // seed being revealed. Ergo, if the encodings are authentic then the purported
        // data is authentic.
        inclusion_proof.verify(hasher, &commitment.root, leaves)?;

        Ok(transcript)
    }

    /// Generates a vector of partial openings with only the direction and sequence without the blinder
    /// 
    /// Returns the opening witout the blinders giving partial openings
    pub fn generate_partial_openings(&self) -> Vec<PartialOpening> {
        let openings = self.openings.clone();
        let mut partial_openings = vec![];
        for (_, opening) in openings {
            partial_openings.push(PartialOpening {
                direction: opening.direction,
                seq: opening.seq,
            });
        }

        partial_openings
    }
}

/// Error for [`EncodingProof`].
#[derive(Debug, thiserror::Error)]
pub struct EncodingProofError {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl EncodingProofError {
    pub fn new<E>(kind: ErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        Self {
            kind,
            source: Some(source.into()),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Provider,
    Commitment,
    Proof,
    Encoding
}

impl fmt::Display for EncodingProofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("encoding proof error: ")?;

        match self.kind {
            ErrorKind::Provider => f.write_str("provider error")?,
            ErrorKind::Commitment => f.write_str("commitment error")?,
            ErrorKind::Proof => f.write_str("proof error")?,
            ErrorKind::Encoding => f.write_str("Encoding error")?,
        }

        if let Some(source) = &self.source {
            write!(f, " caused by: {}", source)?;
        }

        Ok(())
    }
}

impl From<HashProviderError> for EncodingProofError {
    fn from(error: HashProviderError) -> Self {
        Self::new(ErrorKind::Provider, error)
    }
}

impl From<MerkleError> for EncodingProofError {
    fn from(error: MerkleError) -> Self {
        Self::new(ErrorKind::Proof, error)
    }
}
