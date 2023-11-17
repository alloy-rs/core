use alloy_sol_types::sol;

macro_rules! make {
    ($($kw:tt)*) => {$(
        sol! {
            struct $kw {
                uint $kw;
            }

            function $kw(uint $kw);
        }
    )*};
}

// Allowed
make! {
    const
    extern
    fn
    impl
    loop
    mod
    move
    mut
    pub
    ref
    trait
    unsafe
    use
    where
    async
    await
    dyn
    become
    box
    priv
    unsized
    yield
}

// Not allowed, but should be (panics on instantiation)
make! {
    crate
    self
    Self
    super
}

// Not allowed
make! {
    as
    break
    continue
    else
    enum
    false
    for
    if
    in
    let
    match
    return
    static
    struct
    true
    type
    while
    abstract
    do
    final
    macro
    override
    typeof
    virtual
    try
}

fn main() {}
