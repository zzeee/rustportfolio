pragma solidity >0.8.0;

interface iOraoInfo {
    function test()  external;
    function addOraoInfo(uint256 _providerId, uint256 _protocolId, uint256 _vectorId, uint256 _value, uint256 _base, uint256 _timestamp) external returns (uint256);
}