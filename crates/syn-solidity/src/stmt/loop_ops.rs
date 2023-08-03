use syn::{parse::Parse, Error, Token};

#[derive(Debug, Clone, Copy)]
pub enum LoopOps {
    Continue {
        kw: Token!(continue),
        semi: Token!(;),
    },
    Break {
        kw: Token!(break),
        semi: Token!(;),
    },
}

impl Parse for LoopOps {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![continue]) {
            Ok(Self::Continue {
                kw: input.parse()?,
                semi: input.parse()?,
            })
        } else if input.peek(Token![break]) {
            Ok(Self::Break {
                kw: input.parse()?,
                semi: input.parse()?,
            })
        } else {
            Err(Error::new(input.span(), "failed to match on loop op"))
        }
    }
}
