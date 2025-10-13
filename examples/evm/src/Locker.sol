// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {OwnableUpgradeable} from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import {ERC20Upgradeable} from "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";

import {VerifySignature} from "./utils/VerifySignature.sol";

contract Locker is Initializable, UUPSUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    bool initialized;
    string public chainId;
    address public remittanceCanister;
    address constant ZER0_ADDRESS = 0x0000000000000000000000000000000000000000;
    mapping(bytes => bool) usedSignatures;
    mapping(bytes32 => mapping(address => uint256)) public canisters; // keccak256(principal) => tokenAddress => amountDeposited
    event FundsDeposited(string canisterId, address indexed account, uint amount, string chain, address token);
    event FundsWithdrawn(string canisterId, address indexed account, uint amount, string chain, address token);
    event WithdrawCanceled(string canisterId, address indexed account, uint amount, string chain, address token);
    event UpdateRemittanceCanister(address remittanceCanister);
    /**
     * @dev Deposits native tokens (ETH) into the contract for a specified canister ID.
     * Emits a `FundsDeposited` event on success.
     */
    function depositTokens(string calldata _canisterId) public payable nonReentrant {
        uint256 _amount = msg.value;
        address _token = ZER0_ADDRESS;
        require(bytes(_canisterId).length == 27, "INVALID_CANISTERID");
        require(_amount > 0, "amount == 0");
        canisters[keccak256(bytes(_canisterId))][_token] += _amount;
        emit FundsDeposited(_canisterId, msg.sender, _amount, chainId, _token);
    }
    /**
     * @dev Initializes the contract with a remittance canister address and chain ID.
     * This function can only be called once.
     */
    function initialize(address _remittanceCanister, string calldata _chainId) public initializer {
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
        __ReentrancyGuard_init();
        remittanceCanister = _remittanceCanister;
        chainId = _chainId;
        initialized = true;
    }
    /**
     * @dev Deposits ERC20 tokens into the contract for a specified canister ID.
     * Transfers the specified token amount from the sender to this contract.
     * Emits a `FundsDeposited` event on success.
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
     * @dev Sets a new address for the remittance canister. Can only be called by the owner.
     * Emits an `UpdateRemittanceCanister` event on success.
     */
    function setRemittanceCanisterAddress(address _remittanceCanister) public onlyOwner {
        remittanceCanister = _remittanceCanister;
        emit UpdateRemittanceCanister(_remittanceCanister);
    }
    /**
     * @dev Verifies the provided signature against a specified data hash.
     * Returns true if the signature is valid.
     */
    function validateSignature(bytes32 dataHash, bytes calldata signature) internal view returns (bool isValid) {
        isValid = VerifySignature.verify(remittanceCanister, dataHash, signature);
    }
    /**
     * @dev Returns the balance of a specified token for a given canister ID.
     */
    function getBalance(string calldata _canisterId, address _token) public view returns (uint256 balance) {
        balance = canisters[keccak256(bytes(_canisterId))][_token];
    }
    /**
     * @dev Withdraws ERC20 tokens from the contract to the sender’s address for a specified canister ID.
     * Requires a valid signature.
     * Emits a `FundsWithdrawn` event on success.
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
     * @dev Withdraws ERC20 tokens from the contract to a specified recipient address for a specified canister ID.
     * Requires a valid signature.
     * Emits a `FundsWithdrawn` event on success.
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
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        require(validateSignature(dataHash, _signature), "INVALID_SIGNATURE");
        usedSignatures[_signature] = true;
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId, _token);
        bool success = IERC20(_token).transfer(_recipient, _amount);
        return success;
    }
    /**
     * @dev Withdraws native tokens (ETH) to a specified recipient for a specified canister ID.
     * Requires a valid signature.
     * Emits a `FundsWithdrawn` event on success.
     */
    function withdrawTokensTo(
        string calldata _canisterId,
        uint _nonce,
        uint _amount,
        bytes calldata _signature,
        address _recipient
    ) public nonReentrant returns (bool) {
        address _token = ZER0_ADDRESS;
        require(initialized, "CONTRACT_UNINITIALIZED");
        require(_amount <= getBalance(), "INSUFFICIENT_CONTRACT_BALANCE");
        require(getBalance(_canisterId, _token) >= _amount, "WITHDRAW_AMOUNT > CANISTER_TOKEN_BALANCE");
        require(!usedSignatures[_signature], "USED_SIGNATURE");
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        require(validateSignature(dataHash, _signature), "INVALID_SIGNATURE");
        usedSignatures[_signature] = true;
        emit FundsWithdrawn(_canisterId, msg.sender, _amount, chainId, _token);
        (bool success, ) = payable(_recipient).call{value: _amount}("");
        return success;
    }
    /**
     * @dev Withdraws native tokens (ETH) to the sender for a specified canister ID.
     * Requires a valid signature.
     * Emits a `FundsWithdrawn` event on success.
     */
    function withdrawTokens(
        string calldata _canisterId,
        uint _nonce,
        uint _amount,
        bytes calldata _signature
    ) public nonReentrant returns (bool) {
        bool success = withdrawTokensTo(_canisterId, _nonce, _amount, _signature, msg.sender);
        return success;
    }
    /**
     * @dev Returns the contract’s native token (ETH) balance.
     */
    function getBalance() public view returns (uint) {
        return address(this).balance;
    }
    /**
     * @dev Cancels a withdrawal request for a specified canister ID.
     * Requires a valid signature. Emits a `WithdrawCanceled` event on success.
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
        // validate the signature
        bytes32 dataHash = keccak256(abi.encodePacked(_nonce, _amount, msg.sender, chainId, _canisterId, _token));
        require(validateSignature(dataHash, _signature), "INVALID_SIGNATURE");
        // mark signature as used
        usedSignatures[_signature] = true;
        emit WithdrawCanceled(_canisterId, msg.sender, _amount, chainId, _token);
    }

    /**
     * @dev Authorizes contract upgrades. Required by the UUPS module.
     */
    function _authorizeUpgrade(address) internal override onlyOwner {}
}
