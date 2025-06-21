// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

// Contract to receive payments for using a web2-like service. Supports the 
// following functionality:
// Owner can withdraw all ERC-20 tokens that were paid to the contract
// A user can pay monthly fee for the service with a given ERC-20 token
// Payments for a given user and month are recorded onchain and can be 
// accessed with a public view function.
contract PaymentReceiver {
    address private immutable owner;
    IERC20 public immutable erc20Token;
    uint256 public constant AMOUNT_TO_PAY = 10 * 10**18;
    mapping (address => mapping(uint64 => bool)) recordedPayments;

    modifier onlyOwner(address addr) {
        require(addr == owner, "Can be called only by the owner");
        _;
    }

    // Sets owner and the ERC-20 token address
    constructor(address _erc20TokenAddr) {
        owner = msg.sender;
        erc20Token = IERC20(_erc20TokenAddr);
    }

    // Owner withdraws all accumulated ERC-20 payments
    function withdrawAllTokens() public onlyOwner(msg.sender) {
        uint256 amnt = erc20Token.balanceOf(address(this));
        erc20Token.transfer(msg.sender, amnt);
    }

    // User pays for the given month (unix time month count).
    // Function pulls the tokens from msg.sender and updates recordedPayments 
    // mapping. Note that user must approve token transfer in advance
    function pay(uint64 month) public {
        bool success = erc20Token.transferFrom(msg.sender, address(this), AMOUNT_TO_PAY);
        require(success, "Could not receive the payment, check if spending was approved by user");
        recordedPayments[msg.sender][month] = true;
    }

    function hasPaid(address userAddr, uint64 month) public view returns (bool) {
        return recordedPayments[userAddr][month];
    }
}