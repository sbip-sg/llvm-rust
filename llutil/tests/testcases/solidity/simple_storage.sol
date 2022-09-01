// SPDX-License-Identifier: GPL-3.0

// Source:
// https://docs.soliditylang.org/en/v0.8.16/introduction-to-smart-contracts.html

pragma solidity >=0.4.16 <0.9.0;

contract SimpleStorage {
  uint storedData;

  function set(uint x) public {
    storedData = x;
  }

  function get() public view returns (uint) {
    return storedData;
  }
}
