use alloy_primitives::{B256, b256};
use alloy_sol_types::{SolStruct, eip712_domain, sol};

#[test]
fn encode_type_nesting() {
    sol! {
        #![sol(all_derives)]

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
    assert_eq!(
        D::default().eip712_type_hash(),
        b256!("c4acb12ce6b40000061de8f15de55637de14657a4a32b5d85968bbfc2198e898")
    );
}

#[test]
fn encode_data_nesting() {
    sol! {
        #![sol(all_derives)]

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

    assert_eq!(
        Mail::eip712_encode_type(),
        "Mail(Person from,Person to,string contents)Person(string name,address wallet)"
    );
    assert_eq!(
        Mail::default().eip712_type_hash(),
        b256!("a0cedeb2dc280ba39b857546d74f5549c3a1d7bdc2dd96bf881f76108e23dac2")
    );
    assert_eq!(
        Person::default().eip712_type_hash(),
        b256!("b9d8c78acf9b987311de6c7b45bb6a9c8e1bf361fa7fd3467a2163f994c79500")
    );

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

#[test]
fn test_eip712_interface_prefix() {
    sol! {
        interface IParentInterfaceA {
            struct ParentStructA {
                uint256 numberA;
            }
        }

        interface IParentInterfaceB {
            struct ParentStructB {
                uint256 numberB;
            }
        }

        struct MyStruct {
            IParentInterfaceA.ParentStructA structA;
            IParentInterfaceB.ParentStructB structB;
        }
    }

    assert_eq!(
        MyStruct::eip712_encode_type(),
        "MyStruct(ParentStructA structA,ParentStructB structB)ParentStructA(uint256 numberA)ParentStructB(uint256 numberB)"
    );
}

#[test]
fn test_eip712_strips_custom_type_namespaces() {
    sol! {
        contract A {
            struct StructB {
                uint256 x;
            }
        }
        struct StructA {
            A.StructB[] b;
            A.StructB[3] c;
            A.StructB[][] d;
            A.StructB e;
        }
    }

    assert_eq!(
        StructA::eip712_encode_type(),
        "StructA(StructB[] b,StructB[3] c,StructB[][] d,StructB e)StructB(uint256 x)"
    );
}

#[test]
fn encode_type_resolves_in_declaring_namespace() {
    sol! {
        #![sol(all_derives)]

        contract Outer {
            type Id is uint64;

            enum Level {
                Low,
                High
            }

            struct Inner {
                Id id;
                Level level;
            }

            struct Wrapper {
                Inner inner;
                Inner[] more;
            }
        }

        struct Top {
            Outer.Wrapper wrapper;
            Outer.Inner inner;
        }
    }

    assert_eq!(
        Top::eip712_encode_type(),
        "Top(Wrapper wrapper,Inner inner)Inner(uint64 id,uint8 level)Wrapper(Inner inner,Inner[] more)"
    );
    let inner = Outer::Inner { id: 0, level: Outer::Level::Low };
    let top = Top { wrapper: Outer::Wrapper { inner: inner.clone(), more: vec![] }, inner };
    assert_eq!(
        top.eip712_type_hash(),
        b256!("b9e3a672c6f13136c1e3191881e3719beb78be11a605e1b1856d893a2d543067")
    );
}

#[test]
fn encode_type_prefers_local_struct_over_global() {
    sol! {
        struct Inner {
            uint256 a;
        }

        contract Outer {
            struct Inner {
                bytes b;
            }

            struct Wrapper {
                Inner inner;
            }
        }
    }

    assert_eq!(Outer::Wrapper::eip712_encode_type(), "Wrapper(Inner inner)Inner(bytes b)");
}

#[test]
fn encode_type_sorts_by_type_name() {
    sol! {
        #[sol(rename = "A$B")]
        struct Dollar {
            uint256 a;
        }

        struct A {
            uint256 b;
        }

        struct Root {
            Dollar dollar;
            A a;
        }
    }

    assert_eq!(Root::eip712_encode_type(), "Root(A$B dollar,A a)A(uint256 b)A$B(uint256 a)");
}

#[test]
fn encode_type_unescapes_rust_keywords() {
    sol! {
        struct const {
            uint256 ref;
        }

        struct Order {
            const loop;
            address fn;
        }
    }

    assert_eq!(Order::eip712_encode_type(), "Order(const loop,address fn)const(uint256 ref)");
}
