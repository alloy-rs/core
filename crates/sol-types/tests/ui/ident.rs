use alloy_sol_types::sol;

sol! {
    struct Reserved {
        bool uint256;
    }
}

sol! {
    struct NotAllowed {
        bool è;
    }
}

sol! {
    struct NotAllowedRaw {
        bool r#è;
    }
}

sol! {
    struct Allowed {
        bool r#uint256;
        // bool r#$dollars$;
        bool _underscores_;
        // bool $_$_$_;
    }
}

fn main() {}
