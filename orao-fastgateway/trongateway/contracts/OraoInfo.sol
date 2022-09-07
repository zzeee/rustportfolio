// SPDX-License-Identifier: MIT
pragma solidity >0.4.18 < 0.6.0;
// https://tronscan.org/#/contract/TWcDXoSwh1u3kRAoNqj3NSEEsUcJcytR4j
// 4471a89f-3566-425c-8beb-cb4896307521
/*

    (base58) TWcDXoSwh1u3kRAoNqj3NSEEsUcJcytR4j
    (hex) 41e2625945cdaa6e680c5e3102f6adc012fbde6a06

*/
import "./IOraoInfo.sol";

contract OraoInfo
 // is iOraoInfo
{
    address public owner = msg.sender;

    event EOraoInfo(address indexed _from, uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value,uint256 _base, uint256 _timestamp);

    function check2() public pure returns (uint256) {
        return 100;
    }

    function check(uint256 data) public pure returns (uint256) {
        return data;
    }

 function addOraoInfo(uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value, uint256 _base, uint256 _timestamp) public returns (uint256) {
       emit EOraoInfo(msg.sender, _providerId, _protocolId, _vectorId,_base, _value, _timestamp);
        return block.timestamp;
    }
}
