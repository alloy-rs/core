use proc_macro2::{TokenStream, TokenTree};
use std::fmt;
use syn::{parse::ParseStream, punctuated::Punctuated, Token};

#[repr(transparent)]
pub(crate) struct DebugPunctuated<T, P>(Punctuated<T, P>);

impl<T, P> DebugPunctuated<T, P> {
    #[inline(always)]
    pub(crate) fn new(punctuated: &Punctuated<T, P>) -> &Self {
        unsafe { &*(punctuated as *const Punctuated<T, P> as *const Self) }
    }
}

impl<T: fmt::Debug, P> fmt::Debug for DebugPunctuated<T, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

pub(crate) fn tts_until_semi(input: ParseStream<'_>) -> TokenStream {
    let mut tts = TokenStream::new();
    while !input.is_empty() && !input.peek(Token![;]) {
        let tt = input.parse::<TokenTree>().unwrap();
        tts.extend(std::iter::once(tt));
    }
    tts
}
