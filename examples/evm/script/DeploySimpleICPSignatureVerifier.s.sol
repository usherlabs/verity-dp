// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/examples/SimpleICPSignatureVerifier.sol";

/**
 * @title DeploySimpleICPSignatureVerifier
 * @dev Deployment script for the simplified ICP Signature Verifier contract
 * 
 * This script demonstrates how to deploy and initialize the simplified ICP signature verifier
 * contract without external dependencies.
 * 
 * Usage:
 * 1. Set the ICP_CANISTER_ADDRESS environment variable to the Ethereum address
 *    derived from your ICP canister's public key
 * 2. Set the CHAIN_ID environment variable to your target chain identifier
 * 3. Run: forge script script/DeploySimpleICPSignatureVerifier.s.sol --rpc-url <RPC_URL> --broadcast
 */
contract DeploySimpleICPSignatureVerifier is Script {
    // Default values (can be overridden by environment variables)
    address constant DEFAULT_ICP_CANISTER = 0x1234567890123456789012345678901234567890;
    string constant DEFAULT_CHAIN_ID = "ethereum";
    
    function run() external {
        // Get deployment parameters from environment or use defaults
        address icpCanister = vm.envOr("ICP_CANISTER_ADDRESS", DEFAULT_ICP_CANISTER);
        string memory chainId = vm.envOr("CHAIN_ID", DEFAULT_CHAIN_ID);
        
        console.log("Deploying Simple ICP Signature Verifier...");
        console.log("ICP Canister Address:", icpCanister);
        console.log("Chain ID:", chainId);
        
        // Deploy the contract
        vm.startBroadcast();
        
        SimpleICPSignatureVerifier icpVerifier = new SimpleICPSignatureVerifier();
        
        // Initialize the contract
        icpVerifier.initialize(icpCanister, chainId);
        
        vm.stopBroadcast();
        
        console.log("Simple ICP Signature Verifier deployed at:", address(icpVerifier));
        console.log("Contract initialized successfully!");
        
        // Display contract information
        console.log("\n=== Contract Information ===");
        (address owner, address deployedIcpCanister, string memory deployedChainId, bool initialized) = icpVerifier.getContractInfo();
        console.log("Owner:", owner);
        console.log("ICP Canister:", deployedIcpCanister);
        console.log("Chain ID:", deployedChainId);
        console.log("Initialized:", initialized);
        
        // Display usage instructions
        console.log("\n=== Usage Instructions ===");
        console.log("1. Deposit funds using depositTokens()");
        console.log("2. Create ICP signatures for withdrawal authorization");
        console.log("3. Withdraw funds using withdrawTokens()");
        console.log("4. Verify custom messages using verifyCustomMessage()");
        console.log("5. Cancel withdrawals using cancelWithdraw()");
        console.log("\nNote: Ensure your ICP canister is properly configured");
        console.log("and the canister address matches the deployed contract.");
        
        // Display contract functions
        console.log("\n=== Available Functions ===");
        console.log("- depositTokens(string canisterId): Deposit ETH for a canister");
        console.log("- withdrawTokens(string canisterId, uint256 nonce, uint256 amount, bytes signature): Withdraw with ICP signature");
        console.log("- verifyCustomMessage(string message, bytes signature): Verify arbitrary message signatures");
        console.log("- cancelWithdraw(string canisterId, uint256 nonce, uint256 amount, bytes signature): Cancel withdrawal");
        console.log("- getBalance(string canisterId): Get canister balance");
        console.log("- getContractBalance(): Get total contract balance");
        console.log("- isSignatureUsed(bytes signature): Check if signature was used");
        console.log("- getContractInfo(): Get contract configuration");
    }
    
    /**
     * @dev Deploy and verify the contract on a specific network
     * @param icpCanister The ICP canister address
     * @param chainId The chain identifier
     * @return deployedAddress The address of the deployed contract
     */
    function deployAndVerify(
        address icpCanister,
        string memory chainId
    ) external returns (address deployedAddress) {
        console.log("Deploying Simple ICP Signature Verifier with custom parameters...");
        console.log("ICP Canister:", icpCanister);
        console.log("Chain ID:", chainId);
        
        vm.startBroadcast();
        
        SimpleICPSignatureVerifier icpVerifier = new SimpleICPSignatureVerifier();
        icpVerifier.initialize(icpCanister, chainId);
        
        vm.stopBroadcast();
        
        deployedAddress = address(icpVerifier);
        console.log("Contract deployed at:", deployedAddress);
        
        return deployedAddress;
    }
    
    /**
     * @dev Deploy multiple instances for different ICP canisters
     * @param icpCanisters Array of ICP canister addresses
     * @param chainId The chain identifier
     * @return deployedAddresses Array of deployed contract addresses
     */
    function deployMultiple(
        address[] memory icpCanisters,
        string memory chainId
    ) external returns (address[] memory deployedAddresses) {
        console.log("Deploying multiple Simple ICP Signature Verifier instances...");
        console.log("Number of canisters:", icpCanisters.length);
        console.log("Chain ID:", chainId);
        
        deployedAddresses = new address[](icpCanisters.length);
        
        vm.startBroadcast();
        
        for (uint256 i = 0; i < icpCanisters.length; i++) {
            console.log("Deploying for canister", i + 1, ":", icpCanisters[i]);
            
            SimpleICPSignatureVerifier icpVerifier = new SimpleICPSignatureVerifier();
            icpVerifier.initialize(icpCanisters[i], chainId);
            
            deployedAddresses[i] = address(icpVerifier);
            console.log("Deployed at:", deployedAddresses[i]);
        }
        
        vm.stopBroadcast();
        
        console.log("All contracts deployed successfully!");
        return deployedAddresses;
    }
    
    /**
     * @dev Deploy and test the contract functionality
     * @param icpCanister The ICP canister address
     * @param chainId The chain identifier
     * @return deployedAddress The address of the deployed contract
     */
    function deployAndTest(
        address icpCanister,
        string memory chainId
    ) external returns (address deployedAddress) {
        console.log("Deploying and testing Simple ICP Signature Verifier...");
        
        vm.startBroadcast();
        
        SimpleICPSignatureVerifier icpVerifier = new SimpleICPSignatureVerifier();
        icpVerifier.initialize(icpCanister, chainId);
        
        vm.stopBroadcast();
        
        deployedAddress = address(icpVerifier);
        console.log("Contract deployed at:", deployedAddress);
        
        // Test basic functionality
        console.log("\n=== Testing Contract Functionality ===");
        
        // Test contract info
        (address owner, address deployedIcpCanister, string memory deployedChainId, bool initialized) = icpVerifier.getContractInfo();
        console.log("Contract initialized:", initialized);
        console.log("Owner set:", owner);
        console.log("ICP Canister set:", deployedIcpCanister);
        console.log("Chain ID set:", deployedChainId);
        
        // Test balance queries
        console.log("Contract balance:", icpVerifier.getContractBalance());
        console.log("Test canister balance:", icpVerifier.getBalance("rdmx6-jaaaa-aaaah-qcaiq-cai"));
        
        console.log("\nContract is ready for use!");
        
        return deployedAddress;
    }
}

