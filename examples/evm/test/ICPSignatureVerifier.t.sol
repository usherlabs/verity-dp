// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/examples/ICPSignatureVerifier.sol";
import "../src/utils/ICPSignatureVerifier.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/**
 * @title ICPSignatureVerifierTest
 * @dev Comprehensive test suite for ICP signature verification functionality
 * 
 * This test suite covers:
 * - Contract initialization and configuration
 * - ICP signature verification with various message types
 * - Fund deposit and withdrawal operations
 * - Signature replay protection
 * - Error handling and edge cases
 * - Integration with ERC20 tokens
 */
contract ICPSignatureVerifierTest is Test {
    ICPSignatureVerifierExample public icpVerifier;
    MockERC20 public mockToken;
    
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
        // Deploy the ICP signature verifier contract
        icpVerifier = new ICPSignatureVerifierExample();
        icpVerifier.initialize(ICP_CANISTER, CHAIN_ID);
        
        // Deploy mock ERC20 token
        mockToken = new MockERC20("Test Token", "TEST");
        
        // Mint tokens to users
        mockToken.mint(USER1, 10000 ether);
        mockToken.mint(USER2, 10000 ether);
        
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
        assertEq(icpVerifier.icpCanister(), ICP_CANISTER);
        assertEq(icpVerifier.chainId(), CHAIN_ID);
        assertEq(icpVerifier.owner(), address(this));
    }

    /**
     * @dev Test ICP canister address update
     */
    function testSetICPCanisterAddress() public {
        address newCanister = 0x9999999999999999999999999999999999999999;
        
        icpVerifier.setICPCanisterAddress(newCanister);
        assertEq(icpVerifier.icpCanister(), newCanister);
    }

    /**
     * @dev Test native token deposit
     */
    function testDepositTokens() public {
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        assertEq(icpVerifier.getBalance(CANISTER_ID, address(0)), DEPOSIT_AMOUNT);
        assertEq(address(icpVerifier).balance, DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test ERC20 token deposit
     */
    function testDepositFunds() public {
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), DEPOSIT_AMOUNT);
        bool success = icpVerifier.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(mockToken));
        vm.stopPrank();
        
        assertTrue(success);
        assertEq(icpVerifier.getBalance(CANISTER_ID, address(mockToken)), DEPOSIT_AMOUNT);
        assertEq(mockToken.balanceOf(address(icpVerifier)), DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test ICP signature verification with valid signature
     */
    function testValidICPSignature() public {
        // Create a test message hash
        bytes32 messageHash = keccak256(abi.encodePacked("test message"));
        
        // Create a valid signature using the test private key
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
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), DEPOSIT_AMOUNT);
        icpVerifier.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(mockToken));
        vm.stopPrank();
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(mockToken)
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Perform withdrawal
        vm.prank(USER1);
        bool success = icpVerifier.withdraw(
            CANISTER_ID, 
            address(mockToken), 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        
        assertTrue(success);
        assertEq(icpVerifier.getBalance(CANISTER_ID, address(mockToken)), DEPOSIT_AMOUNT - WITHDRAW_AMOUNT);
        assertEq(mockToken.balanceOf(USER1), 10000 ether - DEPOSIT_AMOUNT + WITHDRAW_AMOUNT);
    }

    /**
     * @dev Test withdrawal with invalid ICP signature
     */
    function testWithdrawWithInvalidSignature() public {
        // First deposit some funds
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), DEPOSIT_AMOUNT);
        icpVerifier.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(mockToken));
        vm.stopPrank();
        
        // Create invalid signature (wrong private key)
        uint256 wrongPrivateKey = 0x9999999999999999999999999999999999999999999999999999999999999999;
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(mockToken)
        ));
        
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(wrongPrivateKey, messageHash);
        bytes memory signature = abi.encodePacked(r, s, v);
        
        // Attempt withdrawal should fail
        vm.prank(USER1);
        vm.expectRevert("INVALID_ICP_SIGNATURE");
        icpVerifier.withdraw(
            CANISTER_ID, 
            address(mockToken), 
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
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), DEPOSIT_AMOUNT);
        icpVerifier.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(mockToken));
        vm.stopPrank();
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(mockToken)
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // First withdrawal should succeed
        vm.prank(USER1);
        bool success1 = icpVerifier.withdraw(
            CANISTER_ID, 
            address(mockToken), 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        assertTrue(success1);
        
        // Second withdrawal with same signature should fail
        vm.prank(USER1);
        vm.expectRevert("USED_SIGNATURE");
        icpVerifier.withdraw(
            CANISTER_ID, 
            address(mockToken), 
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
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), WITHDRAW_AMOUNT - 1 ether);
        icpVerifier.depositFunds(CANISTER_ID, WITHDRAW_AMOUNT - 1 ether, address(mockToken));
        vm.stopPrank();
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(mockToken)
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Attempt withdrawal should fail
        vm.prank(USER1);
        vm.expectRevert("WITHDRAW_AMOUNT > CANISTER_TOKEN_BALANCE");
        icpVerifier.withdraw(
            CANISTER_ID, 
            address(mockToken), 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
    }

    /**
     * @dev Test native token withdrawal
     */
    function testWithdrawTokens() public {
        // First deposit some native tokens
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        // Create signature for withdrawal
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(0)
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
        assertEq(icpVerifier.getBalance(CANISTER_ID, address(0)), DEPOSIT_AMOUNT - WITHDRAW_AMOUNT);
        assertEq(USER1.balance, initialBalance + WITHDRAW_AMOUNT);
    }

    /**
     * @dev Test withdrawal cancellation
     */
    function testCancelWithdraw() public {
        // First deposit some funds
        vm.startPrank(USER1);
        mockToken.approve(address(icpVerifier), DEPOSIT_AMOUNT);
        icpVerifier.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(mockToken));
        vm.stopPrank();
        
        // Create signature for withdrawal cancellation
        bytes32 messageHash = keccak256(abi.encodePacked(
            NONCE, WITHDRAW_AMOUNT, USER1, CHAIN_ID, CANISTER_ID, address(mockToken)
        ));
        
        bytes memory signature = signMessageHash(messageHash);
        
        // Cancel withdrawal
        vm.prank(USER1);
        icpVerifier.cancelWithdraw(
            CANISTER_ID, 
            address(mockToken), 
            NONCE, 
            WITHDRAW_AMOUNT, 
            signature
        );
        
        // Balance should remain unchanged
        assertEq(icpVerifier.getBalance(CANISTER_ID, address(mockToken)), DEPOSIT_AMOUNT);
    }

    /**
     * @dev Test invalid canister ID length
     */
    function testInvalidCanisterId() public {
        string memory invalidCanisterId = "invalid";
        
        vm.prank(USER1);
        vm.expectRevert("INVALID_CANISTERID");
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(invalidCanisterId);
    }

    /**
     * @dev Test zero amount deposit
     */
    function testZeroAmountDeposit() public {
        vm.prank(USER1);
        vm.expectRevert("amount == 0");
        icpVerifier.depositTokens{value: 0}(CANISTER_ID);
    }

    /**
     * @dev Test contract balance query
     */
    function testGetBalance() public {
        assertEq(icpVerifier.getBalance(), 0);
        
        vm.prank(USER1);
        icpVerifier.depositTokens{value: DEPOSIT_AMOUNT}(CANISTER_ID);
        
        assertEq(icpVerifier.getBalance(), DEPOSIT_AMOUNT);
    }
}

/**
 * @title MockERC20
 * @dev Simple ERC20 token for testing purposes
 */
contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}
    
    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}
