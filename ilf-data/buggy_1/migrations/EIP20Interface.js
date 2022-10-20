
var ContractName = artifacts.require("EIP20Interface");

module.exports = function(deployer) {
  deployer.deploy(ContractName);
};

