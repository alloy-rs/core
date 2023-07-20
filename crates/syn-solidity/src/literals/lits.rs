use syn::{LitStr, LitInt, LitBool};

#[derive(Debug, Clone)]
pub enum Literals {
    String(LitStr),
    Number(LitInt),
    Bool(LitBool),
    HexString(),
    Unicode(),
}
