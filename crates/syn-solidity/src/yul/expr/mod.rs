use std::fmt;

use syn::parse::{Parse, ParseStream, Result};

use crate::Spanned;

mod path;
use path::YulPath;

#[derive(Clone)]
pub enum YulExpr {
    Path(YulPath),
}

impl Parse for YulExpr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self::Path(YulPath {}))
    }
}

impl Spanned for YulExpr {
    fn span(&self) -> proc_macro2::Span {
        todo!()
    }

    fn set_span(&mut self, span: proc_macro2::Span) {
        todo!()
    }
}

impl fmt::Debug for YulExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YulExpr").finish()
    }
}
