use alloy_json_abi::{
    Constructor, Error, EventParam, Fallback, Function, InternalType, Param, Receive,
    StateMutability,
};
use alloy_primitives::U256;
use alloy_sol_types::{JsonAbiExt, SolCall, sol};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

macro_rules! abi_map {
    ($($k:expr => $v:expr),* $(,)?) => {
        BTreeMap::from([$(($k.into(), vec![$v])),*])
    };
}

#[test]
fn renamed_items_and_parameters() {
    sol! {
        #![sol(abi, rename_all = "camelCase")]

        #[sol(rename = "ActualStruct", rename_all = "snake_case")]
        struct RustStruct {
            uint256 SomeField;
            #[sol(rename = "exactField")]
            bool OtherField;
        }

        function use_struct(RustStruct InputStruct) returns (RustStruct OutputStruct);

        function snake_case_fn(uint256 snake_case_param);

        #[sol(rename = "exactFunction", rename_all = "UPPERCASE")]
        function RustFunction(
            #[sol(rename = "exactInput")] uint256 FirstInput,
            uint256 SecondInput
        ) returns (uint256 OutputValue);

        event RustEvent(#[sol(rename = "exactEventInput")] uint256 EventInput);
        error RustError(#[sol(rename = "exactErrorInput")] uint256 ErrorInput);
    }

    let use_struct = use_structCall::abi();
    assert_eq!(use_struct.name, "useStruct");
    assert_eq!(use_struct.inputs[0].name, "inputStruct");
    assert_eq!(use_struct.outputs[0].name, "outputStruct");
    assert_eq!(use_struct.inputs[0].components[0].name, "some_field");
    assert_eq!(use_struct.inputs[0].components[1].name, "exactField");

    let snake = snake_case_fnCall::abi();
    assert_eq!(snake.name, "snakeCaseFn");
    assert_eq!(snake.inputs[0].name, "snakeCaseParam");
    assert_eq!(snake_case_fnCall::SIGNATURE, "snakeCaseFn(uint256)");
    let _ = snake_case_fnCall { snake_case_param: U256::ZERO };

    let function = RustFunctionCall::abi();
    assert_eq!(function.name, "exactFunction");
    assert_eq!(function.inputs[0].name, "exactInput");
    assert_eq!(function.inputs[1].name, "SECONDINPUT");
    assert_eq!(function.outputs[0].name, "OUTPUTVALUE");

    let event = RustEvent::abi();
    assert_eq!(event.name, "rustEvent");
    assert_eq!(event.inputs[0].name, "exactEventInput");

    let error = RustError::abi();
    assert_eq!(error.name, "rustError");
    assert_eq!(error.inputs[0].name, "exactErrorInput");
}

mod rename_namespace {
    use alloy_sol_types::{JsonAbiExt, SolStruct, sol};
    use pretty_assertions::assert_eq;

    sol! {
        #![sol(abi)]

        struct Global {
            uint256 my_field;
        }

        #[sol(rename_all = "camelCase")]
        contract Scoped {
            struct Local {
                Global the_global;
            }

            function use_local(Local the_local, Global other_global);
        }
    }

    #[test]
    fn rename_all_in_declaring_namespace() {
        assert_eq!(Global::eip712_encode_type(), "Global(uint256 my_field)");
        assert_eq!(
            Scoped::Local::eip712_encode_type(),
            "local(Global theGlobal)Global(uint256 my_field)"
        );

        let function = Scoped::use_localCall::abi();
        assert_eq!(function.inputs[0].components[0].components[0].name, "my_field");
        assert_eq!(function.inputs[1].components[0].name, "my_field");
    }
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
                    internal_type: internal_type("struct Contract.CustomStruct"),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: internal_type("struct Contract.CustomStruct[]"),
                },
                Param {
                    ty: "tuple[][2]".into(),
                    name: String::new(),
                    components: vec![param("uint256 custom")],
                    internal_type: internal_type("struct Contract.CustomStruct[][2]"),
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
        internal_type: internal_type("struct Contract.CustomStruct"),
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
                    internal_type: internal_type("struct Contract.CustomStruct2"),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: String::new(),
                    components: vec![custom.clone(), param("bool cb")],
                    internal_type: internal_type("struct Contract.CustomStruct2[]"),
                },
                Param {
                    ty: "tuple[][3]".into(),
                    name: String::new(),
                    components: vec![custom, param("bool cb")],
                    internal_type: internal_type("struct Contract.CustomStruct2[][3]"),
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
    // `F21Call` and `F22Call` differ only in the contract qualifier of their struct internal types.
    eq_modules!(
        EV00, EV01, EV02, EV10, EV11, EV12, ER0, ER1, ER2, F00Call, F01Call, F02Call, F10Call,
        F11Call, F12Call, F20Call
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
        components: vec![param("uint256 forkId"), param("uint256 chainId")],
        internal_type: internal_type("struct ChainInfo"),
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
        internal_type: internal_type("struct StorageAccess[]"),
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
                    custom_param("uint8 kind", "enum AccountAccessKind"),
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
                internal_type: internal_type("struct AccountAccess[]"),
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
        custom_param("uint32 udvt", "UDVT"),
        custom_param("uint32[] udvtArr", "UDVT[]"),
        custom_param("uint8 e", "enum Enum"),
        custom_param("uint8[] eArr", "enum Enum[]"),
    ];
    let custom_struct2 = vec![
        Param {
            ty: "tuple".into(),
            name: "cs".into(),
            components: custom_struct.clone(),
            internal_type: internal_type("struct CustomStruct"),
        },
        Param {
            ty: "tuple[]".into(),
            name: "csArr".into(),
            components: custom_struct.clone(),
            internal_type: internal_type("struct CustomStruct[]"),
        },
    ];
    assert_eq!(
        myFuncCall::abi(),
        Function {
            name: "myFunc".into(),
            inputs: vec![
                custom_param("uint32 udvt", "UDVT"),
                custom_param("uint32[] udvtArr", "UDVT[]"),
                custom_param("uint8 e", "enum Enum"),
                custom_param("uint8[] eArr", "enum Enum[]"),
                Param {
                    ty: "tuple".into(),
                    name: "cs".into(),
                    components: custom_struct.clone(),
                    internal_type: internal_type("struct CustomStruct"),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "csArr".into(),
                    components: custom_struct,
                    internal_type: internal_type("struct CustomStruct[]"),
                },
                Param {
                    ty: "tuple".into(),
                    name: "cs2".into(),
                    components: custom_struct2.clone(),
                    internal_type: internal_type("struct CustomStruct2"),
                },
                Param {
                    ty: "tuple[]".into(),
                    name: "cs2Arr".into(),
                    components: custom_struct2,
                    internal_type: internal_type("struct CustomStruct2[]"),
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
    let (ty, _) = s.split_once(' ').unwrap();
    custom_param(s, ty)
}

fn custom_param(s: &str, internal: &str) -> Param {
    let (ty, name) = s.split_once(' ').unwrap();
    Param {
        ty: ty.into(),
        name: name.into(),
        internal_type: internal_type(internal),
        components: vec![],
    }
}

fn internal_type(s: &str) -> Option<InternalType> {
    Some(InternalType::parse(s).unwrap())
}

fn eparam(s: &str, indexed: bool) -> EventParam {
    let (ty, name) = s.split_once(' ').unwrap();
    EventParam {
        ty: ty.into(),
        name: name.into(),
        internal_type: internal_type(ty),
        components: vec![],
        indexed,
    }
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

#[test]
fn contract_abi_matches_json() {
    sol! {
        #[sol(abi)]
        contract Kitchen {
            type Id is uint32;

            enum Kind {
                A,
                B
            }

            struct Inner {
                uint256 x;
                Id[] ids;
                Kind kind;
            }

            struct Outer {
                Inner inner;
                Inner[] inners;
                Inner[2] pair;
                bytes32[3] words;
            }

            constructor(Outer outer) payable;
            fallback() external payable;
            receive() external payable;

            event Ev(Outer outer, uint256 indexed a, Id id, Kind[2] kinds) anonymous;
            event Ev(bytes32 indexed topic);

            error Err(Inner[] inners, uint8[2][] grid);

            function pureFn(Id id) external pure returns (Kind kind);
            function viewFn(Outer[] outers, Inner[2][] pairs) external view returns (Inner[] inners, bool);
            function payableFn(address, bytes32[3][] words) external payable returns (Outer outer);
            function plainFn(Kind[][2] kinds);
            function plainFn();
        }
    }

    let expected = serde_json::from_str::<alloy_json_abi::JsonAbi>(
        r#"[
        {"type":"constructor","stateMutability":"payable","inputs":[
            {"name":"outer","type":"tuple","internalType":"struct Kitchen.Outer","components":[
                {"name":"inner","type":"tuple","internalType":"struct Kitchen.Inner","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"pair","type":"tuple[2]","internalType":"struct Kitchen.Inner[2]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"words","type":"bytes32[3]","internalType":"bytes32[3]"}]}]},
        {"type":"fallback","stateMutability":"payable"},
        {"type":"receive","stateMutability":"payable"},
        {"type":"event","name":"Ev","anonymous":true,"inputs":[
            {"name":"outer","type":"tuple","internalType":"struct Kitchen.Outer","indexed":false,"components":[
                {"name":"inner","type":"tuple","internalType":"struct Kitchen.Inner","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"pair","type":"tuple[2]","internalType":"struct Kitchen.Inner[2]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"words","type":"bytes32[3]","internalType":"bytes32[3]"}]},
            {"name":"a","type":"uint256","internalType":"uint256","indexed":true},
            {"name":"id","type":"uint32","internalType":"Kitchen.Id","indexed":false},
            {"name":"kinds","type":"uint8[2]","internalType":"enum Kitchen.Kind[2]","indexed":false}]},
        {"type":"event","name":"Ev","anonymous":false,"inputs":[
            {"name":"topic","type":"bytes32","internalType":"bytes32","indexed":true}]},
        {"type":"error","name":"Err","inputs":[
            {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                {"name":"x","type":"uint256","internalType":"uint256"},
                {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
            {"name":"grid","type":"uint8[2][]","internalType":"uint8[2][]"}]},
        {"type":"function","name":"pureFn","stateMutability":"pure",
            "inputs":[{"name":"id","type":"uint32","internalType":"Kitchen.Id"}],
            "outputs":[{"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
        {"type":"function","name":"viewFn","stateMutability":"view",
            "inputs":[
                {"name":"outers","type":"tuple[]","internalType":"struct Kitchen.Outer[]","components":[
                    {"name":"inner","type":"tuple","internalType":"struct Kitchen.Inner","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"pair","type":"tuple[2]","internalType":"struct Kitchen.Inner[2]","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"words","type":"bytes32[3]","internalType":"bytes32[3]"}]},
                {"name":"pairs","type":"tuple[2][]","internalType":"struct Kitchen.Inner[2][]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]}],
            "outputs":[
                {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                    {"name":"x","type":"uint256","internalType":"uint256"},
                    {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                    {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                {"name":"","type":"bool","internalType":"bool"}]},
        {"type":"function","name":"payableFn","stateMutability":"payable",
            "inputs":[
                {"name":"","type":"address","internalType":"address"},
                {"name":"words","type":"bytes32[3][]","internalType":"bytes32[3][]"}],
            "outputs":[
                {"name":"outer","type":"tuple","internalType":"struct Kitchen.Outer","components":[
                    {"name":"inner","type":"tuple","internalType":"struct Kitchen.Inner","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"inners","type":"tuple[]","internalType":"struct Kitchen.Inner[]","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"pair","type":"tuple[2]","internalType":"struct Kitchen.Inner[2]","components":[
                        {"name":"x","type":"uint256","internalType":"uint256"},
                        {"name":"ids","type":"uint32[]","internalType":"Kitchen.Id[]"},
                        {"name":"kind","type":"uint8","internalType":"enum Kitchen.Kind"}]},
                    {"name":"words","type":"bytes32[3]","internalType":"bytes32[3]"}]}]},
        {"type":"function","name":"plainFn","stateMutability":"nonpayable",
            "inputs":[{"name":"kinds","type":"uint8[][2]","internalType":"enum Kitchen.Kind[][2]"}],"outputs":[]},
        {"type":"function","name":"plainFn","stateMutability":"nonpayable",
            "inputs":[],"outputs":[]}
        ]"#,
    )
    .unwrap();

    assert_eq!(Kitchen::abi::contract(), expected);
}

#[test]
fn internal_types() {
    // This has to be in a module (not a function) because of Rust import rules.
    mod nested {
        alloy_sol_types::sol! {
            #![sol(abi)]

            type Price is int128;
            enum Side { Buy, Sell }
            interface IToken {}

            contract Book {
                struct Level { Price price; Entry[] entries; }
                struct Entry { address payable maker; IToken token; }
            }

            struct Order { Side side; Book.Level[2] levels; }

            contract Exchange {
                function fill(Order order, Book.Level level, IToken[] tokens, address payable to) returns (Side);
            }
        }
    }
    use nested::Exchange;

    let level = r#"
        {"name":"price","type":"int128","internalType":"Price"},
        {"name":"entries","type":"tuple[]","internalType":"struct Book.Entry[]","components":[
            {"name":"maker","type":"address","internalType":"address payable"},
            {"name":"token","type":"address","internalType":"contract IToken"}]}"#;
    let expected = serde_json::from_str::<Function>(&format!(
        r#"{{"type":"function","name":"fill","stateMutability":"nonpayable",
        "inputs":[
            {{"name":"order","type":"tuple","internalType":"struct Order","components":[
                {{"name":"side","type":"uint8","internalType":"enum Side"}},
                {{"name":"levels","type":"tuple[2]","internalType":"struct Book.Level[2]","components":[{level}]}}]}},
            {{"name":"level","type":"tuple","internalType":"struct Book.Level","components":[{level}]}},
            {{"name":"tokens","type":"address[]","internalType":"contract IToken[]"}},
            {{"name":"to","type":"address","internalType":"address payable"}}],
        "outputs":[{{"name":"","type":"uint8","internalType":"enum Side"}}]}}"#
    ))
    .unwrap();

    assert_eq!(Exchange::fillCall::abi(), expected);
}

#[test]
fn internal_type_shadowed_struct() {
    // This has to be in a module (not a function) because of Rust import rules.
    mod nested {
        alloy_sol_types::sol! {
            #![sol(abi)]

            event Foo(uint256 x);

            contract C {
                struct Foo { uint256 a; }
                function f(Foo s);
            }
        }
    }

    assert_eq!(
        nested::C::fCall::abi().inputs,
        [Param {
            ty: "tuple".into(),
            name: "s".into(),
            components: vec![param("uint256 a")],
            internal_type: internal_type("struct C.Foo"),
        }]
    );
}
