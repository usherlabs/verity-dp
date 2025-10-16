// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Script, console} from "forge-std/Script.sol";
import {Upgrades} from "@openzeppelin/foundry-upgrades/Upgrades.sol";

import {Locker} from "../src/Locker.sol";

contract DeployLocker is Script {
    // TODO: update these parameters to reflect the state of thecontract before deployment
    address REMITTANCE_CANISTER_PUBKEY = vm.envAddress("DEPLOY_REMITTANCE_CANISTER_PUBKEY"); // Example address
    string CHAIN_IDENTIFIER = vm.envString("DEPLOY_CHAIN_IDENTIFIER");

    function run() public {
        vm.startBroadcast();

        address _proxyAddress = Upgrades.deployUUPSProxy(
            "Locker.sol",
            abi.encodeCall(Locker.initialize, (REMITTANCE_CANISTER_PUBKEY,  CHAIN_IDENTIFIER))
        );

        console.log("Locker Proxy Implementation deployed at:", _proxyAddress);
        vm.stopBroadcast();
    }
}
