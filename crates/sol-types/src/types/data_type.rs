//! Solidity types.
//!
//! These are the types that are [built into Solidity][ref].
//!
//! [ref]: https://docs.soliditylang.org/en/latest/types.html

#![allow(missing_copy_implementations, missing_debug_implementations)]

use crate::{token::*, util, Encodable, Result, SolType, Word};
use alloc::{borrow::Cow, string::String as RustString, vec::Vec};
use alloy_primitives::{keccak256, Address as RustAddress, I256, U256};
use core::{borrow::Borrow, fmt::*, hash::Hash, marker::PhantomData, ops::*};

/// Bool - `bool`
pub struct Bool;

impl Encodable<Bool> for bool {
    #[inline]
    fn to_tokens(&self) -> WordToken {
        WordToken(Word::with_last_byte(*self as u8))
    }
}

impl SolType for Bool {
    type RustType = bool;
    type TokenType<'a> = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        if util::check_bool(token.0) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        token.0 != Word::ZERO
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        Encodable::<Self>::to_tokens(rust).0
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        out.push(*rust as u8);
    }
}

/// Int - `intX`
pub struct Int<const BITS: usize>;

impl<T, const BITS: usize> Encodable<Int<BITS>> for T
where
    T: Borrow<<IntBitCount<BITS> as SupportedInt>::Int>,
    IntBitCount<BITS>: SupportedInt,
{
    #[inline]
    fn to_tokens(&self) -> WordToken {
        IntBitCount::<BITS>::tokenize_int(*self.borrow())
    }
}

impl<const BITS: usize> SolType for Int<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Int;
    type TokenType<'a> = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::INT_NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        if BITS == 256 {
            return Ok(())
        }

        let is_negative = token.0[IntBitCount::<BITS>::WORD_MSB] & 0x80 == 0x80;
        let sign_extension = is_negative as u8 * 0xff;

        // check that all upper bytes are an extension of the sign bit
        if token.0[..IntBitCount::<BITS>::WORD_MSB]
            .iter()
            .all(|byte| *byte == sign_extension)
        {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_int(token)
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        Encodable::<Self>::to_tokens(rust).0
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_int(*rust, out)
    }
}

/// Uint - `uintX`
pub struct Uint<const BITS: usize>;

impl<const BITS: usize, T> Encodable<Uint<BITS>> for T
where
    T: Borrow<<IntBitCount<BITS> as SupportedInt>::Uint>,
    IntBitCount<BITS>: SupportedInt,
{
    #[inline]
    fn to_tokens(&self) -> WordToken {
        IntBitCount::<BITS>::tokenize_uint(*self.borrow())
    }
}

impl<const BITS: usize> SolType for Uint<BITS>
where
    IntBitCount<BITS>: SupportedInt,
{
    type RustType = <IntBitCount<BITS> as SupportedInt>::Uint;
    type TokenType<'a> = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        IntBitCount::<BITS>::UINT_NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        let sli = &token.0[..<IntBitCount<BITS> as SupportedInt>::WORD_MSB];
        if util::check_zeroes(sli) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        IntBitCount::<BITS>::detokenize_uint(token)
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        Encodable::<Self>::to_tokens(rust).0
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        IntBitCount::<BITS>::encode_packed_to_uint(*rust, out)
    }
}

/// Address - `address`
pub struct Address;

impl<T: Borrow<[u8; 20]>> Encodable<Address> for T {
    #[inline]
    fn to_tokens(&self) -> WordToken {
        WordToken::from(*self.borrow())
    }
}

impl SolType for Address {
    type RustType = RustAddress;
    type TokenType<'a> = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "address".into()
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        RustAddress::from_word(token.0)
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        if util::check_zeroes(&token.0[..12]) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        Encodable::<Self>::to_tokens(rust).0
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        out.extend_from_slice(rust.as_ref());
    }
}

/// Bytes - `bytes`
pub struct Bytes;

impl<T: AsRef<[u8]>> Encodable<Bytes> for T {
    #[inline]
    fn to_tokens(&self) -> PackedSeqToken<'_> {
        PackedSeqToken(self.as_ref())
    }
}

impl SolType for Bytes {
    type RustType = Vec<u8>;
    type TokenType<'a> = PackedSeqToken<'a>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "bytes".into()
    }

    #[inline]
    fn encoded_size(_data: &Self::RustType) -> usize {
        32 + util::padded_len(_data.borrow())
    }

    #[inline]
    fn type_check(_token: &Self::TokenType<'_>) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        token.into_vec()
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        keccak256(Self::encode_packed(rust))
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        out.extend_from_slice(rust);
    }
}

/// Array - `T[]`
pub struct Array<T: SolType>(PhantomData<T>);

impl<T, U> Encodable<Array<T>> for [U]
where
    T: SolType,
    U: Borrow<T::RustType>,
{
    #[inline]
    fn to_tokens(&self) -> DynSeqToken<T::TokenType<'_>> {
        DynSeqToken(self.iter().map(|r| r.borrow().to_tokens()).collect())
    }
}

impl<T, U> Encodable<Array<T>> for Vec<U>
where
    T: SolType,
    U: Borrow<T::RustType>,
{
    #[inline]
    fn to_tokens(&self) -> DynSeqToken<T::TokenType<'_>> {
        <[U] as Encodable<Array<T>>>::to_tokens(self)
    }
}

impl<T: SolType> SolType for Array<T> {
    type RustType = Vec<T::RustType>;
    type TokenType<'a> = DynSeqToken<T::TokenType<'a>>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[]", T::sol_type_name()).into()
    }

    #[inline]
    fn encoded_size(rust: &Self::RustType) -> usize {
        let data = rust;
        32 + data.iter().map(T::encoded_size).sum::<usize>()
            + (T::DYNAMIC as usize * 32 * data.len())
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        token.0.iter().try_for_each(T::type_check)
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        token.0.into_iter().map(T::detokenize).collect()
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        let mut encoded = Vec::new();
        for item in rust {
            encoded.extend_from_slice(T::eip712_data_word(item).as_slice());
        }
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        for item in rust {
            T::encode_packed_to(item, out);
        }
    }
}

/// String - `string`
pub struct String;

impl<T: AsRef<str>> Encodable<String> for T {
    #[inline]
    fn to_tokens(&self) -> <String as SolType>::TokenType<'_> {
        self.as_ref().as_bytes().into()
    }
}

impl SolType for String {
    type RustType = RustString;
    type TokenType<'a> = PackedSeqToken<'a>;

    const ENCODED_SIZE: Option<usize> = None;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "string".into()
    }

    #[inline]
    fn encoded_size(rust: &Self::RustType) -> usize {
        32 + util::padded_len(rust.as_bytes())
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        if core::str::from_utf8(token.as_slice()).is_ok() {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        // NOTE: We're decoding strings using lossy UTF-8 decoding to
        // prevent invalid strings written into contracts by either users or
        // Solidity bugs from causing graph-node to fail decoding event
        // data.
        RustString::from_utf8_lossy(&Bytes::detokenize(token)).into_owned()
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        keccak256(Self::encode_packed(rust))
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        out.extend_from_slice(rust.as_bytes());
    }
}

/// FixedBytes - `bytesX`
#[derive(Clone, Copy, Debug)]
pub struct FixedBytes<const N: usize>;

impl<const N: usize, T> Encodable<FixedBytes<N>> for T
where
    ByteCount<N>: SupportedFixedBytes,
    T: Borrow<[u8; N]>,
{
    #[inline]
    fn to_tokens(&self) -> <FixedBytes<N> as SolType>::TokenType<'_> {
        let mut word = Word::ZERO;
        word[..N].copy_from_slice(self.borrow());
        word.into()
    }
}

impl<const N: usize> SolType for FixedBytes<N>
where
    ByteCount<N>: SupportedFixedBytes,
{
    type RustType = [u8; N];
    type TokenType<'a> = WordToken;

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        <ByteCount<N>>::NAME.into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        if util::check_zeroes(&token.0[N..]) {
            Ok(())
        } else {
            Err(Self::type_check_fail(token.as_slice()))
        }
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        token.0[..N].try_into().unwrap()
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        Encodable::<Self>::to_tokens(rust).0
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        // write only the first n bytes
        out.extend_from_slice(rust);
    }
}

/// FixedArray - `T[M]`
pub struct FixedArray<T, const N: usize>(PhantomData<T>);

impl<T, U, const N: usize> Encodable<FixedArray<T, N>> for [U; N]
where
    T: SolType,
    U: Borrow<T::RustType>,
{
    #[inline]
    fn to_tokens(&self) -> <FixedArray<T, N> as SolType>::TokenType<'_> {
        FixedSeqToken::<_, N>(core::array::from_fn(|i| {
            Encodable::<T>::to_tokens(self[i].borrow())
        }))
    }
}

impl<T: SolType, const N: usize> SolType for FixedArray<T, N> {
    type RustType = [T::RustType; N];
    type TokenType<'a> = FixedSeqToken<T::TokenType<'a>, N>;

    const ENCODED_SIZE: Option<usize> = {
        match T::ENCODED_SIZE {
            Some(size) => Some(size * N),
            None => None,
        }
    };

    #[inline]
    fn encoded_size(rust: &Self::RustType) -> usize {
        if let Some(size) = Self::ENCODED_SIZE {
            return size
        }

        rust.iter().map(T::encoded_size).sum::<usize>() + (T::DYNAMIC as usize * N * 32)
    }

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        format!("{}[{}]", T::sol_type_name(), N).into()
    }

    #[inline]
    fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
        for token in token.as_array().iter() {
            T::type_check(token)?;
        }
        Ok(())
    }

    #[inline]
    fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
        token.0.map(T::detokenize)
    }

    #[inline]
    fn eip712_data_word(rust: &Self::RustType) -> Word {
        let rust = rust;
        let encoded = rust
            .iter()
            .flat_map(|element| T::eip712_data_word(element).0)
            .collect::<Vec<u8>>();
        keccak256(encoded)
    }

    #[inline]
    fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
        for item in rust {
            T::encode_packed_to(item, out);
        }
    }
}

macro_rules! tuple_encodable_impls {
    ($(($ty:ident $uty:ident)),+) => {
        #[allow(non_snake_case)]
        impl<$($ty: SolType, $uty,)+> Encodable<($($ty,)+)> for ($( $uty, )+)
        where
            $($uty: $crate::Encodable<$ty>,)+
         {
            #[inline]
            fn to_tokens(&self) -> <($($ty,)+) as SolType>::TokenType<'_> {
                let ($($ty,)+) = self;
                (
                    $(
                        Encodable::<$ty>::to_tokens($ty),
                    )+
                )
            }
        }
    };
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
            type TokenType<'a> = ($( $ty::TokenType<'a>, )+);

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

            fn encoded_size(rust: &Self::RustType) -> usize {
                if let Some(size) = Self::ENCODED_SIZE {
                    return size
                }

                let ($($ty,)+) = rust;
                0 $(
                    + <$ty as SolType>::encoded_size($ty)
                )+
                $(
                    + (32 * <$ty as SolType>::DYNAMIC as usize)
                )+
            }

            fn type_check(token: &Self::TokenType<'_>) -> Result<()> {
                let ($($ty,)+) = token;
                $(
                    <$ty as SolType>::type_check($ty)?;
                )+
                Ok(())
            }

            fn detokenize(token: Self::TokenType<'_>) -> Self::RustType {
                let ($($ty,)+) = token;
                ($(
                    <$ty as SolType>::detokenize($ty),
                )+)
            }

            fn eip712_data_word(rust: &Self::RustType) -> Word {
                const COUNT: usize = 0usize $(+ tuple_impls!(@one $ty))+;
                let ($($ty,)+) = rust;
                let encoding: [[u8; 32]; COUNT] = [$(
                    <$ty as SolType>::eip712_data_word($ty).0,
                )+];
                // SAFETY: Flattening [[u8; 32]; COUNT] to [u8; COUNT * 32] is valid
                let ptr = encoding.as_ptr() as *const u8;
                let len = COUNT * 32;
                let encoding: &[u8] = unsafe { core::slice::from_raw_parts(ptr, len) };
                keccak256(encoding).into()
            }

            fn encode_packed_to(rust: &Self::RustType, out: &mut Vec<u8>) {
                let ($($ty,)+) = rust;
                // TODO: Reserve
                $(
                    <$ty as SolType>::encode_packed_to($ty, out);
                )+
            }
        }
    };
}

impl Encodable<()> for () {
    #[inline]
    fn to_tokens(&self) {}
}

all_the_tuples!(@double tuple_encodable_impls);

impl SolType for () {
    type RustType = ();
    type TokenType<'a> = ();

    const ENCODED_SIZE: Option<usize> = Some(0);

    #[inline]
    fn sol_type_name() -> Cow<'static, str> {
        "()".into()
    }

    #[inline]
    fn type_check(_token: &Self::TokenType<'_>) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn detokenize(_token: Self::TokenType<'_>) -> Self::RustType {}

    #[inline]
    fn eip712_data_word(_rust: &Self::RustType) -> Word {
        Word::ZERO
    }

    #[inline]
    fn encode_packed_to(_rust: &Self::RustType, _out: &mut Vec<u8>) {}
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
        /// The signed integer Rust representation.
        type Int;
        /// The unsigned integer Rust representation.
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

    /// The difference between the representation's and this integer's bytes:
    /// `(Self::Int::BITS - Self::BITS) / 8`
    ///
    /// E.g.: `word[Self::WORD_MSB - Self::SKIP_BYTES..] == int.to_be_bytes()`
    const SKIP_BYTES: usize;

    /// The index of the most significant byte in the Word type.
    ///
    /// E.g.: `word[Self::WORD_MSB..] == int.to_be_bytes()[Self::SKIP_BYTES..]`
    const WORD_MSB: usize = 32 - Self::BYTES;

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
    ($($n:literal => $i:ident, $u:ident;)+) => {$(
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
    )+};
}

macro_rules! int_impls {
    (@primitive_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            let mut word = [int.is_negative() as u8 * 0xff; 32];
            word[Self::WORD_MSB..].copy_from_slice(&int.to_be_bytes()[Self::SKIP_BYTES..]);
            WordToken::new(word)
        }

        #[inline]
        fn detokenize_int(mut token: WordToken) -> $ity {
            // sign extend bits to ignore
            let is_negative = token.0[Self::WORD_MSB] & 0x80 == 0x80;
            let sign_extension = is_negative as u8 * 0xff;
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(sign_extension);

            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$ity>::from_be_bytes(s.try_into().unwrap())
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
            word[Self::WORD_MSB..].copy_from_slice(&uint.to_be_bytes()[Self::SKIP_BYTES..]);
            WordToken(word)
        }

        #[inline]
        fn detokenize_uint(mut token: WordToken) -> $uty {
            // zero out bits to ignore (u24):
            // mov   byte ptr [rdi + 28], 0
            // movbe eax, dword ptr [rdi + 28]
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(0);
            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$uty>::from_be_bytes(s.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_uint(uint: $uty, out: &mut Vec<u8>) {
            out.extend_from_slice(&uint.to_be_bytes()[Self::SKIP_BYTES..]);
        }
    };

    (@big_int $ity:ident) => {
        #[inline]
        fn tokenize_int(int: $ity) -> WordToken {
            let mut word = [int.is_negative() as u8 * 0xff; 32];
            word[Self::WORD_MSB..].copy_from_slice(&int.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
            WordToken::new(word)
        }

        #[inline]
        fn detokenize_int(mut token: WordToken) -> $ity {
            // sign extend bits to ignore
            let is_negative = token.0[Self::WORD_MSB] & 0x80 == 0x80;
            let sign_extension = is_negative as u8 * 0xff;
            token.0[Self::WORD_MSB - Self::SKIP_BYTES..Self::WORD_MSB].fill(sign_extension);

            let s = &token.0[Self::WORD_MSB - Self::SKIP_BYTES..];
            <$ity>::from_be_bytes::<32>(s.try_into().unwrap())
        }

        #[inline]
        fn encode_packed_to_int(int: $ity, out: &mut Vec<u8>) {
            out.extend_from_slice(&int.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
        }
    };
    (@big_uint $uty:ident) => {
        #[inline]
        fn tokenize_uint(uint: $uty) -> WordToken {
            let mut word = Word::ZERO;
            word[Self::WORD_MSB..].copy_from_slice(&uint.to_be_bytes::<32>()[Self::SKIP_BYTES..]);
            WordToken(word)
        }

        #[inline]
        fn detokenize_uint(mut token: WordToken) -> $uty {
            // zero out bits to ignore
            token.0[..Self::SKIP_BYTES].fill(0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_of_refs() {
        let a = (1u8,);
        let b = (&1u8,);

        type MyTy = (Uint<8>,);

        MyTy::tokenize(&a);
        MyTy::tokenize(&b);
    }

    macro_rules! roundtrip {
        ($($name:ident($st:ty : $t:ty);)+) => {
            proptest::proptest! {$(
                #[test]
                fn $name(i: $t) {
                    proptest::prop_assert_eq!(<$st>::detokenize(<$st>::tokenize(&i)), i);
                }
            )+}
        };
    }

    roundtrip! {
        roundtrip_address(Address: RustAddress);
        roundtrip_bool(Bool: bool);
        roundtrip_bytes(Bytes: Vec<u8>);
        roundtrip_string(String: RustString);
        roundtrip_fixed_bytes_16(FixedBytes<16>: [u8; 16]);
        roundtrip_fixed_bytes_32(FixedBytes<32>: [u8; 32]);

        // can only test corresponding integers
        roundtrip_u8(Uint<8>: u8);
        roundtrip_i8(Int<8>: i8);
        roundtrip_u16(Uint<16>: u16);
        roundtrip_i16(Int<16>: i16);
        roundtrip_u32(Uint<32>: u32);
        roundtrip_i32(Int<32>: i32);
        roundtrip_u64(Uint<64>: u64);
        roundtrip_i64(Int<64>: i64);
        roundtrip_u128(Uint<128>: u128);
        roundtrip_i128(Int<128>: i128);
        roundtrip_u256(Uint<256>: U256);
        roundtrip_i256(Int<256>: I256);
    }

    #[test]
    fn tokenize_uint() {
        macro_rules! test {
            ($($n:literal: $x:expr => $l:literal),+ $(,)?) => {$(
                let uint: <Uint<$n> as SolType>::RustType = $x.into();
                let int = <Int<$n> as SolType>::RustType::try_from(uint).unwrap();

                assert_eq!(
                    <Uint<$n>>::tokenize(&uint),
                    WordToken::new(hex_literal::hex!($l))
                );
                assert_eq!(
                    <Int<$n>>::tokenize(&int),
                    WordToken::new(hex_literal::hex!($l))
                );
            )+};
        }

        let word: Word = Word::new(core::array::from_fn(|i| i as u8 + 1));

        test! {
             8: 0x00u8 => "0000000000000000000000000000000000000000000000000000000000000000",
             8: 0x01u8 => "0000000000000000000000000000000000000000000000000000000000000001",
            24: 0x01020304u32 => "0000000000000000000000000000000000000000000000000000000000020304",
            32: 0x01020304u32 => "0000000000000000000000000000000000000000000000000000000001020304",
            56: 0x0102030405060708u64 => "0000000000000000000000000000000000000000000000000002030405060708",
            64: 0x0102030405060708u64 => "0000000000000000000000000000000000000000000000000102030405060708",

            160: word => "0000000000000000000000000d0e0f101112131415161718191a1b1c1d1e1f20",
            200: word => "0000000000000008090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
            256: word => "0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
        }
    }

    #[test]
    fn detokenize_ints() {
        /*
        for i in range(1, 32 + 1):
            n = "0x"
            for j in range(32, 0, -1):
                if j <= i:
                    n += hex(33 - j)[2:].zfill(2)
                else:
                    n += "00"
            if i > 16:
                n = f'"{n}".parse().unwrap()'
            else:
                n = f" {n}"
            print(f"{i * 8:4} => {n},")
        */
        let word = core::array::from_fn(|i| i as u8 + 1);
        let token = WordToken::new(word);
        macro_rules! test {
            ($($n:literal => $x:expr),+ $(,)?) => {$(
                assert_eq!(<Uint<$n>>::detokenize(token), $x);
                assert_eq!(<Int<$n>>::detokenize(token), $x);
            )+};
        }
        #[rustfmt::skip]
        test! {
             8 =>  0x0000000000000000000000000000000000000000000000000000000000000020,
            16 =>  0x0000000000000000000000000000000000000000000000000000000000001f20,
            24 =>  0x00000000000000000000000000000000000000000000000000000000001e1f20,
            32 =>  0x000000000000000000000000000000000000000000000000000000001d1e1f20,
            40 =>  0x0000000000000000000000000000000000000000000000000000001c1d1e1f20,
            48 =>  0x00000000000000000000000000000000000000000000000000001b1c1d1e1f20,
            56 =>  0x000000000000000000000000000000000000000000000000001a1b1c1d1e1f20,
            64 =>  0x000000000000000000000000000000000000000000000000191a1b1c1d1e1f20,
            72 =>  0x000000000000000000000000000000000000000000000018191a1b1c1d1e1f20,
            80 =>  0x000000000000000000000000000000000000000000001718191a1b1c1d1e1f20,
            88 =>  0x000000000000000000000000000000000000000000161718191a1b1c1d1e1f20,
            96 =>  0x000000000000000000000000000000000000000015161718191a1b1c1d1e1f20,
           104 =>  0x000000000000000000000000000000000000001415161718191a1b1c1d1e1f20,
           112 =>  0x000000000000000000000000000000000000131415161718191a1b1c1d1e1f20,
           120 =>  0x000000000000000000000000000000000012131415161718191a1b1c1d1e1f20,
           128 =>  0x000000000000000000000000000000001112131415161718191a1b1c1d1e1f20,
           136 => "0x000000000000000000000000000000101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           144 => "0x00000000000000000000000000000f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           152 => "0x000000000000000000000000000e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           160 => "0x0000000000000000000000000d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           168 => "0x00000000000000000000000c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           176 => "0x000000000000000000000b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           184 => "0x0000000000000000000a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           192 => "0x0000000000000000090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           200 => "0x0000000000000008090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           208 => "0x0000000000000708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           216 => "0x0000000000060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           224 => "0x0000000005060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           232 => "0x0000000405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           240 => "0x0000030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           248 => "0x0002030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
           256 => "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20".parse().unwrap(),
        };
    }

    #[test]
    fn detokenize_negative_int() {
        let word = [0xff; 32];
        let token = WordToken::new(word);
        assert_eq!(<Int<8>>::detokenize(token), -1);
        assert_eq!(<Int<16>>::detokenize(token), -1);
        assert_eq!(<Int<24>>::detokenize(token), -1);
        assert_eq!(<Int<32>>::detokenize(token), -1);
        assert_eq!(<Int<40>>::detokenize(token), -1);
        assert_eq!(<Int<48>>::detokenize(token), -1);
        assert_eq!(<Int<56>>::detokenize(token), -1);
        assert_eq!(<Int<64>>::detokenize(token), -1);
        assert_eq!(<Int<72>>::detokenize(token), -1);
        assert_eq!(<Int<80>>::detokenize(token), -1);
        assert_eq!(<Int<88>>::detokenize(token), -1);
        assert_eq!(<Int<96>>::detokenize(token), -1);
        assert_eq!(<Int<104>>::detokenize(token), -1);
        assert_eq!(<Int<112>>::detokenize(token), -1);
        assert_eq!(<Int<120>>::detokenize(token), -1);
        assert_eq!(<Int<128>>::detokenize(token), -1);
        assert_eq!(<Int<136>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<144>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<152>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<160>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<168>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<176>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<184>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<192>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<200>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<208>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<216>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<224>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<232>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<240>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<248>>::detokenize(token), I256::MINUS_ONE);
        assert_eq!(<Int<256>>::detokenize(token), I256::MINUS_ONE);
    }

    #[test]
    #[rustfmt::skip]
    fn detokenize_int() {
        let word =
            core::array::from_fn(|i| (i | (0x80 * (i % 2 == 1) as usize)) as u8 + 1);
        let token = WordToken::new(word);
        trait Conv {
            fn as_u256_as_i256(&self) -> I256;
        }
        impl Conv for str {
            fn as_u256_as_i256(&self) -> I256 {
                I256::from_raw(self.parse::<U256>().unwrap())
            }
        }
        assert_eq!(<Int<8>>::detokenize(token),    0x00000000000000000000000000000000000000000000000000000000000000a0_u8 as i8);
        assert_eq!(<Int<16>>::detokenize(token),   0x0000000000000000000000000000000000000000000000000000000000001fa0_u16 as i16);
        assert_eq!(<Int<24>>::detokenize(token),   0x00000000000000000000000000000000000000000000000000000000ff9e1fa0_u32 as i32);
        assert_eq!(<Int<32>>::detokenize(token),   0x000000000000000000000000000000000000000000000000000000001d9e1fa0_u32 as i32);
        assert_eq!(<Int<40>>::detokenize(token),   0x000000000000000000000000000000000000000000000000ffffff9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<48>>::detokenize(token),   0x00000000000000000000000000000000000000000000000000001b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<56>>::detokenize(token),   0x000000000000000000000000000000000000000000000000ff9a1b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<64>>::detokenize(token),   0x000000000000000000000000000000000000000000000000199a1b9c1d9e1fa0_u64 as i64);
        assert_eq!(<Int<72>>::detokenize(token),   0x00000000000000000000000000000000ffffffffffffff98199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<80>>::detokenize(token),   0x000000000000000000000000000000000000000000001798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<88>>::detokenize(token),   0x00000000000000000000000000000000ffffffffff961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<96>>::detokenize(token),   0x000000000000000000000000000000000000000015961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<104>>::detokenize(token),  0x00000000000000000000000000000000ffffff9415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<112>>::detokenize(token),  0x000000000000000000000000000000000000139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<120>>::detokenize(token),  0x00000000000000000000000000000000ff92139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<128>>::detokenize(token),  0x000000000000000000000000000000001192139415961798199a1b9c1d9e1fa0_u128 as i128);
        assert_eq!(<Int<136>>::detokenize(token), "0xffffffffffffffffffffffffffffff901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<144>>::detokenize(token), "0x00000000000000000000000000000f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<152>>::detokenize(token), "0xffffffffffffffffffffffffff8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<160>>::detokenize(token), "0x0000000000000000000000000d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<168>>::detokenize(token), "0xffffffffffffffffffffff8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<176>>::detokenize(token), "0x000000000000000000000b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<184>>::detokenize(token), "0xffffffffffffffffff8a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<192>>::detokenize(token), "0x0000000000000000098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<200>>::detokenize(token), "0xffffffffffffff88098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<208>>::detokenize(token), "0x0000000000000788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<216>>::detokenize(token), "0xffffffffff860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<224>>::detokenize(token), "0x0000000005860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<232>>::detokenize(token), "0xffffff8405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<240>>::detokenize(token), "0x0000038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<248>>::detokenize(token), "0xff82038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
        assert_eq!(<Int<256>>::detokenize(token), "0x0182038405860788098a0b8c0d8e0f901192139415961798199a1b9c1d9e1fa0".as_u256_as_i256());
    }
}
