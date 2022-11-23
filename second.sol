pragma solidity ^0.7.0;

contract VecAdd{
    function vec_add(uint[2] memory a, uint[2] memory b) public returns (uint[2] memory c){
        c[0] = a[0] + b[0]; //overflow
        c[1] = a[1] + b[1]; //overflow
    }
}
