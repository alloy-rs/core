use alloy_json_abi::{AbiItem, Error, JsonAbi, Param};

#[test]
fn complex_error() {
    let json = r#"{
        "inputs": [
            {
                "internalType": "string",
                "name": "reason",
                "type": "string"
            }
        ],
        "name": "SomeName",
        "type": "error"
    }"#;
    let decoded: Error = serde_json::from_str(json).unwrap();
    assert_eq!(
        decoded,
        Error {
            inputs: vec![Param {
                internal_type: Some("string".into()),
                name: "reason".into(),
                ty: "string".into(),
                components: vec![],
            }],
            name: "SomeName".into()
        }
    );
}

macro_rules! abi_parse_tests {
    ($($name:ident($path:literal, $len:literal))*) => {$(
        #[test]
        fn $name() {
            parse_test(include_str!($path), $len);
        }
    )*};
}

abi_parse_tests! {
    abiencoderv2("abi/Abiencoderv2Test.json", 1)
    console("abi/console.json", 379)
    event_with_struct("abi/EventWithStruct.json", 1)
    large_array("abi/LargeArray.json", 1)
    large_struct("abi/LargeStruct.json", 1)
    large_structs("abi/LargeStructs.json", 4)
    large_tuple("abi/LargeTuple.json", 1)
    seaport("abi/Seaport.json", 69)
}

fn parse_test(s: &str, len: usize) {
    let abi_items: Vec<AbiItem<'_>> = serde_json::from_str(s).unwrap();
    assert_eq!(abi_items.len(), len);

    let json = serde_json::to_string(&abi_items).unwrap();
    let abi1: JsonAbi = serde_json::from_str(&json).unwrap();

    let abi2: JsonAbi = serde_json::from_str(s).unwrap();

    assert_eq!(len, abi2.len());
    assert_eq!(abi1, abi2);

    let json = serde_json::to_string(&abi2).unwrap();
    let abi3: JsonAbi = serde_json::from_str(&json).unwrap();
    assert_eq!(abi2, abi3);

    iterator_test(abi1.items(), abi1.items().rev(), len);
    iterator_test(abi1.items().skip(1), abi1.items().skip(1).rev(), len - 1);
    iterator_test(abi1.clone().into_items(), abi1.into_items().rev(), len);
}

fn iterator_test<T, I, R>(items: I, rev: R, len: usize)
where
    T: PartialEq + std::fmt::Debug,
    I: Iterator<Item = T> + DoubleEndedIterator + ExactSizeIterator,
    R: Iterator<Item = T>,
{
    assert_eq!(items.len(), len);
    assert_eq!(items.size_hint(), (len, Some(len)));

    let mut items2: Vec<_> = items.collect();
    items2.reverse();
    assert_eq!(items2, rev.collect::<Vec<_>>());
}
