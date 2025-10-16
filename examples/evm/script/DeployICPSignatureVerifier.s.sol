// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.30;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/examples/ICPSignatureVerifier.sol";

/**
 * @title DeployICPSignatureVerifier
 * @dev Deployment script for the ICP Signature Verifier Example contract
 * 
 * This script demonstrates how to deploy and initialize the ICP signature verifier
 * contract with the necessary parameters for ICP signature verification.
 * 
 * Usage:
 * 1. Set the ICP_CANISTER_ADDRESS environment variable to the Ethereum address
 *    derived from your ICP canister's public key
 * 2. Set the CHAIN_ID environment variable to your target chain identifier
 * 3. Run: forge script script/DeployICPSignatureVerifier.s.sol --rpc-url <RPC_URL> --broadcast
 */
contract DeployICPSignatureVerifier is Script {
    // Default values (can be overridden by environment variables)
    address constant DEFAULT_ICP_CANISTER = 0x1234567890123456789012345678901234567890;
    string constant DEFAULT_CHAIN_ID = "ethereum";
    
    function run() external {
        // Get deployment parameters from environment or use defaults
        address icpCanister = vm.envOr("ICP_CANISTER_ADDRESS", DEFAULT_ICP_CANISTER);
        string memory chainId = vm.envOr("CHAIN_ID", DEFAULT_CHAIN_ID);
        
        console.log("Deploying ICP Signature Verifier Example...");
        console.log("ICP Canister Address:", icpCanister);
        console.log("Chain ID:", chainId);
        
        // Deploy the contract
        vm.startBroadcast();
        
        ICPSignatureVerifierExample icpVerifier = new ICPSignatureVerifierExample();
        
        // Initialize the contract
        icpVerifier.initialize(icpCanister, chainId);
        
        vm.stopBroadcast();
        
        console.log("ICP Signature Verifier deployed at:", address(icpVerifier));
        console.log("Contract initialized successfully!");
        
        // Display contract information
        console.log("\n=== Contract Information ===");
        console.log("ICP Canister:", icpVerifier.icpCanister());
        console.log("Chain ID:", icpVerifier.chainId());
        console.log("Owner:", icpVerifier.owner());
        
        // Display usage instructions
        console.log("\n=== Usage Instructions ===");
        console.log("1. Deposit funds using depositTokens() or depositFunds()");
        console.log("2. Create ICP signatures for withdrawal authorization");
        console.log("3. Withdraw funds using withdraw() or withdrawTokens()");
        console.log("4. Verify custom messages using verifyCustomMessage()");
        console.log("\nNote: Ensure your ICP canister is properly configured");
        console.log("and the canister address matches the deployed contract.");
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
        console.log("Deploying ICP Signature Verifier with custom parameters...");
        console.log("ICP Canister:", icpCanister);
        console.log("Chain ID:", chainId);
        
        vm.startBroadcast();
        
        ICPSignatureVerifierExample icpVerifier = new ICPSignatureVerifierExample();
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
        console.log("Deploying multiple ICP Signature Verifier instances...");
        console.log("Number of canisters:", icpCanisters.length);
        console.log("Chain ID:", chainId);
        
        deployedAddresses = new address[](icpCanisters.length);
        
        vm.startBroadcast();
        
        for (uint256 i = 0; i < icpCanisters.length; i++) {
            console.log("Deploying for canister", i + 1, ":", icpCanisters[i]);
            
            ICPSignatureVerifierExample icpVerifier = new ICPSignatureVerifierExample();
            icpVerifier.initialize(icpCanisters[i], chainId);
            
            deployedAddresses[i] = address(icpVerifier);
            console.log("Deployed at:", deployedAddresses[i]);
        }
        
        vm.stopBroadcast();
        
        console.log("All contracts deployed successfully!");
        return deployedAddresses;
    }
}
