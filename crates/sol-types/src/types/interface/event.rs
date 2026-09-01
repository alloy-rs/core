use crate::{Result, Word, abi::AbiDecoderConfig};
use alloy_primitives::Log;

/// A collection of [`SolEvent`]s.
///
/// [`SolEvent`]: crate::SolEvent
///
/// # Implementer's Guide
///
/// It should not be necessary to implement this trait manually. Instead, use
/// the [`sol!`](crate::sol!) procedural macro to parse Solidity syntax into
/// types that implement this trait.
pub trait SolEventInterface: Sized {
    /// The name of this type.
    const NAME: &'static str;

    /// The number of variants.
    const COUNT: usize;

    /// Decode the events from the given log info.
    fn decode_raw_log(topics: &[Word], data: &[u8]) -> Result<Self>;

    /// Decode the events from the given log info with a custom decoder configuration.
    ///
    /// The default implementation supports only the default configuration.
    #[inline]
    fn decode_raw_log_with_config(
        topics: &[Word],
        data: &[u8],
        config: AbiDecoderConfig,
    ) -> Result<Self> {
        if config.is_default() {
            Self::decode_raw_log(topics, data)
        } else {
            Err(crate::Error::custom(
                "decoder config is unsupported by this SolEventInterface implementation",
            ))
        }
    }

    /// Decode the events from the given log object.
    fn decode_log(log: &Log) -> Result<Log<Self>> {
        Self::decode_raw_log(log.topics(), &log.data.data)
            .map(|data| Log { address: log.address, data })
    }

    /// Decode the events from the given log object with a custom decoder configuration.
    fn decode_log_with_config(log: &Log, config: AbiDecoderConfig) -> Result<Log<Self>> {
        Self::decode_raw_log_with_config(log.topics(), &log.data.data, config)
            .map(|data| Log { address: log.address, data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Legacy {}

    impl SolEventInterface for Legacy {
        const NAME: &'static str = "Legacy";
        const COUNT: usize = 0;

        fn decode_raw_log(_topics: &[Word], _data: &[u8]) -> Result<Self> {
            Err(crate::Error::custom("legacy decoder"))
        }
    }

    #[test]
    fn legacy_interfaces_reject_custom_configs() {
        assert!(Legacy::decode_raw_log_with_config(&[], &[], AbiDecoderConfig::new()).is_err());
        assert!(
            Legacy::decode_raw_log_with_config(&[], &[], AbiDecoderConfig::new().strict(true))
                .is_err()
        );
    }
}
