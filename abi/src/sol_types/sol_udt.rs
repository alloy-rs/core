// TODO: Move this to sol-type-parser proc-macro.

/// Define a solidity user-defined value type.
///
/// Generates a struct of the form `$name { value: B256 }`
#[macro_export]
macro_rules! define_udt {
    (
        $(#[$outer:meta])*
        $name:ident,
        underlying: $underlying:ty,
        type_check: $path:path,
    ) => {
        $(#[$outer])*
        ///
        /// This struct is a Solidity user-defined value type. It wraps the
        #[doc = ::core::concat!("`", stringify!($underlying), "` underlying Solidity type.")]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        #[allow(non_snake_case)]
        pub struct $name (
            <$underlying as $crate::SolType>::RustType,
        );

        #[allow(non_snake_case)]
        const _: () = {
            impl $name {
                /// The solidity type name
                pub const NAME: &'static str = stringify!($name);

                /// Convert from the underlying value type
                pub const fn from(value: <$underlying as $crate::SolType>::RustType) -> Self {
                    Self(value)
                }

                /// Return the underlying value
                pub const fn into(self) -> <$underlying as $crate::SolType>::RustType {
                    self.0
                }

                /// Return the single encoding of this value, delegating to the
                /// underlying type
                pub fn encode_single(&self) -> $crate::no_std_prelude::Vec<u8> {
                    <Self as $crate::SolType>::encode_single(self.0)
                }

                /// Return the packed encoding of this value, delegating to the
                /// underlying type
                pub fn encode_packed(&self) -> $crate::no_std_prelude::Vec<u8> {
                    <Self as $crate::SolType>::encode_packed(self.0)
                }
            }

            impl $crate::SolType for $name {
                type RustType = <$underlying as $crate::SolType>::RustType;
                type TokenType = <$underlying as $crate::SolType>::TokenType;

                fn sol_type_name() -> $crate::no_std_prelude::Cow<'static, str> {
                    Self::NAME.into()
                }

                fn is_dynamic() -> bool {
                    false
                }

                fn is_user_defined() -> bool {
                    true
                }

                fn type_check(token: &Self::TokenType) -> $crate::AbiResult<()> {
                    <$underlying as $crate::SolType>::type_check(token)?;
                    $path(token)
                }

                fn eip712_data_word<B>(rust: B) -> $crate::Word
                where
                    B: $crate::no_std_prelude::Borrow<Self::RustType>
                {
                    Self::tokenize(rust).inner()
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
            underlying: $crate::sol_type::FixedBytes<32>,
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

#[cfg(test)]
#[allow(unreachable_pub)]
mod test {}
