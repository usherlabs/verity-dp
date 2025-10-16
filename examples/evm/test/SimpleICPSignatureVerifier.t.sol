// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/examples/SimpleICPSignatureVerifier.sol";

/**
 * @title SimpleICPSignatureVerifierTest
 * @dev Test suite for the simplified ICP signature verification contract
 * 
 * This test suite covers the core functionality without external dependencies:
 * - Contract initialization and configuration
 * - ICP signature verification
 * - Fund deposit and withdrawal operations
 * - Signature replay protection
 * - Error handling and edge cases
 */
contract SimpleICPSignatureVerifierTest is Test {
    SimpleICPSignatureVerifier public icpVerifier;
    
    // Test addresses and parameters
    address public constant ICP_CANISTER = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    address public constant USER1 = 0x1111111111111111111111111111111111111111;
    address public constant USER2 = 0x2222222222222222222222222222222222222222;
    string public constant CHAIN_ID = "ethereum";
    string public constant CANISTER_ID = "rdmx6-jaaaa-aaaah-qcaiq-cai";
    
    // Test amounts
    uint256 public constant DEPOSIT_AMOUNT = 1000 ether;
    uint256 public constant WITHDRAW_AMOUNT = 500 ether;
    uint256 public constant NONCE = 12345;
    
    // Private key that corresponds to ICP_CANISTER address (DO NOT USE IN PRODUCTION)
    uint256 public constant PRIVATE_KEY = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;

    function setUp() public {
        // Deploy the contract
        icpVerifier = new SimpleICPSignatureVerifier();
        
        // Initialize the contract
        icpVerifier.initialize(ICP_CANISTER, CHAIN_ID);
        
        // Set up user balances
        vm.deal(USER1, 10000 ether);
        vm.deal(USER2, 10000 ether);
    }

    /**
     * @dev Helper function to sign a message hash without Ethereum prefix
     * @param messageHash The message hash to sign
     * @return signature The signature bytes
     */
    function signMessageHash(bytes32 messageHash) internal pure returns (bytes memory signature) {
        // Apply Ethereum message prefix manually
        bytes32 ethSignedMessageHash = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
        
        // Sign the prefixed hash
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(PRIVATE_KEY, ethSignedMessageHash);
        signature = abi.encodePacked(r, s, v);
    }

    /**
     * @dev Test contract initialization
     */
    function testInitialization() public {
        (address owner, address icpCanister, string memory chainId, bool initialized) = icpVerifier.getContractInfo();
        
        assertEq(owner, address(this));
        assertEq(icpCanister, ICP_CANISTER);
        assertEq(chainId, CHAIN_ID);
        assertTrue(initialized);
    }

    /**
     * @dev Test double initialization prevention
     */
    function testDoubleInitialization() public {
        vm.expectRevert("ALREADY_INITIALIZED");
        icpVerifier.initialize(ICP_CANISTER, CHAIN_ID);
    }

    /**
     * @dev Test native token deposit
     */
    function testDepositTokens() public {
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        assertEq(icpVerifier.getBalance(CANISTER_ID), DEPOSIT_AMOUNT);
        assertEq(icpVerifier.getContractBalance(), DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test deposit with invalid canister ID
     */
    function testDepositInvalidCanisterId() public {
        vm.prank(USER1);
        vm.expectRevert("INVALID_CANISTERID");
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}("invalid");
    }

    /**
     * @dev Test deposit with zero amount
     */
    function testDepositZeroAmount() public {
        vm.prank(USER1);
        vm.expectRevert("ZERO_AMOUNT");
        icpVerifier.depositTokens{value: 0}(CANISTER_ID);
    }

    /**
     * @dev Test ICP signature verification with valid signature
     */
    function testValidICPSignature() public {
        // Create a test message hash
        bytes32 messageHash = keccak256(abi.encodePacked("test message"));
        
        // Create a valid signature using the helper function
        bytes memory signature = signMessageHash(messageHash);
        
        // Test signature verification
        bool isValid = icpVerifier.verifyCustomMessage("test message", signature);
        assertTrue(isValid);
    }

    /**
     * @dev Test ICP signature verification with invalid signature
     */
    function testInvalidICPSignature() public {
        // Create an invalid signature (wrong private key)
        uint256 wrongPrivateKey = 0x9999999999999999999999999999999999999999999999999999999999999999;
        bytes32 messageHash = keccak256(abi.encodePacked("test message"));
        
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wrongPrivateKey, messageHash);
        bytes memory signature = abi.encodePacked(r, s, v);
        
        // Test signature verification should fail
        bool isValid = icpVerifier.verifyCustomMessage("test message", signature);
        assertFalse(isValid);
    }

    /**
     * @dev Test withdrawal with valid ICP signature
     */
    function testWithdrawWithValidSignature() public {
        // First deposit some funds
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Perform withdrawal
        uint256 initialBalance = USER1.balance;
        vm.prank(USER1);
        bool success = icpVerifier.withdrawTokens(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        
        assertTrue(success);
        assertEq(icpVerifier.getBalance(CANISTER_ID), DEPOSIT_AMOUNT - WITHDRAW_AMOUNT);
        assertEq(USER1.balance, initialBalance + WITHDRAW_AMOUNT);
    }

    /**
     * @dev Test withdrawal with invalid ICP signature
     */
    function testWithdrawWithInvalidSignature() public {
        // First deposit some funds
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Create invalid signature (wrong private key)
        uint256 wrongPrivateKey = 0x9999999999999999999999999999999999999999999999999999999999999999;
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID
        ));
        
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wrongPrivateKey, messageHash);
        bytes memory signature = abi.encodePacked(r, s, v);
        
        // Attempt withdrawal should fail
        vm.prank(USER1);
        vm.expectRevert("INVALID_ICP_SIGNATURE");
        icpVerifier.withdrawTokens(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
    }

    /**
     * @dev Test signature replay protection
     */
    function testSignatureReplayProtection() public {
        // First deposit some funds
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // First withdrawal should succeed
        vm.prank(USER1);
        bool success1 = icpVerifier.withdrawTokens(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        assertTrue(success1);
        
        // Check that signature is marked as used
        assertTrue(icpVerifier.isSignatureUsed(signature));
        
        // Second withdrawal with same signature should fail
        vm.prank(USER1);
        vm.expectRevert("SIGNATURE_ALREADY_USED");
        icpVerifier.withdrawTokens(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
    }

    /**
     * @dev Test withdrawal with insufficient balance
     */
    function testWithdrawInsufficientBalance() public {
        // Deposit less than withdrawal amount
        vm.prank(USER1);
        icpVerifier.depositTokens{value: WITHDRAW_AMOUNT - 1 ether}(CANISTER_ID);
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Attempt withdrawal should fail
        vm.prank(USER1);
        vm.expectRevert("INSUFFICIENT_BALANCE");
        icpVerifier.withdrawTokens(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
    }

    /**
     * @dev Test withdrawal cancellation
     */
    function testCancelWithdraw() public {
        // First deposit some funds
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Create signature for withdrawal cancellation
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Cancel withdrawal
        vm.prank(USER1);
        icpVerifier.cancelWithdraw(
            CANISTER_ID, 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        
        // Balance should remain unchanged
        assertEq(icpVerifier.getBalance(CANISTER_ID), DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test ownership transfer
     */
    function testTransferOwnership() public {
        address newOwner = 0x3333333333333333333333333333333333333333;
        
        icpVerifier.transferOwnership(newOwner);
        
        (address owner, , , ) = icpVerifier.getContractInfo();
        assertEq(owner, newOwner);
    }

    /**
     * @dev Test ICP canister address update
     */
    function testSetICPCanister() public {
        address newCanister = 0x9999999999999999999999999999999999999999;
        
        icpVerifier.setICPCanister(newCanister);
        
        (, address icpCanister, , ) = icpVerifier.getContractInfo();
        assertEq(icpCanister, newCanister);
    }

    /**
     * @dev Test emergency withdrawal
     */
    function testEmergencyWithdraw() public {
        // Deposit some funds
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Emergency withdraw to USER1
        uint256 initialUserBalance = USER1.balance;
        icpVerifier.emergencyWithdraw(USER1);
        
        assertEq(USER1.balance, initialUserBalance + DEPOSIT_AMOUNT);
        assertEq(icpVerifier.getContractBalance(), 0);
    }

    /**
     * @dev Test access control
     */
    function testAccessControl() public {
        vm.prank(USER1);
        vm.expectRevert("ONLY_OWNER");
        icpVerifier.setICPCanister(0x9999999999999999999999999999999999999999);
        
        vm.prank(USER1);
        vm.expectRevert("ONLY_OWNER");
        icpVerifier.transferOwnership(USER1);
        
        vm.prank(USER1);
        vm.expectRevert("ONLY_OWNER");
        icpVerifier.emergencyWithdraw(USER1);
    }

    /**
     * @dev Test contract balance query
     */
    function testGetContractBalance() public {
        assertEq(icpVerifier.getContractBalance(), 0);
        
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        assertEq(icpVerifier.getContractBalance(), DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test multiple canister balances
     */
    function testMultipleCanisterBalances() public {
        string memory canister1 = "rdmx6-jaaaa-aaaah-qcaiq-cai";
        string memory canister2 = "rdmx6-jaaaa-aaaah-qcaiq-ca2";
        
        vm.prank(USER1);
        icpVerifier.depositTokens{value: 1000 ether}(canister1);
        
        vm.prank(USER2);
        icpVerifier.depositTokens{value: 2000 ether}(canister2);
        
        assertEq(icpVerifier.getBalance(canister1), 1000 ether);
        assertEq(icpVerifier.getBalance(canister2), 2000 ether);
        assertEq(icpVerifier.getContractBalance(), 3000 ether);
    }
}

