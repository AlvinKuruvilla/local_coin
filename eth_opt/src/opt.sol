// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "@thirdweb-dev/contracts/base/ERC721Drop.sol";

contract opt is ERC721Drop {
    // Mapping to store opt-in status for each user
    mapping(address => bool) public optInStatus;

    // Event to log opt-in changes
    event OptInStatusChanged(address indexed user, bool status);

    constructor(
        address _defaultAdmin,
        string memory _name,
        string memory _symbol,
        address _royaltyRecipient,
        uint128 _royaltyBps,
        address _primarySaleRecipient
    )
        ERC721Drop(
            _defaultAdmin,
            _name,
            _symbol,
            _royaltyRecipient,
            _royaltyBps,
            _primarySaleRecipient
        )
    {}
    // Function for users to opt in
    function optIn() public {
        optInStatus[msg.sender] = true;
        emit OptInStatusChanged(msg.sender, true);
    }

    // Function for users to opt out
    function optOut() public {
        optInStatus[msg.sender] = false;
        emit OptInStatusChanged(msg.sender, false);
    }

}
