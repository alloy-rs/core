use proc_macro2::{Literal, TokenStream};
use quote::quote;
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

pub fn selector<T: AsRef<[u8]>>(bytes: T) -> TokenStream {
    let hash = keccak256(bytes);
    let selector: [u8; 4] = hash[..4].try_into().unwrap();
    _selector(selector)
}

pub fn event_selector<T: AsRef<[u8]>>(bytes: T) -> TokenStream {
    _selector(keccak256(bytes))
}

fn _selector<const N: usize>(bytes: [u8; N]) -> TokenStream {
    let bytes = bytes.into_iter().map(Literal::u8_unsuffixed);
    quote!([#(#bytes),*])
}

pub fn combine_errors(v: Vec<syn::Error>) -> Option<syn::Error> {
    v.into_iter().reduce(|mut a, b| {
        a.combine(b);
        a
    })
}
