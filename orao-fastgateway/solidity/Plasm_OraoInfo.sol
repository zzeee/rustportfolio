// SPDX-License-Identifier: MIT
pragma solidity >0.8.0;

import "./IOraoInfo.sol";

contract OraoInfo is iOraoInfo {
    address public owner = msg.sender;

    event EOraoInfo(address indexed _from, uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value,uint256 _base, uint256 _timestamp);

    modifier onlyOwner() {
        require(
            msg.sender == owner,
            "This function is restricted to the contract's owner"
        );
        _;
    }


    function check2() public pure returns (uint256) {
        return 100;
    }

    function check(uint256 data) public pure returns (uint256) {
        return data;
    }

    function check3(uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value,uint256 _base,  uint256 _timestamp) public pure returns (uint256){
        return _providerId;

    }
    /**
     * @dev Add a new provider data and emit an event to let the other listeners, e.g. front end,
     *     listening on that event and consuming the data
     * @param _providerId the provider id
     * @param _protocolId the protocol id
     * @param _vectorId the vector id
     * @param _value the value
     * @param _timestamp the unix based timestamp
     */

    function addOraoInfo(uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value, uint256 _base, uint256 _timestamp) public onlyOwner returns (uint256) {
        emit EOraoInfo(msg.sender, _providerId, _protocolId, _vectorId,_base, _value, _timestamp);
        return block.timestamp;
    }
}
