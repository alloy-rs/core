use proc_macro2::Span;
use syn::parse_quote;
use syn_solidity::{FunctionKind, ItemFunction};

#[test]
fn modifiers() {
    let none: ItemFunction = parse_quote! {
        modifier noParens {
            _;
        }
    };
    let some: ItemFunction = parse_quote! {
        modifier withParens() {
            _;
        }
    };
    assert_eq!(none.kind, FunctionKind::new_modifier(Span::call_site()));
    assert_eq!(none.kind, some.kind);
    assert_eq!(none.paren_token, None);
    assert_eq!(some.paren_token, Some(Default::default()));
}
