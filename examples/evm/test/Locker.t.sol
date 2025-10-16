// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Locker} from "../src/Locker.sol";
import {ERC20Mock} from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract LockerTest is Test {
    Locker public locker;
    ERC20Mock public token;
    address admin;
    address canister;

    uint256 constant NONCE = 10;
    string CHAIN_ID = "ethereum:1";
    uint256 constant INITIAL_BALANCE = 100 ether;
    uint256 constant DEPOSIT_AMOUNT = 10 ether;
    uint256 constant WITHDRAW_AMOUNT = 1 ether;

    string constant CANISTER_ID = "c2lt4-zmaaa-aaaaa-qaaiq-cai";
    bytes32 constant CANISTER_ID_HASH = keccak256("c2lt4-zmaaa-aaaaa-qaaiq-cai");
    address REMITTANCE_CANISTER_PUBKEY = 0x2189a3E395Abbf8ac15CA9159E87FD3A05f92714; // Example address for remittance canister
    bytes constant SIGNATURE =
        hex"d7c29a240bfa3f29197d519b884ca6201c8b3da81275a571e1cd68742a302bd24b83dde5efaa6179efbb2ba9b4a5232afa70f906bf77ef06317fdb3ab4f3eca81c";
    bytes constant INVALID_SIGNATURE =
        hex"9c30b379586b631866bd2b4fd06d4e92342ec8977e3d39b67b410a80957a9e9737e4633712b7683d30b8612e80381a54af0cfb547f44e7a5ec1872d5114c4ba01c";


    event FundsDeposited(string canisterId, address indexed account, uint amount, string chain, address token);
    event FundsWithdrawn(string canisterId, address indexed account, uint amount, string chain, address token);


    function setUp() public {
        admin = address(0x431); // Locker contract's address
        canister = address(0x456); // Canister's address

        token = new ERC20Mock(); // Deploy mock token
        Locker lockerImplementation = new Locker();

        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(lockerImplementation),
            admin,
            abi.encodeWithSelector(Locker.initialize.selector, REMITTANCE_CANISTER_PUBKEY, CHAIN_ID)
        );

        // Initialize the proxy as the Locker contract
        locker = Locker(address(proxy));
        token.mint(admin, INITIAL_BALANCE); // Mint some tokens to admin
    }

    function testDepositFunds() public {
        vm.startPrank(admin);
        token.approve(address(locker), DEPOSIT_AMOUNT); // Approve Locker

        vm.expectEmit(true, false, false, false, address(locker));
        emit FundsDeposited(CANISTER_ID, admin, DEPOSIT_AMOUNT, CHAIN_ID, address(token));

        bool success = locker.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(token));
        assertTrue(success, "Deposit failed");

        vm.stopPrank();

        // Validate balances
        uint256 lockerBalance = token.balanceOf(address(locker));
        assertEq(lockerBalance, DEPOSIT_AMOUNT, "Locker balance should be 0.5 ETH");
    }

    function testWithdrawFunds() public {
        vm.startPrank(admin);
        // Admin deposits funds
        token.approve(address(locker), DEPOSIT_AMOUNT); // Approve Locker
        locker.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(token));

        // Admin signs a withdrawal request (assuming signature verification in Locker)
        // this is the payload being signed and how to generate it.
        // it is used to generate the signature
        // bytes32 dataHash 0x9c603099505d044c9654b57f35b022f1f54322d134673ce34007294e8396a205
        // bytes32 dataHash = keccak256(
        //     abi.encodePacked(NONCE, DEPOSIT_AMOUNT, admin, CHAIN_ID, CANISTER_ID, address(token))
        // );
        // emit log_bytes32(dataHash);

        // Check event emission
        vm.expectEmit(true, false, false, false, address(locker));
        emit FundsWithdrawn(CANISTER_ID, admin, DEPOSIT_AMOUNT, CHAIN_ID, address(token));

        // Attempt to withdraw funds
        bool success = locker.withdraw(CANISTER_ID, address(token), NONCE, DEPOSIT_AMOUNT, SIGNATURE);
        assertTrue(success, "Withdrawal failed");
        vm.stopPrank();

        // Validate post-withdrawal balance
        uint256 recipientBalance = token.balanceOf(admin);
        assertEq(recipientBalance, INITIAL_BALANCE, "Admin should have initial after withdrawal");
    }

    function testInvalidSignatureReverts() public {
        vm.startPrank(admin);

        // Attempt to deposit with an invalid signature
        token.approve(address(locker), DEPOSIT_AMOUNT); // Approve Locker for two withdrawals
        locker.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT, address(token));

        // Attempt to withdraw funds with an invalid signature
        vm.expectRevert("INVALID_SIGNATURE");
        locker.withdraw(CANISTER_ID, address(token), NONCE, DEPOSIT_AMOUNT, INVALID_SIGNATURE);

        vm.stopPrank();
    }

    function testReusedSignatureReverts() public {
        uint256 num_withdrawals = 2;
        vm.startPrank(admin);

        // Attempt to deposit with an invalid signature
        token.approve(address(locker), DEPOSIT_AMOUNT * num_withdrawals); // Approve Locker for two withdrawals
        locker.depositFunds(CANISTER_ID, DEPOSIT_AMOUNT * num_withdrawals, address(token));

        // Attempt to withdraw funds first time
        locker.withdraw(CANISTER_ID, address(token), NONCE, DEPOSIT_AMOUNT, SIGNATURE);


        vm.expectRevert("USED_SIGNATURE");
        // attempt to withdraw funds again with same signature
        locker.withdraw(CANISTER_ID, address(token), NONCE, DEPOSIT_AMOUNT, SIGNATURE);

        vm.stopPrank();
    }
}
