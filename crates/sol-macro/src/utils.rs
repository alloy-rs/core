use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use tiny_keccak::{Hasher, Keccak};

/// Simple interface to the [`keccak256`] hash function.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
pub fn keccak256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}

pub fn selector<T: AsRef<[u8]>>(bytes: T) -> ExprArray<u8, 4> {
    ExprArray::new(keccak256(bytes)[..4].try_into().unwrap())
}

pub fn event_selector<T: AsRef<[u8]>>(bytes: T) -> ExprArray<u8, 32> {
    ExprArray::new(keccak256(bytes))
}

pub fn combine_errors(v: Vec<syn::Error>) -> Option<syn::Error> {
    v.into_iter().reduce(|mut a, b| {
        a.combine(b);
        a
    })
}

pub struct ExprArray<T, const N: usize> {
    pub array: [T; N],
    pub span: Span,
}

impl<T, const N: usize> ExprArray<T, N> {
    fn new(array: [T; N]) -> Self {
        Self {
            array,
            span: Span::call_site(),
        }
    }
}

impl<T: ToTokens, const N: usize> ToTokens for ExprArray<T, N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        syn::token::Bracket(self.span).surround(tokens, |tokens| {
            for t in &self.array {
                t.to_tokens(tokens);
                syn::token::Comma(self.span).to_tokens(tokens);
            }
        });
    }
}
