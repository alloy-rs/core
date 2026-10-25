#![allow(unknown_lints, clippy::incompatible_msrv, missing_docs)]

use alloy_primitives::{Address, B256, Bytes, Log, LogData, hex};
use criterion::{Criterion, criterion_group, criterion_main};
use serde::Deserialize;
use std::hint::black_box;

/// Mirrors the shape of an `eth_getLogs` entry: a flattened [`Log`] plus RPC metadata.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct RpcLog {
    #[serde(flatten)]
    inner: Log,
    block_hash: B256,
    block_number: String,
    block_timestamp: String,
    transaction_hash: B256,
    transaction_index: String,
    log_index: String,
    removed: bool,
}

/// Realistic logs: ERC-20 `Transfer`, Uniswap V2 `Sync` and `Swap`, and an anonymous log.
fn logs() -> Vec<Log> {
    let topic = |s: &str| s.parse::<B256>().unwrap();
    let address = |s: &str| s.parse::<Address>().unwrap();
    let word = |n: u8| {
        let mut w = [0u8; 32];
        w[31] = n;
        w[20] = 0xa5;
        w
    };
    let data = |words: usize| Bytes::from((0..words as u8).flat_map(word).collect::<Vec<_>>());
    let transfer = topic("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
    let sync = topic("0x1c411e9a96e071241c2f21f7726b17ae89e3cab4c78be50e062b03a9fffbbad1");
    let swap = topic("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822");
    let user = topic("0x000000000000000000000000f977814e90da44bfa03b6295a0616a897441acec");
    let pool = topic("0x0000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488d");
    let log = |address, topics: Vec<B256>, words| Log {
        address,
        data: LogData::new(topics, data(words)).unwrap(),
    };
    vec![
        log(address("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"), vec![transfer, user, pool], 1),
        log(address("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc"), vec![sync], 2),
        log(address("0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc"), vec![swap, pool, user], 4),
        log(address("0x00000000000000000000000000000000000000f1"), vec![], 3),
    ]
}

fn rpc_json(log: &Log) -> String {
    let topics = serde_json::to_string(log.topics()).unwrap();
    format!(
        r#"{{"address":"{}","topics":{topics},"data":"0x{}","blockNumber":"0x1312d00","blockHash":"0x{}","blockTimestamp":"0x6553f100","transactionHash":"0x{}","transactionIndex":"0x5a","logIndex":"0x1f4","removed":false}}"#,
        log.address.to_string().to_lowercase(),
        hex::encode(&log.data.data),
        hex::encode([0x11; 32]),
        hex::encode([0x22; 32]),
    )
}

fn log(c: &mut Criterion) {
    let logs = logs();
    let plain: Vec<String> = logs.iter().map(|l| serde_json::to_string(l).unwrap()).collect();
    let rpc: Vec<String> = logs.iter().map(rpc_json).collect();
    let values: Vec<serde_json::Value> =
        plain.iter().map(|s| serde_json::from_str(s).unwrap()).collect();

    let mut g = c.benchmark_group("log");
    g.bench_function("deserialize/json", |b| {
        b.iter(|| {
            for s in &plain {
                black_box(serde_json::from_str::<Log>(black_box(s)).unwrap());
            }
        })
    });
    g.bench_function("deserialize/json_rpc_flatten", |b| {
        b.iter(|| {
            for s in &rpc {
                black_box(serde_json::from_str::<RpcLog>(black_box(s)).unwrap());
            }
        })
    });
    g.bench_function("deserialize/value", |b| {
        b.iter(|| {
            for v in &values {
                black_box(Log::<LogData>::deserialize(black_box(v)).unwrap());
            }
        })
    });
    g.finish();
}

criterion_group!(benches, log);
criterion_main!(benches);
