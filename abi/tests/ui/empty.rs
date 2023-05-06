use ethers_abi_enc::sol;

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
