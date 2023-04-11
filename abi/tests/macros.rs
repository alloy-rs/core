use ethers_abi_enc::{define_udt, domain};

#[allow(clippy::missing_const_for_fn)]
fn ret_ok<T>(_: T) -> ethers_abi_enc::AbiResult<()> {
    Ok(())
}

define_udt!(
    /// My Sol UDT
    MyUdt,
    underlying: ethers_abi_enc::sol_type::Bool,
    type_check: ret_ok,
);

define_udt!(
    /// Some Bytes
    #[derive(Hash)]
    AStruct,
);

#[test]
fn expand_and_use_macros() {
    dbg!(domain! {
        name: "Hello World",
    });

    let a = AStruct::default();
    dbg!(a);
}
