use alloy_sol_types::sol;

sol! {
    #[sol(rename = "same")]
    function first(uint256);
    #[sol(rename = "same")]
    function second(uint256);
}

sol! {
    struct InvalidFieldAttribute {
        #[sol(rename = "one", rename = "two")]
        uint256 field;
    }
}

fn main() {}
