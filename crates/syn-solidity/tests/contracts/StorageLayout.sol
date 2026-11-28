// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.29;

contract A layout at 2**255 - 42 {
    uint x;
}

contract B layout at 0xAAAA + 0x11 {
    uint[3] x;
}

contract P {
    uint p;
}

contract Q {
    uint q;
}

contract C is P, Q layout at 1 {}

contract D layout at 1 is P, Q {}

contract F {
    function f() external payable {}

    function g() external {
        this.f{value: 1}();
        try this.f() {} catch {}
    }
}
