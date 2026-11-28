// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

type Price is uint128;

enum Status {
    Idle,
    Busy
}

struct Field {
    uint256 transient;
}

error Failure(uint256 transient);

event Emitted(uint256 indexed transient);

contract TransientVariables {
    bool transient locked;
    uint256 transient a;
    int8 transient b;
    address payable transient c;
    bytes32 transient d;
    Price transient e;
    Status transient f;

    uint256 public transient g;
    uint256 transient public h;
    uint256 internal transient i;
    uint256 transient private j;

    modifier nonReentrant() {
        require(!locked);
        locked = true;
        _;
        locked = false;
    }
}

contract NamedTransient {
    uint256 transient;
}

contract PublicNamedTransient {
    uint256 public transient = 1;
}

contract ConstantNamedTransient {
    uint256 constant transient = 1;
}

contract TransientNamedTransient {
    uint256 transient transient;
}

contract TransientIdentifiers {
    function parameter(uint256 transient) external pure returns (uint256) {
        return transient;
    }

    function returnValue() external pure returns (uint256 transient) {
        transient = 1;
    }

    function localVariable() external pure returns (uint256) {
        uint256 transient = 1;
        return transient;
    }

    function uninitializedLocal() external pure returns (uint256 x) {
        uint256 transient;
        x = transient;
    }
}
