// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.25;

import {Script, console2} from "forge-std/Script.sol";
import {ITaikoL1} from "../src/interfaces/taiko/ITaikoL1.sol";
import {PreconfTaskManager} from "../src/avs/PreconfTaskManager.sol";
import {IRegistryCoordinator} from "eigenlayer-middleware/interfaces/IRegistryCoordinator.sol";
import {IIndexRegistry} from "eigenlayer-middleware/interfaces/IIndexRegistry.sol";
import {IPreconfServiceManager} from "../src/interfaces/IPreconfServiceManager.sol";
import {IPreconfTaskManager} from "../src/interfaces/IPreconfTaskManager.sol";
import {ProxyAdmin} from "openzeppelin-contracts/proxy/transparent/ProxyAdmin.sol";
import {
    TransparentUpgradeableProxy,
    ITransparentUpgradeableProxy
} from "openzeppelin-contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";

contract TaikoL1Dummy is ITaikoL1 {
    function proposeBlock(bytes calldata _params, bytes calldata _txList)
        external
        payable
        returns (ITaikoL1.BlockMetadata memory meta_, ITaikoL1.EthDeposit[] memory deposits_)
    {}
}

contract TaikoTokenDummy is ERC20 {
    constructor() ERC20("Taiko Token", "TKO") {}
}

contract Deploy is Script {
    uint256 internal PRIVATE_KEY = vm.envUint("PRIVATE_KEY");
    address internal PROXY_OWNER = vm.addr(PRIVATE_KEY);

    modifier broadcast() {
        vm.startBroadcast(PRIVATE_KEY);
        _;
        vm.stopBroadcast();
    }

    function deployImplementationDummy(address preconfer) public broadcast {
        TaikoL1Dummy taikoL1Dummy = new TaikoL1Dummy();
        TaikoTokenDummy taikoTokenDummy = new TaikoTokenDummy();

        PreconfTaskManager taskManagerImplementation = new PreconfTaskManager(
            IPreconfServiceManager(address(0)),
            IRegistryCoordinator(address(0)),
            IIndexRegistry(address(0)),
            ITaikoL1(address(taikoL1Dummy)),
            address(0),
            preconfer
        );

        console2.log("Taiko L1 deployed at: ", address(taikoL1Dummy));
        console2.log("Taiko Token deployed at: ", address(taikoTokenDummy));
        console2.log("Task manager implementation deployed at: ", address(taskManagerImplementation));
    }

    function deployImplementation(address taikoL1, address preconfer) public broadcast {
        PreconfTaskManager taskManagerImplementation = new PreconfTaskManager(
            IPreconfServiceManager(address(0)),
            IRegistryCoordinator(address(0)),
            IIndexRegistry(address(0)),
            ITaikoL1(taikoL1),
            address(0),
            preconfer
        );

        console2.log("Task manager implementation deployed at: ", address(taskManagerImplementation));
    }

    function deployProxy(address implementation, IERC20 taikoToken) public broadcast {
        ProxyAdmin proxyAdmin = new ProxyAdmin();

        IPreconfTaskManager taskManager = IPreconfTaskManager(
            address(
                new TransparentUpgradeableProxy(
                    implementation, address(proxyAdmin), abi.encodeCall(PreconfTaskManager.initialize, taikoToken)
                )
            )
        );

        console2.log("Proxy admin deployed at: ", address(proxyAdmin));
        console2.log("Task Manager Proxy deployed at: ", address(taskManager));
    }

    function upgrade(address proxyAdmin, address proxy, address newImplementation) public broadcast {
        ProxyAdmin(proxyAdmin).upgrade(ITransparentUpgradeableProxy(payable(proxy)), address(newImplementation));
        console2.log("Upgraded!");
    }
}
