use alloy_json_abi::{
    Constructor, Error, EventParam, Fallback, Function, Param, Receive, StateMutability,
};
use alloy_sol_types::{sol, JsonAbiExt};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

macro_rules! abi_map {
    ($($k:expr => $v:expr),* $(,)?) => {
        BTreeMap::from([$(($k.into(), vec![$v])),*])
    };
}

#[test]
fn equal_abis() {
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
                    internal_type: None,
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: None,
                },
                Param {
                    ty: "tuple[][2]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: None,
                },
            ],
            outputs: vec![],
            state_mutability: StateMutability::NonPayable,
        }
    );
    let custom = Param {
        ty: "tuple".into(),
        name: "cs".into(),
        // TODO: should be `uint256 custom`, but name is lost in recursive resolution
        components: vec![param("uint256 ")],
        internal_type: None,
    };
    assert_eq!(
        *contract.function("F22").unwrap().first().unwrap(),
        Function {
            name: "F22".into(),
            inputs: vec![
                Param {
                    ty: "tuple".into(),
                    name: String::new(),
                    components: vec![custom.clone(), param("bool cb")],
                    internal_type: None,
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![custom.clone(), param("bool cb")],
                    internal_type: None,
                },
                Param {
                    ty: "tuple[][3]".into(),
                    name: String::new(),
                    components: vec![custom, param("bool cb")],
                    internal_type: None,
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

    macro_rules! eq_modules {
        ($($items:ident),* $(,)?) => {$(
            assert_eq!(Contract::$items::abi(), not_contract::$items::abi());
        )*};
    }
    eq_modules!(
        EV00, EV01, EV02, EV10, EV11, EV12, ER0, ER1, ER2, F00Call, F01Call, F02Call, F10Call,
        F11Call, F12Call, F20Call, F21Call, F22Call
    );
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

    let chain_info = Param {
        ty: "tuple".into(),
        name: "chainInfo".into(),
        components: vec![
            param("uint256 "), // forkId
            param("uint256 "), // chainId
        ],
        internal_type: None,
    };
    let storage_accesses = Param {
        ty: "tuple[]".into(),
        name: "storageAccesses".into(),
        components: vec![
            param("address "), // account
            param("bytes32 "), // slot
            param("bool "),    // isWrite
            param("bytes32 "), // previousValue
            param("bytes32 "), // newValue
            param("bool "),    // reverted
        ],
        internal_type: None,
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
                    param("uint8 kind"), // TODO: enum
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
                internal_type: None,
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

    let custom_struct = vec![
        param("uint256 custom"),
        param("uint256[] customArr"),
        param("uint32 udvt"),
        param("uint32[] udvtArr"),
        param("uint8 e"),
        param("uint8[] eArr"),
    ];
    let custom_struct_erased = vec![
        param("uint256 "),   // custom
        param("uint256[] "), // customArr
        param("uint32 "),    // udvt
        param("uint32[] "),  // udvtArr
        param("uint8 "),     // e
        param("uint8[] "),   // eArr
    ];
    let custom_struct2 = vec![
        Param {
            ty: "tuple".into(),
            name: "cs".into(),
            components: custom_struct_erased.clone(),
            internal_type: None,
        },
        Param {
            ty: "tuple[]".into(),
            name: "csArr".into(),
            components: custom_struct_erased,
            internal_type: None,
        },
    ];
    assert_eq!(
        myFuncCall::abi(),
        Function {
            name: "myFunc".into(),
            inputs: vec![
                param("uint32 udvt"),
                param("uint32[] udvtArr"),
                param("uint8 e"),
                param("uint8[] eArr"),
                Param {
                    ty: "tuple".into(),
                    name: "cs".into(),
                    components: custom_struct.clone(),
                    internal_type: None,
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "csArr".into(),
                    components: custom_struct,
                    internal_type: None,
                },
                Param {
                    ty: "tuple".into(),
                    name: "cs2".into(),
                    components: custom_struct2.clone(),
                    internal_type: None,
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "cs2Arr".into(),
                    components: custom_struct2,
                    internal_type: None,
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
    let (ty, name) = s.split_once(' ').unwrap();
    Param { ty: ty.into(), name: name.into(), internal_type: None, components: vec![] }
}

fn eparam(s: &str, indexed: bool) -> EventParam {
    let (ty, name) = s.split_once(' ').unwrap();
    EventParam {
        ty: ty.into(),
        name: name.into(),
        internal_type: None,
        components: vec![],
        indexed,
    }
}
