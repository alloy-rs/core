#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_dyn_abi::TypedData;
use criterion::{Criterion, criterion_group, criterion_main};
use serde_json::{Value, json};
use std::{hint::black_box, time::Duration};

const DOMAIN: [(&str, &str); 4] = [
    ("name", "string"),
    ("version", "string"),
    ("chainId", "uint256"),
    ("verifyingContract", "address"),
];

fn fields(fields: &[(&str, &str)]) -> Value {
    fields.iter().map(|(name, ty)| json!({ "name": name, "type": ty })).collect()
}

fn typed_data(types: Value, primary_type: &str, message: Value) -> TypedData {
    let mut types = types.as_object().unwrap().clone();
    types.insert("EIP712Domain".into(), fields(&DOMAIN));
    serde_json::from_value(json!({
        "types": types,
        "primaryType": primary_type,
        "domain": {
            "name": "Ether Mail",
            "version": "1",
            "chainId": 1,
            "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC",
        },
        "message": message,
    }))
    .unwrap()
}

/// The `Mail` example: a struct with nested structs, arrays of structs and arrays of atoms.
fn mail() -> TypedData {
    let person = |name: &str, wallets: &[&str]| json!({ "name": name, "wallets": wallets });
    typed_data(
        json!({
            "Person": fields(&[("name", "string"), ("wallets", "address[]")]),
            "Mail": fields(&[
                ("from", "Person"),
                ("to", "Person[]"),
                ("contents", "string"),
                ("attachments", "Attachment[]"),
            ]),
            "Attachment": fields(&[("name", "string"), ("size", "uint256")]),
        }),
        "Mail",
        json!({
            "from": person("Cow", &[
                "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826",
                "0xDeaDbeefdEAdbeefdEadbEEFdeadbeEFdEaDbeeF",
            ]),
            "to": [person("Bob", &["0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"])],
            "contents": "Hello, Bob!",
            "attachments": [
                { "name": "a.png", "size": 1024 },
                { "name": "b.png", "size": 2048 },
            ],
        }),
    )
}

/// A struct with an array of many nested structs.
fn array_of_structs() -> TypedData {
    let items: Vec<_> = (0..32)
        .map(|i| {
            json!({
                "token": "0xA604060890923Ff400e8c6f5290461A83AEDACec",
                "id": i,
                "amount": 1_000_000 + i,
                "memo": format!("item {i}"),
            })
        })
        .collect();
    typed_data(
        json!({
            "Order": fields(&[("offerer", "address"), ("items", "Item[]"), ("counter", "uint256")]),
            "Item": fields(&[
                ("token", "address"),
                ("id", "uint256"),
                ("amount", "uint256"),
                ("memo", "string"),
            ]),
        }),
        "Order",
        json!({
            "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "items": items,
            "counter": 0,
        }),
    )
}

fn signing_hash(c: &mut Criterion) {
    let mut g = c.benchmark_group("eip712/signing_hash");
    g.noise_threshold(0.03)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .sample_size(200);

    for (name, data) in [("mail", mail()), ("array_of_structs", array_of_structs())] {
        g.bench_function(name, |b| b.iter(|| black_box(&data).eip712_signing_hash().unwrap()));
    }

    g.finish();
}

criterion_group!(benches, signing_hash);
criterion_main!(benches);
