use alloy_sol_types::sol;

sol! {
    #[sol(rename = "same")]
    function first(uint256);
    #[sol(rename = "same")]
    function second(uint256);
}

sol! {
    function overloaded(uint256);
    function overloaded(string);
    #[sol(rename = "overloaded")]
    function other(uint256);
}

sol! {
    event Old(uint256);
    #[sol(rename = "Old")]
    event New(uint256);
}

sol! {
    uint256 public value;
    #[sol(rename = "value")]
    function getter();
}

sol! {
    struct InvalidFieldAttribute {
        #[sol(rename = "one", rename = "two")]
        uint256 field;
    }
}

fn main() {}
