use alloy_sol_types::sol;

sol! {
    contract C {
        contract Nested {}
    }
}

sol! {
    interface C {
        library Nested {}
    }
}

sol! {
    abstract contract C {
        interface Nested {}
    }
}

fn main() {}
