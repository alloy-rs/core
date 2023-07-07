//! Ethereum bloom filter and utility functions.

use super::Log;
use alloy_primitives::Bloom;

/// Calculate receipt logs bloom.
pub fn logs_bloom<'a, I: IntoIterator<Item = &'a Log>>(logs: I) -> Bloom {
    let mut bloom = Bloom::ZERO;
    for log in logs {
        bloom.m3_2048(log.address.as_slice());
        for topic in &log.topics {
            bloom.m3_2048(topic.as_slice());
        }
    }
    bloom
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex;

    #[test]
    fn hardcoded_bloom() {
        let logs = vec![
            Log {
                address: hex!("22341ae42d6dd7384bc8584e50419ea3ac75b83f").into(),
                topics: vec![hex!(
                    "04491edcd115127caedbd478e2e7895ed80c7847e903431f94f9cfa579cad47f"
                )
                .into()],
                data: vec![].into(),
            },
            Log {
                address: hex!("e7fb22dfef11920312e4989a3a2b81e2ebf05986").into(),
                topics: vec![
                    hex!("7f1fef85c4b037150d3675218e0cdb7cf38fea354759471e309f3354918a442f").into(),
                    hex!("d85629c7eaae9ea4a10234fed31bc0aeda29b2683ebe0c1882499d272621f6b6").into(),
                ],
                data: hex::decode("2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7b")
                    .unwrap()
                    .into(),
            },
        ];
        assert_eq!(
            logs_bloom(&logs),
            Bloom::from(hex!(
                "000000000000000000810000000000000000000000000000000000020000000000000000000000000000008000"
                "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                "000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000"
                "000000000000000000000000000000000000000000000000000000280000000000400000800000004000000000"
                "000000000000000000000000000000000000000000000000000000000000100000100000000000000000000000"
                "00000000001400000000000000008000000000000000000000000000000000"
            ))
        );
    }
}
