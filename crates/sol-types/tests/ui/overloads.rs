use alloy_sol_types::sol;

mod function {
    use super::*;

    sol! {
        function overloaded();
        function overloaded(uint256);
        function overloaded(uint256,address);
        function overloaded(address);
        function overloaded(address,string);
    }

    sol! {
        function overloadTaken();
        function overloadTaken(uint256);
        function overloadTaken_0();
        function overloadTaken_1();
        function overloadTaken_2();
    }

    sol! {
        function sameOverload();
        function sameOverload();
    }

    sol! {
        function sameTysOverload1(uint256[]memory a);
        function sameTysOverload1(uint256[]storage b);
    }

    sol! {
        function sameTysOverload2(string memory,string storage);
        function sameTysOverload2(string storage b,string calldata);
    }
}

mod event {
    use super::*;

    sol! {
        event overloaded();
        event overloaded(uint256);
        event overloaded(uint256,address);
        event overloaded(address);
        event overloaded(address,string);
    }

    sol! {
        event overloadTaken();
        event overloadTaken(uint256);
        event overloadTaken_0();
        event overloadTaken_1();
        event overloadTaken_2();
    }

    sol! {
        event sameOverload();
        event sameOverload();
    }

    sol! {
        event sameTysOverload1(uint256[] a);
        event sameTysOverload1(uint256[] b);
    }

    sol! {
        event sameTysOverload2(string, string);
        event sameTysOverload2(string, string);
    }
}

/*
mod error {
    use super::*;

    sol! {
        error overloaded();
        error overloaded(uint256);
        error overloaded(uint256,address);
        error overloaded(address);
        error overloaded(address,string);
    }

    sol! {
        error overloadTaken();
        error overloadTaken(uint256);
        error overloadTaken_0();
        error overloadTaken_1();
        error overloadTaken_2();
    }

    sol! {
        error sameOverload();
        error sameOverload();
    }

    sol! {
        error sameTysOverload1(uint256[] a);
        error sameTysOverload1(uint256[] b);
    }

    sol! {
        error sameTysOverload2(string, string);
        error sameTysOverload2(string, string);
    }
}
*/

fn main() {}
