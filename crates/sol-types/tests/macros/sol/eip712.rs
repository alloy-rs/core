use alloy_primitives::B256;
use alloy_sol_types::{eip712_domain, sol, SolStruct};

#[test]
fn encode_type_nesting() {
    sol! {
        struct A {
            uint256 a;
        }

        struct B {
            bytes32 b;
        }

        struct C {
            A a;
            B b;
        }

        struct D {
            C c;
            A a;
            B b;
        }
    }

    assert_eq!(A::eip712_encode_type(), "A(uint256 a)");
    assert_eq!(B::eip712_encode_type(), "B(bytes32 b)");
    assert_eq!(C::eip712_encode_type(), "C(A a,B b)A(uint256 a)B(bytes32 b)");
    assert_eq!(D::eip712_encode_type(), "D(C c,A a,B b)A(uint256 a)B(bytes32 b)C(A a,B b)");
}

#[test]
fn encode_data_nesting() {
    sol! {
        struct Person {
            string name;
            address wallet;
        }

        struct Mail {
            Person from;
            Person to;
            string contents;
        }
    }
    let domain = eip712_domain! {};

    let mail = Mail {
        from: Person {
            name: "Cow".to_owned(),
            wallet: "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826".parse().unwrap(),
        },
        to: Person {
            name: "Bob".to_owned(),
            wallet: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB".parse().unwrap(),
        },
        contents: "Hello, Bob!".to_owned(),
    };

    assert_eq!(
        alloy_sol_types::SolStruct::eip712_signing_hash(&mail, &domain),
        "25c3d40a39e639a4d0b6e4d2ace5e1281e039c88494d97d8d08f99a6ea75d775".parse::<B256>().unwrap()
    )
}
