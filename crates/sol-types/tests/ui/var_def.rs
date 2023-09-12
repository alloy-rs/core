use alloy_sol_types::sol;

// OK
sol! {
    struct Simple {
        uint a;
    }

    mapping(int => Simple) public simpleMap;
}

// Not OK
sol! {
    struct Complex1 {
        uint[] a;
    }

    mapping(int => Complex1) public complexMap;
}

// OK
sol! {
    struct DoubleComplex {
        Complex2 a;
    }
    struct Complex2 {
        uint[] a;
    }

    mapping(int => DoubleComplex) public complexMap;
}

fn main() {}
