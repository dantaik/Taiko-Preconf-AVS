// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Script, console2} from "forge-std/Script.sol";
import {ITaikoL1} from "../src/interfaces/taiko/ITaikoL1.sol";
import {PreconfTaskManager} from "../src/avs/PreconfTaskManager.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IIndexRegistry} from "eigenlayer-middleware/interfaces/IIndexRegistry.sol";
import {IPreconfServiceManager} from "../src/interfaces/IPreconfServiceManager.sol";

contract TaikoL1Dummy is ITaikoL1 {
    function proposeBlock(bytes calldata _params, bytes calldata _txList)
        external
        payable
        returns (ITaikoL1.BlockMetadata memory meta_, ITaikoL1.EthDeposit[] memory deposits_)
    {}
}

contract Deploy is Script {
    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        TaikoL1Dummy taikoL1Dummy = new TaikoL1Dummy();
        PreconfTaskManager taskManager = new PreconfTaskManager(
            IPreconfServiceManager(address(0)),
            IRegistryCoordinator(address(0)),
            IIndexRegistry(address(0)),
            ITaikoL1(address(taikoL1Dummy)),
            address(0)
        );
        console2.log("Task Manager deployed at: ", address(taskManager));
        vm.stopBroadcast();
    }
}
