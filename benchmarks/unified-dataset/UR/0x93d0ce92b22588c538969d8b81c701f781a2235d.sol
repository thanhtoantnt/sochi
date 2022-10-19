 

 

pragma solidity ^0.4.11;

contract Token {
    string public standard = 'Token 0.1.8 diceybit.com';
    string public name = 'DICEYBIT.COM';
    string public symbol = 'dСBT';
    uint8 public decimals = 0;
    uint256 public totalSupply = 100000000;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowed;

    function Token() {
        balanceOf[msg.sender] = totalSupply;
    }

     
     
     
    function transfer(address _to, uint256 _value) {
        require(_value > 0 && balanceOf[msg.sender] >= _value);

        balanceOf[msg.sender] -= _value;
        balanceOf[_to] += _value;

        Transfer(msg.sender, _to, _value);
    }

     
     
     
     
    function transferFrom(address _from, address _to, uint256 _value) {
        require(_value > 0 && balanceOf[_from] >= _value && allowed[_from][msg.sender] >= _value);

        balanceOf[_from] -= _value;
        balanceOf[_to] += _value;
        allowed[_from][msg.sender] -= _value;

        Transfer(_from, _to, _value);
    }

     
     
     
    function approve(address _spender, uint256 _value) {
        allowed[msg.sender][_spender] = _value;
    }

     
     
     
     
    function allowance(address _owner, address _spender) constant returns(uint256 remaining) {
        return allowed[_owner][_spender];
    }

     
     
     
    function getBalanceOf(address _who) returns(uint256 amount) {
        return balanceOf[_who];
    }
}