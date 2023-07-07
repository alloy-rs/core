use super::typed::{
    EIP1559TransactionRequest, EIP2930TransactionRequest, LegacyTransactionRequest,
    TransactionKind, TypedTransactionRequest,
};
use crate::AccessList;
use alloy_primitives::{Address, Bytes, U128, U256, U8};
use serde::{Deserialize, Serialize};

/// Represents _all_ transaction requests received from RPC
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TransactionRequest {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, gas Price
    #[serde(default)]
    pub gas_price: Option<U128>,
    /// max base fee per gas sender is willing to pay
    #[serde(default)]
    pub max_fee_per_gas: Option<U128>,
    /// miner tip
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<U128>,
    /// gas
    pub gas: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    #[serde(alias = "input")]
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U256>,
    /// warm storage access pre-payment
    #[serde(default)]
    pub access_list: Option<AccessList>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U8>,
}

impl TransactionRequest {
    /// Converts the request into a [`TypedTransactionRequest`]
    ///
    /// Returns None if mutual exclusive fields `gasPrice` and `max_fee_per_gas`
    /// are either missing or both set.
    pub fn into_typed_request(self) -> Option<TypedTransactionRequest> {
        let TransactionRequest {
            to,
            gas_price,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            gas,
            value,
            data,
            nonce,
            mut access_list,
            ..
        } = self;
        match (gas_price, max_fee_per_gas, access_list.take()) {
            // legacy transaction
            (Some(_), None, None) => {
                Some(TypedTransactionRequest::Legacy(LegacyTransactionRequest {
                    nonce: nonce.unwrap_or(U256::ZERO),
                    gas_price: gas_price.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    value: value.unwrap_or(U256::ZERO),
                    input: data.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    chain_id: None,
                }))
            }
            // EIP2930
            (_, None, Some(access_list)) => Some(TypedTransactionRequest::EIP2930(
                EIP2930TransactionRequest {
                    nonce: nonce.unwrap_or(U256::ZERO),
                    gas_price: gas_price.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    value: value.unwrap_or(U256::ZERO),
                    input: data.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    chain_id: 0,
                    access_list,
                },
            )),
            // EIP1559
            (None, Some(_), access_list) | (None, None, access_list @ None) => {
                // Empty fields fall back to the canonical transaction schema.
                Some(TypedTransactionRequest::EIP1559(
                    EIP1559TransactionRequest {
                        nonce: nonce.unwrap_or(U256::ZERO),
                        max_fee_per_gas: max_fee_per_gas.unwrap_or_default(),
                        max_priority_fee_per_gas: max_priority_fee_per_gas.unwrap_or(U128::ZERO),
                        gas_limit: gas.unwrap_or_default(),
                        value: value.unwrap_or(U256::ZERO),
                        input: data.unwrap_or_default(),
                        kind: match to {
                            Some(to) => TransactionKind::Call(to),
                            None => TransactionKind::Create,
                        },
                        chain_id: 0,
                        access_list: access_list.unwrap_or_default(),
                    },
                ))
            }
            _ => None,
        }
    }
}
