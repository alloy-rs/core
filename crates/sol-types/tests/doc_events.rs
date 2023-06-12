#![allow(clippy::assertions_on_constants)]

use alloy_primitives::{keccak256, B256};
use alloy_sol_types::{sol, token::WordToken, SolEvent};

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
    event MyEvent2(Data indexed data, );
}

#[test]
fn event() {
    assert_event_signature::<MyEvent>("MyEvent(bytes32,uint256,string,bytes)");
    assert!(!MyEvent::ANONYMOUS);
    let event = MyEvent::default();
    // topics are `(SELECTOR, a, keccak256(c))`
    assert_eq!(
        event.encode_topics_array::<3>(),
        [
            WordToken(MyEvent::SIGNATURE_HASH),
            WordToken(B256::ZERO),
            WordToken(keccak256(""))
        ]
    );
    // dynamic data is `abi.encode(b, c, d)`
    assert_eq!(
        event.data_size(),
        32 + (64 + (event.c.len() / 31) * 32) + (64 + (event.d.len() / 31) * 32)
    );

    assert_event_signature::<LogNote>("LogNote(bytes4,address,bytes32,bytes32,uint,bytes)");
    assert!(LogNote::ANONYMOUS);
}

fn assert_event_signature<T: SolEvent>(expected: &str) {
    assert_eq!(T::SIGNATURE, expected);
    assert_eq!(T::SIGNATURE_HASH, keccak256(expected));
}
