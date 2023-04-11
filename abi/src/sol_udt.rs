/// Define a solidity user-defined value type
///
/// Generates a struct of the form `$name { value: B256 }`
///
#[macro_export]
macro_rules! define_udt {
    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
        type_check: $path:path,
    ) => {
        $(#[$outer])*
        #[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $name {
            value: ::ethers_primitives::B256,
            _pd: $crate::no_std_prelude::PhantomData<$underlying>
        }

        impl $name {
            /// The type name
            pub const NAME: &'static str = stringify!($name);
        }

        impl $crate::SolType for $name {
            type RustType = <$underlying as $crate::SolType>::RustType;
            type TokenType = <$underlying as $crate::SolType>::TokenType;

            fn sol_type_name() -> $crate::no_std_prelude::String {
                $crate::no_std_prelude::ToOwned::to_owned(Self::NAME)
            }

            fn is_dynamic() -> bool {
                false
            }

            fn is_user_defined() -> bool {
                true
            }

            fn type_check(token: &Self::TokenType) -> $crate::AbiResult<()> {
                $path(token)
            }

            fn tokenize<B>(rust: B) -> Self::TokenType
            where
                B: $crate::no_std_prelude::Borrow<Self::RustType>
            {
                <$underlying as $crate::SolType>::tokenize(rust)
            }

            fn detokenize(token: Self::TokenType) -> $crate::AbiResult<Self::RustType> {
                <$underlying as $crate::SolType>::detokenize(token)
            }

            fn encode_packed_to<B>(target: &mut $crate::no_std_prelude::Vec<u8>, rust: B)
            where
                B: $crate::no_std_prelude::Borrow<Self::RustType>
            {
                <$underlying as $crate::SolType>::encode_packed_to(target, rust)
            }
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
    ) => {
        define_udt!(
            $(#[$outer])*
            $name,
            underlying: $underlying,
            type_check: Ok(()),
        );
    };

    (
        $(#[$outer:meta])*
        $name:ident,
        type_check: $type_check:path,
    ) => {
        define_udt!(
            $(#[$outer])*
            $name,
            underlying: $crate::sol_type::FixedBytes<32>,
            type_check: $type_check,
        );
    };

    (
        $(#[$outer:meta])*
        $name:ident,
    ) => {
        define_udt!(
            $(#[$outer])*
            $name,
            type_check: $crate::just_ok,
        );
    };

    (
        $(#[$outer:meta])*
        $name:ident
    )  => {
        define_udt!(
            $(#[$outer])*
            name: $name,
        );
    };
}

#[cfg(test)]
#[allow(unreachable_pub)]
mod test {}
