#[allow(missing_docs)]
#[allow(unused)]
mod json {
    use alloy_json_abi::{Function, JsonAbi, Param, StateMutability};
    use alloy_primitives::{Address, B256, I256, Signed, U256};
    use alloy_sol_types::{SolCall, SolError, SolEvent, SolStruct, sol};
    use pretty_assertions::assert_eq;
    use std::borrow::Cow;
    const _ : & 'static [u8] = b"[\n  {\n    \"type\": \"function\",\n    \"name\": \"handle\",\n    \"inputs\": [\n      {\n        \"name\": \"foobar\",\n        \"type\": \"tuple\",\n        \"internalType\": \"struct IHandler.FooBar\",\n        \"components\": [\n          {\n            \"name\": \"foo\",\n            \"type\": \"tuple\",\n            \"internalType\": \"struct Foo\",\n            \"components\": [\n              {\n                \"name\": \"newNumber\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n              }\n            ]\n          }\n        ]\n      }\n    ],\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\"\n  }\n]" ;
    ///Module containing a contract's types and functions.
    ///
    ///
    ///```solidity
    ///library IHandler {
    ///    struct FooBar { Foo foo; }
    ///}
    ///```
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style,
        clippy::empty_structs_with_brackets
    )]
    pub mod IHandler {
        use super::*;
        use :: alloy_sol_types;
        ///```solidity
        ///struct FooBar { Foo foo; }
        ///```
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct FooBar {
            #[allow(missing_docs)]
            pub foo: <Foo as ::alloy_sol_types::SolType>::RustType,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for FooBar {
            #[inline]
            fn clone(&self) -> FooBar {
                FooBar {
                    foo: ::core::clone::Clone::clone(&self.foo),
                }
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use :: alloy_sol_types;
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Foo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<Foo as ::alloy_sol_types::SolType>::RustType,);
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<FooBar> for UnderlyingRustTuple<'_> {
                fn from(value: FooBar) -> Self {
                    (value.foo,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for FooBar {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { foo: tuple.0 }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolValue for FooBar {
                type SolType = Self;
            }
            #[automatically_derived]
            impl alloy_sol_types::private::SolTypeValue<Self> for FooBar {
                #[inline]
                fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                    (<Foo as alloy_sol_types::SolType>::tokenize(&self.foo),)
                }
                #[inline]
                fn stv_abi_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                        return size;
                    }
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
                }
                #[inline]
                fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                    <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
                }
                #[inline]
                fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                        &tuple, out,
                    )
                }
                #[inline]
                fn stv_abi_packed_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                        return size;
                    }
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                        &tuple,
                    )
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolType for FooBar {
                type RustType = Self;
                type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
                const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
                const ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
                const PACKED_ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
                #[inline]
                fn valid_token(token: &Self::Token<'_>) -> bool {
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
                }
                #[inline]
                fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                    let tuple =
                        <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                    <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolStruct for FooBar {
                const NAME: &'static str = "FooBar";
                #[inline]
                fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                    alloy_sol_types::private::Cow::Borrowed("FooBar(Foo foo)")
                }
                #[inline]
                fn eip712_components(
                ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
                {
                    let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                    components.push(<Foo as alloy_sol_types::SolStruct>::eip712_root_type());
                    components.extend(<Foo as alloy_sol_types::SolStruct>::eip712_components());
                    components
                }
                #[inline]
                fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                    <Foo as alloy_sol_types::SolType>::eip712_data_word(&self.foo)
                        .0
                        .to_vec()
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::EventTopic for FooBar {
                #[inline]
                fn topic_preimage_length(rust: &Self::RustType) -> usize {
                    0usize + <Foo as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.foo)
                }
                #[inline]
                fn encode_topic_preimage(
                    rust: &Self::RustType,
                    out: &mut alloy_sol_types::private::Vec<u8>,
                ) {
                    out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                    <Foo as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.foo, out);
                }
                #[inline]
                fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                    let mut out = alloy_sol_types::private::Vec::new();
                    <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                    alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
                }
            }
        };
    }
    ///
    ///
    ///Generated by the following Solidity interface...
    ///```solidity
    ///library IHandler {
    ///    struct FooBar {
    ///        Foo foo;
    ///    }
    ///}
    ///
    ///interface Handler {
    ///    struct Foo {
    ///        uint256 newNumber;
    ///    }
    ///
    ///    function handle(IHandler.FooBar memory foobar) external;
    ///}
    ///```
    ///
    ///...which was generated by the following JSON ABI:
    ///```json
    ///[
    ///  {
    ///    "type": "function",
    ///    "name": "handle",
    ///    "inputs": [
    ///      {
    ///        "name": "foobar",
    ///        "type": "tuple",
    ///        "internalType": "struct IHandler.FooBar",
    ///        "components": [
    ///          {
    ///            "name": "foo",
    ///            "type": "tuple",
    ///            "internalType": "struct Foo",
    ///            "components": [
    ///              {
    ///                "name": "newNumber",
    ///                "type": "uint256",
    ///                "internalType": "uint256"
    ///              }
    ///            ]
    ///          }
    ///        ]
    ///      }
    ///    ],
    ///    "outputs": [],
    ///    "stateMutability": "nonpayable"
    ///  }
    ///]
    ///```
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style,
        clippy::empty_structs_with_brackets
    )]
    pub mod Handler {
        use super::*;
        use :: alloy_sol_types;
        ///```solidity
        ///struct Foo { uint256 newNumber; }
        ///```
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct Foo {
            #[allow(missing_docs)]
            pub newNumber: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for Foo {
            #[inline]
            fn clone(&self) -> Foo {
                Foo {
                    newNumber: ::core::clone::Clone::clone(&self.newNumber),
                }
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use :: alloy_sol_types;
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::primitives::aliases::U256,);
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<Foo> for UnderlyingRustTuple<'_> {
                fn from(value: Foo) -> Self {
                    (value.newNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for Foo {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newNumber: tuple.0 }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolValue for Foo {
                type SolType = Self;
            }
            #[automatically_derived]
            impl alloy_sol_types::private::SolTypeValue<Self> for Foo {
                #[inline]
                fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                    (< :: alloy_sol_types :: sol_data :: Uint < 256 > as alloy_sol_types :: SolType > :: tokenize (& self . newNumber) ,)
                }
                #[inline]
                fn stv_abi_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                        return size;
                    }
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
                }
                #[inline]
                fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                    <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
                }
                #[inline]
                fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                        &tuple, out,
                    )
                }
                #[inline]
                fn stv_abi_packed_encoded_size(&self) -> usize {
                    if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                        return size;
                    }
                    let tuple = <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(
                        self.clone(),
                    );
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                        &tuple,
                    )
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolType for Foo {
                type RustType = Self;
                type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
                const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
                const ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
                const PACKED_ENCODED_SIZE: Option<usize> =
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
                #[inline]
                fn valid_token(token: &Self::Token<'_>) -> bool {
                    <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
                }
                #[inline]
                fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                    let tuple =
                        <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                    <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolStruct for Foo {
                const NAME: &'static str = "Foo";
                #[inline]
                fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                    alloy_sol_types::private::Cow::Borrowed("Foo(uint256 newNumber)")
                }
                #[inline]
                fn eip712_components(
                ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
                {
                    alloy_sol_types::private::Vec::new()
                }
                #[inline]
                fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                    <Self as alloy_sol_types::SolStruct>::eip712_root_type()
                }
                #[inline]
                fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                    < :: alloy_sol_types :: sol_data :: Uint < 256 > as alloy_sol_types :: SolType > :: eip712_data_word (& self . newNumber) . 0 . to_vec ()
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::EventTopic for Foo {
                #[inline]
                fn topic_preimage_length(rust: &Self::RustType) -> usize {
                    0usize + < :: alloy_sol_types :: sol_data :: Uint < 256 > as alloy_sol_types :: EventTopic > :: topic_preimage_length (& rust . newNumber)
                }
                #[inline]
                fn encode_topic_preimage(
                    rust: &Self::RustType,
                    out: &mut alloy_sol_types::private::Vec<u8>,
                ) {
                    out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                    < :: alloy_sol_types :: sol_data :: Uint < 256 > as alloy_sol_types :: EventTopic > :: encode_topic_preimage (& rust . newNumber , out) ;
                }
                #[inline]
                fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                    let mut out = alloy_sol_types::private::Vec::new();
                    <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                    alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
                }
            }
        };
        ///Function with signature `handle(((uint256)))` and selector `0xf150aec6`.
        ///```solidity
        ///function handle(IHandler.FooBar memory foobar) external;
        ///```
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct handleCall {
            #[allow(missing_docs)]
            pub foobar: <IHandler::FooBar as ::alloy_sol_types::SolType>::RustType,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for handleCall {
            #[inline]
            fn clone(&self) -> handleCall {
                handleCall {
                    foobar: ::core::clone::Clone::clone(&self.foobar),
                }
            }
        }
        ///Container type for the return parameters of the [`handle(((uint256)))`](handleCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct handleReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for handleReturn {
            #[inline]
            fn clone(&self) -> handleReturn {
                handleReturn {}
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use :: alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (IHandler::FooBar,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> =
                (<IHandler::FooBar as ::alloy_sol_types::SolType>::RustType,);
                #[allow(dead_code, unreachable_patterns)]
                fn _type_assertion(
                    _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
                ) {
                    match _t {
                        alloy_sol_types::private::AssertTypeEq::<
                            <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                        >(_) => {}
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<handleCall> for UnderlyingRustTuple<'_> {
                    fn from(value: handleCall) -> Self {
                        (value.foobar,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { foobar: tuple.0 }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = ();
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = ();
                #[allow(dead_code, unreachable_patterns)]
                fn _type_assertion(
                    _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
                ) {
                    match _t {
                        alloy_sol_types::private::AssertTypeEq::<
                            <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                        >(_) => {}
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<handleReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: handleReturn) -> Self {
                        ()
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self {}
                    }
                }
            }
            impl handleReturn {
                fn _tokenize(&self) -> <handleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                    ()
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for handleCall {
                type Parameters<'a> = (IHandler::FooBar,);
                type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
                type Return = handleReturn;
                type ReturnTuple<'a> = ();
                type ReturnToken<'a> =
                <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "handle(((uint256)))";
                const SELECTOR: [u8; 4] = [241u8, 80u8, 174u8, 198u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (<IHandler::FooBar as alloy_sol_types::SolType>::tokenize(
                        &self.foobar,
                    ),)
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    handleReturn::_tokenize(ret)
                }
                #[inline]
                fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(Into::into)
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    < Self :: ReturnTuple < '_ > as alloy_sol_types :: SolType > :: abi_decode_sequence_validate (data) . map (Into :: into)
                }
            }
        };
        ///Container for all the [`Handler`](self) function calls.
        pub enum HandlerCalls {
            #[allow(missing_docs)]
            handle(handleCall),
        }
        #[automatically_derived]
        impl HandlerCalls {
            /// All the selectors of this enum.
            ///
            /// Note that the selectors might not be in the same order as the variants.
            /// No guarantees are made about the order of the selectors.
            ///
            /// Prefer using `SolInterface` methods instead.
            pub const SELECTORS: &'static [[u8; 4usize]] = &[[241u8, 80u8, 174u8, 198u8]];
        }
        #[automatically_derived]
        impl alloy_sol_types::SolInterface for HandlerCalls {
            const NAME: &'static str = "HandlerCalls";
            const MIN_DATA_LENGTH: usize = 32usize;
            const COUNT: usize = 1usize;
            #[inline]
            fn selector(&self) -> [u8; 4] {
                match self {
                    Self::handle(_) => <handleCall as alloy_sol_types::SolCall>::SELECTOR,
                }
            }
            #[inline]
            fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
                Self::SELECTORS.get(i).copied()
            }
            #[inline]
            fn valid_selector(selector: [u8; 4]) -> bool {
                Self::SELECTORS.binary_search(&selector).is_ok()
            }
            #[inline]
            #[allow(non_snake_case)]
            fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
                static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<HandlerCalls>] = &[{
                    fn handle(data: &[u8]) -> alloy_sol_types::Result<HandlerCalls> {
                        <handleCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(HandlerCalls::handle)
                    }
                    handle
                }];
                let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                    return Err(alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ));
                };
                DECODE_SHIMS[idx](data)
            }
            #[inline]
            #[allow(non_snake_case)]
            fn abi_decode_raw_validate(
                selector: [u8; 4],
                data: &[u8],
            ) -> alloy_sol_types::Result<Self> {
                static DECODE_VALIDATE_SHIMS: &[fn(
                    &[u8],
                )
                    -> alloy_sol_types::Result<HandlerCalls>] = &[{
                    fn handle(data: &[u8]) -> alloy_sol_types::Result<HandlerCalls> {
                        <handleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(HandlerCalls::handle)
                    }
                    handle
                }];
                let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                    return Err(alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ));
                };
                DECODE_VALIDATE_SHIMS[idx](data)
            }
            #[inline]
            fn abi_encoded_size(&self) -> usize {
                match self {
                    Self::handle(inner) => {
                        <handleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                    }
                }
            }
            #[inline]
            fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                match self {
                    Self::handle(inner) => {
                        <handleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                    }
                }
            }
        }
    }
}
