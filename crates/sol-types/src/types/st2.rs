use super::{SolType, SolTypeEncodable};
use crate::sol_data::{self, ByteCount, SupportedFixedBytes};
use alloc::{borrow::Cow, string::String, vec::Vec};
use alloy_primitives::{Address, Bytes, FixedBytes, Function, I256, U256};

trait Encodable {
    type SolType: SolType;

    #[inline]
    fn abi_encode(&self) -> Vec<u8>
    where
        Self: SolTypeEncodable<Self::SolType>,
    {
        SolType::abi_encode(self)
    }

    #[inline]
    fn sol_type_name(&self) -> Cow<'static, str> {
        Self::SolType::sol_type_name()
    }
}

macro_rules! impl_encodable {
    ($($(#[$attr:meta])* [$($gen:tt)*] $rust:ty => $sol:ty [$($where:tt)*];)+) => {$(
        $(#[$attr])*
        impl<$($gen)*> Encodable for $rust $($where)* {
            type SolType = $sol;
        }
    )*};
}

impl_encodable! {
    // Basic
    [] bool => sol_data::Bool [];

    [] i8 => sol_data::Int::<8> [];
    [] i16 => sol_data::Int::<16> [];
    [] i32 => sol_data::Int::<32> [];
    [] i64 => sol_data::Int::<64> [];
    [] i128 => sol_data::Int::<128> [];
    [] I256 => sol_data::Int::<256> [];
    #[cfg(pointer_width = "32")]
    [] isize => sol_data::Int::<32> [];
    #[cfg(pointer_width = "64")]
    [] isize => sol_data::Int::<64> [];

    // TODO: Array<u8> is specialized to encode as `bytes`
    // [] u8 => sol_data::Uint::<8> [];
    [] u16 => sol_data::Uint::<16> [];
    [] u32 => sol_data::Uint::<32> [];
    [] u64 => sol_data::Uint::<64> [];
    [] u128 => sol_data::Uint::<128> [];
    [] U256 => sol_data::Uint::<256> [];
    #[cfg(pointer_width = "32")]
    [] usize => sol_data::Uint::<32> [];
    #[cfg(pointer_width = "64")]
    [] usize => sol_data::Uint::<64> [];

    [] Address => sol_data::Address [];
    [] Function => sol_data::Function [];
    [const N: usize] FixedBytes<N> => sol_data::FixedBytes<N> [where ByteCount<N>: SupportedFixedBytes];
    [] String => sol_data::String [];
    [] str => sol_data::String [];
    [] Bytes => sol_data::Bytes [];

    // Specialize u8 to bytes
    [] Vec<u8> => sol_data::Bytes [];
    [] [u8] => sol_data::Bytes [];
    [const N: usize] [u8; N] => sol_data::Bytes [];

    // Generic
    [T: Encodable] Vec<T> => sol_data::Array<T::SolType> [];
    [T: Encodable] [T] => sol_data::Array<T::SolType> [];
    [T: Encodable, const N: usize] [T; N] => sol_data::FixedArray<T::SolType, N> [];
}

// Have to override the `Self: SolTypeEncodable<Self::SolType>` bound for these
// because `SolTypeEncodable` is not implemented for references
macro_rules! deref_impls {
    ($($(#[$attr:meta])* [$($gen:tt)*] $rust:ty => $sol:ty [$($where:tt)*];)+) => {$(
        $(#[$attr])*
        impl<$($gen)*> Encodable for $rust $($where)* {
            type SolType = $sol;

            #[inline]
            fn abi_encode(&self) -> Vec<u8> {
                (**self).abi_encode()
            }
        }
    )*};
}

deref_impls! {
    [T: ?Sized + Encodable + SolTypeEncodable<T::SolType>] &T => T::SolType [];
    [T: ?Sized + Encodable + SolTypeEncodable<T::SolType>] &mut T => T::SolType [];
    [T: ?Sized + Encodable + SolTypeEncodable<T::SolType>] alloc::boxed::Box<T> => T::SolType [];
    [T: ?Sized + alloc::borrow::ToOwned + Encodable + SolTypeEncodable<T::SolType>] alloc::borrow::Cow<'_, T> => T::SolType [];
    [T: ?Sized + Encodable + SolTypeEncodable<T::SolType>] alloc::rc::Rc<T> => T::SolType [];
    [T: ?Sized + Encodable + SolTypeEncodable<T::SolType>] alloc::sync::Arc<T> => T::SolType [];
}

macro_rules! tuple_impls {
    ($count:literal $($ty:ident),+) => {
        impl<$($ty: Encodable,)+> Encodable for ($($ty,)+) {
            type SolType = ($($ty::SolType,)+);
        }
    };
}

impl Encodable for () {
    type SolType = ();
}

all_the_tuples!(tuple_impls);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Word;

    // Make sure these are in scope
    #[allow(unused_imports)]
    use crate::{SolType as _, SolTypeEncodable as _};

    #[test]
    fn basic() {
        assert_eq!(false.abi_encode(), Word::ZERO[..]);
        assert_eq!(true.abi_encode(), Word::with_last_byte(1)[..]);

        assert_eq!(0i8.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i16.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i32.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i64.abi_encode(), Word::ZERO[..]);
        assert_eq!(0i128.abi_encode(), Word::ZERO[..]);
        assert_eq!(I256::ZERO.abi_encode(), Word::ZERO[..]);

        assert_eq!(0u16.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u32.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u64.abi_encode(), Word::ZERO[..]);
        assert_eq!(0u128.abi_encode(), Word::ZERO[..]);
        assert_eq!(U256::ZERO.abi_encode(), Word::ZERO[..]);

        assert_eq!(Address::ZERO.abi_encode(), Word::ZERO[..]);
        assert_eq!(Function::ZERO.abi_encode(), Word::ZERO[..]);

        let encode_bytes = |b: &[u8]| {
            let last = Word::new({
                let mut buf = [0u8; 32];
                buf[..b.len()].copy_from_slice(b);
                buf
            });
            [
                &Word::with_last_byte(0x20)[..],
                &Word::with_last_byte(b.len() as u8)[..],
                if b.is_empty() { b } else { &last[..] },
            ]
            .concat()
        };
        assert_eq!("".abi_encode(), encode_bytes(b""));
        assert_eq!("a".abi_encode(), encode_bytes(b"a"));
        assert_eq!(String::new().abi_encode(), encode_bytes(b""));
        assert_eq!(String::from("a").abi_encode(), encode_bytes(b"a"));
        assert_eq!(b"".abi_encode(), encode_bytes(b""));
        assert_eq!(b"a".abi_encode(), encode_bytes(b"a"));
        assert_eq!((b"" as &[_]).abi_encode(), encode_bytes(b""));
        assert_eq!((b"a" as &[_]).abi_encode(), encode_bytes(b"a"));
        assert_eq!(Vec::<u8>::new().abi_encode(), encode_bytes(b""));
        assert_eq!(Vec::<u8>::from(b"a").abi_encode(), encode_bytes(b"a"));
    }

    #[test]
    fn big() {
        let tuple = (
            false,
            0i8,
            0i16,
            0i32,
            0i64,
            0i128,
            I256::ZERO,
            // 0u8,
            0u16,
            0u32,
            0u64,
            0u128,
            U256::ZERO,
            Address::ZERO,
            Function::ZERO,
        );
        let encoded = tuple.abi_encode();
        assert_eq!(encoded.len(), 32 * 14);
        assert!(encoded.iter().all(|&b| b == 0));
    }

    #[test]
    fn complex() {
        let tuple = ((((((false,),),),),),);
        assert_eq!(tuple.abi_encode(), Word::ZERO[..]);
        assert_eq!(tuple.sol_type_name(), "((((((bool))))))");

        let tuple = (
            42u64,
            "hello world",
            true,
            (
                String::from("aaaa"),
                Address::with_last_byte(69),
                Vec::from(b"bbbb"),
                b"cccc",
                &b"dddd"[..],
            ),
        );
        assert_eq!(
            tuple.sol_type_name(),
            "(uint64,string,bool,(string,address,bytes,bytes,bytes))"
        );
    }

    #[test]
    fn derefs() {
        let x: &[Address; 0] = &[];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[0]");

        let x = &[Address::ZERO];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[1]");

        let x = &[Address::ZERO, Address::ZERO];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[2]");

        let x = &[Address::ZERO][..];
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "address[]");

        let mut x = *b"";
        let x = (&mut x, *b"aaaa", &mut &mut b"");
        x.abi_encode();
        assert_eq!(x.sol_type_name(), "(bytes,bytes,bytes)");

        let tuple = &(&0u16, &"", &mut b"", &mut [Address::ZERO][..]);
        tuple.abi_encode();
        assert_eq!(tuple.sol_type_name(), "(uint16,string,bytes,address[])");
    }
}
