// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";

import {ICPSignatureVerifier} from "../utils/ICPSignatureVerifier.sol";

/**
 * @title ICPSignatureVerifierExample
 * @dev Example contract demonstrating ICP signature verification for fund management
 * @notice This contract is based on the Locker contract but uses ICP signatures instead of EVM signatures
 * 
 * Key differences from the original Locker:
 * - Uses ICP signature verification instead of EVM signature verification
 * - Verifies signatures against ICP canister public keys
 * - Maintains the same fund management functionality
 * - Includes additional validation for ICP-specific signature format
 */
contract ICPSignatureVerifierExample is Initializable, UUPSUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    bool initialized;
    string public chainId;
    address public icpCanister; // ICP canister address (Ethereum address derived from ICP public key)
    address constant ZERO_ADDRESS = 0x0000000000000000000000000000000000000000;
    
    // Track used signatures to prevent replay attacks
    mapping(bytes => bool) usedSignatures;
    
    // Track canister balances: keccak256(principal) => tokenAddress => amountDeposited
    mapping(bytes32 => mapping(address => uint256)) public canisters;
    
    // Events
    event FundsDeposited(string canisterId, address indexed account, uint amount, string chain, address token);
    event FundsWithdrawn(string canisterId, address indexed account, uint amount, string chain, address token);
    event WithdrawCanceled(string canisterId, address indexed account, uint amount, string chain, address token);
    event UpdateICPCanister(address icpCanister);
    event ICPSignatureVerified(string canisterId, bytes32 messageHash, bool isValid);

    /**
     * @dev Initializes the contract with an ICP canister address and chain ID
     * @param _icpCanister The Ethereum address derived from the ICP canister's public key
     * @param _chainId The chain identifier for this contract
     */
    function initialize(address _icpCanister, string calldata _chainId) public initializer {
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();
        icpCanister = _icpCanister;
        chainId = _chainId;
        initialized = true;
    }

    /**
     * @dev Deposits native tokens (ETH) into the contract for a specified canister ID
     * @param _canisterId The ICP canister ID (27 characters)
     * Emits a `FundsDeposited` event on success
     */
    function depositTokens(string calldata _canisterId) public payable nonReentrant {
        uint256 _amount = msg.value;
        address _token = ZERO_ADDRESS;
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(_amount > 0, "amount == 0");
        canisters[keccak256(bytes(_canisterId))][_token] += _amount;
        emit FundsDeposited(_canisterId, msg.sender, _amount, chainId, _token);
    }

    /**
     * @dev Deposits ERC20 tokens into the contract for a specified canister ID
     * @param _canisterId The ICP canister ID (27 characters)
     * @param _amount The amount of tokens to deposit
     * @param _token The token contract address
     * @return success Whether the transfer was successful
     * Emits a `FundsDeposited` event on success
     */
    function depositFunds(
        string calldata _canisterId,
        uint256 _amount,
        address _token
    ) public payable nonReentrant returns (bool) {
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(_amount > 0, "amount == 0");
        canisters[keccak256(bytes(_canisterId))][_token] += _amount;
        emit FundsDeposited(_canisterId, msg.sender, _amount, chainId, _token);
        bool response = IERC20(_token).transferFrom(msg.sender, address(this), _amount);
        return response;
    }

    /**
     * @dev Sets a new address for the ICP canister
     * @param _icpCanister The new ICP canister address
     * Can only be called by the owner
     * Emits an `UpdateICPCanister` event on success
     */
    function setICPCanisterAddress(address _icpCanister) public onlyOwner {
        icpCanister = _icpCanister;
        emit UpdateICPCanister(_icpCanister);
    }

    /**
     * @dev Verifies an ICP signature against a specified data hash
     * @param dataHash The hash of the data to verify
     * @param signature The ICP signature to verify
     * @return isValid Whether the signature is valid
     * 
     * This function demonstrates ICP signature verification by:
     * 1. Recovering the signer from the signature
     * 2. Comparing it with the expected ICP canister address
     * 3. Validating the signature format
     */
    function validateICPSignature(bytes32 dataHash, bytes calldata signature) internal view returns (bool isValid) {
        isValid = ICPSignatureVerifier.verify(icpCanister, dataHash, signature);
    }

    /**
     * @dev Returns the balance of a specified token for a given canister ID
     * @param _canisterId The ICP canister ID
     * @param _token The token address
     * @return balance The balance for the canister and token
     */
    function getBalance(string calldata _canisterId, address _token) public view returns (uint256 balance) {
        balance = canisters[keccak256(bytes(_canisterId))][_token];
    }

    /**
     * @dev Withdraws ERC20 tokens from the contract to the sender's address for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _token The token address
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount to withdraw
     * @param _signature The ICP signature authorizing the withdrawal
     * @return success Whether the withdrawal was successful
     * 
     * Requires a valid ICP signature from the authorized canister
     * Emits a `FundsWithdrawn` event on success
     */
    function withdraw(
        string calldata _canisterId,
        address _token,
        uint _nonce,
        uint _amount,
        bytes calldata _signature
    ) public returns (bool) {
        bool success = withdrawTo(_canisterId, _token, _nonce, _amount, _signature, msg.sender);
        return success;
    }

    /**
     * @dev Withdraws ERC20 tokens from the contract to a specified recipient address for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _token The token address
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount to withdraw
     * @param _signature The ICP signature authorizing the withdrawal
     * @param _recipient The address to receive the tokens
     * @return success Whether the withdrawal was successful
     * 
     * This function demonstrates the core ICP signature verification workflow:
     * 1. Validates the contract is initialized
     * 2. Checks sufficient balance
     * 3. Prevents signature replay
     * 4. Creates message hash for verification
     * 5. Verifies ICP signature
     * 6. Executes the withdrawal
     */
    function withdrawTo(
        string calldata _canisterId,
        address _token,
        uint _nonce,
        uint _amount,
        bytes calldata _signature,
        address _recipient
    ) public nonReentrant returns (bool) {
        require(initialized, "CONTRACT_UNINITIALIZED");
        require(getBalance(_canisterId, _token) >= _amount, "WITHDRAW_AMOUNT > CANISTER_TOKEN_BALANCE");
        require(!usedSignatures[_signature], "USED_SIGNATURE");
        
        // Create the message hash that was signed by the ICP canister
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        
        // Verify the ICP signature
        bool isValidSignature = validateICPSignature(dataHash, _signature);
        require(isValidSignature, "INVALID_ICP_SIGNATURE");
        
        // Emit verification event for transparency
        emit ICPSignatureVerified(_canisterId, dataHash, isValidSignature);
        
        // Mark signature as used to prevent replay
        usedSignatures[_signature] = true;
        
        // Update balance and transfer tokens
        canisters[keccak256(bytes(_canisterId))][_token] -= _amount;
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId, _token);
        
        bool success = IERC20(_token).transfer(_recipient, _amount);
        return success;
    }

    /**
     * @dev Withdraws native tokens (ETH) to a specified recipient for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount to withdraw
     * @param _signature The ICP signature authorizing the withdrawal
     * @param _recipient The address to receive the ETH
     * @return success Whether the withdrawal was successful
     * 
     * Requires a valid ICP signature from the authorized canister
     * Emits a `FundsWithdrawn` event on success
     */
    function withdrawTokensTo(
        string calldata _canisterId,
        uint _nonce,
        uint _amount,
        bytes calldata _signature,
        address _recipient
    ) public nonReentrant returns (bool) {
        address _token = ZERO_ADDRESS;
        require(initialized, "CONTRACT_UNINITIALIZED");
        require(_amount <= getBalance(), "INSUFFICIENT_CONTRACT_BALANCE");
        require(getBalance(_canisterId, _token) >= _amount, "WITHDRAW_AMOUNT > CANISTER_TOKEN_BALANCE");
        require(!usedSignatures[_signature], "USED_SIGNATURE");
        
        // Create the message hash that was signed by the ICP canister
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        
        // Verify the ICP signature
        bool isValidSignature = validateICPSignature(dataHash, _signature);
        require(isValidSignature, "INVALID_ICP_SIGNATURE");
        
        // Emit verification event for transparency
        emit ICPSignatureVerified(_canisterId, dataHash, isValidSignature);
        
        // Mark signature as used to prevent replay
        usedSignatures[_signature] = true;
        
        // Update balance and transfer ETH
        canisters[keccak256(bytes(_canisterId))][_token] -= _amount;
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId, _token);
        
        (bool success, ) = payable(_recipient).call{value: _amount}("");
        return success;
    }

    /**
     * @dev Withdraws native tokens (ETH) to the sender for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount to withdraw
     * @param _signature The ICP signature authorizing the withdrawal
     * @return success Whether the withdrawal was successful
     */
    function withdrawTokens(
        string calldata _canisterId,
        uint _nonce,
        uint _amount,
        bytes calldata _signature
    ) public returns (bool) {
        bool success = withdrawTokensTo(_canisterId, _nonce, _amount, _signature, msg.sender);
        return success;
    }

    /**
     * @dev Returns the contract's native token (ETH) balance
     * @return The contract's ETH balance
     */
    function getBalance() public view returns (uint) {
        return address(this).balance;
    }

    /**
     * @dev Cancels a withdrawal request for a specified canister ID
     * @param _canisterId The ICP canister ID
     * @param _token The token address
     * @param _nonce The nonce to prevent replay attacks
     * @param _amount The amount that was to be withdrawn
     * @param _signature The ICP signature authorizing the cancellation
     * 
     * Requires a valid ICP signature from the authorized canister
     * Emits a `WithdrawCanceled` event on success
     */
    function cancelWithdraw(
        string calldata _canisterId,
        address _token,
        uint _nonce,
        uint _amount,
        bytes calldata _signature
    ) public {
        require(initialized, "CONTRACT_UNINITIALIZED");
        require(!usedSignatures[_signature], "USED_SIGNATURE");
        
        // Create the message hash that was signed by the ICP canister
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        
        // Verify the ICP signature
        bool isValidSignature = validateICPSignature(dataHash, _signature);
        require(isValidSignature, "INVALID_ICP_SIGNATURE");
        
        // Emit verification event for transparency
        emit ICPSignatureVerified(_canisterId, dataHash, isValidSignature);
        
        // Mark signature as used to prevent replay
        usedSignatures[_signature] = true;
        
        emit WithdrawCanceled(_canisterId, msg.sender, _amount, chainId, _token);
    }

    /**
     * @dev Demonstrates ICP signature verification for a custom message
     * @param message The message to verify
     * @param signature The ICP signature to verify
     * @return isValid Whether the signature is valid
     * 
     * This function shows how to verify ICP signatures for arbitrary messages,
     * which can be useful for various verification scenarios beyond fund management
     */
    function verifyCustomMessage(
        string calldata message,
        bytes calldata signature
    ) public view returns (bool isValid) {
        bytes32 messageHash = keccak256(abi.encodePacked(message));
        isValid = ICPSignatureVerifier.verify(icpCanister, messageHash, signature);
    }

    /**
     * @dev Authorizes contract upgrades. Required by the UUPS module
     * @param newImplementation The address of the new implementation
     */
    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {}
}
