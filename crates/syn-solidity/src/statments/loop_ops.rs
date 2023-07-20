use syn::Token;

#[derive(Debug, Clone)]
pub enum LoopOps {
    Continue(Token!(continue)),
    Break(Token!(break)),
}
