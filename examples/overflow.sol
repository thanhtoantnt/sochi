contract Overflow {
    uint private r = 0;
    function addValue(uint value) public returns (bool) {
        // possible over flow
        r += value;
        return true;
    }
}
