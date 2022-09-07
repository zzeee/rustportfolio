

var interfaces = artifacts.require("./IOraoInfo.sol");
var OraoInfo2 = artifacts.require("./OraoInfo.sol");

module.exports = function(deployer) {
  deployer.deploy(interfaces);
  deployer.link(interfaces, OraoInfo2);
  deployer.link(interfaces, OraoInfo2);
  deployer.deploy(OraoInfo2, 12000);
};
