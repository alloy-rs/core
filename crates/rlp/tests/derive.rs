#![cfg(feature = "derive")]

use alloy_rlp::*;

#[test]
fn simple_derive() {
    #[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen, PartialEq, Debug)]
    struct MyThing(#[rlp] [u8; 12]);

    let thing = MyThing([0; 12]);

    // roundtrip fidelity
    let mut buf = Vec::new();
    thing.encode(&mut buf);
    let decoded = MyThing::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(thing, decoded);

    // does not panic on short input
    assert_eq!(
        Err(Error::InputTooShort),
        MyThing::decode(&mut [0x8c; 11].as_ref())
    )
}

#[test]
fn wrapper() {
    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, RlpMaxEncodedLen, PartialEq, Debug)]
    struct Wrapper([u8; 8]);

    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, PartialEq, Debug)]
    struct ConstWrapper<const N: usize>([u8; N]);
}

#[test]
fn generics() {
    trait LT<'a> {}

    #[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen)]
    struct Generic<T, U: for<'a> LT<'a>, V: Default, const N: usize>(T, usize, U, V, [u8; N])
    where
        U: std::fmt::Display;

    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, RlpMaxEncodedLen)]
    struct GenericWrapper<T>(T)
    where
        T: Sized;
}

#[test]
fn opt() {
    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(trailing)]
    struct Options<T>(Option<Vec<T>>);

    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(trailing)]
    struct Options2<T> {
        a: Option<T>,
        #[rlp(default)]
        b: Option<T>,
    }
}
