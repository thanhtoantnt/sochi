 

pragma solidity ^0.4.11;

 
library SafeMath {
  function mul(uint256 a, uint256 b) internal constant returns (uint256) {
    uint256 c = a * b;
    assert(a == 0 || c / a == b);
    return c;
  }

  function div(uint256 a, uint256 b) internal constant returns (uint256) {
     
    uint256 c = a / b;
     
    return c;
  }

  function sub(uint256 a, uint256 b) internal constant returns (uint256) {
    assert(b <= a);
    return a - b;
  }

  function add(uint256 a, uint256 b) internal constant returns (uint256) {
    uint256 c = a + b;
    assert(c >= a);
    return c;
  }
}

 
contract token { function transfer(address receiver, uint amount){  } }
contract Crowdsale {
  using SafeMath for uint256;

   
   
  address public wallet;
   
  address public addressOfTokenUsedAsReward;

  uint256 public price = 300;
  uint256 public minBuy;
  uint256 public maxBuy;

  token tokenReward;

   
  


   
  uint256 public startTime;
   
   
  uint256 public weiRaised;

   
  event TokenPurchase(address indexed purchaser, address indexed beneficiary, uint256 value, uint256 amount);


  function Crowdsale() {
     
    wallet = 0x60248650a112DDaEe8b4D53eE0D1BEdB52BC3A6b;
     
     
    addressOfTokenUsedAsReward = 0xAA6296f88c3fb16881750E7166240cD9c82e8172;


    tokenReward = token(addressOfTokenUsedAsReward);
  }

  bool public started = false;

  function startSale(uint256 _delayInMinutes){
    if (msg.sender != wallet) throw;
    startTime = now + _delayInMinutes*1 minutes;
    started = true;
  }

  function stopSale(){
    if(msg.sender != wallet) throw;
    started = false;
  }

  function setPrice(uint256 _price){
    if(msg.sender != wallet) throw;
    price = _price;
  }

  function setMinBuy(uint256 _minBuy){
    if(msg.sender!=wallet) throw;
    minBuy = _minBuy;
  }

  function setMaxBuy(uint256 _maxBuy){
    if(msg.sender != wallet) throw;
    maxBuy = _maxBuy;
  }

  function changeWallet(address _wallet){
    if(msg.sender != wallet) throw;
    wallet = _wallet;
  }

  function changeTokenReward(address _token){
    if(msg.sender!=wallet) throw;
    tokenReward = token(_token);
  }

   
  function () payable {
    buyTokens(msg.sender);
  }

   
  function buyTokens(address beneficiary) payable {
    require(beneficiary != 0x0);
    require(validPurchase());

    uint256 weiAmount = msg.value;

     
    uint256 tokens = (weiAmount) * price; 

    if(minBuy!=0){
      if(tokens < minBuy*10**18) throw;
    }

    if(maxBuy!=0){
      if(tokens > maxBuy*10**18) throw;
    }

     
    weiRaised = weiRaised.add(weiAmount);
    
     
     

    tokenReward.transfer(beneficiary, tokens);
    TokenPurchase(msg.sender, beneficiary, weiAmount, tokens);
    forwardFunds();
  }

   
   
  function forwardFunds() internal {
     
    if (!wallet.send(msg.value)) {
      throw;
    }
  }

   
  function validPurchase() internal constant returns (bool) {
    bool withinPeriod = started&&(now>=startTime);
    bool nonZeroPurchase = msg.value != 0;
    return withinPeriod && nonZeroPurchase;
  }

  function withdrawTokens(uint256 _amount) {
    if(msg.sender!=wallet) throw;
    tokenReward.transfer(wallet,_amount);
  }
}