/* From: https://github.com/crytic/slither/blob/master/examples/printers/call_graph.sol */

library Library {
    function library_func() public {
    }
}

contract ContractA {
    uint256 public val = 0;

    function my_func_a() public {
        Library.library_func();
    }
}

contract ContractB {
    ContractA a;

    constructor() {
        a = new ContractA();
    }

    function my_func_b() public {
        a.my_func_a();
        my_second_func_b();
    }

    function my_func_a() public {
        my_second_func_b();
    }

    function my_second_func_b() public {
        a.val();
    }
}
