var ContractName = artifacts.require("HotDollarsToken");

module.exports = function(deployer) {
  deployer.deploy(ContractName);
};

