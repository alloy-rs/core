use syn_solidity::{sol_path, SolIdent, SolPath};

#[test]
fn ident() {
    let id: SolIdent = syn::parse_str("a").unwrap();
    assert_eq!(id, SolIdent::new("a"));
}

#[test]
fn keywords() {
    // keywords in Rust, but not Solidity; we try to make them "raw", although some, like `crate`
    // can never be made identifiers. See `src/ident/kw.c`.
    let difference: &[&str] = &include!("../src/ident/difference.expr");
    for &s in difference {
        let id: SolIdent = syn::parse_str(s).unwrap();
        assert_eq!(id, SolIdent::new(s));
        assert_eq!(id.to_string(), format!("r#{s}"));
        assert_eq!(id.as_string(), s);
    }

    // keywords in both languages; we don't make them "raw" because they are always invalid.
    let intersection: &[&str] = &include!("../src/ident/intersection.expr");
    for &s in intersection {
        let id: SolIdent = syn::parse_str(s).unwrap();
        assert_eq!(id, SolIdent::new(s));
        assert_eq!(id.to_string(), s);
        assert_eq!(id.as_string(), s);
    }
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
    assert!(syn::parse_str::<SolIdent>("$hello")
        .unwrap_err()
        .to_string()
        .contains("Solidity identifiers starting with `$` are unsupported."));
}
