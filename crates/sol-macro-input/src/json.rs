use crate::{SolAttrs, SolInput, SolInputKind};
use alloy_json_abi::{ContractObject, JsonAbi, ToSolConfig};
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;
use syn::{AttrStyle, Result};

impl SolInput {
    /// Normalize JSON ABI inputs into Sol inputs.
    pub fn normalize_json(self) -> Result<Self> {
        let SolInput {
            attrs,
            path,
            kind: SolInputKind::Json(name, ContractObject { abi, bytecode, deployed_bytecode }),
        } = self
        else {
            return Ok(self);
        };

        let mut abi = abi.ok_or_else(|| syn::Error::new(name.span(), "ABI not found in JSON"))?;
        let (sol_attrs, _) = SolAttrs::parse(&attrs)?;
        let standalone_globals = sol_attrs.standalone_globals.unwrap_or(false);
        let config = ToSolConfig::new()
            .print_constructors(true)
            .for_sol_macro(true)
            .standalone_globals(standalone_globals);
        let sol = abi_to_sol(&name, &mut abi, config);
        let all_tokens = tokens_for_sol(&name, &sol)?;
        let mut ast: ast::File = syn::parse2(all_tokens).map_err(|e| {
            let msg = format!(
                "failed to parse ABI-generated tokens into a Solidity AST for `{name}`: {e}.\n\
                 This is a bug. We would appreciate a bug report: \
                 https://github.com/alloy-rs/core/issues/new/choose"
            );
            syn::Error::new(name.span(), msg)
        })?;

        let (inner_attrs, attrs) = attrs
            .into_iter()
            .partition::<Vec<_>, _>(|attr| matches!(attr.style, AttrStyle::Inner(_)));

        let (derives, sol_derives) = extract_derive_attrs(&attrs);

        let bytecode = bytecode.map(|bytes| {
            let s = bytes.to_string();
            quote!(bytecode = #s,)
        });
        let deployed_bytecode = deployed_bytecode.map(|bytes| {
            let s = bytes.to_string();
            quote!(deployed_bytecode = #s)
        });

        let ctx = ApplyAttrsCtx {
            derives: &derives,
            sol_derives: &sol_derives,
            interface_attrs: &attrs,
            bytecode: bytecode.as_ref(),
            deployed_bytecode: deployed_bytecode.as_ref(),
            sol: &sol,
            abi: &abi,
        };
        apply_attrs_to_items(&mut ast.items, &ctx);
        ast.attrs.extend(inner_attrs);

        let kind = SolInputKind::Sol(ast);
        Ok(SolInput { attrs, path, kind })
    }
}

/// Shared context for applying user attributes to ABI-derived items.
struct ApplyAttrsCtx<'a> {
    derives: &'a [&'a syn::Attribute],
    sol_derives: &'a [&'a syn::Attribute],
    interface_attrs: &'a [syn::Attribute],
    bytecode: Option<&'a TokenStream>,
    deployed_bytecode: Option<&'a TokenStream>,
    sol: &'a str,
    abi: &'a JsonAbi,
}

// doesn't parse Json

fn abi_to_sol(name: &Ident, abi: &mut JsonAbi, config: ToSolConfig) -> String {
    abi.dedup();
    abi.to_sol(&name.to_string(), Some(config))
}

/// Returns `sol!` tokens.
pub fn tokens_for_sol(name: &Ident, sol: &str) -> Result<TokenStream> {
    let mk_err = |s: &str| {
        let msg = format!(
            "`JsonAbi::to_sol` generated invalid Rust tokens for `{name}`: {s}\n\
             This is a bug. We would appreciate a bug report: \
             https://github.com/alloy-rs/core/issues/new/choose"
        );
        syn::Error::new(name.span(), msg)
    };
    let tts = syn::parse_str::<TokenStream>(sol).map_err(|e| mk_err(&e.to_string()))?;
    Ok(tts
        .into_iter()
        .map(|mut tt| {
            if matches!(&tt, TokenTree::Ident(id) if id == name) {
                tt.set_span(name.span());
            }
            tt
        })
        .collect())
}

/// Extract both regular and `sol` derive attributes for propagation further.
fn extract_derive_attrs(attrs: &[syn::Attribute]) -> (Vec<&syn::Attribute>, Vec<&syn::Attribute>) {
    attrs.iter().fold((Vec::new(), Vec::new()), |(mut derives, mut sol_derives), attr| {
        if attr.path().is_ident("derive") {
            derives.push(attr);
        } else if attr.path().is_ident("sol") {
            if let Ok(meta) = attr.meta.require_list() {
                let mut contains_derives = false;
                let _ = meta.parse_nested_meta(|meta| {
                    contains_derives |=
                        meta.path.is_ident("all_derives") || meta.path.is_ident("extra_derives");
                    Ok(())
                });
                if contains_derives {
                    sol_derives.push(attr);
                }
            }
        }
        (derives, sol_derives)
    })
}

/// Applies derive/`sol` attributes to ABI-derived items.
///
/// - Non-interface contracts, structs, enums, and UDVTs get the user-specified derive and `sol`
///   attributes cloned onto them.
/// - The single interface gets the outer attributes, a generated doc (including the original
///   Solidity/JSON ABI), and the `#[sol(bytecode = ..., deployed_bytecode = ...)]` attribute.
fn apply_attrs_to_items(items: &mut [ast::Item], ctx: &ApplyAttrsCtx<'_>) {
    for item in items {
        match item {
            ast::Item::Contract(contract) if contract.kind.is_interface() => {
                apply_interface_attrs(contract, ctx);
            }
            ast::Item::Contract(contract) => {
                extend_attrs(&mut contract.attrs, ctx.derives, ctx.sol_derives);
            }
            ast::Item::Struct(strukt) => {
                extend_attrs(&mut strukt.attrs, ctx.derives, ctx.sol_derives);
            }
            ast::Item::Udt(udt) => {
                extend_attrs(&mut udt.attrs, ctx.derives, ctx.sol_derives);
            }
            // Globals from `to_sol` are only structs, UDVTs; enums are flattened to `uint8`,
            // while errors/functions/events,etc are emitted in the interface.
            _ => debug_assert!(false, "unexpected global item type"),
        }
    }
}

/// Merge user outer attrs with generated docs/metadata for the sole interface.
fn apply_interface_attrs(contract: &mut ast::ItemContract, ctx: &ApplyAttrsCtx<'_>) {
    let bytecode = ctx.bytecode;
    let deployed_bytecode = ctx.deployed_bytecode;
    let doc_str = format!(
        "\n\n\
Generated by the following Solidity interface...\
```solidity\
{sol}\
```\
\n\
...which was generated by the following JSON ABI:\
```json\
{json_s}\
```",
        sol = ctx.sol,
        json_s = serde_json::to_string_pretty(ctx.abi).unwrap(),
    );
    let doc_attr: syn::Attribute = syn::parse_quote!(#[doc = #doc_str]);
    let sol_attr: syn::Attribute = syn::parse_quote!(#[sol(#bytecode #deployed_bytecode)]);

    let mut merged = ctx.interface_attrs.to_vec();
    merged.push(doc_attr);
    merged.push(sol_attr);
    contract.attrs = merged;
}

/// Clone user-specified `derive`/`sol(...)` attributes onto the given item.
/// Used for globals (structs/UDVTs) and non-interface contracts emitted by `to_sol`.
fn extend_attrs(
    attrs: &mut Vec<syn::Attribute>,
    derives: &[&syn::Attribute],
    sol_derives: &[&syn::Attribute],
) {
    attrs.reserve(derives.len() + sol_derives.len());
    for attr in derives {
        attrs.push((*attr).clone());
    }
    for attr in sol_derives {
        attrs.push((*attr).clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn id(s: impl AsRef<str>) -> Ident {
        // Ident::new panics on Rust keywords and `r#` prefixes
        syn::parse_str(s.as_ref()).unwrap()
    }

    #[test]
    #[cfg_attr(miri, ignore = "no fs")]
    fn abi() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../json-abi/tests/abi");
        for file in std::fs::read_dir(path).unwrap() {
            let path = file.unwrap().path();
            if path.extension() != Some("json".as_ref()) {
                continue;
            }

            if path.file_name() == Some("LargeFunction.json".as_ref())
                || path.file_name() == Some("SomeLibUser.json".as_ref())
            {
                continue;
            }
            parse_test(&std::fs::read_to_string(&path).unwrap(), path.to_str().unwrap());
        }
    }

    fn parse_test(s: &str, path: &str) {
        let mut abi: JsonAbi = serde_json::from_str(s).unwrap();
        let name = Path::new(path).file_stem().unwrap().to_str().unwrap();

        let name_id = id(name);
        let config = ToSolConfig::new().print_constructors(true).for_sol_macro(true);
        let sol = abi_to_sol(&name_id, &mut abi, config);
        let tokens = match tokens_for_sol(&name_id, &sol) {
            Ok(tokens) => tokens,
            Err(e) => {
                let path = write_tmp_sol(name, &sol);
                panic!(
                    "couldn't expand JSON ABI for {name:?}: {e}\n\
                     emitted interface: {}",
                    path.display()
                );
            }
        };

        let _ast = match syn::parse2::<ast::File>(tokens.clone()) {
            Ok(ast) => ast,
            Err(e) => {
                let spath = write_tmp_sol(name, &sol);
                let tpath = write_tmp_sol(&format!("{name}.tokens"), &tokens.to_string());
                panic!(
                    "couldn't parse expanded JSON ABI back to AST for {name:?}: {e}\n\
                     emitted interface: {}\n\
                     emitted tokens:    {}",
                    spath.display(),
                    tpath.display(),
                );
            }
        };
    }

    fn write_tmp_sol(name: &str, contents: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("sol-macro-{name}.sol"));
        std::fs::write(&path, contents).unwrap();
        let _ = std::process::Command::new("forge").arg("fmt").arg(&path).output();
        path
    }
}
