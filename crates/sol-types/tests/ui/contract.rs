use alloy_sol_types::sol;

sol! {
    contract MissingBraces1
}

sol! {
    contract MissingBraces2 is A
}

sol! {
    contract MissingInheritance1 is
}

sol! {
    contract MissingInheritance2 is;
}

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
