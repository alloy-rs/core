use alloy_json_abi::{
    Constructor, Error, EventParam, Fallback, Function, Param, Receive, StateMutability,
};
use alloy_sol_types::{JsonAbiExt, sol};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

macro_rules! abi_map {
    ($($k:expr => $v:expr),* $(,)?) => {
        BTreeMap::from([$(($k.into(), vec![$v])),*])
    };
}

#[test]
fn equal_abis() {
    use alloy_json_abi::InternalType;
    let contract = Contract::abi::contract();

    assert_eq!(contract.constructor, Contract::abi::constructor());
    assert_eq!(
        contract.constructor,
        Some(Constructor { inputs: Vec::new(), state_mutability: StateMutability::NonPayable })
    );

    assert_eq!(contract.fallback, Contract::abi::fallback());
    assert_eq!(contract.fallback, Some(Fallback { state_mutability: StateMutability::NonPayable }));

    assert_eq!(contract.receive, Contract::abi::receive());
    assert_eq!(contract.receive, Some(Receive { state_mutability: StateMutability::Payable }));

    assert_eq!(contract.functions, Contract::abi::functions());
    assert_eq!(
        *contract.function("F00").unwrap().first().unwrap(),
        Function {
            name: "F00".into(),
            inputs: vec![],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
    assert_eq!(
        *contract.function("F01").unwrap().first().unwrap(),
        Function {
            name: "F01".into(),
            inputs: vec![param("uint256 a")],
            outputs: vec![],
            state_mutability: StateMutability::Payable,
        }
    );
    assert_eq!(
        *contract.function("F02").unwrap().first().unwrap(),
        Function {
            name: "F02".into(),
            inputs: vec![param("uint256 "), param("bool b")],
            outputs: vec![],
            state_mutability: StateMutability::View,
        }
    );
    assert_eq!(
        *contract.function("F10").unwrap().first().unwrap(),
        Function {
            name: "F10".into(),
            inputs: vec![],
            outputs: vec![],
            state_mutability: StateMutability::Pure,
        }
    );
    assert_eq!(
        *contract.function("F11").unwrap().first().unwrap(),
        Function {
            name: "F11".into(),
            inputs: vec![param("uint256 a")],
            outputs: vec![param("uint256 a")],
            state_mutability: StateMutability::NonPayable,
        }
    );
    assert_eq!(
        *contract.function("F12").unwrap().first().unwrap(),
        Function {
            name: "F12".into(),
            inputs: vec![param("uint256 "), param("bool b")],
            outputs: vec![param("uint256 "), param("bool b")],
            state_mutability: StateMutability::NonPayable,
        }
    );
    assert_eq!(
        *contract.function("F20").unwrap().first().unwrap(),
        Function {
            name: "F20".into(),
            inputs: vec![param("uint256 "), param("uint256[] "), param("uint256[][1] ")],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
    assert_eq!(
        *contract.function("F21").unwrap().first().unwrap(),
        Function {
            name: "F21".into(),
            inputs: vec![
                Param {
                    ty: "tuple".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct".into()
                    }),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct[]".into()
                    }),
                },
                Param {
                    ty: "tuple[][2]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct[][2]".into()
                    }),
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
    let custom = Param {
        ty: "tuple".into(),
        name: "cs".into(),
        components: vec![param("uint256 custom")],
        internal_type: Some(InternalType::Struct {
            contract: Some("Contract".into()),
            ty: "CustomStruct".into(),
        }),
    };
    println!("{custom:#?}");
    assert_eq!(
        *contract.function("F22").unwrap().first().unwrap(),
        Function {
            name: "F22".into(),
            inputs: vec![
                Param {
                    ty: "tuple".into(),
                    name: String::new(),
                    components: vec![custom.clone(), param("bool cb")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct2".into()
                    }),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![custom.clone(), param("bool cb")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct2[]".into()
                    }),
                },
                Param {
                    ty: "tuple[][3]".into(),
                    name: String::new(),
                    components: vec![custom, param("bool cb")],
                    internal_type: Some(InternalType::Struct {
                        contract: Some("Contract".into()),
                        ty: "CustomStruct2[][3]".into()
                    }),
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
    assert_eq!(
        contract.functions,
        abi_map! {
            "F00" => Contract::F00Call::abi(),
            "F01" => Contract::F01Call::abi(),
            "F02" => Contract::F02Call::abi(),
            "F10" => Contract::F10Call::abi(),
            "F11" => Contract::F11Call::abi(),
            "F12" => Contract::F12Call::abi(),
            "F20" => Contract::F20Call::abi(),
            "F21" => Contract::F21Call::abi(),
            "F22" => Contract::F22Call::abi(),
        }
    );

    assert_eq!(contract.events, Contract::abi::events());
    assert_eq!(
        *contract.event("EV00").unwrap().first().unwrap(),
        alloy_json_abi::Event { name: "EV00".into(), inputs: vec![], anonymous: false }
    );
    assert_eq!(
        *contract.event("EV01").unwrap().first().unwrap(),
        alloy_json_abi::Event {
            name: "EV01".into(),
            inputs: vec![eparam("uint256 a", false)],
            anonymous: false,
        }
    );
    assert_eq!(
        *contract.event("EV02").unwrap().first().unwrap(),
        alloy_json_abi::Event {
            name: "EV02".into(),
            inputs: vec![eparam("uint256 ", false), eparam("bool b", false)],
            anonymous: false,
        }
    );
    assert_eq!(
        *contract.event("EV10").unwrap().first().unwrap(),
        alloy_json_abi::Event { name: "EV10".into(), inputs: vec![], anonymous: true }
    );
    assert_eq!(
        *contract.event("EV11").unwrap().first().unwrap(),
        alloy_json_abi::Event {
            name: "EV11".into(),
            inputs: vec![eparam("uint256 a", true)],
            anonymous: true,
        }
    );
    assert_eq!(
        *contract.event("EV12").unwrap().first().unwrap(),
        alloy_json_abi::Event {
            name: "EV12".into(),
            inputs: vec![eparam("uint256 ", false), eparam("bool b", true)],
            anonymous: true,
        }
    );
    assert_eq!(
        contract.events,
        abi_map! {
            "EV00" => Contract::EV00::abi(),
            "EV01" => Contract::EV01::abi(),
            "EV02" => Contract::EV02::abi(),
            "EV10" => Contract::EV10::abi(),
            "EV11" => Contract::EV11::abi(),
            "EV12" => Contract::EV12::abi(),
        }
    );

    assert_eq!(contract.errors, Contract::abi::errors());
    assert_eq!(
        *contract.error("ER0").unwrap().first().unwrap(),
        Error { name: "ER0".into(), inputs: vec![] }
    );
    assert_eq!(
        *contract.error("ER1").unwrap().first().unwrap(),
        Error { name: "ER1".into(), inputs: vec![param("uint256 a")] }
    );
    assert_eq!(
        *contract.error("ER2").unwrap().first().unwrap(),
        Error { name: "ER2".into(), inputs: vec![param("uint256 "), param("bool b")] }
    );
    assert_eq!(
        contract.errors,
        abi_map! {
            "ER0" => Contract::ER0::abi(),
            "ER1" => Contract::ER1::abi(),
            "ER2" => Contract::ER2::abi(),
        }
    );

    // Verify that contract-scoped and top-level items have identical ABIs
    // since they only use primitive types (uint256, bool) with no custom types.
    // Custom types would have different internal type qualifiers (contract: Some vs None).
    macro_rules! eq_modules {
        ($($items:ident),* $(,)?) => {$(
            assert_eq!(Contract::$items::abi(), not_contract::$items::abi());
        )*};
    }
    eq_modules!(
        EV00, EV01, EV02, EV10, EV11, EV12, ER0, ER1, ER2, F00Call, F01Call, F02Call, F10Call,
        F11Call, F12Call, F20Call,
    );

    // F21Call and F22Call use CustomStruct, so they will differ in internal types:
    // Contract-scoped will have contract: Some("Contract"), top-level will have contract: None
    // We verify the structure is correct but internal types differ as expected
    macro_rules! assert_contract_qualifier_differs {
        ($item:ident) => {{
            let contract_item = Contract::$item::abi();
            let toplevel_item = not_contract::$item::abi();
            assert_eq!(contract_item.name, toplevel_item.name);
            assert_eq!(contract_item.state_mutability, toplevel_item.state_mutability);
            assert_eq!(contract_item.inputs.len(), toplevel_item.inputs.len());
            assert_eq!(contract_item.outputs.len(), toplevel_item.outputs.len());

            // Helper function to recursively assert params match except for contract qualifiers
            fn assert_params_match(c: &Param, t: &Param) {
                assert_eq!(c.ty, t.ty);
                assert_eq!(c.name, t.name);
                assert_eq!(c.components.len(), t.components.len());

                // Internal types differ: Contract has Some("Contract"), toplevel has None
                match (&c.internal_type, &t.internal_type) {
                    (
                        Some(InternalType::Struct { contract: c_contract, ty: c_ty }),
                        Some(InternalType::Struct { contract: t_contract, ty: t_ty }),
                    ) => {
                        assert_eq!(c_contract, &Some("Contract".to_string()));
                        assert_eq!(t_contract, &None);
                        assert_eq!(c_ty, t_ty); // Type name is the same
                    }
                    (Some(c_int), Some(t_int)) => {
                        // Other internal types should match exactly
                        assert_eq!(c_int, t_int);
                    }
                    (None, None) => {}
                    _ => panic!(
                        "Internal type mismatch: {:?} vs {:?}",
                        c.internal_type, t.internal_type
                    ),
                }

                // Recursively check components
                for (c_comp, t_comp) in c.components.iter().zip(t.components.iter()) {
                    assert_params_match(c_comp, t_comp);
                }
            }

            // Input types match but internal type qualifiers differ
            for (c, t) in contract_item.inputs.iter().zip(toplevel_item.inputs.iter()) {
                assert_params_match(c, t);
            }

            // Output types match but internal type qualifiers differ
            for (c, t) in contract_item.outputs.iter().zip(toplevel_item.outputs.iter()) {
                assert_params_match(c, t);
            }
        }};
    }

    assert_contract_qualifier_differs!(F21Call);
    assert_contract_qualifier_differs!(F22Call);
}

#[test]
fn recursive() {
    sol! {
        #![sol(abi)]

        enum AccountAccessKind {
            Call,
            DelegateCall,
            CallCode,
            StaticCall,
            Create,
            SelfDestruct,
            Resume,
        }

        struct ChainInfo {
            uint256 forkId;
            uint256 chainId;
        }

        struct AccountAccess {
            ChainInfo chainInfo;
            AccountAccessKind kind;
            address account;
            address accessor;
            bool initialized;
            uint256 oldBalance;
            uint256 newBalance;
            bytes deployedCode;
            uint256 value;
            bytes data;
            bool reverted;
            StorageAccess[] storageAccesses;
        }

        struct StorageAccess {
            address account;
            bytes32 slot;
            bool isWrite;
            bytes32 previousValue;
            bytes32 newValue;
            bool reverted;
        }

        function stopAndReturnStateDiff() external returns (AccountAccess[] memory accesses);
    }

    use alloy_json_abi::InternalType;
    let chain_info = Param {
        ty: "tuple".into(),
        name: "chainInfo".into(),
        components: vec![param("uint256 forkId"), param("uint256 chainId")],
        internal_type: Some(InternalType::Struct { contract: None, ty: "ChainInfo".into() }),
    };
    let storage_accesses = Param {
        ty: "tuple[]".into(),
        name: "storageAccesses".into(),
        components: vec![
            param("address account"),
            param("bytes32 slot"),
            param("bool isWrite"),
            param("bytes32 previousValue"),
            param("bytes32 newValue"),
            param("bool reverted"),
        ],
        internal_type: Some(InternalType::Struct { contract: None, ty: "StorageAccess[]".into() }),
    };
    assert_eq!(
        stopAndReturnStateDiffCall::abi(),
        Function {
            name: "stopAndReturnStateDiff".into(),
            inputs: vec![],
            outputs: vec![Param {
                ty: "tuple[]".into(),
                name: "accesses".into(),
                components: vec![
                    chain_info,
                    Param {
                        ty: "uint8".into(),
                        name: "kind".into(),
                        components: vec![],
                        internal_type: Some(InternalType::Enum {
                            contract: None,
                            ty: "AccountAccessKind".into()
                        }),
                    },
                    param("address account"),
                    param("address accessor"),
                    param("bool initialized"),
                    param("uint256 oldBalance"),
                    param("uint256 newBalance"),
                    param("bytes deployedCode"),
                    param("uint256 value"),
                    param("bytes data"),
                    param("bool reverted"),
                    storage_accesses,
                ],
                internal_type: Some(InternalType::Struct {
                    contract: None,
                    ty: "AccountAccess[]".into()
                }),
            }],
            state_mutability: StateMutability::NonPayable,
        }
    );
}

#[test]
fn custom() {
    sol! {
        #![sol(abi)]

        type UDVT is uint32;

        enum Enum {
            A,
            B
        }

        struct CustomStruct {
            uint256 custom;
            uint256[] customArr;
            UDVT udvt;
            UDVT[] udvtArr;
            Enum e;
            Enum[] eArr;
        }

        struct CustomStruct2 {
            CustomStruct cs;
            CustomStruct[] csArr;
        }

        function myFunc(
            UDVT udvt,
            UDVT[] udvtArr,
            Enum e,
            Enum[] eArr,
            CustomStruct cs,
            CustomStruct[] csArr,
            CustomStruct2 cs2,
            CustomStruct2[] cs2Arr
        );
    }

    use alloy_json_abi::InternalType;
    let custom_struct = vec![
        param("uint256 custom"),
        param("uint256[] customArr"),
        Param {
            ty: "uint32".into(),
            name: "udvt".into(),
            components: vec![],
            internal_type: Some(InternalType::Other { contract: None, ty: "UDVT".into() }),
        },
        Param {
            ty: "uint32[]".into(),
            name: "udvtArr".into(),
            components: vec![],
            internal_type: Some(InternalType::Other { contract: None, ty: "UDVT[]".into() }),
        },
        Param {
            ty: "uint8".into(),
            name: "e".into(),
            components: vec![],
            internal_type: Some(InternalType::Enum { contract: None, ty: "Enum".into() }),
        },
        Param {
            ty: "uint8[]".into(),
            name: "eArr".into(),
            components: vec![],
            internal_type: Some(InternalType::Enum { contract: None, ty: "Enum[]".into() }),
        },
    ];
    let custom_struct_erased = custom_struct.clone();
    let custom_struct2 = vec![
        Param {
            ty: "tuple".into(),
            name: "cs".into(),
            components: custom_struct_erased.clone(),
            internal_type: Some(InternalType::Struct { contract: None, ty: "CustomStruct".into() }),
        },
        Param {
            ty: "tuple[]".into(),
            name: "csArr".into(),
            components: custom_struct_erased,
            internal_type: Some(InternalType::Struct {
                contract: None,
                ty: "CustomStruct[]".into(),
            }),
        },
    ];
    assert_eq!(
        myFuncCall::abi(),
        Function {
            name: "myFunc".into(),
            inputs: vec![
                Param {
                    ty: "uint32".into(),
                    name: "udvt".into(),
                    components: vec![],
                    internal_type: Some(InternalType::Other { contract: None, ty: "UDVT".into() }),
                },
                Param {
                    ty: "uint32[]".into(),
                    name: "udvtArr".into(),
                    components: vec![],
                    internal_type: Some(InternalType::Other {
                        contract: None,
                        ty: "UDVT[]".into()
                    }),
                },
                Param {
                    ty: "uint8".into(),
                    name: "e".into(),
                    components: vec![],
                    internal_type: Some(InternalType::Enum { contract: None, ty: "Enum".into() }),
                },
                Param {
                    ty: "uint8[]".into(),
                    name: "eArr".into(),
                    components: vec![],
                    internal_type: Some(InternalType::Enum { contract: None, ty: "Enum[]".into() }),
                },
                Param {
                    ty: "tuple".into(),
                    name: "cs".into(),
                    components: custom_struct.clone(),
                    internal_type: Some(InternalType::Struct {
                        contract: None,
                        ty: "CustomStruct".into()
                    }),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "csArr".into(),
                    components: custom_struct,
                    internal_type: Some(InternalType::Struct {
                        contract: None,
                        ty: "CustomStruct[]".into()
                    }),
                },
                Param {
                    ty: "tuple".into(),
                    name: "cs2".into(),
                    components: custom_struct2.clone(),
                    internal_type: Some(InternalType::Struct {
                        contract: None,
                        ty: "CustomStruct2".into()
                    }),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "cs2Arr".into(),
                    components: custom_struct2,
                    internal_type: Some(InternalType::Struct {
                        contract: None,
                        ty: "CustomStruct2[]".into()
                    }),
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
}

sol! {
    #![sol(abi)]

    #[allow(dead_code)]
    contract Contract {
        struct CustomStruct {
            uint256 custom;
        }

        struct CustomStruct2 {
            CustomStruct cs;
            bool cb;
        }

        event EV00();
        event EV01(uint256 a);
        event EV02(uint256, bool b);

        event EV10() anonymous;
        event EV11(uint256 indexed a) anonymous;
        event EV12(uint256, bool indexed b) anonymous;

        error ER0();
        error ER1(uint256 a);
        error ER2(uint256, bool b);

        constructor ctor();
        fallback();
        receive();

        function F00();
        function F01(uint256 a) payable;
        function F02(uint256, bool b) view;

        function F10() pure;
        function F11(uint256 a) returns (uint256 a);
        function F12(uint256, bool b) returns (uint256, bool b);

        function F20(uint256, uint256[], uint256[][1]);
        function F21(CustomStruct, CustomStruct[], CustomStruct[][2]);
        function F22(CustomStruct2, CustomStruct2[], CustomStruct2[][3]);
    }
}

#[allow(dead_code)]
mod not_contract {
    use super::*;

    sol! {
        #![sol(abi)]

        struct CustomStruct {
            uint256 custom;
        }

        struct CustomStruct2 {
            CustomStruct cs;
            bool cb;
        }

        event EV00();
        event EV01(uint256 a);
        event EV02(uint256, bool b);

        event EV10() anonymous;
        event EV11(uint256 indexed a) anonymous;
        event EV12(uint256, bool indexed b) anonymous;

        error ER0();
        error ER1(uint256 a);
        error ER2(uint256, bool b);

        function F00();
        function F01(uint256 a) payable;
        function F02(uint256, bool b) view;

        function F10() pure;
        function F11(uint256 a) returns (uint256 a);
        function F12(uint256, bool b) returns (uint256, bool b);

        function F20(uint256, uint256[], uint256[][1]);
        function F21(CustomStruct, CustomStruct[], CustomStruct[][2]);
        function F22(CustomStruct2, CustomStruct2[], CustomStruct2[][3]);
    }
}

fn param(s: &str) -> Param {
    use alloy_json_abi::InternalType;
    let (ty, name) = s.split_once(' ').unwrap();
    let internal_type = Some(InternalType::Other { contract: None, ty: ty.to_string() });
    Param { ty: ty.into(), name: name.into(), internal_type, components: vec![] }
}

fn eparam(s: &str, indexed: bool) -> EventParam {
    use alloy_json_abi::InternalType;
    let (ty, name) = s.split_once(' ').unwrap();
    let internal_type = Some(InternalType::Other { contract: None, ty: ty.to_string() });
    EventParam { ty: ty.into(), name: name.into(), internal_type, components: vec![], indexed }
}

#[test]
fn special_funcs() {
    sol! {
        #[sol(abi)]
        contract NoAttrs {
            fallback() external;
            receive() external;
        }

        #[sol(abi)]
        contract WithAttrs {
            fallback() external payable;
            receive() external payable;
        }
    }

    assert_eq!(
        NoAttrs::abi::fallback(),
        Some(Fallback { state_mutability: StateMutability::NonPayable })
    );
    assert_eq!(
        NoAttrs::abi::receive(),
        Some(Receive { state_mutability: StateMutability::Payable })
    );

    assert_eq!(
        WithAttrs::abi::fallback(),
        Some(Fallback { state_mutability: StateMutability::Payable })
    );
    assert_eq!(
        WithAttrs::abi::receive(),
        Some(Receive { state_mutability: StateMutability::Payable })
    );
}
