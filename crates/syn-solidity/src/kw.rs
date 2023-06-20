//! Solidity keywords.

pub use syn::token::{Abstract, Override, Virtual};

macro_rules! custom_keywords {
    ($($name:ident),+ $(,)?) => {
        $(syn::custom_keyword!($name);)+
    };
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

    // Other
    is,
    unicode,
);
