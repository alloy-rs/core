use crate::Block;

pub enum Expr {
    Loop(),
    If(),
    ForLoop(),
    Cast(),
    Assign(),
    Block(Block),
    Index(),
}
