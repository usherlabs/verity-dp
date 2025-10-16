// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

/**
 * @title SimpleICPSignatureVerifier
 * @dev Simplified example of ICP signature verification without external dependencies
 * 
 * This contract demonstrates the core ICP signature verification functionality
 * without requiring OpenZeppelin contracts, making it easier to understand
 * and deploy in environments with limited dependencies.
 * 
 * Key features:
 * - ICP signature verification using ECDSA with secp256k1
 * - Basic fund management with signature-based withdrawals
 * - Replay protection using signature tracking
 * - Simple ownership model
 */
contract SimpleICPSignatureVerifier {
    // State variables
    address public owner;
    address public icpCanister;
    string public chainId;
    bool public initialized;
    
    // Track used signatures to prevent replay attacks
    mapping(bytes => bool) public usedSignatures;
    
    // Track canister balances: keccak256(principal) => amountDeposited
    mapping(bytes32 => uint256) public canisterBalances;
    
    // Events
    event FundsDeposited(string canisterId, address indexed account, uint256 amount, string chain);
    event FundsWithdrawn(string canisterId, address indexed account, uint256 amount, string chain);
    event ICPSignatureVerified(string canisterId, bytes32 messageHash, bool isValid);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event ICPCanisterUpdated(address indexed previousCanister, address indexed newCanister);

    // Modifiers
    modifier onlyOwner() {
        require(msg.sender == owner, "ONLY_OWNER");
        _;
    }
    
    modifier onlyInitialized() {
        require(initialized, "NOT_INITIALIZED");
        _;
    }

    /**
     * @dev Constructor sets the initial owner
     */
    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Initialize the contract with ICP canister address and chain ID
     * @param _icpCanister The Ethereum address derived from the ICP canister's public key
     * @param _chainId The chain identifier for this contract
     */
    function initialize(address _icpCanister, string calldata _chainId) external onlyOwner {
        require(!initialized, "ALREADY_INITIALIZED");
        require(_icpCanister != address(0), "INVALID_ICP_CANISTER");
        require(bytes(_chainId).length > 0, "INVALID_CHAIN_ID");
        
        icpCanister = _icpCanister;
        chainId = _chainId;
        initialized = true;
    }

    /**
     * @dev Deposit native tokens (ETH) into the contract for a specified canister ID
     * @param _canisterId The ICP canister ID (27 characters)
     */
    function depositTokens(string calldata _canisterId) external payable onlyInitialized {
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(msg.value > 0, "ZERO_AMOUNT");
        
        canisterBalances[keccak256(bytes(_canisterId))] += msg.value;
        emit FundsDeposited(_canisterId, msg.sender, msg.value, chainId);
    }

    /**
     * @dev Get the balance of a specified canister ID
     * @param _canisterId The ICP canister ID
     * @return balance The balance for the canister
     */
    function getBalance(string calldata _canisterId) external view returns (uint256 balance) {
        balance = canisterBalances[keccak256(bytes(_canisterId))];
    }

    /**
     * @dev Get the contract's native token (ETH) balance
     * @return The contract's ETH balance
     */
    function getContractBalance() external view returns (uint256) {
        return address(this).balance;
    }

    /**
     * @dev Verify an ICP signature against a message hash
     * @param dataHash The hash of the data to verify
     * @param signature The ICP signature to verify (65 bytes)
     * @return isValid Whether the signature is valid
     */
    function verifyICPSignature(bytes32 dataHash, bytes calldata signature) public view returns (bool isValid) {
        require(signature.length == 65, "INVALID_SIGNATURE_LENGTH");
        
        // Split signature into r, s, v components
        bytes32 r;
        bytes32 s;
        uint8 v;
        
        assembly {
            r := calldataload(signature.offset)
            s := calldataload(add(signature.offset, 0x20))
            v := byte(0, calldataload(add(signature.offset, 0x40)))
        }
        
        // Apply Ethereum message prefix
        bytes32 ethSignedMessageHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", dataHash));
        
        // Recover the signer address
        address recovered = ecrecover(ethSignedMessageHash, v, r, s);
        
        // Check if the recovered address matches the expected ICP canister
        isValid = (recovered == icpCanister);
    }

    /**
     * @dev Verify a custom message signature
     * @param message The message to verify
     * @param signature The ICP signature to verify
     * @return isValid Whether the signature is valid
     */
    function verifyCustomMessage(string calldata message, bytes calldata signature) external view returns (bool isValid) {
        bytes32 messageHash = keccak256(abi.encodePacked(message));
        isValid = verifyICPSignature(messageHash, signature);
    }

    /**
     * @dev Withdraw native tokens (ETH) for a specified canister ID with ICP signature verification
     * @param _canisterId The ICP canister ID
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount to withdraw
     * @param _signature The ICP signature authorizing the withdrawal
     * @return success Whether the withdrawal was successful
     */
    function withdrawTokens(
        string calldata _canisterId,
        uint256 _nonce,
        uint256 _amount,
        bytes calldata _signature
    ) external onlyInitialized returns (bool success) {
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(_amount > 0, "ZERO_AMOUNT");
        require(canisterBalances[keccak256(bytes(_canisterId))] >= _amount, "INSUFFICIENT_BALANCE");
        require(!usedSignatures[_signature], "SIGNATURE_ALREADY_USED");
        
        // Create the message hash that was signed by the ICP canister
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId));
        
        // Verify the ICP signature
        bool isValidSignature = verifyICPSignature(dataHash, _signature);
        require(isValidSignature, "INVALID_ICP_SIGNATURE");
        
        // Emit verification event for transparency
        emit ICPSignatureVerified(_canisterId, dataHash, isValidSignature);
        
        // Mark signature as used to prevent replay
        usedSignatures[_signature] = true;
        
        // Update balance and transfer ETH
        canisterBalances[keccak256(bytes(_canisterId))] -= _amount;
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId);
        
        (success, ) = payable(msg.sender).call{value: _amount}("");
        require(success, "TRANSFER_FAILED");
    }

    /**
     * @dev Cancel a withdrawal request for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount that was to be withdrawn
     * @param _signature The ICP signature authorizing the cancellation
     */
    function cancelWithdraw(
        string calldata _canisterId,
        uint256 _nonce,
        uint256 _amount,
        bytes calldata _signature
    ) external onlyInitialized {
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(!usedSignatures[_signature], "SIGNATURE_ALREADY_USED");
        
        // Create the message hash that was signed by the ICP canister
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId));
        
        // Verify the ICP signature
        bool isValidSignature = verifyICPSignature(dataHash, _signature);
        require(isValidSignature, "INVALID_ICP_SIGNATURE");
        
        // Emit verification event for transparency
        emit ICPSignatureVerified(_canisterId, dataHash, isValidSignature);
        
        // Mark signature as used to prevent replay
        usedSignatures[_signature] = true;
        
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId);
    }

    /**
     * @dev Transfer ownership of the contract
     * @param newOwner The address of the new owner
     */
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "INVALID_NEW_OWNER");
        require(newOwner != owner, "SAME_OWNER");
        
        address previousOwner = owner;
        owner = newOwner;
        emit OwnershipTransferred(previousOwner, newOwner);
    }

    /**
     * @dev Update the ICP canister address
     * @param _icpCanister The new ICP canister address
     */
    function setICPCanister(address _icpCanister) external onlyOwner {
        require(_icpCanister != address(0), "INVALID_ICP_CANISTER");
        require(_icpCanister != icpCanister, "SAME_ICP_CANISTER");
        
        address previousCanister = icpCanister;
        icpCanister = _icpCanister;
        emit ICPCanisterUpdated(previousCanister, _icpCanister);
    }

    /**
     * @dev Emergency function to withdraw all funds (only owner)
     * @param _recipient The address to receive the funds
     */
    function emergencyWithdraw(address _recipient) external onlyOwner {
        require(_recipient != address(0), "INVALID_RECIPIENT");
        require(address(this).balance > 0, "NO_FUNDS");
        
        uint256 amount = address(this).balance;
        (bool success, ) = payable(_recipient).call{value: amount}("");
        require(success, "TRANSFER_FAILED");
    }

    /**
     * @dev Check if a signature has been used
     * @param signature The signature to check
     * @return used Whether the signature has been used
     */
    function isSignatureUsed(bytes calldata signature) external view returns (bool used) {
        used = usedSignatures[signature];
    }

    /**
     * @dev Get contract information
     * @return _owner The contract owner
     * @return _icpCanister The ICP canister address
     * @return _chainId The chain identifier
     * @return _initialized Whether the contract is initialized
     */
    function getContractInfo() external view returns (
        address _owner,
        address _icpCanister,
        string memory _chainId,
        bool _initialized
    ) {
        _owner = owner;
        _icpCanister = icpCanister;
        _chainId = chainId;
        _initialized = initialized;
    }
}

