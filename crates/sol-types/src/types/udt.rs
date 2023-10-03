// TODO: Move this to the `sol` proc macro

/// Define a Solidity user-defined value type.
///
/// Generates a struct of the form `$name($underlying)`.
#[macro_export]
macro_rules! define_udt {
    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
        type_check: $path:path,
    ) => {
        $(#[$outer])*
        /// This struct is a Solidity user-defined value type. It wraps
        /// an underlying type.
        #[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $name (
            <$underlying as $crate::SolType>::RustType,
        );

        impl $crate::Encodable<$name> for <$underlying as $crate::SolType>::RustType {
            #[inline]
            fn to_tokens(&self) -> <$underlying as $crate::SolType>::TokenType<'_> {
                $crate::Encodable::<$underlying>::to_tokens(self)
            }
        }

        impl $name {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!($name);

            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: <$underlying as $crate::SolType>::RustType) -> Self {
                Self(value)
            }

            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> <$underlying as $crate::SolType>::RustType {
                self.0
            }

            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> $crate::private::Vec<u8> {
                <Self as $crate::SolType>::abi_encode(&self.0)
            }

            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> $crate::private::Vec<u8> {
                <Self as $crate::SolType>::abi_encode_packed(&self.0)
            }
        }

        impl $crate::SolType for $name {
            type RustType = <$underlying as $crate::SolType>::RustType;
            type TokenType<'a> = <$underlying as $crate::SolType>::TokenType<'a>;

            const DYNAMIC: bool = false;

            #[inline]
            fn sol_type_name() -> $crate::private::Cow<'static, str> {
                Self::NAME.into()
            }

            #[inline]
            fn valid_token(token: &Self::TokenType<'_>) -> bool {
                Self::type_check(token).is_ok()
            }

            #[inline]
            fn type_check(token: &Self::TokenType<'_>) -> $crate::Result<()> {
                <$underlying as $crate::SolType>::type_check(token)?;
                $path(token)
            }

            #[inline]
            fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
                <$underlying as $crate::SolType>::detokenize(token)
            }

            #[inline]
            fn eip712_data_word(rust: &Self::RustType) -> $crate::Word {
                <Self as $crate::SolType>::tokenize(rust).0
            }

            #[inline]
            fn abi_encode_packed_to(rust: &Self::RustType, out: &mut $crate::private::Vec<u8>) {
                <$underlying as $crate::SolType>::abi_encode_packed_to(rust, out)
            }
        }

        impl $crate::EventTopic for $name {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <$underlying as $crate::EventTopic>::topic_preimage_length(rust)
            }

            #[inline]
            fn encode_topic_preimage(rust: &Self::RustType, out: &mut $crate::private::Vec<u8>) {
                <$underlying as $crate::EventTopic>::encode_topic_preimage(rust, out)
            }

            #[inline]
            fn encode_topic(rust: &Self::RustType) -> $crate::abi::token::WordToken {
                <$underlying as $crate::EventTopic>::encode_topic(rust)
            }
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
    ) => {
        $crate::define_udt!(
            $(#[$outer])*
            $name,
            underlying: $underlying,
            type_check: $crate::private::just_ok,
        );
    };

    (
        $(#[$outer:meta])*
        $name:ident,
        type_check: $type_check:path,
    ) => {
        $crate::define_udt!(
            $(#[$outer])*
            $name,
            underlying: $crate::sol_data::FixedBytes<32>,
            type_check: $type_check,
        );
    };
    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
    ) => {
        $crate::define_udt!(
            $(#[$outer])*
            $name,
            underlying: $underlying,
            type_check: $crate::just_ok,
        );
    };
    (
        $(#[$outer:meta])*
        $name:ident,
    ) => {
        $crate::define_udt!(
            $(#[$outer])*
            $name,
            type_check: $crate::private::just_ok,
        );
    };

    (
        $(#[$outer:meta])*
        $name:ident
    )  => {
        $crate::define_udt!(
            $(#[$outer])*
            name: $name,
        );
    };
}
