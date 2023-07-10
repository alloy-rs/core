use alloy_json_abi::{
    AbiItem, ContractObject, Error, Event, EventParam, InternalType, JsonAbi, Param,
    StateMutability,
};
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenStream};
use quote::{quote, TokenStreamExt};
use std::collections::{BTreeMap, BTreeSet};
use syn::Result;

pub fn expand(name: Ident, json: ContractObject) -> Result<TokenStream> {
    let ContractObject {
        abi,
        bytecode,
        deployed_bytecode,
    } = json;

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
        #![sol(all_derives)]

        #[sol(#bytecode #deployed_bytecode)]
        #abi
    };

    let ast = syn::parse2(tokens).map_err(|e| {
        let msg = "\
            failed to parse generated tokens into a Solidity AST.\n\
            This is a bug, please report it at \
            https://github.com/alloy-rs/core/issues/new/choose";
        let mut e2 = syn::Error::new(name.span(), msg);
        e2.combine(e);
        e2
    })?;
    crate::expand::expand(ast)
}

/// Returns `sol!` tokens.
fn expand_abi(name: &Ident, abi: JsonAbi) -> Result<TokenStream> {
    let mut structs = BTreeMap::new();
    let mut enums = BTreeSet::new();
    let mut udvts = BTreeMap::new();
    let mut add_items = |internal_type: Option<&_>, components: &Vec<_>, real_ty: &str| {
        if let Some(internal_type) = internal_type {
            match internal_type {
                InternalType::AddressPayable(_) | InternalType::Contract(_) => {}
                InternalType::Struct { contract: _, ty } => {
                    structs.insert(struct_ident(ty).to_owned(), components.clone());
                }
                InternalType::Enum { contract: _, ty } => {
                    enums.insert(struct_ident(ty).to_owned());
                }
                InternalType::Other { contract: _, ty } => {
                    // `Other` is a UDVT if it's not a basic Solidity type
                    if let Some(it) = internal_type.other_specifier() {
                        if it.try_basic_solidity().is_err() {
                            let _ = dbg!(it.try_basic_solidity());
                            udvts.insert(struct_ident(ty).to_owned(), real_ty.to_owned());
                        }
                    }
                }
            }
        }
    };
    for item in abi.items() {
        recurse_item_params(item, &mut add_items);
    }

    let enums = enums.iter().map(expand_enum);
    let udvts = udvts.iter().map(expand_udvt);

    let structs = structs.iter().map(expand_struct);
    let events = abi.events.values().flatten().map(expand_event);
    let errors = abi.errors.values().flatten().map(expand_error);

    let constructor = abi
        .constructor
        .as_ref()
        .map(|c| AbiFunction::Constructor(&c.inputs).expand(c.state_mutability));
    let fallback = abi
        .fallback
        .as_ref()
        .map(|f| AbiFunction::Fallback.expand(f.state_mutability));
    let receive = abi
        .receive
        .as_ref()
        .map(|r| AbiFunction::Receive.expand(r.state_mutability));
    let functions =
        abi.functions.values().flatten().map(|f| {
            AbiFunction::Function(&f.name, &f.inputs, &f.outputs).expand(f.state_mutability)
        });

    let tokens = quote! {
        interface #name {
            #(#enums)*
            #(#udvts)*

            #(#structs)*
            #(#events)*
            #(#errors)*

            #constructor
            #fallback
            #receive
            #(#functions)*
        }
    };
    Ok(tokens)
}

fn recurse_item_params<F>(item: AbiItem<'_>, f: &mut F)
where
    F: FnMut(Option<&InternalType>, &Vec<Param>, &str),
{
    if let Some(params) = item.inputs() {
        recurse_params(params, f)
    }
    if let Some(params) = item.outputs() {
        recurse_params(params, f)
    }
    if let Some(params) = item.event_inputs() {
        recurse_event_params(params, f)
    }
}

fn recurse_params<F>(params: &[Param], f: &mut F)
where
    F: FnMut(Option<&InternalType>, &Vec<Param>, &str),
{
    params.iter().for_each(|param| recurse_param(param, f));
}

fn recurse_event_params<F>(params: &[EventParam], f: &mut F)
where
    F: FnMut(Option<&InternalType>, &Vec<Param>, &str),
{
    for param in params {
        f(param.internal_type.as_ref(), &param.components, &param.ty);
        recurse_params(&param.components, f);
    }
}

fn recurse_param<F>(param: &Param, f: &mut F)
where
    F: FnMut(Option<&InternalType>, &Vec<Param>, &str),
{
    f(param.internal_type.as_ref(), &param.components, &param.ty);
    recurse_params(&param.components, f);
}

/// There is no way to get the variants of the enum from the ABI.
///
/// `type #name is uint8;`
fn expand_enum(name: &String) -> TokenStream {
    let name = id(name);
    quote!(type #name is uint8;)
}

/// `type #name is #ty;`
fn expand_udvt((name, ty): (&String, &String)) -> TokenStream {
    let name = id(name);
    let ty = syn::parse_str::<TokenStream>(ty).unwrap();
    quote!(type #name is #ty;)
}

/// `struct #name { #(#fields;)* }`
fn expand_struct((name, fields): (&String, &Vec<Param>)) -> TokenStream {
    let name = id(name);
    let fields = expand_params(fields);
    quote!(struct #name { #(#fields;)* })
}

/// `event #name(#inputs) #anonymous;`
fn expand_event(event: &Event) -> TokenStream {
    let name = id(&event.name);
    let inputs = expand_event_params(&event.inputs);
    let anonymous = event.anonymous.then(|| id("anonymous"));
    quote!(event #name(#(#inputs),*) #anonymous;)
}

/// `error #name(#inputs);`
fn expand_error(error: &Error) -> TokenStream {
    let name = id(&error.name);
    let inputs = expand_params(&error.inputs);
    quote!(error #name(#(#inputs),*);)
}

/// `#kind #(#name)? (#inputs) #state_mutability #(returns (#outputs))?;`
enum AbiFunction<'a> {
    Constructor(&'a [Param]),
    Fallback,
    Receive,
    Function(&'a str, &'a [Param], &'a [Param]),
}

impl AbiFunction<'_> {
    fn expand(self, state_mutability: StateMutability) -> TokenStream {
        let (kw, name, inputs, visibility, outputs) = match self {
            AbiFunction::Constructor(inputs) => ("constructor", None, Some(inputs), None, None),
            AbiFunction::Fallback => ("fallback", None, None, Some("external"), None),
            AbiFunction::Receive => ("receive", None, None, Some("external"), None),
            AbiFunction::Function(name, inputs, outputs) => (
                "function",
                Some(name),
                Some(inputs),
                Some("external"),
                Some(outputs),
            ),
        };

        let mut tokens = TokenStream::new();

        tokens.append(id(kw));
        if let Some(name) = name {
            tokens.append(id(name));
        }

        let inputs = match inputs.map(expand_params) {
            Some(inputs) => quote!(#(#inputs),*),
            None => quote!(),
        };
        tokens.append(Group::new(Delimiter::Parenthesis, inputs));

        if let Some(visibility) = visibility {
            tokens.append(id(visibility));
        }

        if let Some(state_mutability) = state_mutability.as_str() {
            tokens.append(id(state_mutability));
        }

        if let Some(outputs) = outputs {
            if !outputs.is_empty() {
                tokens.append(id("returns"));
                let outputs = expand_params(outputs);
                tokens.append(Group::new(Delimiter::Parenthesis, quote!(#(#outputs),*)));
            }
        }

        tokens.append(punct(';'));

        tokens
    }
}

// Param list
fn expand_params(params: &[Param]) -> impl Iterator<Item = TokenStream> + '_ {
    expand_params_(params.iter().map(|p| {
        (
            &p.name[..],
            &p.ty[..],
            p.internal_type.as_ref(),
            &p.components[..],
            false,
        )
    }))
}

fn expand_event_params(params: &[EventParam]) -> impl Iterator<Item = TokenStream> + '_ {
    expand_params_(params.iter().map(|p| {
        (
            &p.name[..],
            &p.ty[..],
            p.internal_type.as_ref(),
            &p.components[..],
            p.indexed,
        )
    }))
}

type Tuple<'a> = (
    &'a str,
    &'a str,
    Option<&'a InternalType>,
    &'a [Param],
    bool,
);

fn expand_params_<'a, I>(params: I) -> impl Iterator<Item = TokenStream> + 'a
where
    I: Iterator<Item = Tuple<'a>> + 'a,
{
    params.map(|(name, ty, internal_type, _components, indexed)| {
        let mut tokens = TokenStream::new();
        let mut type_name = ty;
        if let Some(it) = internal_type {
            match it {
                InternalType::Struct { ty, .. }
                | InternalType::Enum { ty, .. }
                | InternalType::Other { ty, .. } => {
                    type_name = ty;
                }
                _ => {}
            }
        }

        tokens.extend(syn::parse_str::<TokenStream>(type_name).unwrap());
        if indexed {
            tokens.append(id("indexed"))
        }
        if !name.is_empty() {
            tokens.append(id(name));
        }

        tokens
    })
}

#[inline]
fn struct_ident(s: &str) -> &str {
    s.split('[').next().unwrap()
}

#[track_caller]
#[inline]
fn id(s: impl AsRef<str>) -> Ident {
    // Ident::new panics on rust keywords
    syn::parse_str(s.as_ref()).unwrap()
}

#[track_caller]
#[inline]
fn punct(s: char) -> Punct {
    Punct::new(s, Spacing::Alone)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::Item;
    use std::path::Path;

    macro_rules! abi_tests {
        ($($name:ident($path:literal))*) => {$(
            #[test]
            fn $name() {
                parse_test(include_str!(concat!("../../json-abi/tests/", $path)), $path);
            }
        )*};
    }

    abi_tests! {
        abiencoderv2("abi/Abiencoderv2Test.json")
        console("abi/console.json")
        event_with_struct("abi/EventWithStruct.json")
        large_array("abi/LargeArray.json")
        large_struct("abi/LargeStruct.json")
        large_structs("abi/LargeStructs.json")
        large_tuple("abi/LargeTuple.json")
        seaport("abi/Seaport.json")
        udvts("abi/Udvts.json")
    }

    #[allow(clippy::single_match)]
    fn parse_test(s: &str, path: &'static str) {
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

    fn expand_test(s: &str, path: &'static str) -> (ast::ItemContract, &'static str) {
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
