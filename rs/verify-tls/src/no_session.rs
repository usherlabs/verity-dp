use tlsn_core_no_session::proof::{SessionProof, TlsProof};

/// A simple verifier which reads a proof generated by `simple_prover.rs` from "proof.json", verifies
/// it and prints the verified data to the console.
pub fn verify_proof(proof: &String) -> Result<(String, String), String> {
    // Deserialize the proof
    let proof: TlsProof =
        serde_json::from_str(proof.as_str()).or(Err("INVALID PROOF".to_owned()))?;

    let TlsProof {
        // The session proof establishes the identity of the server and the commitments
        // to the TLS transcript.
        session,
        // The substrings proof proves select portions of the transcript, while redacting
        // anything the Prover chose not to disclose.
        substrings,
    } = proof;

    let SessionProof {
        // The session header that was signed by the Notary is a succinct commitment to the TLS transcript.
        header,
        // This is the server name, checked against the certificate chain shared in the TLS handshake.
        // server_name,
        ..
    } = session;

    // Verify the substrings proof against the session header.
    //
    // This returns the redacted transcripts
    let (mut sent, mut recv) = substrings
        .verify(&header)
        .or(Err("PROOF VERIFICATION FAILED".to_string()))?;

    // Replace the bytes which the Prover chose not to disclose with 'X'
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    Ok((
        String::from_utf8(recv.data().to_vec()).unwrap(),
        String::from_utf8(sent.data().to_vec()).unwrap(),
    ))
}
