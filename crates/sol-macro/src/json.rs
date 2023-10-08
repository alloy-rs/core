use alloy_json_abi::{ContractObject, JsonAbi};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{Attribute, Result};

pub fn expand(name: Ident, json: ContractObject, attrs: Vec<Attribute>) -> Result<TokenStream> {
    let ContractObject {
        abi,
        bytecode,
        deployed_bytecode,
    } = json;

    let abi = abi.ok_or_else(|| syn::Error::new(name.span(), "ABI not found in JSON"))?;
    let abi = expand_abi(&name, abi)?;
    let bytecode = bytecode.map(|bytes| {
        let s = bytes.to_string();
        quote!(bytecode = #s,)
    });
    let deployed_bytecode = deployed_bytecode.map(|bytes| {
        let s = bytes.to_string();
        quote!(deployed_bytecode = #s)
    });

    let tokens = quote! {
        #(#attrs)*
        #[sol(#bytecode #deployed_bytecode)]
        #abi
    };

    let ast = syn::parse2(tokens).map_err(|e| {
        let msg = format!(
            "failed to parse ABI-generated tokens into a Solidity AST: {e}.\n\
             This is a bug. We would appreciate a bug report: \
             https://github.com/alloy-rs/core/issues/new/choose"
        );
        syn::Error::new(name.span(), msg)
    })?;
    crate::expand::expand(ast)
}

/// Returns `sol!` tokens.
fn expand_abi(name: &Ident, mut abi: JsonAbi) -> Result<TokenStream> {
    let mk_err = |s: &str| {
        let msg = format!(
            "`JsonAbi::to_sol` generated invalid Rust tokens: {s}\n\
             This is a bug. We would appreciate a bug report: \
             https://github.com/alloy-rs/core/issues/new/choose"
        );
        syn::Error::new(name.span(), msg)
    };
    dedup_abi(&mut abi);
    let s = abi.to_sol(&name.to_string());
    let brace_idx = s.find('{').ok_or_else(|| mk_err("missing `{`"))?;
    let tts = syn::parse_str::<TokenStream>(&s[brace_idx..]).map_err(|e| mk_err(&e.to_string()))?;

    let mut tokens = TokenStream::new();
    // append `name` manually for the span
    tokens.append(id("interface"));
    tokens.append(name.clone());
    tokens.extend(tts);
    Ok(tokens)
}

fn dedup_abi(abi: &mut JsonAbi) {
    macro_rules! deduper {
        () => {
            |a, b| {
                assert_eq!(a.name, b.name);
                a.inputs == b.inputs
            }
        };
    }
    for functions in abi.functions.values_mut() {
        functions.dedup_by(deduper!());
    }
    for errors in abi.errors.values_mut() {
        errors.dedup_by(deduper!());
    }
    for events in abi.events.values_mut() {
        events.dedup_by(deduper!());
    }
}

#[track_caller]
#[inline]
fn id(s: impl AsRef<str>) -> Ident {
    // Ident::new panics on rust keywords
    syn::parse_str(s.as_ref()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Item;
    use std::path::Path;

    #[test]
    #[cfg_attr(miri, ignore = "no fs")]
    fn abi() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../json-abi/tests/abi");
        for file in std::fs::read_dir(path).unwrap() {
            let path = file.unwrap().path();
            assert_eq!(path.extension(), Some("json".as_ref()));
            if path.file_name() == Some("LargeFunction.json".as_ref()) {
                continue
            }
            parse_test(
                &std::fs::read_to_string(&path).unwrap(),
                path.to_str().unwrap(),
            );
        }
    }

    #[allow(clippy::single_match)]
    fn parse_test(s: &str, path: &str) {
        let (c, name) = expand_test(s, path);
        match name {
            "Udvts" => {
                assert_eq!(c.name, "Udvts");
                assert_eq!(c.body.len(), 12, "{}, {:#?}", c.body.len(), c);
                let [Item::Udt(a), Item::Udt(b), Item::Udt(c), rest @ ..] = &c.body[..] else {
                    for item in &c.body {
                        eprintln!("{item:?}\n");
                    }
                    panic!();
                };

                assert_eq!(a.name, "ItemType");
                assert_eq!(a.ty.to_string(), "bytes32");

                assert_eq!(b.name, "OrderType");
                assert_eq!(b.ty.to_string(), "uint256");

                assert_eq!(c.name, "Side");
                assert_eq!(c.ty.to_string(), "bool");

                rest[..8]
                    .iter()
                    .for_each(|item| assert!(matches!(item, Item::Struct(_))));

                let last = &rest[8];
                assert!(rest[9..].is_empty());
                let Item::Function(f) = last else {
                    panic!("{last:#?}")
                };
                assert_eq!(f.name.as_ref().unwrap(), "fulfillAvailableAdvancedOrders");
                assert!(f.attributes.contains(&ast::FunctionAttribute::Mutability(
                    ast::Mutability::Payable(Default::default())
                )));
                assert!(f.attributes.contains(&ast::FunctionAttribute::Visibility(
                    ast::Visibility::External(Default::default())
                )));

                let args = &f.arguments;
                assert_eq!(args.len(), 7);

                assert_eq!(args[0].ty.to_string(), "AdvancedOrder[]");
                assert_eq!(args[0].name.as_ref().unwrap(), "a");
                assert_eq!(args[1].ty.to_string(), "CriteriaResolver[]");
                assert_eq!(args[1].name.as_ref().unwrap(), "b");
                assert_eq!(args[2].ty.to_string(), "FulfillmentComponent[][]");
                assert_eq!(args[2].name.as_ref().unwrap(), "c");
                assert_eq!(args[3].ty.to_string(), "FulfillmentComponent[][]");
                assert_eq!(args[3].name.as_ref().unwrap(), "d");
                assert_eq!(args[4].ty.to_string(), "bytes32");
                assert_eq!(args[4].name.as_ref().unwrap(), "fulfillerConduitKey");
                assert_eq!(args[5].ty.to_string(), "address");
                assert_eq!(args[5].name.as_ref().unwrap(), "recipient");
                assert_eq!(args[6].ty.to_string(), "uint256");
                assert_eq!(args[6].name.as_ref().unwrap(), "maximumFulfilled");

                let returns = &f.returns.as_ref().unwrap().returns;
                assert_eq!(returns.len(), 2);

                assert_eq!(returns[0].ty.to_string(), "bool[]");
                assert_eq!(returns[0].name.as_ref().unwrap(), "e");
                assert_eq!(returns[1].ty.to_string(), "Execution[]");
                assert_eq!(returns[1].name.as_ref().unwrap(), "f");
            }
            _ => {}
        }
    }

    fn expand_test<'a>(s: &str, path: &'a str) -> (ast::ItemContract, &'a str) {
        let abi: JsonAbi = serde_json::from_str(s).unwrap();
        let name = Path::new(path).file_stem().unwrap().to_str().unwrap();
        let tokens = expand_abi(&id(name), abi).expect("couldn't expand JSON ABI");
        let ast: ast::File = syn::parse2(tokens).expect("couldn't ABI parse back to AST");
        let mut items = ast.items.into_iter();
        let Some(Item::Contract(c)) = items.next() else {
            panic!()
        };
        let next = items.next();
        assert!(next.is_none(), "{next:#?}, {items:#?}");
        assert!(!c.body.is_empty());
        (c, name)
    }
}
