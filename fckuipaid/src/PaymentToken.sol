// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

// Simple ERC-20 token for testing payments etc. In the constructor mints
// total supply of all tokens to the given several addresses
contract PaymentToken is ERC20 {
    constructor(uint256 initialSupply, address _receiver1, address _receiver2, address _receiver3) ERC20("Test", "TST") {
        uint amnt = initialSupply / 3;
        _mint(_receiver1, amnt);
        _mint(_receiver2, amnt);
        _mint(_receiver3, amnt);
    }
}