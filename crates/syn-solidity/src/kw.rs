//! Solidity keywords.

macro_rules! custom_keywords {
    ($($name:ident),+ $(,)?) => {$(
        syn::custom_keyword!($name);

        impl $crate::Spanned for $name {
            #[inline]
            fn span(&self) -> ::proc_macro2::Span {
                self.span
            }

            #[inline]
            fn set_span(&mut self, span: ::proc_macro2::Span) {
                self.span = span;
            }
        }
    )+};
}

#[rustfmt::skip]
custom_keywords!(
    // Storage
    memory,
    storage,
    calldata,

    // Visibility
    external,
    public,
    internal,
    private,

    // Mutability
    pure,
    view,
    constant,
    payable,
    immutable,

    // Contract
    contract,
    interface,
    library,

    // Error
    error,
    panic,

    // Event
    event,
    indexed,
    anonymous,

    // Function
    constructor,
    function,
    fallback,
    receive,
    modifier,
    returns,

    // Types
    tuple,
    mapping,

    // Import directives
    import,
    from,

    // Pragma directives
    pragma,
    solidity,
    abicoder,
    experimental,

    // Using directives
    using,
    global,

    // Literals
    unicode,
    hex,

    // Sub-denominations
    wei,
    gwei,
    ether,
    seconds,
    minutes,
    hours,
    days,
    weeks,
    years,

    // Other
    assembly,
    catch,
    delete,
    emit,
    is,
    new,
    revert,
    unchecked,

    // EVM opcodes
    stop,
    add,
    sub,
    mul,
    div,
    sdiv,
    r#mod,
    smod,
    exp,
    not,
    lt,
    gt,
    slt,
    sgt,
    eq,
    iszero,
    and,
    or,
    xor,
    byte,
    shl,
    shr,
    sar,
    addmod,
    mulmod,
    signextend,
    keccak256,
    pop,
    mload,
    mstore,
    mstore8,
    sload,
    sstore,
    msize,
    gas,
    address,
    balance,
    selfbalance,
    caller,
    callvalue,
    calldataload,
    calldatasize,
    extcodesize,
    extcodecopy,
    returndatasize,
    returndatacopy,
    extcodehash,
    create,
    create2,
    call,
    callcode,
    delegatecall,
    staticcall,
    r#return,
    //revert,
    selfdestruct,
    invalid,
    log0,
    log1,
    log2,
    log3,
    log4,
    chainid,
    origin,
    gasprice,
    blockhash,
    coinbase,
    timestamp,
    number,
    difficulty,
    prevrandao,
    gaslimit,
    basefee,
);
