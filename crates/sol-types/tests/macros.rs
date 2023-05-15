use ethers_sol_types::{define_udt, domain};

#[allow(clippy::missing_const_for_fn)]
fn ret_ok<T>(_: T) -> ethers_sol_types::Result<()> {
    Ok(())
}

define_udt!(
    /// My Sol UDT.
    MyUdt,
    underlying: ethers_sol_types::sol_data::Bool,
    type_check: ret_ok,
);

define_udt!(
    /// Some Bytes.
    #[derive(Hash)]
    AStruct,
);

#[test]
fn expand_and_use_macros() {
    let domain = domain! {
        name: "Hello World",
    };
    assert_eq!(domain.name.as_deref(), Some("Hello World"));

    let a = AStruct::default();
    assert_eq!(a.0, [0u8; 32]);
}
