use alloy_sol_types::sol;

sol! {
    import *;
}

sol! {
    import * as foo;
}

sol! {
    import * as foo from;
}

// OK
sol! {
    import "path";
    import "path" as foo;

    import {} from "path";
    import { a, b as c, d } from "path";

    import * from "path";
    import * as foo from "path";
}

fn main() {}
