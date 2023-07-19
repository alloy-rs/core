use crate::{kw, Block, Parameters, VariableDeclaration};
use syn::{parse::Parse, token::Paren, Token};

// annoying since there is 4 differnt types of error catching methods
#[derive(Debug, Clone)]
pub struct TryCatch {
    pub r#try: Token![try],
    pub block: Block,
    pub catch: Catch,
}

pub struct Catch {
    pub error: Option<kw::error>,
    pub panic: Option<kw::panic>,
    // this is tech o
    pub args: Option<Parameters<Token![,]>>,
    pub block: Block,
}

impl Parse for Catch {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let (error, panic) = if input.peek(kw::error) {
            (Some(kw::error(input.span())), None)
        } else if input.peek(kw::panic) {
            (None, Some(kw::panic(input.span())))
        } else {
            (None, None)
        };

        let args = if error.is_none() && panic.is_none() {
            if input.peek(Paren) {
                // we have args
                let f = input.fork();
                let mut args = Parameters::new();
                while let Ok(arg) = input.parse::<VariableDeclaration>() {
                    args.push(arg);
                }
                Some(args)
            } else {
                // they raw doggin
                None
            }
        } else {
            let mut args = Parameters::new();
            while let Ok(arg) = input.parse::<VariableDeclaration>() {
                args.push(arg);
            }
            Some(args)
        };

        Ok(Self {
            error,
            panic,
            args,
            block: input.parse()?,
        })
    }
}

impl Parse for TryCatch {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let a_try = input.parse()?;
        let block = input.parse()?;
        let catch = input.parse()?;

        Ok(Self {
            r#try: a_try,
            block,
            catch,
        })
    }
}
