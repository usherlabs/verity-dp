// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import {Script, console} from "forge-std/Script.sol";
import {Upgrades} from "@openzeppelin/foundry-upgrades/Upgrades.sol";
import {Options} from "@openzeppelin/foundry-upgrades/Options.sol";

contract UpgradeLocker is Script {
    // TODO: update contract address
    address contractAddress = vm.envAddress("UPGRADE_CONTRACT_ADDRESS");
    // TODO: REPLACE "Locker.sol" with old contract name
    string oldContractName = vm.envString("UPGRADE_OLD_CONTRACT_NAME");

    function run() public {
        vm.startBroadcast();

        Options memory opts;
        opts.referenceContract = oldContractName;

        // TODO: REPLACE "LockerV2.sol" with new contract name
        Upgrades.upgradeProxy(contractAddress, vm.envString("UPGRADE_NEW_CONTRACT_NAME"), "", opts);

        console.log("Locker Proxy Implementation upgraded");
        vm.stopBroadcast();
    }
}
