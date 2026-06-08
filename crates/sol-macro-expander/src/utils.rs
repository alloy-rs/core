use ast::Spanned;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use sha3::{Digest, Keccak256};
use std::cell::RefCell;
use std::panic::{self, AssertUnwindSafe};

/// Simple interface to the [`keccak256`] hash function.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
pub(crate) fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    Keccak256::digest(bytes).into()
}

pub(crate) fn selector<T: AsRef<[u8]>>(bytes: T) -> ExprArray<u8> {
    ExprArray::new(keccak256(bytes)[..4].to_vec())
}

pub(crate) fn event_selector<T: AsRef<[u8]>>(bytes: T) -> ExprArray<u8> {
    ExprArray::new(keccak256(bytes).to_vec())
}

pub(crate) fn combine_errors(v: impl IntoIterator<Item = syn::Error>) -> syn::Result<()> {
    match v.into_iter().reduce(|mut a, b| {
        a.combine(b);
        a
    }) {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

thread_local! {
    static DIAGNOSTICS: RefCell<Vec<proc_macro2_diagnostics::Diagnostic>> =
        const { RefCell::new(Vec::new()) };
}

pub(crate) fn emit_diagnostic(diag: proc_macro2_diagnostics::Diagnostic) {
    DIAGNOSTICS.with(|diagnostics| diagnostics.borrow_mut().push(diag));
}

fn take_diagnostics() -> Vec<proc_macro2_diagnostics::Diagnostic> {
    DIAGNOSTICS.with(|diagnostics| diagnostics.take())
}

fn diagnostics_to_error(
    diagnostics: Vec<proc_macro2_diagnostics::Diagnostic>,
) -> Option<syn::Error> {
    let mut diagnostics = diagnostics.into_iter();
    let mut error = syn::Error::from(diagnostics.next()?);
    for diagnostic in diagnostics {
        error.combine(syn::Error::from(diagnostic));
    }
    Some(error)
}

#[derive(Clone, Debug)]
pub(crate) struct ExprArray<T> {
    pub(crate) array: Vec<T>,
    pub(crate) span: Span,
}

impl<T: PartialOrd> PartialOrd for ExprArray<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.array.partial_cmp(&other.array)
    }
}

impl<T: Ord> Ord for ExprArray<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.array.cmp(&other.array)
    }
}

impl<T: PartialEq> PartialEq for ExprArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.array == other.array
    }
}

impl<T: Eq> Eq for ExprArray<T> {}

impl<T> Spanned for ExprArray<T> {
    fn span(&self) -> Span {
        self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl<T> ExprArray<T> {
    fn new(array: Vec<T>) -> Self {
        Self { array, span: Span::call_site() }
    }
}

impl<T: ToTokens> ToTokens for ExprArray<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        syn::token::Bracket(self.span).surround(tokens, |tokens| {
            for t in &self.array {
                t.to_tokens(tokens);
                syn::token::Comma(self.span).to_tokens(tokens);
            }
        });
    }
}

/// Applies macro expansion compatibility handling programmatically.
pub(crate) fn pme_compat(f: impl FnOnce() -> TokenStream) -> TokenStream {
    pme_compat_result(|| Ok(f())).unwrap()
}

/// Applies macro expansion compatibility handling programmatically.
pub(crate) fn pme_compat_result(
    f: impl FnOnce() -> syn::Result<TokenStream>,
) -> syn::Result<TokenStream> {
    take_diagnostics();
    let mut result = match panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(payload) => {
            let message = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("procedural macro expansion aborted");
            Err(syn::Error::new(Span::call_site(), message))
        }
    };
    if let Some(diagnostics) = diagnostics_to_error(take_diagnostics()) {
        match &mut result {
            Ok(_) => result = Err(diagnostics),
            Err(error) => error.combine(diagnostics),
        }
    }
    result
}
