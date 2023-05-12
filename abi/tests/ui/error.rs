use ethers_abi_enc::sol;

sol! {
    error MissingParens1
}

sol! {
    error MissingParens2;
}

sol! {
    error MissingSemi()
}

sol! {
    error TrailingComma(uint256,);
    error Valid(uint256);
}

fn main() {}
