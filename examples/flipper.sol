// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract flipper {
	bool private value;

	/// Constructor that initializes the `bool` value to the given `init_value`.
	constructor() {
      internal_call(random_value());
	}

  function internal_call(bool initvalue) public {
      value = initvalue;
  }

  function random_value() pure public returns (bool) {
      return true;
  }

  /// A message that can be called on instantiated contracts.
	/// This one flips the value of the stored `bool` from `true`
	/// to `false` and vice versa.
	function flip() public {
		value = !value;
	}

	/// Simply returns the current value of our `bool`.
	function get() public view returns (bool) {
		return value;
	}

  /// Example of a function with no name
  receive() external payable {}
}
