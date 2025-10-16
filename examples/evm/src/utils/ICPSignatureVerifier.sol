// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

/**
 * @title ICPSignatureVerifier
 * @dev Library for verifying ICP (Internet Computer Protocol) signatures in Ethereum
 * 
 * This library provides functionality to verify signatures created by ICP canisters
 * using ECDSA with secp256k1 curve, which is compatible with Ethereum's signature scheme.
 * 
 * Key features:
 * - Verifies ICP signatures against expected canister addresses
 * - Handles Ethereum message prefix for signature verification
 * - Supports both raw message hashes and Ethereum-signed message hashes
 * - Compatible with ICP's ECDSA signature format
 * 
 * @notice ICP signatures are created using ECDSA with secp256k1 curve and are
 * compatible with Ethereum's signature verification system.
 */
library ICPSignatureVerifier {
    /**
     * @dev Verifies an ICP signature against a message hash and expected signer
     * @param signer The expected signer address (derived from ICP canister public key)
     * @param messageHash The hash of the message that was signed
     * @param signature The ICP signature to verify (65 bytes: r(32) + s(32) + v(1))
     * @return isValid True if the signature is valid and matches the expected signer
     * 
     * This function:
     * 1. Splits the signature into r, s, and v components
     * 2. Applies Ethereum message prefix to the hash
     * 3. Recovers the signer address using ecrecover
     * 4. Compares the recovered address with the expected signer
     */
    function verify(
        address signer, 
        bytes32 messageHash, 
        bytes memory signature
    ) internal pure returns (bool isValid) {
        // Validate signature length (must be 65 bytes)
        require(signature.length == 65, "INVALID_SIGNATURE_LENGTH");
        
        // Split signature into r, s, v components
        (bytes32 r, bytes32 s, uint8 v) = splitSignature(signature);
        
        // Apply Ethereum message prefix to the hash
        bytes32 ethSignedMessageHash = getEthSignedMessageHash(messageHash);
        
        // Recover the signer address from the signature
        address recovered = ecrecover(ethSignedMessageHash, v, r, s);
        
        // Check if the recovered address matches the expected signer
        return signer == recovered;
    }

    /**
     * @dev Verifies an ICP signature against raw message data (without pre-hashing)
     * @param signer The expected signer address
     * @param message The raw message that was signed
     * @param signature The ICP signature to verify
     * @return isValid True if the signature is valid
     * 
     * This function is useful when you have the raw message and want to verify
     * the signature without manually hashing the message first.
     */
    function verifyRawMessage(
        address signer,
        bytes memory message,
        bytes memory signature
    ) internal pure returns (bool isValid) {
        // Hash the raw message
        bytes32 messageHash = keccak256(message);
        
        // Verify the signature against the hashed message
        return verify(signer, messageHash, signature);
    }

    /**
     * @dev Verifies an ICP signature against a string message
     * @param signer The expected signer address
     * @param message The string message that was signed
     * @param signature The ICP signature to verify
     * @return isValid True if the signature is valid
     * 
     * This function is convenient for verifying signatures of string messages,
     * which is common in many applications.
     */
    function verifyStringMessage(
        address signer,
        string memory message,
        bytes memory signature
    ) internal pure returns (bool isValid) {
        // Encode the string message and hash it
        bytes32 messageHash = keccak256(abi.encodePacked(message));
        
        // Verify the signature against the hashed message
        return verify(signer, messageHash, signature);
    }

    /**
     * @dev Splits a 65-byte signature into r, s, and v components
     * @param sig The signature to split
     * @return r The r component of the signature (32 bytes)
     * @return s The s component of the signature (32 bytes)
     * @return v The recovery ID (1 byte)
     * 
     * ICP signatures follow the same format as Ethereum signatures:
     * - First 32 bytes: r component
     * - Next 32 bytes: s component  
     * - Last byte: recovery ID (v)
     */
    function splitSignature(bytes memory sig) internal pure returns (bytes32 r, bytes32 s, uint8 v) {
        require(sig.length == 65, "INVALID_SIGNATURE_LENGTH");

        assembly {
            /*
            First 32 bytes stores the length of the signature
            add(sig, 32) = pointer of sig + 32
            effectively, skips first 32 bytes of signature
            mload(p) loads next 32 bytes starting at the memory address p into memory
            */

            // first 32 bytes, after the length prefix
            r := mload(add(sig, 32))
            // second 32 bytes
            s := mload(add(sig, 64))
            // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(sig, 96)))
        }

        // implicitly return (r, s, v)
    }

    /**
     * @dev Applies Ethereum message prefix to a message hash
     * @param _messageHash The message hash to prefix
     * @return The Ethereum signed message hash
     * 
     * Ethereum signatures are created by signing a keccak256 hash with the following format:
     * "\x19Ethereum Signed Message\n" + len(msg) + msg
     * 
     * This function applies the same prefix that Ethereum uses for message signing,
     * making ICP signatures compatible with Ethereum's signature verification.
     */
    function getEthSignedMessageHash(bytes32 _messageHash) internal pure returns (bytes32) {
        /*
        Signature is produced by signing a keccak256 hash with the following format:
        "\x19Ethereum Signed Message\n" + len(msg) + msg
        */
        return keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", _messageHash));
    }

    /**
     * @dev Recovers the signer address from a signature and message hash
     * @param messageHash The message hash that was signed
     * @param signature The signature to recover the address from
     * @return recovered The recovered signer address
     * 
     * This function can be used to determine who signed a message without
     * knowing the expected signer in advance.
     */
    function recoverSigner(bytes32 messageHash, bytes memory signature) internal pure returns (address recovered) {
        require(signature.length == 65, "INVALID_SIGNATURE_LENGTH");
        
        (bytes32 r, bytes32 s, uint8 v) = splitSignature(signature);
        bytes32 ethSignedMessageHash = getEthSignedMessageHash(messageHash);
        
        recovered = ecrecover(ethSignedMessageHash, v, r, s);
    }

    /**
     * @dev Validates that a signature is properly formatted
     * @param signature The signature to validate
     * @return isValid True if the signature has the correct format
     * 
     * This function performs basic validation on the signature format
     * without attempting to recover the signer address.
     */
    function isValidSignatureFormat(bytes memory signature) internal pure returns (bool isValid) {
        // Check signature length
        if (signature.length != 65) {
            return false;
        }
        
        // Check that v is either 27 or 28 (valid recovery IDs)
        uint8 v = uint8(signature[64]);
        if (v != 27 && v != 28) {
            return false;
        }
        
        return true;
    }

    /**
     * @dev Creates a message hash for ICP signature verification
     * @param nonce The nonce to prevent replay attacks
     * @param amount The amount involved in the transaction
     * @param sender The address of the transaction sender
     * @param chainId The chain identifier
     * @param canisterId The ICP canister ID
     * @param token The token address (use address(0) for native tokens)
     * @return messageHash The hash of the encoded parameters
     * 
     * This function creates a standardized message hash for ICP signature verification
     * that includes all the parameters typically used in fund management operations.
     */
    function createMessageHash(
        uint256 nonce,
        uint256 amount,
        address sender,
        string memory chainId,
        string memory canisterId,
        address token
    ) internal pure returns (bytes32 messageHash) {
        messageHash = keccak256(abi.encodePacked(nonce, amount, sender, chainId, canisterId, token));
    }
}
