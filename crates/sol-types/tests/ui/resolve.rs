use alloy_sol_types::sol;

sol! {
    struct A {
        B a;
    }

    struct B {
        A a;
    }
}

sol! {
    struct A {
        B a;
    }

    struct B {
        C c;
    }

    struct C {
        A a;
    }
}

sol! {
    struct A {
        B a;
    }

    struct B {
        C c;
    }

    struct C {
        D d;
    }

    struct D {
        A a;
    }
}

fn main() {}
