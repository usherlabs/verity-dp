use easy_hasher::easy_hasher;

/// Validates an ECDSA signature against a given public key
pub fn validate_ecdsa_signature(
    signature_hex: &String,
    message: &String,
    public_key_hex: &String,
) -> anyhow::Result<bool> {
    let signature_hex = signature_hex.replace("0x", "");
    let public_key_hex = public_key_hex.replace("0x", "");

    let recovered_key = recover_address_from_eth_signature(signature_hex, message.clone())?;
    let is_equal = recovered_key.to_lowercase() == public_key_hex.to_lowercase();

    Ok(is_equal)
}

/// Recovers an Ethereum address from an ECDSA signature and message
fn recover_address_from_eth_signature(
    metamask_signature: String,
    message: String,
) -> anyhow::Result<String> {
    let metamask_signature = hex::decode(metamask_signature)?;

    let signature_bytes: [u8; 64] = metamask_signature[0..64].try_into()?;
    let signature_bytes_64 = libsecp256k1::Signature::parse_standard(&signature_bytes)?;

    let recovery_id = metamask_signature[64];
    let recovery_id_byte = libsecp256k1::RecoveryId::parse_rpc(recovery_id)?;

    let message_bytes: [u8; 32] = hash_eth_message(message).try_into().unwrap();
    let message_bytes_32 = libsecp256k1::Message::parse(&message_bytes);

    let public_key =
        libsecp256k1::recover(&message_bytes_32, &signature_bytes_64, &recovery_id_byte)?;

    let address = get_address_from_public_key(public_key.serialize_compressed().to_vec()).unwrap();

    Ok(address)
}

/// Hashes an Ethereum message to prepare it for public key derivation
fn hash_eth_message<T: AsRef<[u8]>>(message: T) -> Vec<u8> {
    const PREFIX: &str = "\x19Ethereum Signed Message:\n";

    let message = message.as_ref();
    let len = message.len();
    let len_string = len.to_string();

    let mut eth_message = Vec::with_capacity(PREFIX.len() + len_string.len() + len);
    eth_message.extend_from_slice(PREFIX.as_bytes());
    eth_message.extend_from_slice(len_string.as_bytes());
    eth_message.extend_from_slice(message);

    easy_hasher::raw_keccak256(eth_message).to_vec()
}

/// Converts a compressed SEC1 public key (33 bytes) to an Ethereum address (20 bytes)
fn get_address_from_public_key(public_key: Vec<u8>) -> Result<String, String> {
    if public_key.len() != 33 {
        return Err("INVALID_PK_LENGTH".to_string());
    }

    let pub_key_arr: [u8; 33] = public_key[..].try_into().unwrap();
    let pub_key = libsecp256k1::PublicKey::parse_compressed(&pub_key_arr)
        .map_err(|e| format!("{}", e))?
        .serialize();

    let keccak256 = easy_hasher::raw_keccak256(pub_key[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let address: String = keccak256_hex[24..].to_string();

    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_signature() {
        let message =
            "77ad25504555a257256919c8372236844ef886c4a1b2efa157be0e1e3a26d40a".to_string();
        let public_key = "c4bb0da5d7cc269bca64a55e2149e6dc91dc7157".to_string();
        let expected_signature =
			"eeae5aee33e7ae31c84ff37dd85e1e25d8750a2b8598c67795b6246e18cb8ffe1b45b9e394b57e0b840e6d8e8b501c75a44b4580904660f11c8a435bbb8a37411c".to_string();

        let is_valid =
            validate_ecdsa_signature(&expected_signature, &message, &public_key).unwrap();

        assert!(is_valid, "invalid message or signature")
    }
}
