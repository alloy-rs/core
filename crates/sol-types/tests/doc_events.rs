#![allow(clippy::assertions_on_constants)]

use alloy_primitives::{keccak256, B256, U256};
use alloy_sol_types::{sol, token::WordToken, SolEvent};
use hex_literal::hex;

sol! {
    #[derive(Default)]
    event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);

    event LogNote(
        bytes4   indexed  sig,
        address  indexed  guy,
        bytes32  indexed  foo,
        bytes32  indexed  bar,
        uint              wad,
        bytes             fax
    ) anonymous;

    struct Data {
        bytes data;
    }
    event MyEvent2(Data indexed data);
}

#[test]
fn event() {
    assert_event_signature::<MyEvent>("MyEvent(bytes32,uint256,string,bytes)");
    assert!(!MyEvent::ANONYMOUS);
    let event = MyEvent {
        a: [0x11; 32],
        b: U256::from(1u64),
        c: keccak256("Hello World").into(),
        d: Vec::new(),
    };
    // topics are `(SELECTOR, a, keccak256(c))`
    assert_eq!(
        event.encode_topics_array::<3>(),
        [
            WordToken(MyEvent::SIGNATURE_HASH),
            WordToken(B256::repeat_byte(0x11)),
            WordToken(keccak256("Hello World"))
        ]
    );
    // dynamic data is `abi.encode(b, d)`
    assert_eq!(
        event.encode_data(),
        hex!(
            // b
            "0000000000000000000000000000000000000000000000000000000000000001"
            // d offset
            "0000000000000000000000000000000000000000000000000000000000000040"
            // d length
            "0000000000000000000000000000000000000000000000000000000000000000"
        ),
    );

    assert_event_signature::<LogNote>("LogNote(bytes4,address,bytes32,bytes32,uint,bytes)");
    assert!(LogNote::ANONYMOUS);

    assert_event_signature::<MyEvent2>("MyEvent2((bytes))");
    assert!(!MyEvent2::ANONYMOUS);
}

fn assert_event_signature<T: SolEvent>(expected: &str) {
    assert_eq!(T::SIGNATURE, expected);
    assert_eq!(T::SIGNATURE_HASH, keccak256(expected));
}
