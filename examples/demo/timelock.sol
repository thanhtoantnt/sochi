// @source: https://github.com/sigp/solidity-security-blog
// Modified & simplified from smartbugs benchmark

pragma solidity ^0.5.1;

// User deposits balance and locks it for 1 year.
// Logic:
//    + User cannot withdraw before the deadline
//    + User can only increase locktime
// Bugs:
//    + High Severity: Increase-locktime cause integer overflow => locktime decrease
//    + Medium Severity: Use of timestamp opcode to measure time, can be inaccurate.

contract TimeLockVault {
    // Data strutures for storing balances + locktime
     mapping(address => uint) public balances;
     mapping(address => uint) public lockTime;

     // User deposit and lock.
     function deposit() public payable {
         balances[msg.sender] += msg.value;
         lockTime[msg.sender] = now + 365 days;
     }

     // Intended logic: increase lockTime to more than 1 year
     // Unexpected behavior: lockTime is reduced because of overflow bug.
     function increaseLockTime(uint secondsToIncrease) public {
         lockTime[msg.sender] = lockTime[msg.sender] + secondsToIncrease;
     }

     // Enforcing specific domain logic (locking time & balance)
     function withdraw() public {
         require(balances[msg.sender] > 0);
         require(now > lockTime[msg.sender]);
         uint transferValue = balances[msg.sender];
         balances[msg.sender] = 0;
         msg.sender.transfer(transferValue);
     }
 }
