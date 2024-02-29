// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/opt.sol";

contract ContractTest is Test {
    OptInContract optInContract;
    address owner;
    address user1 = address(0x1);

    function setUp() public {
        // Setup code here. Deploy contract, prepare environment.
        owner = address(this); // In Forge, `address(this)` is the deploying address
        optInContract = new OptInContract(owner, "TestNFT", "TNFT", owner, 500, owner);

    }

    function testExample() public {
        assertTrue(true);
    }
    function testOptIn() public {
        // Simulate user1 calling the optIn function
        vm.prank(user1);
        optInContract.optIn();
        assertTrue(optInContract.optInStatus(user1));
    }

    function testOptOut() public {
        // First opt in, then opt out
        vm.prank(user1);
        optInContract.optIn();
        vm.prank(user1);
        optInContract.optOut();
        assertFalse(optInContract.optInStatus(user1));
    }

}
