use alloy_sol_types::sol;

sol! {}

sol! {
    struct EmptyStruct {}
}

sol! {
    enum EmptyEnum {}
}

// OK
sol! {
    contract EmptyContract {}
}

sol! {
    error EmptyError();
}

sol! {
    event EmptyEvent();
}

sol! {
    function emptyFunction();
}

fn main() {}
