use alloy_sol_types::sol;

sol! {
    #![sol(abi)]

    contract C {
        event EV0();
        event EV1(uint a);
        event EV2(uint, bool b);

        error ER0();
        error ER1(uint a);
        error ER2(uint, bool b);

        function F00();
        function F01(uint a);
        function F02(uint, bool b);

        function F10();
        function F11(uint a) returns (uint a);
        function F12(uint, bool b) returns (uint, bool b);
    }
}

mod other {
    use super::*;

    sol! {
        #![sol(abi)]

        event EV0();
        event EV1(uint a);
        event EV2(uint, bool b);

        error ER0();
        error ER1(uint a);
        error ER2(uint, bool b);

        function F00();
        function F01(uint a);
        function F02(uint, bool b);

        function F10();
        function F11(uint a) returns (uint a);
        function F12(uint, bool b) returns (uint, bool b);
    }
}

fn main() {}
