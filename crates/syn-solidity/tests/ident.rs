use syn_solidity::{SolIdent, SolPath};

#[macro_use]
mod macros;

#[test]
fn ident() {
    let id: SolIdent = syn::parse_str("a").unwrap();
    assert_eq!(id, SolIdent::new("a"));
}

#[test]
fn ident_path() {
    let path: SolPath = syn::parse_str("a.b.c").unwrap();
    assert_eq!(path, path![a, b, c]);
}

#[test]
fn ident_path_trailing() {
    let _e = syn::parse_str::<SolPath>("a.b.").unwrap_err();
}
