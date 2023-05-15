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
            pub fn encode_single(&self) -> $crate::no_std_prelude::Vec<u8> {
                <Self as $crate::SolType>::encode_single(self.0)
            }

            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn encode_packed(&self) -> $crate::no_std_prelude::Vec<u8> {
                <Self as $crate::SolType>::encode_packed(self.0)
            }
        }

        impl $crate::SolType for $name {
            type RustType = <$underlying as $crate::SolType>::RustType;
            type TokenType = <$underlying as $crate::SolType>::TokenType;

            #[inline]
            fn sol_type_name() -> $crate::no_std_prelude::Cow<'static, str> {
                Self::NAME.into()
            }

            #[inline]
            fn is_dynamic() -> bool {
                false
            }

            #[inline]
            fn type_check(token: &Self::TokenType) -> $crate::Result<()> {
                <$underlying as $crate::SolType>::type_check(token)?;
                $path(token)
            }

            #[inline]
            fn tokenize<B>(rust: B) -> Self::TokenType
            where
                B: $crate::no_std_prelude::Borrow<Self::RustType>
            {
                <$underlying as $crate::SolType>::tokenize(rust)
            }

            #[inline]
            fn detokenize(token: Self::TokenType) -> $crate::Result<Self::RustType> {
                <$underlying as $crate::SolType>::detokenize(token)
            }

            #[inline]
            fn eip712_data_word<B>(rust: B) -> $crate::Word
            where
                B: $crate::no_std_prelude::Borrow<Self::RustType>
            {
                <Self as $crate::SolType>::tokenize(rust).inner()
            }

            #[inline]
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
            type_check: $crate::just_ok,
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
