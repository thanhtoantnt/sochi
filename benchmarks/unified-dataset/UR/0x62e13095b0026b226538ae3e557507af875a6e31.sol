 

pragma solidity ^0.4.11;

 
 
 
 
 
 
 


 
 
library SafeMath {
    function mul(uint256 a, uint256 b) internal constant returns (uint256) {
        uint256 c = a * b;
        assert(a == 0 || c / a == b);
        return c;
    }

    function div(uint256 a, uint256 b) internal constant returns (uint256) {
        assert(b > 0);
        uint256 c = a / b;
        assert(a == b * c + a % b);
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


 
contract Ownable {
    address public owner;

     
    function Ownable() {
        owner = msg.sender;
    }

     
    modifier onlyOwner() {
        require(msg.sender == owner);
        _;
    }


     
     
    function transferOwnership(address newOwner) onlyOwner {
        if (newOwner != address(0)) {
            owner = newOwner;
        }
    }

}


 
contract Claimable is Ownable {
    address public pendingOwner;

     
    modifier onlyPendingOwner() {
        require(msg.sender == pendingOwner);
        _;
    }

     
     
    function transferOwnership(address newOwner) onlyOwner {
        pendingOwner = newOwner;
    }

     
    function claimOwnership() onlyPendingOwner {
        owner = pendingOwner;
        pendingOwner = 0x0;
    }
}


 
contract Contactable is Ownable{

    string public contactInformation;

     
     
    function setContactInformation(string info) onlyOwner{
        contactInformation = info;
    }
}


 
contract HasNoEther is Ownable {

     
    function HasNoEther() payable {
        require(msg.value == 0);
    }

     
    function() external {
    }

     
    function reclaimEther() external onlyOwner {
        assert(owner.send(this.balance));
    }
}


 
contract ERC20 {
    using SafeMath for uint256;

     
    mapping(address => uint256) balances;
    mapping (address => mapping (address => uint256)) allowed;
    uint256 _totalSupply;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

     
    function totalSupply() constant returns (uint256) {
        return _totalSupply;
    }

     
     
     
    function balanceOf(address _owner) constant returns (uint256 balance) {
        return balances[_owner];
    }

     
     
     
    function transfer(address _to, uint256 _value) returns (bool) {
        require(_to != 0x0 );
        require(_value > 0 );

        balances[msg.sender] = balances[msg.sender].sub(_value);
        balances[_to] = balances[_to].add(_value);

        Transfer(msg.sender, _to, _value);
        return true;
    }

     
     
     
     
    function transferFrom(address _from, address _to, uint256 _value) returns (bool) {
        require(_from != 0x0 );
        require(_to != 0x0 );
        require(_value > 0 );

        var _allowance = allowed[_from][msg.sender];

        balances[_to] = balances[_to].add(_value);
        balances[_from] = balances[_from].sub(_value);
        allowed[_from][msg.sender] = _allowance.sub(_value);

        Transfer(_from, _to, _value);
        return true;
    }

     
     
     
    function approve(address _spender, uint256 _value) returns (bool) {
        require(_spender != 0x0 );
         
         
         
         
        require((_value == 0) || (allowed[msg.sender][_spender] == 0));

        allowed[msg.sender][_spender] = _value;

        Approval(msg.sender, _spender, _value);
        return true;
    }

     
     
     
     
    function allowance(address _owner, address _spender) constant returns (uint256 remaining) {
        return allowed[_owner][_spender];
    }
}

contract StandardToken is ERC20 {
    string public name;
    string public symbol;
    uint256 public decimals;

    function isToken() public constant returns (bool) {
        return true;
    }
}

 
contract FreezableToken is StandardToken, Ownable {
    mapping (address => bool) public frozenAccounts;
    event FrozenFunds(address target, bool frozen);

     
    function freezeAccount(address target, bool freeze) onlyOwner {
        frozenAccounts[target] = freeze;
        FrozenFunds(target, freeze);
    }

     
    modifier canTransfer(address _sender) {
        require(!frozenAccounts[_sender]);

        _;
    }

    function transfer(address _to, uint256 _value) canTransfer(msg.sender) returns (bool success) {
         
        return super.transfer(_to, _value);
    }

    function transferFrom(address _from, address _to, uint256 _value) canTransfer(_from) returns (bool success) {
         
        return super.transferFrom(_from, _to, _value);
    }
}

 
contract TbkToken is Claimable, Contactable, HasNoEther, FreezableToken {
     
    function TbkToken(){
        uint256 _decimals = 18;
        uint256 _supply = 1000000000*(10**_decimals);

        _totalSupply = _supply;
        balances[msg.sender] = _supply;
        name = "Time Bank Coin";
        symbol = "TBK";
        decimals = _decimals;
        contactInformation = "Time Bank Contact Email:<a class="__cf_email__" data-cfemail="afc6c1c9c0efdbcdc4ccc7cec6c181c6c0" href="/cdn-cgi/l/email-protection">[email??protected]</a>";
    }
}


contract TbkTokenLock is Ownable, HasNoEther {
    using SafeMath for uint256;

     
    uint256 public investorCount;
     
    uint256 public totalClaimed;
     
    uint256 public tokensAllocatedTotal;

     
    uint256 public tokensAtLeastHold;

    struct balance{
        address investor;
        uint256 amount;
        uint256 freezeEndAt;
        bool claimed;
    }

    mapping(address => balance[]) public balances;
     
    mapping(address => uint256) public claimed;

     
    FreezableToken public token;

     
    event Invested(address investor, uint256 amount, uint256 hour);

     
    event Distributed(address investors, uint256 count);

     
    function TbkTokenLock(address _owner, address _token) {
        require(_owner != 0x0);
        require(_token != 0x0);

        owner = _owner;
        token = FreezableToken(_token);
    }

   

     
    function withdrawLeftTokens() onlyOwner {
        token.transfer(owner, token.balanceOf(address(this))-tokensAtLeastHold);
    }

     
     
    function getBalance() public constant returns (uint256) {
        return token.balanceOf(address(this));
    }

   
}