use alloy_primitives::{keccak256, B256};
use alloy_sol_types::{sol, token::WordToken, SolEvent};

sol! {
    #[derive(Default)]
    event MyEvent(bytes32 indexed a, uint256 b, string indexed c, bytes d);

    event MyAnonEvent(bool) anonymous;
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
    // dynamic data is `abi_encode(b, c, d)`
    assert_eq!(
        event.data_size(),
        32 + (64 + (event.c.len() / 31) * 32) + (64 + (event.d.len() / 31) * 32)
    );

    assert_event_signature::<MyAnonEvent>("MyAnonEvent(bool)");
    assert!(MyAnonEvent::ANONYMOUS);
}

fn assert_event_signature<T: SolEvent>(expected: &str) {
    assert_eq!(T::SIGNATURE, expected);
    assert_eq!(T::SIGNATURE_HASH, keccak256(expected));
}
