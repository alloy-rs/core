#![allow(unused_macros, unused_macro_rules)]

macro_rules! path {
    ($($e:ident),* $(,)?) => {{
        let mut path = syn_solidity::SolPath::new();
        $(path.push(syn_solidity::SolIdent::new(stringify!($e)));)+
        path
    }}
}
