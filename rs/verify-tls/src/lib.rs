use blake3::Hasher;
use mpz_core::serialize::CanonicalSerialize;
use serde::{Deserialize, Serialize};

use crate::tlsn_core::{presentation::Presentation, CryptoProvider};

pub mod tlsn_core;

pub type NotaryPubKey = Vec<u8>;

#[derive(Serialize, Deserialize, Clone)]
pub struct PresentationBatch {
    pub notary_pub_key: NotaryPubKey,
    pub presentations: Vec<Presentation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ZkTlsProof {
    pub presentation_batches: Vec<PresentationBatch>,
    pub hash: Vec<u8>,
}

#[cfg(feature = "private-facets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    // pub host: String,
    pub sent: String,
    pub received: String,
}

#[cfg(feature = "private-facets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayloadBatch {
    pub notary_pub_key: NotaryPubKey,
    pub payloads: Vec<Payload>,
}

#[cfg(feature = "private-facets")]
pub fn verify_private_facets(
    batches: Vec<PresentationBatch>,
) -> Result<(ZkTlsProof, Vec<PayloadBatch>), Box<dyn std::error::Error>> {
    let crypto_provider = CryptoProvider::default();

    let mut presentation_batches: Vec<PresentationBatch> = Vec::new();
    let mut payload_batches: Vec<PayloadBatch> = Vec::new();
    let mut hasher = Hasher::new();

    for PresentationBatch {
        notary_pub_key,
        presentations,
    } in batches
    {
        use crate::Payload;

        let bytes = bincode::serialize(&notary_pub_key)?;
        hasher.update(&bytes);

        let mut public_presentations: Vec<Presentation> = Vec::new();
        let mut payloads: Vec<Payload> = Vec::new();

        for presentation in presentations {
            verify_notary_pub_key(&notary_pub_key, &presentation)?;

            let public_presentation = presentation.public_clone()?;

            let presentation_output = presentation.verify_private_facets(&crypto_provider)?;
            let transcript = presentation_output.transcript.unwrap();

            // transcript.set_unauthed(b'X');

            let payload = Payload {
                sent: String::from_utf8(transcript.sent_unsafe().to_vec()).unwrap(),
                received: String::from_utf8(transcript.received_unsafe().to_vec()).unwrap(),
            };

            let bytes = bincode::serialize(&public_presentation)?;
            hasher.update(&bytes);

            payloads.push(payload);
            public_presentations.push(public_presentation);
        }

        let payload_batch = PayloadBatch {
            notary_pub_key: notary_pub_key.clone(),
            payloads,
        };

        let presentation_batch = PresentationBatch {
            notary_pub_key,
            presentations: public_presentations,
        };

        presentation_batches.push(presentation_batch);
        payload_batches.push(payload_batch);
    }

    let result_proof = ZkTlsProof {
        presentation_batches,
        hash: hasher.finalize().to_bytes(),
    };

    Ok((result_proof, payload_batches))
}

#[cfg(feature = "public-facets")]
pub fn verify_public_facets(
    batches: Vec<PresentationBatch>,
) -> Result<ZkTlsProof, Box<dyn std::error::Error>> {
    let crypto_provider = CryptoProvider::default();

    let mut presentation_batches: Vec<PresentationBatch> = Vec::new();
    let mut hasher = Hasher::new();

    for PresentationBatch {
        notary_pub_key,
        presentations,
    } in batches
    {
        let bytes = bincode::serialize(&notary_pub_key)?;
        hasher.update(&bytes);

        let mut public_presentations: Vec<Presentation> = Vec::new();
        for presentation in presentations {
            verify_notary_pub_key(&notary_pub_key, &presentation)?;

            let public_presentation = presentation.public_clone()?;

            presentation.verify_public_facets(&crypto_provider)?;

            let bytes = bincode::serialize(&public_presentation)?;
            hasher.update(&bytes);

            public_presentations.push(public_presentation);
        }

        let batch = PresentationBatch {
            notary_pub_key,
            presentations: public_presentations,
        };

        presentation_batches.push(batch);
    }

    Ok(ZkTlsProof {
        presentation_batches,
        hash: hasher.finalize().to_bytes(),
    })
}

fn verify_notary_pub_key(
    notary_pub_key: &NotaryPubKey,
    presentation: &Presentation,
) -> Result<(), Box<dyn std::error::Error>> {
    if notary_pub_key != &presentation.verifying_key().data {
        return Err("Failed to verify notary public key".into());
    }

    Ok(())
}
