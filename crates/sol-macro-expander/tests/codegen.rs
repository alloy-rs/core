//! Tests for the `codegen` module public API surface.
//!
//! These tests pin the generated code structure so that changes to the public codegen API
//! surface are caught as test failures. They use structural assertions via `syn::parse2`
//! and extract specific syn nodes for string validation of literal values.

use alloy_sol_macro_expander::{
    CallCodegen, CallLayout, ConstructorInfo, ContractCodegen, ContractEventInfo,
    ContractFunctionInfo, Eip712Options, EnumCodegen, ErrorCodegen, EventCodegen, EventFieldInfo,
    InterfaceCodegen, ReturnInfo, SolInterfaceKind, StructCodegen, StructLayout,
    gen_from_into_tuple, gen_tokenize, is_reserved_method_name,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

// --- PUBLIC API FUNCTION SIGNATURES -----------------------------------------

#[test]
#[allow(clippy::type_complexity)]
fn test_public_api_signatures() {
    let _: fn(Vec<Ident>, Vec<TokenStream>, Vec<TokenStream>, Eip712Options) -> StructCodegen =
        StructCodegen::new;

    let _: fn(Vec<Ident>, bool) -> EnumCodegen = EnumCodegen::new;

    let _: fn(Vec<Ident>, Vec<TokenStream>, Vec<TokenStream>, bool) -> ErrorCodegen =
        ErrorCodegen::new;

    let _: fn(bool, Vec<EventFieldInfo>) -> EventCodegen = EventCodegen::new;

    let _: fn(TokenStream, TokenStream, TokenStream, ReturnInfo) -> CallCodegen = CallCodegen::new;

    let _: fn(
        Ident,
        Vec<ContractFunctionInfo>,
        Vec<ContractEventInfo>,
        bool,
        Option<ConstructorInfo>,
    ) -> ContractCodegen = ContractCodegen::new;

    let _: fn(&str) -> bool = is_reserved_method_name;

    let _: fn(&Ident, &[Ident], &[TokenStream], &[TokenStream], StructLayout) -> TokenStream =
        gen_from_into_tuple;

    let _: fn(&[Ident], &[TokenStream], bool) -> TokenStream = gen_tokenize;

    let _: EventFieldInfo = EventFieldInfo {
        name: ident("x"),
        sol_type: quote!(sol_data::Uint<256>),
        is_indexed: false,
        indexed_as_hash: false,
        span: Span::call_site(),
    };

    let _: ContractFunctionInfo = ContractFunctionInfo {
        method_name: ident("transfer"),
        call_name: ident("transferCall"),
        param_names: vec![],
        rust_types: vec![],
        layout: CallLayout::Unit,
    };

    let _: ContractEventInfo = ContractEventInfo { event_name: ident("Transfer") };

    let _: ConstructorInfo = ConstructorInfo { param_names: vec![], rust_types: vec![] };

    let _: Eip712Options =
        Eip712Options { root: String::new(), components_impl: None, encode_type_impl: None };
}

// --- TRAIT BOUNDS AND EXHAUSTIVENESS ----------------------------------------

#[test]
fn test_trait_bounds_and_exhaustiveness() {
    fn assert_copy_clone_debug_eq<T: Copy + Clone + std::fmt::Debug + PartialEq + Eq>() {}
    assert_copy_clone_debug_eq::<StructLayout>();
    assert_copy_clone_debug_eq::<CallLayout>();

    fn assert_copy_clone_debug<T: Copy + Clone + std::fmt::Debug>() {}
    assert_copy_clone_debug::<SolInterfaceKind>();

    fn assert_default<T: Default>() {}
    assert_default::<SolInterfaceKind>();
    assert_default::<Eip712Options>();

    fn assert_debug<T: std::fmt::Debug>() {}
    assert_debug::<Eip712Options>();
    assert_debug::<EventFieldInfo>();
    assert_debug::<ReturnInfo>();

    match StructLayout::Unit {
        StructLayout::Unit | StructLayout::Tuple | StructLayout::Named => {}
    }
    match CallLayout::Unit {
        CallLayout::Unit | CallLayout::Tuple | CallLayout::Named => {}
    }
    match SolInterfaceKind::Call {
        SolInterfaceKind::Call | SolInterfaceKind::Error => {}
    }
    let info = ReturnInfo::Empty { return_name: ident("fooReturn") };
    match info {
        ReturnInfo::Empty { .. } | ReturnInfo::Single { .. } | ReturnInfo::Multiple { .. } => {}
    }
}

// --- CODEGEN TYPES: StructCodegen ------------------------------------------

#[test]
fn test_struct_codegen_unit() {
    let codegen = StructCodegen::new(
        vec![],
        vec![],
        vec![],
        Eip712Options { root: "Empty()".into(), ..Default::default() },
    );
    let tokens = codegen.expand(&ident("Empty"));
    let file = helpers::parse(&tokens);

    helpers::find_impl(&file, "SolValue");
    helpers::find_impl(&file, "SolTypeValue");
    helpers::find_impl(&file, "SolType");
    let sol_struct = helpers::find_impl(&file, "SolStruct");
    helpers::find_impl(&file, "EventTopic");

    assert_eq!(helpers::const_expr_to_string(sol_struct, "NAME"), "\"Empty\"");

    helpers::assert_has_methods(
        sol_struct,
        &["eip712_root_type", "eip712_components", "eip712_encode_type", "eip712_encode_data"],
    );
}

#[test]
fn test_struct_codegen_named() {
    let codegen = StructCodegen::new(
        vec![ident("a"), ident("b")],
        vec![quote!(alloy_sol_types::private::U256), quote!(alloy_sol_types::private::Address)],
        vec![
            quote!(alloy_sol_types::sol_data::Uint<256>),
            quote!(alloy_sol_types::sol_data::Address),
        ],
        Eip712Options { root: "MyStruct(uint256 a,address b)".into(), ..Default::default() },
    );
    let tokens = codegen.expand(&ident("MyStruct"));
    let file = helpers::parse(&tokens);

    let sol_struct = helpers::find_impl(&file, "SolStruct");
    assert_eq!(helpers::const_expr_to_string(sol_struct, "NAME"), "\"MyStruct\"");

    let sol_type = helpers::find_impl(&file, "SolType");
    helpers::assert_has_types(sol_type, &["RustType", "Token"]);
    helpers::assert_has_consts(sol_type, &["SOL_NAME", "ENCODED_SIZE", "PACKED_ENCODED_SIZE"]);
    helpers::assert_has_methods(sol_type, &["valid_token", "detokenize"]);

    let stv = helpers::find_impl(&file, "SolTypeValue");
    helpers::assert_has_methods(
        stv,
        &[
            "stv_to_tokens",
            "stv_abi_encoded_size",
            "stv_eip712_data_word",
            "stv_abi_encode_packed_to",
            "stv_abi_packed_encoded_size",
        ],
    );

    let et = helpers::find_impl(&file, "EventTopic");
    helpers::assert_has_methods(
        et,
        &["topic_preimage_length", "encode_topic_preimage", "encode_topic"],
    );

    helpers::assert_has_methods(sol_struct, &["eip712_encode_type"]);
}

#[test]
fn test_struct_codegen_eip712_components() {
    let codegen = StructCodegen::new(
        vec![ident("inner")],
        vec![quote!(Inner)],
        vec![quote!(Inner)],
        Eip712Options {
            root: "Outer(Inner inner)".into(),
            components_impl: Some(
                quote! { <Inner as alloy_sol_types::SolStruct>::eip712_type_chain() },
            ),
            encode_type_impl: None,
        },
    );
    let tokens = codegen.expand(&ident("Outer"));
    let file = helpers::parse(&tokens);

    let sol_struct = helpers::find_impl(&file, "SolStruct");
    helpers::assert_has_methods(sol_struct, &["eip712_components"]);
    helpers::assert_not_has_methods(sol_struct, &["eip712_encode_type"]);
}

#[test]
fn test_struct_codegen_single_field() {
    let codegen = StructCodegen::new(
        vec![ident("value")],
        vec![quote!(alloy_sol_types::private::U256)],
        vec![quote!(alloy_sol_types::sol_data::Uint<256>)],
        Eip712Options { root: "Single(uint256 value)".into(), ..Default::default() },
    );
    let tokens = codegen.expand(&ident("Single"));
    let file = helpers::parse(&tokens);

    let sol_struct = helpers::find_impl(&file, "SolStruct");
    assert_eq!(helpers::const_expr_to_string(sol_struct, "NAME"), "\"Single\"");

    helpers::find_impl(&file, "SolValue");
    helpers::find_impl(&file, "SolType");
    helpers::find_impl(&file, "EventTopic");

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), 2);
}

// --- CODEGEN TYPES: EnumCodegen --------------------------------------------

#[test]
fn test_enum_codegen_basic() {
    let codegen = EnumCodegen::new(vec![ident("Pending"), ident("Active")], true);
    let tokens = codegen.expand(&ident("Status"));
    let file = helpers::parse(&tokens);

    assert_eq!(helpers::find_all_impls(&file, "From").len(), 1, "should have From<Status> for u8");
    assert_eq!(
        helpers::find_all_impls(&file, "TryFrom").len(),
        1,
        "should have TryFrom<u8> for Status"
    );
    helpers::find_impl(&file, "SolValue");
    helpers::find_impl(&file, "SolTypeValue");
    helpers::find_impl(&file, "SolType");
    helpers::find_impl(&file, "EventTopic");

    let sol_enum = helpers::find_impl(&file, "SolEnum");
    assert_eq!(helpers::const_expr_to_string(sol_enum, "COUNT"), "2usize");
}

#[test]
fn test_enum_codegen_full() {
    let variants: Vec<Ident> = (0..256).map(|i| ident(&format!("V{i}"))).collect();
    let codegen = EnumCodegen::new(variants, false);
    let tokens = codegen.expand(&ident("FullEnum"));
    let file = helpers::parse(&tokens);

    let sol_enum = helpers::find_impl(&file, "SolEnum");
    assert_eq!(helpers::const_expr_to_string(sol_enum, "COUNT"), "256usize");

    let sol_type = helpers::find_impl(&file, "SolType");
    let detokenize_method = sol_type
        .items
        .iter()
        .find_map(|item| match item {
            syn::ImplItem::Fn(m) if m.sig.ident == "detokenize" => Some(m),
            _ => None,
        })
        .expect("detokenize method not found");
    let body = detokenize_method.block.to_token_stream().to_string();
    assert!(body.contains("expect"), "full enum (256 variants) should use expect, not unwrap_or");
    assert!(!body.contains("unwrap_or"), "full enum should not use unwrap_or");
}

// --- CODEGEN TYPES: ErrorCodegen -------------------------------------------

#[test]
fn test_error_codegen_unit() {
    let codegen = ErrorCodegen::new(vec![], vec![], vec![], false);
    let tokens = codegen.expand(&ident("EmptyError"), "EmptyError()");
    let file = helpers::parse(&tokens);

    let sol_error = helpers::find_impl(&file, "SolError");
    helpers::assert_has_types(sol_error, &["Parameters", "Token"]);
    helpers::assert_has_consts(sol_error, &["SIGNATURE", "SELECTOR"]);
    assert_eq!(helpers::const_expr_to_string(sol_error, "SIGNATURE"), "\"EmptyError()\"");
    helpers::assert_has_methods(sol_error, &["new", "tokenize", "abi_decode_raw_validate"]);
}

#[test]
fn test_error_codegen_named() {
    let codegen = ErrorCodegen::new(
        vec![ident("code"), ident("message")],
        vec![
            quote!(alloy_sol_types::sol_data::Uint<256>),
            quote!(alloy_sol_types::sol_data::String),
        ],
        vec![quote!(alloy_sol_types::private::U256), quote!(alloy_sol_types::private::String)],
        false,
    );
    let tokens = codegen.expand(&ident("MyError"), "MyError(uint256,string)");
    let file = helpers::parse(&tokens);

    let sol_error = helpers::find_impl(&file, "SolError");
    assert_eq!(
        helpers::const_expr_to_string(sol_error, "SIGNATURE"),
        "\"MyError(uint256,string)\""
    );
}

#[test]
fn test_error_codegen_tuple() {
    let codegen = ErrorCodegen::new(
        vec![ident("_0")],
        vec![quote!(alloy_sol_types::sol_data::Uint<256>)],
        vec![quote!(alloy_sol_types::private::U256)],
        true,
    );
    let tokens = codegen.expand(&ident("SingleError"), "SingleError(uint256)");
    let file = helpers::parse(&tokens);

    let sol_error = helpers::find_impl(&file, "SolError");
    assert_eq!(helpers::const_expr_to_string(sol_error, "SIGNATURE"), "\"SingleError(uint256)\"");
    helpers::assert_has_methods(sol_error, &["new", "tokenize", "abi_decode_raw_validate"]);

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), 2);
}

// --- CODEGEN TYPES: EventCodegen -------------------------------------------

#[test]
fn test_event_codegen_basic() {
    let fields = vec![
        event_field("from", quote!(alloy_sol_types::sol_data::Address), true, false),
        event_field("value", quote!(alloy_sol_types::sol_data::Uint<256>), false, false),
    ];
    let codegen = EventCodegen::new(false, fields);
    let tokens = codegen.expand(&ident("Transfer"), "Transfer(address,uint256)");
    let file = helpers::parse(&tokens);

    let sol_event = helpers::find_impl(&file, "SolEvent");
    helpers::find_impl(&file, "IntoLogData");

    let from_impls = helpers::find_all_impls(&file, "From");
    let has_log_data_from =
        from_impls.iter().any(|imp| imp.self_ty.to_token_stream().to_string().contains("LogData"));
    assert!(has_log_data_from, "should have a From impl for LogData");

    helpers::assert_has_consts(sol_event, &["SIGNATURE", "SIGNATURE_HASH", "ANONYMOUS"]);
    assert_eq!(helpers::const_expr_to_string(sol_event, "ANONYMOUS"), "false");
    helpers::assert_has_methods(
        sol_event,
        &["check_signature", "new", "tokenize_body", "topics", "encode_topics_raw"],
    );
}

#[test]
fn test_event_codegen_anonymous() {
    let fields =
        vec![event_field("value", quote!(alloy_sol_types::sol_data::Uint<256>), false, false)];
    let codegen = EventCodegen::new(true, fields);
    let tokens = codegen.expand(&ident("AnonEvent"), "AnonEvent(uint256)");
    let file = helpers::parse(&tokens);

    let sol_event = helpers::find_impl(&file, "SolEvent");
    assert_eq!(helpers::const_expr_to_string(sol_event, "ANONYMOUS"), "true");

    helpers::assert_not_has_methods(sol_event, &["check_signature"]);
}

#[test]
fn test_event_codegen_indexed_as_hash() {
    let fields = vec![event_field("data", quote!(alloy_sol_types::sol_data::String), true, true)];
    let codegen = EventCodegen::new(false, fields);
    let tokens = codegen.expand(&ident("HashEvent"), "HashEvent(string)");
    let file = helpers::parse(&tokens);

    let sol_event = helpers::find_impl(&file, "SolEvent");
    let topic_list_type = helpers::assoc_type_to_string(sol_event, "TopicList");
    assert!(
        topic_list_type.contains("FixedBytes"),
        "indexed-as-hash should use FixedBytes<32> in TopicList, got: {topic_list_type}"
    );
}

#[test]
fn test_event_codegen_multi_indexed() {
    let fields = vec![
        event_field("from", quote!(alloy_sol_types::sol_data::Address), true, false),
        event_field("to", quote!(alloy_sol_types::sol_data::Address), true, false),
        event_field("value", quote!(alloy_sol_types::sol_data::Uint<256>), false, false),
    ];
    let codegen = EventCodegen::new(false, fields);
    let tokens = codegen.expand(&ident("Transfer"), "Transfer(address,address,uint256)");
    let file = helpers::parse(&tokens);

    let sol_event = helpers::find_impl(&file, "SolEvent");
    helpers::assert_has_methods(
        sol_event,
        &["check_signature", "new", "tokenize_body", "topics", "encode_topics_raw"],
    );

    let topic_list_type = helpers::assoc_type_to_string(sol_event, "TopicList");
    assert!(
        topic_list_type.contains("FixedBytes"),
        "non-anonymous event TopicList should include FixedBytes<32> for signature hash, got: {topic_list_type}"
    );
}

// --- CODEGEN TYPES: CallCodegen --------------------------------------------

#[test]
fn test_call_codegen_empty_return() {
    let call = CallCodegen::new(
        quote!((alloy_sol_types::sol_data::Uint<256>,)),
        quote!(()),
        quote!((<alloy_sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
            &self._0
        ),)),
        ReturnInfo::Empty { return_name: ident("fooReturn") },
    );
    let ret_ty = call.return_type().to_string();
    assert_eq!(ret_ty, "fooReturn");

    let tokens = call.expand(&ident("fooCall"), "foo(uint256)");
    let file = helpers::parse(&tokens);

    let sol_call = helpers::find_impl(&file, "SolCall");
    helpers::assert_has_types(
        sol_call,
        &["Parameters", "Token", "Return", "ReturnTuple", "ReturnToken"],
    );
    helpers::assert_has_consts(sol_call, &["SIGNATURE", "SELECTOR"]);
    assert_eq!(helpers::const_expr_to_string(sol_call, "SIGNATURE"), "\"foo(uint256)\"");
    helpers::assert_has_methods(
        sol_call,
        &[
            "new",
            "tokenize",
            "tokenize_returns",
            "abi_decode_returns",
            "abi_decode_returns_validate",
        ],
    );
}

#[test]
fn test_call_codegen_single_return() {
    let call = CallCodegen::new(
        quote!(()),
        quote!((alloy_sol_types::sol_data::Uint<256>,)),
        quote!(()),
        ReturnInfo::Single {
            sol_type: quote!(alloy_sol_types::sol_data::Uint<256>),
            rust_type: quote!(alloy_sol_types::private::U256),
            field_name: ident("_0"),
            return_name: ident("getValueReturn"),
        },
    );
    let ret_ty = call.return_type().to_string();
    assert!(
        ret_ty.contains("U256"),
        "single return should use the concrete rust type, got: {ret_ty}"
    );

    let tokens = call.expand(&ident("getValueCall"), "getValue()");
    let file = helpers::parse(&tokens);
    let sol_call = helpers::find_impl(&file, "SolCall");

    let return_type = helpers::assoc_type_to_string(sol_call, "Return");
    assert!(return_type.contains("U256"), "Return type should be U256, got: {return_type}");
}

#[test]
fn test_call_codegen_multiple_return() {
    let call = CallCodegen::new(
        quote!(()),
        quote!((alloy_sol_types::sol_data::Uint<256>, alloy_sol_types::sol_data::Address,)),
        quote!(()),
        ReturnInfo::Multiple { return_name: ident("multiReturn") },
    );
    let ret_ty = call.return_type().to_string();
    assert_eq!(ret_ty, "multiReturn");

    let tokens = call.expand(&ident("multiCall"), "multi()");
    let file = helpers::parse(&tokens);
    helpers::find_impl(&file, "SolCall");
}

// --- CODEGEN TYPES: InterfaceCodegen ---------------------------------------

#[test]
fn test_interface_codegen_calls() {
    let sel_a = alloy_sol_macro_expander::selector("foo(uint256)");
    let sel_b = alloy_sol_macro_expander::selector("bar()");

    let codegen = InterfaceCodegen::precomputed(
        ident("MyCalls"),
        vec![ident("foo"), ident("bar")],
        vec![ident("fooCall"), ident("barCall")],
        vec![sel_a, sel_b],
        vec!["foo(uint256)".into(), "bar()".into()],
        0,
        SolInterfaceKind::Call,
    );
    let tokens = codegen.expand();
    let file = helpers::parse(&tokens);

    let e = helpers::find_enum(&file, "MyCalls");
    assert_eq!(e.variants.len(), 2);

    let iface = helpers::find_impl(&file, "SolInterface");
    helpers::assert_has_consts(iface, &["NAME", "MIN_DATA_LENGTH", "COUNT"]);
    assert_eq!(helpers::const_expr_to_string(iface, "COUNT"), "2usize");
    helpers::assert_has_methods(
        iface,
        &[
            "selector",
            "selector_at",
            "valid_selector",
            "abi_decode_raw",
            "abi_decode_raw_validate",
            "abi_encoded_size",
            "abi_encode_raw",
        ],
    );

    let inherent = helpers::find_inherent_impl(&file, "MyCalls");
    helpers::assert_has_consts(inherent, &["SELECTORS", "SIGNATURES"]);
    helpers::assert_has_methods(inherent, &["signature_by_selector", "name_by_selector"]);

    let variant_count = e.variants.len();

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), variant_count, "expected one From impl per variant");

    let try_from_impls = helpers::find_all_impls(&file, "TryFrom");
    assert_eq!(try_from_impls.len(), variant_count, "expected one TryFrom impl per variant");

    assert!(
        helpers::has_ident(&tokens, "SolCall"),
        "call interface should reference SolCall trait"
    );
}

#[test]
fn test_interface_codegen_errors() {
    let sel = alloy_sol_macro_expander::selector("MyError(uint256)");

    let codegen = InterfaceCodegen::precomputed(
        ident("MyErrors"),
        vec![ident("MyError")],
        vec![ident("MyError")],
        vec![sel],
        vec!["MyError(uint256)".into()],
        32,
        SolInterfaceKind::Error,
    );
    let tokens = codegen.expand();
    let file = helpers::parse(&tokens);

    helpers::find_impl(&file, "SolInterface");

    assert!(
        helpers::has_ident(&tokens, "SolError"),
        "error interface should reference SolError trait"
    );
    assert!(
        !helpers::has_ident(&tokens, "SolCall"),
        "error interface should not reference SolCall"
    );
}

// --- CODEGEN TYPES: ContractCodegen ----------------------------------------

fn make_contract_codegen(has_bytecode: bool) -> ContractCodegen {
    let functions = vec![
        ContractFunctionInfo {
            method_name: ident("transfer"),
            call_name: ident("transferCall"),
            param_names: vec![ident("to"), ident("amount")],
            rust_types: vec![
                quote!(alloy_sol_types::private::Address),
                quote!(alloy_sol_types::private::U256),
            ],
            layout: CallLayout::Named,
        },
        ContractFunctionInfo {
            method_name: ident("totalSupply"),
            call_name: ident("totalSupplyCall"),
            param_names: vec![],
            rust_types: vec![],
            layout: CallLayout::Unit,
        },
        ContractFunctionInfo {
            method_name: ident("set"),
            call_name: ident("setCall"),
            param_names: vec![ident("_0")],
            rust_types: vec![quote!(alloy_sol_types::private::U256)],
            layout: CallLayout::Tuple,
        },
        ContractFunctionInfo {
            method_name: ident("new_call"),
            call_name: ident("newCall"),
            param_names: vec![],
            rust_types: vec![],
            layout: CallLayout::Unit,
        },
    ];
    let events = vec![ContractEventInfo { event_name: ident("Transfer") }];
    let constructor = if has_bytecode {
        Some(ConstructorInfo {
            param_names: vec![ident("name"), ident("symbol")],
            rust_types: vec![
                quote!(alloy_sol_types::private::String),
                quote!(alloy_sol_types::private::String),
            ],
        })
    } else {
        None
    };

    ContractCodegen::new(ident("ERC20"), functions, events, has_bytecode, constructor)
}

#[test]
fn test_contract_codegen_with_bytecode() {
    let codegen = make_contract_codegen(true);
    let tokens = codegen.expand();
    let file = helpers::parse(&tokens);

    helpers::find_struct(&file, "ERC20Instance");
    helpers::assert_all_methods_contains(
        &file,
        "ERC20Instance",
        &[
            "deploy",
            "deploy_builder",
            "transfer",
            "totalSupply",
            "set",
            "new_call",
            "Transfer_filter",
        ],
    );
}

#[test]
fn test_contract_codegen_without_bytecode() {
    let codegen = make_contract_codegen(false);
    let tokens = codegen.expand();
    let file = helpers::parse(&tokens);

    helpers::find_struct(&file, "ERC20Instance");
    helpers::assert_all_methods_not_contains(&file, "ERC20Instance", &["deploy", "deploy_builder"]);
}

// --- CODEGEN HELPERS: gen_from_into_tuple, gen_tokenize --------------------

#[test]
fn test_gen_from_into_tuple_unit() {
    let tokens = gen_from_into_tuple(&ident("Foo"), &[], &[], &[], StructLayout::Unit);
    let file = helpers::parse(&tokens);

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), 2, "should have two From impls (struct->tuple, tuple->struct)");

    assert!(helpers::has_ident(&tokens, "UnderlyingSolTuple"));
    assert!(helpers::has_ident(&tokens, "UnderlyingRustTuple"));
}

#[test]
fn test_gen_from_into_tuple_tuple() {
    let tokens = gen_from_into_tuple(
        &ident("Bar"),
        &[ident("_0")],
        &[quote!(alloy_sol_types::sol_data::Uint<256>)],
        &[quote!(alloy_sol_types::private::U256)],
        StructLayout::Tuple,
    );
    let file = helpers::parse(&tokens);

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), 2);
}

#[test]
fn test_gen_from_into_tuple_named() {
    let tokens = gen_from_into_tuple(
        &ident("Baz"),
        &[ident("x"), ident("y")],
        &[quote!(alloy_sol_types::sol_data::Uint<256>), quote!(alloy_sol_types::sol_data::Address)],
        &[quote!(alloy_sol_types::private::U256), quote!(alloy_sol_types::private::Address)],
        StructLayout::Named,
    );
    let file = helpers::parse(&tokens);

    let from_impls = helpers::find_all_impls(&file, "From");
    assert_eq!(from_impls.len(), 2);
}

#[test]
fn test_gen_tokenize() {
    let empty = gen_tokenize(&[], &[], false);
    assert_eq!(empty.to_string(), "()");

    let tuple = gen_tokenize(&[ident("_0")], &[quote!(alloy_sol_types::sol_data::Uint<256>)], true);
    assert!(helpers::has_ident(&tuple, "SolType"), "tuple tokenize should reference SolType");

    let named = gen_tokenize(
        &[ident("a"), ident("b")],
        &[quote!(alloy_sol_types::sol_data::Uint<256>), quote!(alloy_sol_types::sol_data::Address)],
        false,
    );
    assert!(helpers::has_ident(&named, "SolType"), "named tokenize should reference SolType");
}

// --- TEST HELPERS ----------------------------------------------------------

mod helpers {
    use super::*;
    use syn::{File, ImplItem, Item, ItemEnum, ItemImpl, ItemStruct};

    #[derive(Clone, Copy)]
    enum ItemKind {
        Method,
        Const,
        Type,
    }

    pub(super) fn parse(tokens: &TokenStream) -> File {
        syn::parse2(tokens.clone()).expect("generated code should be valid Rust")
    }

    pub(super) fn find_impl<'a>(file: &'a File, trait_name: &str) -> &'a ItemImpl {
        find_all_impls(file, trait_name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("no impl block found for trait `{trait_name}`"))
    }

    pub(super) fn find_all_impls<'a>(file: &'a File, trait_name: &str) -> Vec<&'a ItemImpl> {
        file.items
            .iter()
            .filter_map(|item| match item {
                Item::Impl(imp) => {
                    let path = imp.trait_.as_ref()?.1.segments.last()?;
                    if path.ident == trait_name { Some(imp) } else { None }
                }
                _ => None,
            })
            .collect()
    }

    pub(super) fn find_inherent_impl<'a>(file: &'a File, type_name: &str) -> &'a ItemImpl {
        find_all_inherent_impls(file, type_name)
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("no inherent impl found for type `{type_name}`"))
    }

    fn impl_item_names(imp: &ItemImpl, kind: ItemKind) -> Vec<String> {
        imp.items
            .iter()
            .filter_map(|item| match (item, kind) {
                (ImplItem::Fn(m), ItemKind::Method) => Some(m.sig.ident.to_string()),
                (ImplItem::Const(c), ItemKind::Const) => Some(c.ident.to_string()),
                (ImplItem::Type(t), ItemKind::Type) => Some(t.ident.to_string()),
                _ => None,
            })
            .collect()
    }

    fn impl_item_to_string(imp: &ItemImpl, kind: ItemKind, name: &str) -> String {
        imp.items
            .iter()
            .find_map(|item| match (item, kind) {
                (ImplItem::Const(c), ItemKind::Const) if c.ident == name => {
                    Some(c.expr.to_token_stream().to_string())
                }
                (ImplItem::Type(t), ItemKind::Type) if t.ident == name => {
                    Some(t.ty.to_token_stream().to_string())
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("`{name}` not found in impl"))
    }

    pub(super) fn const_expr_to_string(imp: &ItemImpl, name: &str) -> String {
        impl_item_to_string(imp, ItemKind::Const, name)
    }

    pub(super) fn assoc_type_to_string(imp: &ItemImpl, name: &str) -> String {
        impl_item_to_string(imp, ItemKind::Type, name)
    }

    pub(super) fn find_enum<'a>(file: &'a File, name: &str) -> &'a ItemEnum {
        file.items
            .iter()
            .find_map(|item| match item {
                Item::Enum(e) if e.ident == name => Some(e),
                _ => None,
            })
            .unwrap_or_else(|| panic!("enum `{name}` not found"))
    }

    pub(super) fn assert_has_methods(imp: &ItemImpl, expected: &[&str]) {
        assert_has_items(imp, ItemKind::Method, expected);
    }

    pub(super) fn assert_not_has_methods(imp: &ItemImpl, unexpected: &[&str]) {
        assert_not_has_items(imp, ItemKind::Method, unexpected);
    }

    pub(super) fn assert_has_consts(imp: &ItemImpl, expected: &[&str]) {
        assert_has_items(imp, ItemKind::Const, expected);
    }

    pub(super) fn assert_has_types(imp: &ItemImpl, expected: &[&str]) {
        assert_has_items(imp, ItemKind::Type, expected);
    }

    fn assert_has_items(imp: &ItemImpl, kind: ItemKind, expected: &[&str]) {
        let items = impl_item_names(imp, kind);
        for name in expected {
            assert!(
                items.contains(&name.to_string()),
                "expected `{name}` not found in impl; found: {items:?}"
            );
        }
    }

    fn assert_not_has_items(imp: &ItemImpl, kind: ItemKind, unexpected: &[&str]) {
        let items = impl_item_names(imp, kind);
        for name in unexpected {
            assert!(
                !items.contains(&name.to_string()),
                "unexpected `{name}` found in impl; found: {items:?}"
            );
        }
    }

    pub(super) fn find_all_inherent_impls<'a>(
        file: &'a File,
        type_name: &str,
    ) -> Vec<&'a ItemImpl> {
        file.items
            .iter()
            .filter_map(|item| match item {
                Item::Impl(imp) if imp.trait_.is_none() => {
                    if let syn::Type::Path(tp) = imp.self_ty.as_ref() {
                        if tp.path.segments.last().is_some_and(|s| s.ident == type_name) {
                            return Some(imp);
                        }
                    }
                    None
                }
                _ => None,
            })
            .collect()
    }

    pub(super) fn all_method_names(file: &File, type_name: &str) -> Vec<String> {
        find_all_inherent_impls(file, type_name)
            .iter()
            .flat_map(|imp| impl_item_names(imp, ItemKind::Method))
            .collect()
    }

    pub(super) fn assert_all_methods_contains(file: &File, type_name: &str, expected: &[&str]) {
        let methods = all_method_names(file, type_name);
        for name in expected {
            assert!(
                methods.contains(&name.to_string()),
                "expected method `{name}` not found on `{type_name}`; found: {methods:?}"
            );
        }
    }

    pub(super) fn assert_all_methods_not_contains(
        file: &File,
        type_name: &str,
        unexpected: &[&str],
    ) {
        let methods = all_method_names(file, type_name);
        for name in unexpected {
            assert!(
                !methods.contains(&name.to_string()),
                "unexpected method `{name}` found on `{type_name}`; found: {methods:?}"
            );
        }
    }

    pub(super) fn find_struct<'a>(file: &'a File, name: &str) -> &'a ItemStruct {
        file.items
            .iter()
            .find_map(|item| match item {
                Item::Struct(s) if s.ident == name => Some(s),
                _ => None,
            })
            .unwrap_or_else(|| panic!("struct `{name}` not found"))
    }

    /// Checks if the token stream contains an identifier matching `name`.
    ///
    /// Walks the raw token tree recursively, which is more robust than
    /// `TokenStream::to_string().contains()` since it matches complete idents
    /// rather than arbitrary substrings.
    pub(super) fn has_ident(tokens: &TokenStream, name: &str) -> bool {
        use proc_macro2::TokenTree;
        for tt in tokens.clone() {
            match tt {
                TokenTree::Ident(ref id) if *id == name => return true,
                TokenTree::Group(ref g) => {
                    if has_ident(&g.stream(), name) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

fn event_field(
    name: &str,
    sol_type: TokenStream,
    indexed: bool,
    indexed_as_hash: bool,
) -> EventFieldInfo {
    EventFieldInfo {
        name: ident(name),
        sol_type,
        is_indexed: indexed,
        indexed_as_hash,
        span: Span::call_site(),
    }
}
