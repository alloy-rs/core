use syn_solidity::{sol_path, SolIdent, SolPath};

#[test]
fn ident() {
    let id: SolIdent = syn::parse_str("a").unwrap();
    assert_eq!(id, SolIdent::new("a"));
}

#[test]
fn ident_path() {
    let path: SolPath = syn::parse_str("a.b.c").unwrap();
    assert_eq!(path, sol_path!["a", "b", "c"]);
}

#[test]
fn ident_path_trailing() {
    let _e = syn::parse_str::<SolPath>("a.b.").unwrap_err();
}

#[test]
fn ident_dollar() {
    let id: Result<SolIdent, _> = syn::parse_str("$hello");
    assert!(id.is_err());
    assert!(id
        .unwrap_err()
        .to_string()
        .contains("Solidity identifiers starting with `$` are unsupported."));
}
