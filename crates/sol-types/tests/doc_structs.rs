use ethers_primitives::{Address, U256};
use ethers_sol_types::{sol, SolType};
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
        baz: vec![Address::repeat_byte(0x11), Address::repeat_byte(0x22)],
    };

    let _nested = Nested {
        a: [my_foo.clone(), my_foo.clone()],
        b: Address::zero(),
    };

    let abi_encoded = Foo::encode(my_foo);
    assert_eq!(
        abi_encoded,
        hex! {
            "000000000000000000000000000000000000000000000000000000000000002a" // bar
            "0000000000000000000000000000000000000000000000000000000000000040" // baz offset
            "0000000000000000000000000000000000000000000000000000000000000002" // baz length
            "0000000000000000000000001111111111111111111111111111111111111111" // baz[0]
            "0000000000000000000000002222222222222222222222222222222222222222" // baz[1]
        }
    )
}
