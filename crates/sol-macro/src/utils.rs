use proc_macro2::TokenStream;
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
    quote!([#(#selector),*])
}
