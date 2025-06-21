// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../src/PaymentToken.sol";
import "../src/PaymentReceiver.sol";

contract DeploymentScript is Script {
    Counter public counter;

    function setUp() public {}

    function run() public {
        // Load deployer key and addresses from env
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address a1 = address(0xabadc4402C14844431fC7521613b6922c7bdde80);
        address a2 = address(0x27e2bcbC20c3D7257C171443e95f0c6D579a2E80);
        address a3 = address(0x7737aE71528BF34D5c4e2a535Fb51a321e5Be97C);
        vm.startBroadcast(deployerPrivateKey);

        // Deploy the payment token
        IERC20 token = new PaymentToken(3000 * 10**18, a1, a2, a3);
        console2.log("Payment token address: %s", address(token));

        // Deploy payment receiver
        PaymentReceiver receiver = new PaymentReceiver(address(token));
        console2.log("PaymentReceiver address: %s", address(receiver));
        vm.stopBroadcast();
    }
}
