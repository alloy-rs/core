use alloy_primitives::{Address, Bytes, B256, U256};
use serde::{Deserialize, Serialize};

/// <https://github.com/ethereum/go-ethereum/blob/91cb6f863a965481e51d5d9c0e5ccd54796fd967/eth/tracers/native/call.go#L44>
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallFrame {
    #[serde(rename = "type")]
    pub typ: String,
    pub from: Address,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    #[serde(default)]
    pub gas: U256,
    #[serde(default, rename = "gasUsed")]
    pub gas_used: U256,
    pub input: Bytes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Bytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calls: Option<Vec<CallFrame>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<CallLogFrame>>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallLogFrame {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<B256>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallConfig {
    /// When set to true, this will only trace the primary (top-level) call and
    /// not any sub-calls. It eliminates the additional processing for each
    /// call frame
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_top_call: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_log: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::geth::*;

    // See <https://github.com/ethereum/go-ethereum/tree/master/eth/tracers/internal/tracetest/testdata>
    const DEFAULT: &str = include_str!("../../../../test_data/call_tracer/default.json");
    const LEGACY: &str = include_str!("../../../../test_data/call_tracer/legacy.json");
    const ONLY_TOP_CALL: &str =
        include_str!("../../../../test_data/call_tracer/only_top_call.json");
    const WITH_LOG: &str = include_str!("../../../../test_data/call_tracer/with_log.json");

    #[test]
    fn test_serialize_call_trace() {
        let mut opts = GethDebugTracingCallOptions::default();
        opts.tracing_options.config.disable_storage = Some(false);
        opts.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
            GethDebugBuiltInTracerType::CallTracer,
        ));
        opts.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
            GethDebugBuiltInTracerConfig::CallTracer(CallConfig {
                only_top_call: Some(true),
                with_log: Some(true),
            }),
        ));

        assert_eq!(
            serde_json::to_string(&opts).unwrap(),
            r#"{"disableStorage":false,"tracer":"callTracer","tracerConfig":{"onlyTopCall":true,"withLog":true}}"#
        );
    }

    #[test]
    fn test_deserialize_call_trace() {
        let _trace: CallFrame = serde_json::from_str(DEFAULT).unwrap();
        let _trace: CallFrame = serde_json::from_str(LEGACY).unwrap();
        let _trace: CallFrame = serde_json::from_str(ONLY_TOP_CALL).unwrap();
        let trace: CallFrame = serde_json::from_str(WITH_LOG).unwrap();
        let _logs = trace.logs.unwrap();
    }
}
