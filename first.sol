pragma solidity ^0.8.0;

contract Test {
    struct Signature { uint8 _v; bytes32 _r; bytes32 _s;}
        function Claim(bytes32 signedMsg, Signature memory sig) public {
        address signer = ecrecover(signedMsg, sig._v, sig._r, sig._s);
        // require(signer == owner);
        payable(msg.sender).transfer(address(this).balance); // unchecked-send
    }
}
