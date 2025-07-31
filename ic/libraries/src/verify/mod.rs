use std::fmt;

use risc0_zkvm::Receipt;
use verity_verify_tls::{
    tlsn_core::presentation::PresentationError, verify_public_facets, ZkTlsProof,
};

pub mod types;

pub fn verify_receipt(
    zkvm_receipt: Vec<u8>,
    zkvm_image_id: [u32; 8],
) -> Result<Vec<u8>, ZkTlsProofError> {
    let receipt: Receipt = bincode::deserialize(&zkvm_receipt)?;
    receipt.verify(zkvm_image_id)?;

    let zkvm_result_bytes: Vec<u8> = receipt.journal.decode()?;

    let (data, mut proof): (Vec<u8>, ZkTlsProof) = bincode::deserialize(&zkvm_result_bytes)?;

    // save the original hash of the proof for further compare
    let original_hash = proof.hash.clone();

    for batch in &mut proof.presentation_batches {
        for presentation in &mut batch.presentations {
            presentation.precompute_encodings()?;
        }
    }

    let proof = verify_public_facets(proof.presentation_batches)?;

    if original_hash != proof.hash {
        return Err(ZkTlsProofError::hash_mismatch(original_hash, proof.hash));
    }

    Ok(data)
}

/// Error for [`ZkTlsProof`].
#[derive(Debug, thiserror::Error)]
pub struct ZkTlsProofError {
    kind: ErrorKind,
    source: Option<Box<dyn std::error::Error>>,
}

impl ZkTlsProofError {
    pub fn hash_mismatch(expected: Vec<u8>, received: Vec<u8>) -> Self {
        Self {
            kind: ErrorKind::HashMismatch { expected, received },
            source: None,
        }
    }
}

#[derive(Debug)]
enum ErrorKind {
    Bincode,
    Journal,
    HashMismatch {
        expected: Vec<u8>,
        received: Vec<u8>,
    },
    Presentation,
    Receipt,
}

impl fmt::Display for ZkTlsProofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ZkTlsProof error: ")?;

        match &self.kind {
            ErrorKind::Bincode => f.write_str("bincode error")?,
            ErrorKind::Journal => f.write_str("journal error")?,
            ErrorKind::HashMismatch { expected, received } => f.write_fmt(format_args!(
                "hash mismatch (expected:{expected:?} and received:{received:?})"
            ))?,
            ErrorKind::Presentation => f.write_str("presentation error")?,
            ErrorKind::Receipt => f.write_str("receipt error")?,
        }

        if let Some(source) = &self.source {
            write!(f, " caused by: {}", source)?;
        }

        Ok(())
    }
}

impl From<PresentationError> for ZkTlsProofError {
    fn from(error: PresentationError) -> Self {
        Self {
            kind: ErrorKind::Presentation,
            source: Some(Box::new(error)),
        }
    }
}

impl From<bincode::Error> for ZkTlsProofError {
    fn from(error: bincode::Error) -> Self {
        Self {
            kind: ErrorKind::Bincode,
            source: Some(error),
        }
    }
}

impl From<risc0_zkvm::serde::Error> for ZkTlsProofError {
    fn from(error: risc0_zkvm::serde::Error) -> Self {
        Self {
            kind: ErrorKind::Journal,
            source: Some(error.to_string().into()),
        }
    }
}

impl From<risc0_zkp::verify::VerificationError> for ZkTlsProofError {
    fn from(error: risc0_zkp::verify::VerificationError) -> Self {
        Self {
            kind: ErrorKind::Receipt,
            source: Some(error.to_string().into()),
        }
    }
}

impl From<Box<dyn std::error::Error>> for ZkTlsProofError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self {
            kind: ErrorKind::Presentation,
            source: Some(error),
        }
    }
}
