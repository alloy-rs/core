use alloy_sol_types::sol;

// https://github.com/alloy-rs/core/issues/729

sol! {
    error func_2093253501(bytes);
    error transfer(address,uint256);

    function func_2093253501(bytes);
    function transfer(address,uint256);

    error BlazingIt4490597615();

    contract A {
        error func_2093253501(bytes);
        error transfer(address,uint256);

        function func_2093253501(bytes);
        function transfer(address,uint256);

        error BlazingIt4490597615();
    }
}

// This is OK.
mod namespaced {
    use alloy_sol_types::sol;

    sol! {
        function func_2093253501(bytes);

        contract B {
            function transfer(address,uint256);
        }
    }
}

fn main() {}
