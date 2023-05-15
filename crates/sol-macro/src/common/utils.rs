use super::Parameters;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use tiny_keccak::{Hasher, Keccak};

pub fn from_into_tuples<P>(name: &Ident, fields: &Parameters<P>) -> TokenStream {
    let names = fields.names();
    let names2 = names.clone();
    let idxs = (0..fields.len()).map(syn::Index::from);

    let tys = fields.types();
    let tys2 = tys.clone();

    quote! {
        type UnderlyingSolTuple = (#(#tys,)*);
        type UnderlyingRustTuple = (#(<#tys2 as ::ethers_sol_types::SolType>::RustType,)*);

        #[automatically_derived]
        impl From<#name> for UnderlyingRustTuple {
            fn from(value: #name) -> Self {
                (#(value.#names,)*)
            }
        }

        #[automatically_derived]
        impl From<UnderlyingRustTuple> for #name {
            fn from(tuple: UnderlyingRustTuple) -> Self {
                #name {
                    #(#names2: tuple.#idxs),*
                }
            }
        }
    }
}

pub fn keccak256(bytes: impl AsRef<[u8]>) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes.as_ref());
    hasher.finalize(&mut output);
    output
}
