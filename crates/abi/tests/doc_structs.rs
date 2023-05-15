use ethers_abi_enc::{sol, SolType};
use ethers_primitives::{Address, U256};
use hex_literal::hex;

// Struct definitions will generate a struct with the same name and fields.
// No casing convention is enforced.
sol! {
    struct Foo {
        uint256 bar;
        address[] baz;
    }

    /// Nested struct.
    struct Nested {
        Foo[2] a;
        address b;
    }

    // TODO: enums
    /*
    enum Enum {
        A,
        B,
        C,
    }
    */
}

#[test]
fn structs() {
    let my_foo = Foo {
        bar: U256::from(42),
        baz: vec![Address::zero(); 2],
    };

    let _nested = Nested {
        a: [my_foo.clone(), my_foo.clone()],
        b: Address::zero(),
    };

    let abi_encoded = Foo::encode(my_foo);
    assert_eq!(
        abi_encoded,
        hex! {
            "000000000000000000000000000000000000000000000000000000000000002a"
            "0000000000000000000000000000000000000000000000000000000000000040"
            "0000000000000000000000000000000000000000000000000000000000000002"
            "0000000000000000000000000000000000000000000000000000000000000000"
            "0000000000000000000000000000000000000000000000000000000000000000"
        }
    )
}
