use crate::{
    common::{from_into_tuples, kw, FunctionAttributes, Parameters, SolIdent},
    r#type::{SolTuple, Type},
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::fmt;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Attribute, Error, Result, Token,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Returns {
    returns_token: kw::returns,
    paren_token: Paren,
    pub returns: Parameters<Token![,]>,
}

impl fmt::Debug for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Returns").field(&self.returns).finish()
    }
}

impl fmt::Display for Returns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("returns (")?;
        for (i, r) in self.returns.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            write!(f, "{r}")?;
        }
        f.write_str(")")
    }
}

impl Parse for Returns {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let this = Self {
            returns_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            returns: content.parse()?,
        };
        if this.returns.is_empty() {
            Err(Error::new(
                this.paren_token.span.join(),
                "expected at least one return type",
            ))
        } else {
            Ok(this)
        }
    }
}

impl Returns {
    #[allow(dead_code)]
    pub fn span(&self) -> Span {
        let span = self.returns_token.span;
        span.join(self.paren_token.span.join()).unwrap_or(span)
    }
}

pub struct Function {
    _function_token: kw::function,
    /// The original name of the function, before any overload renaming.
    pub original_name: SolIdent,
    /// The name of the function, after any overload renaming.
    pub name: SolIdent,
    _paren_token: Paren,
    pub arguments: Parameters<Token![,]>,
    pub attributes: FunctionAttributes,
    pub returns: Option<Returns>,
    _semi_token: Token![;],
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("name", &self.name)
            .field("arguments", &self.arguments)
            .field("attributes", &self.attributes)
            .field("returns", &self.returns)
            .finish()
    }
}

impl Parse for Function {
    fn parse(input: ParseStream) -> Result<Self> {
        fn parse_check_brace<T: Parse>(input: ParseStream) -> Result<T> {
            if input.peek(Brace) {
                Err(input.error("functions cannot have an implementation"))
            } else {
                input.parse()
            }
        }
        let name: SolIdent;
        let content;
        Ok(Self {
            _function_token: input.parse()?,
            original_name: {
                name = input.parse()?;
                name.clone()
            },
            name,
            _paren_token: parenthesized!(content in input),
            arguments: content.parse()?,
            attributes: parse_check_brace(input)?,
            returns: if input.peek(kw::returns) {
                Some(input.parse()?)
            } else {
                None
            },
            _semi_token: parse_check_brace(input)?,
        })
    }
}

impl Function {
    fn expand(
        &self,
        call_name: &Ident,
        params: &Parameters<Token![,]>,
        attrs: &[Attribute],
    ) -> TokenStream {
        params.assert_resolved();

        let fields = params.iter();

        let (signature, selector) = params.sig_and_sel(self.original_name.as_string());

        let size = params.data_size(None);

        let converts = from_into_tuples(call_name, params);

        quote! {
            #(#attrs)*
            #[allow(non_camel_case_types, non_snake_case)]
            #[derive(Clone)]
            pub struct #call_name {
                #(pub #fields,)*
            }

            #[allow(non_camel_case_types, non_snake_case, clippy::style)]
            const _: () = {
                #converts

                #[automatically_derived]
                impl ::ethers_sol_types::SolCall for #call_name {
                    type Tuple = UnderlyingSolTuple;
                    type Token = <UnderlyingSolTuple as ::ethers_sol_types::SolType>::TokenType;

                    const SIGNATURE: &'static str = #signature;
                    const SELECTOR: [u8; 4] = [#(#selector),*];

                    fn to_rust(&self) -> <Self::Tuple as ::ethers_sol_types::SolType>::RustType {
                        self.clone().into()
                    }

                    fn from_rust(tuple: <Self::Tuple as ::ethers_sol_types::SolType>::RustType) -> Self {
                        tuple.into()
                    }

                    fn data_size(&self) -> usize {
                        #size
                    }
                }
            };
        }
    }

    pub fn to_tokens(&self, tokens: &mut TokenStream, attrs: &[Attribute]) {
        let call = self.expand(&self.call_name(), &self.arguments, attrs);
        tokens.extend(call);
        if let Some(ret) = &self.returns {
            let ret = self.expand(&self.return_name(), &ret.returns, attrs);
            tokens.extend(ret);
        }
    }

    pub fn call_name(&self) -> Ident {
        format_ident!("{}Call", self.name.0.unraw())
    }

    pub fn return_name(&self) -> Ident {
        format_ident!("{}Return", self.name.0.unraw())
    }

    #[allow(dead_code)]
    pub fn call_type(&self) -> Type {
        let mut args = self
            .arguments
            .iter()
            .map(|arg| arg.ty.clone())
            .collect::<SolTuple>();
        // ensure trailing comma for single item tuple
        if !args.types.trailing_punct() && args.types.len() == 1 {
            args.types.push_punct(Default::default())
        }
        Type::Tuple(args)
    }
}
