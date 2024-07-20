//SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

contract FuckYouAlice {
    //contract goes here
    string public message;

    constructor() {
        message = "Fuck you, Alice";
    }

    function breakupMessage() public view returns (string memory) {
        return message;
    }
}
