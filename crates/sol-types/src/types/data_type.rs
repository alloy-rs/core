//! Solidity Primitives. These are the types that are built into Solidity.

#![allow(missing_copy_implementations, missing_debug_implementations)]

use crate::{
    no_std_prelude::{String as RustString, *},
    token::*,
    util, Result, SolType, Word,
};
use alloc::borrow::Cow;
use alloy_primitives::{keccak256, Address as RustAddress, I256, U256};
use core::{fmt::*, hash::Hash, marker::PhantomData, ops::*};

/// Address - `address`
pub struct Address;

impl SolType for Address {
    type RustType = RustAddress;
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "address".into()
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        RustAddress::new(token.0[12..].try_into().unwrap())
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        WordToken::from(*rust.borrow())
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if util::check_zeroes(&token.0[..12]) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).0
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        out.extend_from_slice(rust.borrow().as_ref());
    }
}

/// Bytes - `bytes`
pub struct Bytes;

impl SolType for Bytes {
    type RustType = Vec<u8>;
    type TokenType = PackedSeqToken;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bytes".into()
    }

    #[inline]
    fn encoded_size<B: Borrow<Self::RustType>>(_data: B) -> usize {
        32 + util::padded_len(_data.borrow())
    }

    #[inline]
    fn type_check(_token: &Self::TokenType) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.into_vec()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        rust.borrow().to_owned().into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        out.extend_from_slice(rust.borrow());
    }
}

/// Int - `intX`
pub struct Int<const BITS: usize>;

impl<const BITS: usize> SolType for Int<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Int;
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::INT_NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if BITS == 256 {
            return Ok(())
        }

        let sign_extension = (token.0[IntBitCount::<BITS>::MSB] & 0x80 == 0x80) as u8;

        // check that all upper bytes are an extension of the sign bit
        if token.0[..IntBitCount::<BITS>::MSB]
            .iter()
            .all(|byte| *byte == sign_extension)
        {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_int(token)
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        IntBitCount::<BITS>::tokenize_int(*rust.borrow())
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).0
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_int(*rust.borrow(), out)
    }
}

/// Uint - `uintX`
pub struct Uint<const BITS: usize>;

impl<const BITS: usize> SolType for Uint<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Uint;
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::UINT_NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        let sli = &token.0[..<IntBitCount<BITS> as SupportedInt>::MSB];
        if util::check_zeroes(sli) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_uint(token)
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        IntBitCount::<BITS>::tokenize_uint(*rust.borrow())
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).0
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_uint(*rust.borrow(), out)
    }
}

/// Bool - `bool`
pub struct Bool;

impl SolType for Bool {
    type RustType = bool;
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if util::check_bool(token.0) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.0 != Word::ZERO
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        Word::with_last_byte(*rust.borrow() as u8).into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).0
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        out.push(*rust.borrow() as u8);
    }
}

/// Array - `T[]`
pub struct Array<T: SolType>(PhantomData<T>);

impl<T> SolType for Array<T>
where
    T: SolType,
{
    type RustType = Vec<T::RustType>;
    type TokenType = DynSeqToken<T::TokenType>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[]", T::sol_type_name()).into()
    }

    #[inline]
    fn encoded_size<B: Borrow<Self::RustType>>(rust: B) -> usize {
        let data = rust.borrow();
        32 + data.iter().map(T::encoded_size).sum::<usize>()
            + (T::DYNAMIC as usize * 32 * data.len())
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        token.0.iter().try_for_each(T::type_check)
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.0.into_iter().map(T::detokenize).collect()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        let v = rust.borrow().iter().map(T::tokenize).collect();
        DynSeqToken(v)
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        let mut encoded = Vec::new();
        for item in rust.borrow() {
            encoded.extend_from_slice(T::eip712_data_word(item).as_slice());
        }
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        for item in rust.borrow() {
            T::encode_packed_to(item, out);
        }
    }
}

/// String - `string`
pub struct String;

impl SolType for String {
    type RustType = RustString;
    type TokenType = PackedSeqToken;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "string".into()
    }

    #[inline]
    fn encoded_size<B: Borrow<Self::RustType>>(_data: B) -> usize {
        32 + util::padded_len(_data.borrow().as_bytes())
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if core::str::from_utf8(token.as_slice()).is_ok() {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        RustString::from_utf8_lossy(&Bytes::detokenize(token)).into_owned()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        rust.borrow().as_bytes().to_vec().into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        keccak256(Self::encode_packed(rust.borrow()))
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        out.extend_from_slice(rust.borrow().as_bytes());
    }
}

/// FixedBytes - `bytesX`
#[derive(Clone, Copy, Debug)]
pub struct FixedBytes<const N: usize>;

impl<const N: usize> SolType for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    type RustType = [u8; N];
    type TokenType = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        <ByteCount<N>>::NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        if util::check_zeroes(&token.0[N..]) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.0[..N].try_into().unwrap()
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        let mut word = Word::ZERO;
        word[..N].copy_from_slice(rust.borrow());
        word.into()
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        Self::tokenize(rust).0
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        // write only the first n bytes
        out.extend_from_slice(rust.borrow());
    }
}

/// FixedArray - `T[M]`
pub struct FixedArray<T, const N: usize>(PhantomData<T>);

impl<T, const N: usize> SolType for FixedArray<T, N>
where
    T: SolType,
{
    type RustType = [T::RustType; N];
    type TokenType = FixedSeqToken<T::TokenType, N>;

    const ENCODED_SIZE: Option<usize> = {
        match T::ENCODED_SIZE {
            Some(size) => Some(size * N),
            None => None,
        }
    };

    /// Calculate the encoded size of the data, counting both head and tail
    /// words. For a single-word type this will always be 32.
    #[inline]
    fn encoded_size<B: Borrow<Self::RustType>>(rust: B) -> usize {
        if let Some(size) = Self::ENCODED_SIZE {
            return size
        }

        rust.borrow().iter().map(T::encoded_size).sum::<usize>() + (T::DYNAMIC as usize * N * 32)
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[{}]", T::sol_type_name(), N).into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType) -> Result<()> {
        for token in token.as_array().iter() {
            T::type_check(token)?;
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType) -> Self::RustType {
        token.0.map(T::detokenize)
    }

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
        let arr: &[T::RustType; N] = rust.borrow();
        FixedSeqToken::<_, N>(core::array::from_fn(|i| T::tokenize(&arr[i])))
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
        let rust = rust.borrow();
        let encoded = rust
            .iter()
            .flat_map(|element| T::eip712_data_word(element).0)
            .collect::<Vec<u8>>();
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
        for item in rust.borrow() {
            T::encode_packed_to(item, out);
        }
    }
}

macro_rules! tuple_impls {
    (@one $ty:ident) => { 1usize };

    // compile time `join(",")` format string
    (@fmt $other:ident) => { ",{}" };
    (@fmt $first:ident, $($other:ident,)*) => {
        concat!(
            "{}",
            $(tuple_impls! { @fmt $other }),*
        )
    };

    ($($ty:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($ty: SolType,)+> SolType for ($($ty,)+) {
            type RustType = ($( $ty::RustType, )+);
            type TokenType = ($( $ty::TokenType, )+);

            const ENCODED_SIZE: Option<usize> = {
                let mut acc = Some(0);
                $(
                    match (acc, <$ty as SolType>::ENCODED_SIZE) {
                        (Some(i), Some(size)) => acc = Some(i + size),
                        (Some(_), None) => acc = None,
                        (None, _) => {}
                    }
                )+
                acc
            };

            fn sol_type_name() -> Cow<'static, str> {
                format!(
                    concat!(
                        "(",
                        tuple_impls! { @fmt $($ty,)+ },
                        ")",
                    ),
                    $(<$ty as SolType>::sol_type_name(),)+
                ).into()
            }

            fn encoded_size<B: Borrow<Self::RustType>>(rust: B) -> usize {
                if let Some(size) = Self::ENCODED_SIZE {
                    return size
                }

                let ($($ty,)+) = rust.borrow();
                0 $(
                    + <$ty as SolType>::encoded_size($ty)
                )+
                $(
                    + (32 * <$ty as SolType>::DYNAMIC as usize)
                )+
            }

            fn type_check(token: &Self::TokenType) -> Result<()> {
                let ($($ty,)+) = token;
                $(
                    <$ty as SolType>::type_check($ty)?;
                )+
                Ok(())
            }

            fn detokenize(token: Self::TokenType) -> Self::RustType {
                let ($($ty,)+) = token;
                ($(
                    <$ty as SolType>::detokenize($ty),
                )+)
            }

            fn tokenize<B: Borrow<Self::RustType>>(rust: B) -> Self::TokenType {
                let ($($ty,)+) = rust.borrow();
                ($(
                    <$ty as SolType>::tokenize($ty),
                )+)
            }

            fn eip712_data_word<B: Borrow<Self::RustType>>(rust: B) -> Word {
                const COUNT: usize = 0usize $(+ tuple_impls!(@one $ty))+;
                let ($($ty,)+) = rust.borrow();
                let encoding: [[u8; 32]; COUNT] = [$(
                    <$ty as SolType>::eip712_data_word($ty).0,
                )+];
                // SAFETY: Flattening [[u8; 32]; COUNT] to [u8; COUNT * 32] is valid
                let ptr = encoding.as_ptr() as *const u8;
                let len = COUNT * 32;
                let encoding: &[u8] = unsafe { core::slice::from_raw_parts(ptr, len) };
                keccak256(encoding).into()
            }

            fn encode_packed_to<B: Borrow<Self::RustType>>(rust: B, out: &mut Vec<u8>) {
                let ($($ty,)+) = rust.borrow();
                // TODO: Reserve
                $(
                    <$ty as SolType>::encode_packed_to($ty, out);
                )+
            }
        }
    };
}

impl SolType for () {
    type RustType = ();
    type TokenType = FixedSeqToken<(), 0>;

    const ENCODED_SIZE: Option<usize> = Some(0);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "()".into()
    }

    #[inline]
    fn type_check(_token: &Self::TokenType) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn detokenize(_token: Self::TokenType) -> Self::RustType {}

    #[inline]
    fn tokenize<B: Borrow<Self::RustType>>(_rust: B) -> Self::TokenType {
        FixedSeqToken([])
    }

    #[inline]
    fn eip712_data_word<B: Borrow<Self::RustType>>(_rust: B) -> Word {
        Word::ZERO
    }

    #[inline]
    fn encode_packed_to<B: Borrow<Self::RustType>>(_rust: B, _out: &mut Vec<u8>) {}
}

all_the_tuples!(tuple_impls);

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

/// Specifies the number of bytes in a [`FixedBytes`] array as a type.
pub struct ByteCount<const N: usize>;

impl<const N: usize> Sealed for ByteCount<N> {}

/// Statically guarantees that a `FixedBytes` byte count is marked as supported.
///
/// This trait is *sealed*: the list of implementors below is total.
///
/// Users do not have the ability to mark additional [`ByteCount<N>`] values as
/// supported. Only `FixedBytes` with supported byte counts are constructable.
pub trait SupportedFixedBytes: Sealed {
    /// The name of the `FixedBytes` type: `bytes<N>`
    const NAME: &'static str;
}

macro_rules! supported_fixed_bytes {
    ($($n:literal),+) => {$(
        impl SupportedFixedBytes for ByteCount<$n> {
            const NAME: &'static str = concat!("bytes", $n);
        }
    )+};
}

supported_fixed_bytes!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

/// Specifies the number of bits in an [`Int`] or [`Uint`] as a type.
pub struct IntBitCount<const N: usize>;

impl<const N: usize> Sealed for IntBitCount<N> {}

// Declares types with the same traits
// TODO: Add more traits
// TODO: Integrate `num_traits` (needs `ruint`)
macro_rules! declare_int_types {
    ($($(#[$attr:meta])* type $name:ident;)*) => {$(
        $(#[$attr])*
        type $name: Sized + Copy + PartialOrd + Ord + Eq + Hash
            + Not + BitAnd + BitOr + BitXor
            + Add + Sub + Mul + Div + Rem
            + AddAssign + SubAssign + MulAssign + DivAssign + RemAssign
            + Debug + Display + LowerHex + UpperHex + Octal + Binary;
    )*};
}

/// Statically guarantees that a [`Int`] or [`Uint`] bit count is marked as
/// supported.
///
/// This trait is *sealed*: the list of implementors below is total.
///
/// Users do not have the ability to mark additional [`IntBitCount<N>`] values
/// as supported. Only `FixedBytes` with supported byte counts are
/// constructable.
pub trait SupportedInt: Sealed {
    declare_int_types! {
        /// A signed integer of at least `N` bits.
        type Int;
        /// An unsigned integer of at least `N` bits.
        type Uint;
    }

    /// The name of the `Int` type: `int<N>`
    const INT_NAME: &'static str;

    /// The name of the `Uint` type: `uint<N>`
    const UINT_NAME: &'static str;

    /// The number of bits in the integer: `BITS`
    ///
    /// Note that this is not equal to `Self::Int::BITS`.
    const BITS: usize;

    /// The number of bytes in the integer: `BITS / 8`
    const BYTES: usize = Self::BITS / 8;

    /// The number of bytes in the integer: `(<$t>::BITS - N) / 8`
    const SKIP_BYTES: usize;

    /// The index of the most significant byte in the Word type.
    const MSB: usize = 32 - Self::BYTES;

    /// Tokenizes a signed integer.
    fn tokenize_int(int: Self::Int) -> WordToken;
    /// Detokenizes a signed integer.
    fn detokenize_int(token: WordToken) -> Self::Int;
    /// ABI-encode a signed integer in packed mode.
    fn encode_packed_to_int(int: Self::Int, out: &mut Vec<u8>);

    /// Tokenizes an unsigned integer.
    fn tokenize_uint(uint: Self::Uint) -> WordToken;
    /// Detokenizes an unsigned integer.
    fn detokenize_uint(token: WordToken) -> Self::Uint;
    /// ABI-encode an unsigned integer in packed mode.
    fn encode_packed_to_uint(uint: Self::Uint, out: &mut Vec<u8>);
}

macro_rules! supported_int {
    ($($n:literal => $i:ident, $u:ident;)+) => {
        $(
            impl SupportedInt for IntBitCount<$n> {
                type Int = $i;
                type Uint = $u;

                const UINT_NAME: &'static str = concat!("uint", $n);
                const INT_NAME: &'static str = concat!("int", $n);

                const BITS: usize = $n;
                const SKIP_BYTES: usize = (<$i>::BITS as usize - <Self as SupportedInt>::BITS) / 8;

                int_impls2!($i);
                int_impls2!($u);
            }
        )+
    };
}

macro_rules! int_impls {
    (@primitive_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            let mut word = [int.is_negative() as u8 * 0xff; 32];
            word[Self::MSB..].copy_from_slice(&int.to_be_bytes());
            WordToken(alloy_primitives::FixedBytes(word))
        }

        #[inline]
        fn detokenize_int(token: WordToken) -> $ity {
            let sli = &token.0[Self::MSB..];
            <$ity>::from_be_bytes(sli.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_int(int: $ity, out: &mut Vec<u8>) {
            out.extend_from_slice(&int.to_be_bytes()[Self::SKIP_BYTES..]);
        }
    };
    (@primitive_uint $uty:ident) => {
        #[inline]
        fn tokenize_uint(uint: $uty) -> WordToken {
            let mut word = Word::ZERO;
            word[Self::MSB..].copy_from_slice(&uint.to_be_bytes());
            WordToken(word)
        }

        #[inline]
        fn detokenize_uint(token: WordToken) -> $uty {
            let sli = &token.0[Self::MSB..];
            <$uty>::from_be_bytes(sli.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_uint(uint: $uty, out: &mut Vec<u8>) {
            out.extend_from_slice(&uint.to_be_bytes()[Self::SKIP_BYTES..]);
        }
    };

    (@big_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            int.into()
        }

        #[inline]
        fn detokenize_int(token: WordToken) -> $ity {
            <$ity>::from_be_bytes::<32>(token.0 .0)
        }

        #[inline]
        fn encode_packed_to_int(int: $ity, out: &mut Vec<u8>) {
            out.extend_from_slice(&int.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
        }
    };
    (@big_uint $uty:ident) => {
        #[inline]
        fn tokenize_uint(uint: $uty) -> WordToken {
            uint.into()
        }

        #[inline]
        fn detokenize_uint(token: WordToken) -> $uty {
            <$uty>::from_be_bytes::<32>(token.0 .0)
        }

        #[inline]
        fn encode_packed_to_uint(uint: $uty, out: &mut Vec<u8>) {
            out.extend_from_slice(&uint.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
        }
    };
}

#[rustfmt::skip]
macro_rules! int_impls2 {
    (  i8) => { int_impls! { @primitive_int i8 } };
    ( i16) => { int_impls! { @primitive_int i16 } };
    ( i32) => { int_impls! { @primitive_int i32 } };
    ( i64) => { int_impls! { @primitive_int i64 } };
    (i128) => { int_impls! { @primitive_int i128 } };

    (  u8) => { int_impls! { @primitive_uint u8 } };
    ( u16) => { int_impls! { @primitive_uint u16 } };
    ( u32) => { int_impls! { @primitive_uint u32 } };
    ( u64) => { int_impls! { @primitive_uint u64 } };
    (u128) => { int_impls! { @primitive_uint u128 } };

    (I256) => { int_impls! { @big_int I256 } };
    (U256) => { int_impls! { @big_uint U256 } };
}

supported_int!(
      8 => i8, u8;
     16 => i16, u16;
     24 => i32, u32;
     32 => i32, u32;
     40 => i64, u64;
     48 => i64, u64;
     56 => i64, u64;
     64 => i64, u64;
     72 => i128, u128;
     80 => i128, u128;
     88 => i128, u128;
     96 => i128, u128;
    104 => i128, u128;
    112 => i128, u128;
    120 => i128, u128;
    128 => i128, u128;
    136 => I256, U256;
    144 => I256, U256;
    152 => I256, U256;
    160 => I256, U256;
    168 => I256, U256;
    176 => I256, U256;
    184 => I256, U256;
    192 => I256, U256;
    200 => I256, U256;
    208 => I256, U256;
    216 => I256, U256;
    224 => I256, U256;
    232 => I256, U256;
    240 => I256, U256;
    248 => I256, U256;
    256 => I256, U256;
);
