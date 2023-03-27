use ethers_rlp::*;

#[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen, PartialEq, Debug)]
pub struct MyThing(#[rlp] [u8; 12]);

#[test]
fn simple_derive() {
    let thing = MyThing([0; 12]);

    // roundtrip fidelity
    let mut buf = Vec::new();
    thing.encode(&mut buf);
    let decoded = MyThing::decode(&mut buf.as_slice()).unwrap();
    assert_eq!(thing, decoded);

    // does not panic on short input
    assert_eq!(
        Err(DecodeError::InputTooShort),
        MyThing::decode(&mut [0x8c; 11].as_ref())
    )
}
