use ethers_sol_types::sol;

sol! {}

sol! {
    struct EmptyStruct {}
}

sol! {
    error EmptyError();
}

sol! {
    function emptyFunction();
}

fn main() {}
