use bytes::{Buf, BytesMut};
use derive_more::{AsRef, Deref};
use ethers_primitives::{keccak256, Address, Bytes, ChainId, TxHash, B256};
use ethers_rlp::{
    length_of_length, Decodable, DecodeError, Encodable, Header, EMPTY_LIST_CODE, EMPTY_STRING_CODE,
};
use serde::{Deserialize, Serialize};

mod access_list;
mod error;
mod meta;
mod signature;
mod tx_type;

pub use access_list::{AccessList, AccessListItem, AccessListWithGasUsed};
pub use error::InvalidTransactionError;
pub use meta::TransactionMeta;
pub use signature::Signature;
pub use tx_type::{TxType, EIP1559_TX_TYPE_ID, EIP2930_TX_TYPE_ID, LEGACY_TX_TYPE_ID};

/// Legacy transaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct TxLegacy {
    /// Added as EIP-155: Simple replay attack protection
    pub chain_id: Option<ChainId>,
    /// A scalar value equal to the number of transactions sent by the sender;
    /// formally Tn.
    pub nonce: u64,
    /// A scalar value equal to the number of
    /// Wei to be paid per unit of gas for all computation
    /// costs incurred as a result of the execution of this transaction;
    /// formally Tp.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub gas_price: u128,
    /// A scalar value equal to the maximum
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased
    /// later; formally Tg.
    pub gas_limit: u64,
    /// The 160-bit address of the message call’s recipient or, for a contract
    /// creation transaction, ∅, used here to denote the only member of B0 ;
    /// formally Tt.
    pub to: TransactionKind,
    /// A scalar value equal to the number of Wei to
    /// be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment
    /// to the newly created account; formally Tv.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub value: u128,
    /// Input has two uses depending if transaction is Create or Call (if `to`
    /// field is None or Some). pub init: An unlimited size byte array
    /// specifying the EVM-code for the account initialisation procedure
    /// CREATE, data: An unlimited size byte array specifying the
    /// input data of the message call, formally Td.
    pub input: Bytes,
}

/// Transaction with an [`AccessList`] ([EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct TxEip2930 {
    /// Added as EIP-pub 155: Simple replay attack protection
    pub chain_id: ChainId,
    /// A scalar value equal to the number of transactions sent by the sender;
    /// formally Tn.
    pub nonce: u64,
    /// A scalar value equal to the number of
    /// Wei to be paid per unit of gas for all computation
    /// costs incurred as a result of the execution of this transaction;
    /// formally Tp.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub gas_price: u128,
    /// A scalar value equal to the maximum
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased
    /// later; formally Tg.
    pub gas_limit: u64,
    /// The 160-bit address of the message call’s recipient or, for a contract
    /// creation transaction, ∅, used here to denote the only member of B0 ;
    /// formally Tt.
    pub to: TransactionKind,
    /// A scalar value equal to the number of Wei to
    /// be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment
    /// to the newly created account; formally Tv.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub value: u128,
    /// The accessList specifies a list of addresses and storage keys;
    /// these addresses and storage keys are added into the `accessed_addresses`
    /// and `accessed_storage_keys` global sets (introduced in EIP-2929).
    /// A gas cost is charged, though at a discount relative to the cost of
    /// accessing outside the list.
    pub access_list: AccessList,
    /// Input has two uses depending if transaction is Create or Call (if `to`
    /// field is None or Some). pub init: An unlimited size byte array
    /// specifying the EVM-code for the account initialisation procedure
    /// CREATE, data: An unlimited size byte array specifying the
    /// input data of the message call, formally Td.
    pub input: Bytes,
}

/// A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub struct TxEip1559 {
    /// Added as EIP-pub 155: Simple replay attack protection
    pub chain_id: u64,
    /// A scalar value equal to the number of transactions sent by the sender;
    /// formally Tn.
    pub nonce: u64,
    /// A scalar value equal to the maximum
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased
    /// later; formally Tg.
    pub gas_limit: u64,
    /// A scalar value equal to the maximum
    /// amount of gas that should be used in executing
    /// this transaction. This is paid up-front, before any
    /// computation is done and may not be increased
    /// later; formally Tg.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub max_fee_per_gas: u128,
    /// Max Priority fee that transaction is paying
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub max_priority_fee_per_gas: u128,
    /// The 160-bit address of the message call’s recipient or, for a contract
    /// creation transaction, ∅, used here to denote the only member of B0 ;
    /// formally Tt.
    pub to: TransactionKind,
    /// A scalar value equal to the number of Wei to
    /// be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment
    /// to the newly created account; formally Tv.
    ///
    /// As ethereum circulation is around 120mil eth as of 2022 that is around
    /// 120000000000000000000000000 wei we are safe to use u128 as its max
    /// number is: 340282366920938463463374607431768211455
    pub value: u128,
    /// The accessList specifies a list of addresses and storage keys;
    /// these addresses and storage keys are added into the `accessed_addresses`
    /// and `accessed_storage_keys` global sets (introduced in EIP-2929).
    /// A gas cost is charged, though at a discount relative to the cost of
    /// accessing outside the list.
    pub access_list: AccessList,
    /// Input has two uses depending if transaction is Create or Call (if `to`
    /// field is None or Some). pub init: An unlimited size byte array
    /// specifying the EVM-code for the account initialisation procedure
    /// CREATE, data: An unlimited size byte array specifying the
    /// input data of the message call, formally Td.
    pub input: Bytes,
}

/// A raw transaction.
///
/// Transaction types were introduced in [EIP-2718](https://eips.ethereum.org/EIPS/eip-2718).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub enum Transaction {
    /// Legacy transaction.
    Legacy(TxLegacy),
    /// Transaction with an [`AccessList`] ([EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)).
    Eip2930(TxEip2930),
    /// A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)).
    Eip1559(TxEip1559),
}

impl Transaction {
    /// This encodes the transaction _without_ the signature, and is only
    /// suitable for creating a hash intended for signing.
    pub fn encode_without_signature(&self, out: &mut dyn bytes::BufMut) {
        Encodable::encode(self, out);
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait
    /// and for calculating hash that for eip2718 does not require rlp
    /// header
    pub fn encode_with_signature(
        &self,
        signature: &Signature,
        out: &mut dyn bytes::BufMut,
        with_header: bool,
    ) {
        match self {
            Transaction::Legacy(TxLegacy { chain_id, .. }) => {
                // do nothing w/ with_header
                let payload_length =
                    self.fields_len() + signature.payload_len_with_eip155_chain_id(*chain_id);
                let header = Header {
                    list: true,
                    payload_length,
                };
                header.encode(out);
                self.encode_fields(out);
                signature.encode_with_eip155_chain_id(out, *chain_id);
            }
            _ => {
                let payload_length = self.fields_len() + signature.payload_len();
                if with_header {
                    Header {
                        list: false,
                        payload_length: 1 + length_of_length(payload_length) + payload_length,
                    }
                    .encode(out);
                }
                out.put_u8(self.tx_type() as u8);
                let header = Header {
                    list: true,
                    payload_length,
                };
                header.encode(out);
                self.encode_fields(out);
                signature.encode(out);
            }
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::Legacy(TxLegacy::default())
    }
}

/// This encodes the transaction _without_ the signature, and is only suitable
/// for creating a hash intended for signing.
impl Encodable for Transaction {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Transaction::Legacy { .. } => {
                Header {
                    list: true,
                    payload_length: self.fields_len() + self.eip155_fields_len(),
                }
                .encode(out);
                self.encode_fields(out);
                self.encode_eip155_fields(out);
            }
            _ => {
                out.put_u8(self.tx_type() as u8);
                Header {
                    list: true,
                    payload_length: self.fields_len(),
                }
                .encode(out);
                self.encode_fields(out);
            }
        }
    }

    fn length(&self) -> usize {
        match self {
            Transaction::Legacy { .. } => {
                let payload_length = self.fields_len() + self.eip155_fields_len();
                // 'header length' + 'payload length'
                length_of_length(payload_length) + payload_length
            }
            _ => {
                let payload_length = self.fields_len();
                // 'transaction type byte length' + 'header length' + 'payload length'
                1 + length_of_length(payload_length) + payload_length
            }
        }
    }
}

impl Transaction {
    /// Returns the signature hash of the RLP-encoded transaction.
    ///
    /// It is only useful for signature signing or signer recovery.
    pub fn signature_hash(&self) -> B256 {
        let mut buf = BytesMut::new();
        self.encode(&mut buf);
        keccak256(&buf)
    }

    /// Get chain_id.
    pub fn chain_id(&self) -> Option<u64> {
        match self {
            Transaction::Legacy(TxLegacy { chain_id, .. }) => *chain_id,
            Transaction::Eip2930(TxEip2930 { chain_id, .. }) => Some(*chain_id),
            Transaction::Eip1559(TxEip1559 { chain_id, .. }) => Some(*chain_id),
        }
    }

    /// Sets the transaction's chain id to the provided value.
    pub fn set_chain_id(&mut self, chain_id: u64) {
        match self {
            Transaction::Legacy(TxLegacy {
                chain_id: ref mut c,
                ..
            }) => *c = Some(chain_id),
            Transaction::Eip2930(TxEip2930 {
                chain_id: ref mut c,
                ..
            }) => *c = chain_id,
            Transaction::Eip1559(TxEip1559 {
                chain_id: ref mut c,
                ..
            }) => *c = chain_id,
        }
    }

    /// Gets the transaction's [`TransactionKind`], which is the address of the
    /// recipient or [`TransactionKind::Create`] if the transaction is a
    /// contract creation.
    pub fn kind(&self) -> &TransactionKind {
        match self {
            Transaction::Legacy(TxLegacy { to, .. })
            | Transaction::Eip2930(TxEip2930 { to, .. })
            | Transaction::Eip1559(TxEip1559 { to, .. }) => to,
        }
    }

    /// Get the transaction's nonce.
    pub fn to(&self) -> Option<Address> {
        self.kind().to()
    }

    /// Get transaction type
    pub fn tx_type(&self) -> TxType {
        match self {
            Transaction::Legacy { .. } => TxType::Legacy,
            Transaction::Eip2930 { .. } => TxType::EIP2930,
            Transaction::Eip1559 { .. } => TxType::EIP1559,
        }
    }

    /// Gets the transaction's value field.
    pub fn value(&self) -> u128 {
        *match self {
            Transaction::Legacy(TxLegacy { value, .. }) => value,
            Transaction::Eip2930(TxEip2930 { value, .. }) => value,
            Transaction::Eip1559(TxEip1559 { value, .. }) => value,
        }
    }

    /// Get the transaction's nonce.
    pub fn nonce(&self) -> u64 {
        match self {
            Transaction::Legacy(TxLegacy { nonce, .. }) => *nonce,
            Transaction::Eip2930(TxEip2930 { nonce, .. }) => *nonce,
            Transaction::Eip1559(TxEip1559 { nonce, .. }) => *nonce,
        }
    }

    /// Get the gas limit of the transaction.
    pub fn gas_limit(&self) -> u64 {
        match self {
            Transaction::Legacy(TxLegacy { gas_limit, .. })
            | Transaction::Eip2930(TxEip2930 { gas_limit, .. })
            | Transaction::Eip1559(TxEip1559 { gas_limit, .. }) => *gas_limit,
        }
    }

    /// Max fee per gas for eip1559 transaction, for legacy transactions this is
    /// gas_price.
    ///
    /// This is also commonly referred to as the "Gas Fee Cap" (`GasFeeCap`).
    pub fn max_fee_per_gas(&self) -> u128 {
        match self {
            Transaction::Legacy(TxLegacy { gas_price, .. })
            | Transaction::Eip2930(TxEip2930 { gas_price, .. }) => *gas_price,
            Transaction::Eip1559(TxEip1559 {
                max_fee_per_gas, ..
            }) => *max_fee_per_gas,
        }
    }

    /// Max priority fee per gas for eip1559 transaction, for legacy and eip2930
    /// transactions this is `None`
    ///
    /// This is also commonly referred to as the "Gas Tip Cap" (`GasTipCap`).
    pub fn max_priority_fee_per_gas(&self) -> Option<u128> {
        match self {
            Transaction::Legacy(_) => None,
            Transaction::Eip2930(_) => None,
            Transaction::Eip1559(TxEip1559 {
                max_priority_fee_per_gas,
                ..
            }) => Some(*max_priority_fee_per_gas),
        }
    }

    /// Return the max priority fee per gas if the transaction is an EIP-1559
    /// transaction, and otherwise return the gas price.
    ///
    /// # Warning
    ///
    /// This is different than the `max_priority_fee_per_gas` method, which
    /// returns `None` for non-EIP-1559 transactions.
    pub(crate) fn priority_fee_or_price(&self) -> u128 {
        match self {
            Transaction::Legacy(TxLegacy { gas_price, .. })
            | Transaction::Eip2930(TxEip2930 { gas_price, .. }) => *gas_price,
            Transaction::Eip1559(TxEip1559 {
                max_priority_fee_per_gas,
                ..
            }) => *max_priority_fee_per_gas,
        }
    }

    /// Returns the effective gas price for the given base fee.
    ///
    /// If the transaction is a legacy or EIP2930 transaction, the gas price is
    /// returned.
    pub fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        let dynamic_tx = match self {
            Transaction::Legacy(tx) => return tx.gas_price,
            Transaction::Eip2930(tx) => return tx.gas_price,
            Transaction::Eip1559(dynamic_tx) => dynamic_tx,
        };

        dynamic_tx.effective_gas_price(base_fee)
    }

    // TODO: dedup with effective_tip_per_gas
    /// Determine the effective gas limit for the given transaction and base
    /// fee. If the base fee is `None`, the `max_priority_fee_per_gas`, or
    /// gas price for non-EIP1559 transactions is returned.
    ///
    /// If the `max_fee_per_gas` is less than the base fee, `None` returned.
    pub fn effective_gas_tip(&self, base_fee: Option<u64>) -> Option<u128> {
        if let Some(base_fee) = base_fee {
            let max_fee_per_gas = self.max_fee_per_gas();
            if max_fee_per_gas < base_fee as u128 {
                None
            } else {
                let effective_max_fee = max_fee_per_gas - base_fee as u128;
                Some(std::cmp::min(
                    effective_max_fee,
                    self.priority_fee_or_price(),
                ))
            }
        } else {
            Some(self.priority_fee_or_price())
        }
    }

    /// Returns the effective miner gas tip cap (`gasTipCap`) for the given base
    /// fee: `min(maxFeePerGas - baseFee, maxPriorityFeePerGas)`
    ///
    /// Returns `None` if the basefee is higher than the
    /// [Transaction::max_fee_per_gas].
    pub fn effective_tip_per_gas(&self, base_fee: u64) -> Option<u128> {
        let base_fee = base_fee as u128;
        let max_fee_per_gas = self.max_fee_per_gas();

        if max_fee_per_gas < base_fee {
            return None
        }

        // the miner tip is the difference between the max fee and the base fee or the
        // max_priority_fee_per_gas, whatever is lower

        // SAFETY: max_fee_per_gas >= base_fee
        let fee = max_fee_per_gas - base_fee;

        if let Some(priority_fee) = self.max_priority_fee_per_gas() {
            return Some(fee.min(priority_fee))
        }

        Some(fee)
    }

    /// Get the transaction's input field.
    pub fn input(&self) -> &Bytes {
        match self {
            Transaction::Legacy(TxLegacy { input, .. }) => input,
            Transaction::Eip2930(TxEip2930 { input, .. }) => input,
            Transaction::Eip1559(TxEip1559 { input, .. }) => input,
        }
    }

    /// Encodes EIP-155 arguments into the desired buffer. Only encodes values
    /// for legacy transactions.
    pub(crate) fn encode_eip155_fields(&self, out: &mut dyn bytes::BufMut) {
        // if this is a legacy transaction without a chain ID, it must be pre-EIP-155
        // and does not need to encode the chain ID for the signature hash encoding
        if let Transaction::Legacy(TxLegacy {
            chain_id: Some(id), ..
        }) = self
        {
            // EIP-155 encodes the chain ID and two zeroes
            id.encode(out);
            0x00u8.encode(out);
            0x00u8.encode(out);
        }
    }

    /// Outputs the length of EIP-155 fields. Only outputs a non-zero value for
    /// EIP-155 legacy transactions.
    pub(crate) fn eip155_fields_len(&self) -> usize {
        if let Transaction::Legacy(TxLegacy {
            chain_id: Some(id), ..
        }) = self
        {
            // EIP-155 encodes the chain ID and two zeroes, so we add 2 to the length of the
            // chain ID to get the length of all 3 fields
            // len(chain_id) + (0x00) + (0x00)
            id.length() + 2
        } else {
            // this is either a pre-EIP-155 legacy transaction or a typed transaction
            0
        }
    }

    /// Outputs the length of the transaction's fields, without a RLP header or
    /// length of the eip155 fields.
    pub(crate) fn fields_len(&self) -> usize {
        match self {
            Transaction::Legacy(TxLegacy {
                chain_id: _,
                nonce,
                gas_price,
                gas_limit,
                to,
                value,
                input,
            }) => {
                let mut len = 0;
                len += nonce.length();
                len += gas_price.length();
                len += gas_limit.length();
                len += to.length();
                len += value.length();
                len += input.0.length();
                len
            }
            Transaction::Eip2930(TxEip2930 {
                chain_id,
                nonce,
                gas_price,
                gas_limit,
                to,
                value,
                input,
                access_list,
            }) => {
                let mut len = 0;
                len += chain_id.length();
                len += nonce.length();
                len += gas_price.length();
                len += gas_limit.length();
                len += to.length();
                len += value.length();
                len += input.0.length();
                len += access_list.length();
                len
            }
            Transaction::Eip1559(TxEip1559 {
                chain_id,
                nonce,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                to,
                value,
                input,
                access_list,
            }) => {
                let mut len = 0;
                len += chain_id.length();
                len += nonce.length();
                len += max_priority_fee_per_gas.length();
                len += max_fee_per_gas.length();
                len += gas_limit.length();
                len += to.length();
                len += value.length();
                len += input.0.length();
                len += access_list.length();
                len
            }
        }
    }

    /// Encodes only the transaction's fields into the desired buffer, without a
    /// RLP header.
    pub(crate) fn encode_fields(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Transaction::Legacy(TxLegacy {
                chain_id: _,
                nonce,
                gas_price,
                gas_limit,
                to,
                value,
                input,
            }) => {
                nonce.encode(out);
                gas_price.encode(out);
                gas_limit.encode(out);
                to.encode(out);
                value.encode(out);
                input.0.encode(out);
            }
            Transaction::Eip2930(TxEip2930 {
                chain_id,
                nonce,
                gas_price,
                gas_limit,
                to,
                value,
                input,
                access_list,
            }) => {
                chain_id.encode(out);
                nonce.encode(out);
                gas_price.encode(out);
                gas_limit.encode(out);
                to.encode(out);
                value.encode(out);
                input.0.encode(out);
                access_list.encode(out);
            }
            Transaction::Eip1559(TxEip1559 {
                chain_id,
                nonce,
                gas_limit,
                max_fee_per_gas,
                max_priority_fee_per_gas,
                to,
                value,
                input,
                access_list,
            }) => {
                chain_id.encode(out);
                nonce.encode(out);
                max_priority_fee_per_gas.encode(out);
                max_fee_per_gas.encode(out);
                gas_limit.encode(out);
                to.encode(out);
                value.encode(out);
                input.encode(out);
                access_list.encode(out);
            }
        }
    }
}

impl TxEip1559 {
    /// Returns the effective gas price for the given `base_fee`.
    pub fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        match base_fee {
            None => self.max_fee_per_gas,
            Some(base_fee) => {
                // if the tip is greater than the max priority fee per gas, set it to the max
                // priority fee per gas + base fee
                let tip = self.max_fee_per_gas - base_fee as u128;
                if tip > self.max_priority_fee_per_gas {
                    self.max_priority_fee_per_gas + base_fee as u128
                } else {
                    // otherwise return the max fee per gas
                    self.max_fee_per_gas
                }
            }
        }
    }
}

/// Whether or not the transaction is a contract creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[cfg_attr(
    any(test, feature = "arbitrary"),
    derive(arbitrary::Arbitrary, proptest_derive::Arbitrary)
)]
pub enum TransactionKind {
    /// A transaction that creates a contract.
    #[default]
    Create,
    /// A transaction that calls a contract or transfer.
    Call(Address),
}

impl TransactionKind {
    /// Returns the address of the contract that will be called or will receive
    /// the transfer.
    pub fn to(self) -> Option<Address> {
        match self {
            TransactionKind::Create => None,
            TransactionKind::Call(to) => Some(to),
        }
    }
}

impl Encodable for TransactionKind {
    fn encode(&self, out: &mut dyn ethers_rlp::BufMut) {
        match self {
            TransactionKind::Call(to) => to.encode(out),
            TransactionKind::Create => out.put_u8(EMPTY_STRING_CODE),
        }
    }
    fn length(&self) -> usize {
        match self {
            TransactionKind::Call(to) => to.length(),
            TransactionKind::Create => 1, // EMPTY_STRING_CODE is a single byte
        }
    }
}

impl Decodable for TransactionKind {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        if let Some(&first) = buf.first() {
            if first == EMPTY_STRING_CODE {
                buf.advance(1);
                Ok(TransactionKind::Create)
            } else {
                let addr = <Address as Decodable>::decode(buf)?;
                Ok(TransactionKind::Call(addr))
            }
        } else {
            Err(DecodeError::InputTooShort)
        }
    }
}

/// Signed transaction without its Hash. Used type for inserting into the DB.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Deref, Default, Serialize, Deserialize)]
pub struct TransactionSignedNoHash {
    /// The transaction signature values
    pub signature: Signature,
    /// Raw transaction info
    #[deref]
    #[as_ref]
    pub transaction: Transaction,
}

impl TransactionSignedNoHash {
    /// Calculates the transaction hash. If used more than once, it's better to
    /// convert it to [`TransactionSigned`] first.
    pub fn hash(&self) -> B256 {
        let mut buf = Vec::new();
        self.transaction
            .encode_with_signature(&self.signature, &mut buf, false);
        keccak256(&buf)
    }

    /// Converts into a transaction type with its hash: [`TransactionSigned`].
    pub fn with_hash(self) -> TransactionSigned {
        self.into()
    }
}

impl From<TransactionSignedNoHash> for TransactionSigned {
    fn from(tx: TransactionSignedNoHash) -> Self {
        TransactionSigned::from_transaction_and_signature(tx.transaction, tx.signature)
    }
}

impl From<TransactionSigned> for TransactionSignedNoHash {
    fn from(tx: TransactionSigned) -> Self {
        TransactionSignedNoHash {
            signature: tx.signature,
            transaction: tx.transaction,
        }
    }
}

/// Signed transaction.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Deref, Default, Serialize, Deserialize)]
pub struct TransactionSigned {
    /// Transaction hash
    pub hash: TxHash,
    /// The transaction signature values
    pub signature: Signature,
    /// Raw transaction info
    #[deref]
    #[as_ref]
    pub transaction: Transaction,
}

impl AsRef<Self> for TransactionSigned {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl TransactionSigned {
    /// Transaction signature.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Transaction hash. Used to identify transaction.
    pub fn hash(&self) -> TxHash {
        self.hash
    }

    /// Reference to transaction hash. Used to identify transaction.
    pub fn hash_ref(&self) -> &TxHash {
        &self.hash
    }

    /// Recover signer from signature and hash.
    ///
    /// Returns `None` if the transaction's signature is invalid.
    pub fn recover_signer(&self) -> Option<Address> {
        let signature_hash = self.signature_hash();
        self.signature.recover_signer(signature_hash)
    }

    /// Devour Self, recover signer and return [`TransactionSignedEcRecovered`]
    ///
    /// Returns `None` if the transaction's signature is invalid, see also
    /// [TransactionSigned::recover_signer].
    pub fn into_ecrecovered(self) -> Option<TransactionSignedEcRecovered> {
        let signer = self.recover_signer()?;
        Some(TransactionSignedEcRecovered {
            signed_transaction: self,
            signer,
        })
    }

    /// try to recover signer and return [`TransactionSignedEcRecovered`]
    pub fn try_ecrecovered(&self) -> Option<TransactionSignedEcRecovered> {
        let signer = self.recover_signer()?;
        Some(TransactionSignedEcRecovered {
            signed_transaction: self.clone(),
            signer,
        })
    }

    /// Returns the enveloped encoded transactions.
    ///
    /// See also [TransactionSigned::encode_enveloped]
    pub fn envelope_encoded(&self) -> Bytes {
        let mut buf = BytesMut::new();
        self.encode_enveloped(&mut buf);
        buf.freeze().into()
    }

    /// Encodes the transaction into the "raw" format (e.g.
    /// `eth_sendRawTransaction`). This format is also referred to as
    /// "binary" encoding.
    ///
    /// For legacy transactions, it encodes the RLP of the transaction into the
    /// buffer: `rlp(tx)` For EIP-2718 typed it encodes the type of the
    /// transaction followed by the rlp of the transaction: `type` +
    /// `rlp(tx)`
    pub fn encode_enveloped(&self, out: &mut dyn bytes::BufMut) {
        self.encode_inner(out, false)
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait
    /// and for calculating hash that for eip2718 does not require rlp
    /// header
    pub(crate) fn encode_inner(&self, out: &mut dyn bytes::BufMut, with_header: bool) {
        self.transaction
            .encode_with_signature(&self.signature, out, with_header);
    }

    /// Output the length of the encode_inner(out, true). Note to assume that
    /// `with_header` is only `true`.
    pub(crate) fn payload_len_inner(&self) -> usize {
        match self.transaction {
            Transaction::Legacy(TxLegacy { chain_id, .. }) => {
                let payload_length = self.transaction.fields_len()
                    + self.signature.payload_len_with_eip155_chain_id(chain_id);
                // 'header length' + 'payload length'
                length_of_length(payload_length) + payload_length
            }
            _ => {
                let payload_length = self.transaction.fields_len() + self.signature.payload_len();
                // 'transaction type byte length' + 'header length' + 'payload length'
                let len = 1 + length_of_length(payload_length) + payload_length;
                length_of_length(len) + len
            }
        }
    }

    /// Calculate transaction hash, eip2728 transaction does not contain rlp
    /// header and start with tx type.
    pub fn recalculate_hash(&self) -> B256 {
        let mut buf = Vec::new();
        self.encode_inner(&mut buf, false);
        keccak256(&buf)
    }

    /// Create a new signed transaction from a transaction and its signature.
    /// This will also calculate the transaction hash using its encoding.
    pub fn from_transaction_and_signature(transaction: Transaction, signature: Signature) -> Self {
        let mut initial_tx = Self {
            transaction,
            hash: Default::default(),
            signature,
        };
        initial_tx.hash = initial_tx.recalculate_hash();
        initial_tx
    }

    /// Decodes legacy transaction from the data buffer.
    ///
    /// This expects `rlp(legacy_tx)`
    fn decode_rlp_legacy_transaction(data: &mut &[u8]) -> Result<TransactionSigned, DecodeError> {
        // keep this around, so we can use it to calculate the hash
        let original_encoding = *data;

        let header = Header::decode(data)?;

        let mut transaction = Transaction::Legacy(TxLegacy {
            nonce: Decodable::decode(data)?,
            gas_price: Decodable::decode(data)?,
            gas_limit: Decodable::decode(data)?,
            to: Decodable::decode(data)?,
            value: Decodable::decode(data)?,
            input: Bytes(Decodable::decode(data)?),
            chain_id: None,
        });
        let (signature, extracted_id) = Signature::decode_with_eip155_chain_id(data)?;
        if let Some(id) = extracted_id {
            transaction.set_chain_id(id);
        }

        let tx_length = header.payload_length + header.length();
        let hash = keccak256(&original_encoding[..tx_length]);
        let signed = TransactionSigned {
            transaction,
            hash,
            signature,
        };
        Ok(signed)
    }

    /// Decodes en enveloped EIP-2718 typed transaction.
    ///
    /// CAUTION: this expects that `data` is `[id, rlp(tx)]`
    fn decode_enveloped_typed_transaction(
        data: &mut &[u8],
    ) -> Result<TransactionSigned, DecodeError> {
        // keep this around so we can use it to calculate the hash
        let original_encoding = *data;

        let tx_type = *data.first().ok_or(DecodeError::InputTooShort)?;
        data.advance(1);
        // decode the list header for the rest of the transaction
        let header = Header::decode(data)?;
        if !header.list {
            return Err(DecodeError::Custom(
                "typed tx fields must be encoded as a list",
            ))
        }

        // length of tx encoding = tx type byte (size = 1) + length of header + payload
        // length
        let tx_length = 1 + header.length() + header.payload_length;

        // decode common fields
        let transaction = match tx_type {
            1 => Transaction::Eip2930(TxEip2930 {
                chain_id: Decodable::decode(data)?,
                nonce: Decodable::decode(data)?,
                gas_price: Decodable::decode(data)?,
                gas_limit: Decodable::decode(data)?,
                to: Decodable::decode(data)?,
                value: Decodable::decode(data)?,
                input: Bytes(Decodable::decode(data)?),
                access_list: Decodable::decode(data)?,
            }),
            2 => Transaction::Eip1559(TxEip1559 {
                chain_id: Decodable::decode(data)?,
                nonce: Decodable::decode(data)?,
                max_priority_fee_per_gas: Decodable::decode(data)?,
                max_fee_per_gas: Decodable::decode(data)?,
                gas_limit: Decodable::decode(data)?,
                to: Decodable::decode(data)?,
                value: Decodable::decode(data)?,
                input: Bytes(Decodable::decode(data)?),
                access_list: Decodable::decode(data)?,
            }),
            _ => return Err(DecodeError::Custom("unsupported typed transaction type")),
        };

        let signature = Signature::decode(data)?;

        let hash = keccak256(&original_encoding[..tx_length]);
        let signed = TransactionSigned {
            transaction,
            hash,
            signature,
        };
        Ok(signed)
    }

    /// Decodes the "raw" format of transaction (e.g. `eth_sendRawTransaction`).
    ///
    /// The raw transaction is either a legacy transaction or EIP-2718 typed
    /// transaction For legacy transactions, the format is encoded as:
    /// `rlp(tx)` For EIP-2718 typed transaction, the format is encoded as
    /// the type of the transaction followed by the rlp of the transaction:
    /// `type` + `rlp(tx)`
    pub fn decode_enveloped(tx: Bytes) -> Result<Self, DecodeError> {
        let mut data = tx.as_ref();

        if data.is_empty() {
            return Err(DecodeError::InputTooShort)
        }

        // Check if the tx is a list
        if data[0] >= EMPTY_LIST_CODE {
            // decode as legacy transaction
            TransactionSigned::decode_rlp_legacy_transaction(&mut data)
        } else {
            TransactionSigned::decode_enveloped_typed_transaction(&mut data)
        }
    }
}

impl From<TransactionSignedEcRecovered> for TransactionSigned {
    fn from(recovered: TransactionSignedEcRecovered) -> Self {
        recovered.signed_transaction
    }
}

impl Encodable for TransactionSigned {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.encode_inner(out, true);
    }

    fn length(&self) -> usize {
        self.payload_len_inner()
    }
}

/// This `Decodable` implementation only supports decoding rlp encoded
/// transactions as it's used by p2p.
///
/// CAUTION: this expects that the given buf contains rlp
impl Decodable for TransactionSigned {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        // decode header
        let mut original_encoding = *buf;
        let header = Header::decode(buf)?;

        // if the transaction is encoded as a string then it is a typed transaction
        if !header.list {
            TransactionSigned::decode_enveloped_typed_transaction(buf)
        } else {
            let tx = TransactionSigned::decode_rlp_legacy_transaction(&mut original_encoding)?;

            // advance the buffer based on how far `decode_rlp_legacy_transaction` advanced
            // the buffer
            *buf = original_encoding;
            Ok(tx)
        }
    }
}

#[cfg(any(test, feature = "arbitrary"))]
impl proptest::arbitrary::Arbitrary for TransactionSigned {
    type Parameters = ();
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::{any, Strategy};

        any::<(Transaction, Signature)>()
            .prop_map(move |(mut transaction, sig)| {
                if let Some(chain_id) = transaction.chain_id() {
                    // Otherwise we might overflow when calculating `v` on `recalculate_hash`
                    transaction.set_chain_id(chain_id % (u64::MAX / 2 - 36));
                }
                let mut tx = TransactionSigned {
                    hash: Default::default(),
                    signature: sig,
                    transaction,
                };
                tx.hash = tx.recalculate_hash();
                tx
            })
            .boxed()
    }

    type Strategy = proptest::strategy::BoxedStrategy<TransactionSigned>;
}

#[cfg(any(test, feature = "arbitrary"))]
impl<'a> arbitrary::Arbitrary<'a> for TransactionSigned {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut transaction = Transaction::arbitrary(u)?;
        if let Some(chain_id) = transaction.chain_id() {
            // Otherwise we might overflow when calculating `v` on `recalculate_hash`
            transaction.set_chain_id(chain_id % (u64::MAX / 2 - 36));
        }

        let mut tx = TransactionSigned {
            hash: Default::default(),
            signature: Signature::arbitrary(u)?,
            transaction,
        };
        tx.hash = tx.recalculate_hash();

        Ok(tx)
    }
}

/// Signed transaction with recovered signer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Deref, Default)]
pub struct TransactionSignedEcRecovered {
    /// Signer of the transaction
    signer: Address,
    /// Signed transaction
    #[deref]
    #[as_ref]
    signed_transaction: TransactionSigned,
}

impl TransactionSignedEcRecovered {
    /// Signer of transaction recovered from signature
    pub fn signer(&self) -> Address {
        self.signer
    }

    /// Transform back to [`TransactionSigned`]
    pub fn into_signed(self) -> TransactionSigned {
        self.signed_transaction
    }

    /// Desolve Self to its component
    pub fn to_components(self) -> (TransactionSigned, Address) {
        (self.signed_transaction, self.signer)
    }

    /// Create [`TransactionSignedEcRecovered`] from [`TransactionSigned`] and
    /// [`Address`] of the signer.
    pub fn from_signed_transaction(signed_transaction: TransactionSigned, signer: Address) -> Self {
        Self {
            signed_transaction,
            signer,
        }
    }
}

impl Encodable for TransactionSignedEcRecovered {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.signed_transaction.encode(out)
    }

    fn length(&self) -> usize {
        self.signed_transaction.length()
    }
}

impl Decodable for TransactionSignedEcRecovered {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let signed_transaction = TransactionSigned::decode(buf)?;
        let signer = signed_transaction
            .recover_signer()
            .ok_or(DecodeError::Custom(
                "Unable to recover decoded transaction signer.",
            ))?;
        Ok(TransactionSignedEcRecovered {
            signer,
            signed_transaction,
        })
    }
}

/// A transaction type that can be created from a
/// [`TransactionSignedEcRecovered`] transaction.
///
/// This is a conversion trait that'll ensure transactions received via P2P can
/// be converted to the transaction type that the transaction pool uses.
pub trait FromRecoveredTransaction {
    /// Converts to this type from the given [`TransactionSignedEcRecovered`].
    fn from_recovered_transaction(tx: TransactionSignedEcRecovered) -> Self;
}

// Noop conversion
impl FromRecoveredTransaction for TransactionSignedEcRecovered {
    #[inline]
    fn from_recovered_transaction(tx: TransactionSignedEcRecovered) -> Self {
        tx
    }
}

/// The inverse of [`FromRecoveredTransaction`] that ensure the transaction can
/// be sent over the network
pub trait IntoRecoveredTransaction {
    /// Converts to this type into a [`TransactionSignedEcRecovered`].
    ///
    /// Note: this takes `&self` since indented usage is via `Arc<Self>`.
    fn to_recovered_transaction(&self) -> TransactionSignedEcRecovered;
}

impl IntoRecoveredTransaction for TransactionSignedEcRecovered {
    #[inline]
    fn to_recovered_transaction(&self) -> TransactionSignedEcRecovered {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccessList, TransactionSigned, TransactionSignedEcRecovered};
    use bytes::BytesMut;
    use ethers_primitives::{Address, Bytes, B256, U256};
    use ethers_rlp::{Decodable, DecodeError, Encodable};
    use hex_literal::hex;
    use std::str::FromStr;

    macro_rules! bytes {
        ($($l:literal)+) => {
            Bytes::from_static(&hex_literal::hex!($($l)+))
        };
    }

    #[test]
    fn test_decode_empty_typed_tx() {
        let input = [0x80u8];
        let res = TransactionSigned::decode(&mut &input[..]).unwrap_err();
        assert_eq!(DecodeError::InputTooShort, res);
    }

    #[test]
    fn test_decode_create() {
        // tests that a contract creation tx encodes and decodes properly
        let request = Transaction::Eip2930(TxEip2930 {
            chain_id: 1u64,
            nonce: 0,
            gas_price: 1,
            gas_limit: 2,
            to: TransactionKind::Create,
            value: 3,
            input: Bytes::from_static(&[1, 2]),
            access_list: Default::default(),
        });
        let signature = Signature {
            odd_y_parity: true,
            r: U256::default(),
            s: U256::default(),
        };
        let tx = TransactionSigned::from_transaction_and_signature(request, signature);

        let mut encoded = BytesMut::new();
        tx.encode(&mut encoded);
        assert_eq!(encoded.len(), tx.length());

        let decoded = TransactionSigned::decode(&mut &*encoded).unwrap();
        assert_eq!(decoded, tx);
    }

    #[test]
    fn test_decode_create_goerli() {
        // test that an example create tx from goerli decodes properly
        let tx_bytes = bytes!("b901f202f901ee05228459682f008459682f11830209bf8080b90195608060405234801561001057600080fd5b50610175806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80630c49c36c14610030575b600080fd5b61003861004e565b604051610045919061011d565b60405180910390f35b60606020600052600f6020527f68656c6c6f2073746174656d696e64000000000000000000000000000000000060405260406000f35b600081519050919050565b600082825260208201905092915050565b60005b838110156100be5780820151818401526020810190506100a3565b838111156100cd576000848401525b50505050565b6000601f19601f8301169050919050565b60006100ef82610084565b6100f9818561008f565b93506101098185602086016100a0565b610112816100d3565b840191505092915050565b6000602082019050818103600083015261013781846100e4565b90509291505056fea264697066735822122051449585839a4ea5ac23cae4552ef8a96b64ff59d0668f76bfac3796b2bdbb3664736f6c63430008090033c080a0136ebffaa8fc8b9fda9124de9ccb0b1f64e90fbd44251b4c4ac2501e60b104f9a07eb2999eec6d185ef57e91ed099afb0a926c5b536f0155dd67e537c7476e1471");

        let decoded = TransactionSigned::decode(&mut &tx_bytes[..]).unwrap();
        assert_eq!(tx_bytes.len(), decoded.length());

        let mut encoded = BytesMut::new();
        decoded.encode(&mut encoded);

        assert_eq!(tx_bytes, Bytes::from(encoded.freeze()));
    }

    #[test]
    fn test_decode_call() {
        let request = Transaction::Eip2930(TxEip2930 {
            chain_id: 1u64,
            nonce: 0,
            gas_price: 1,
            gas_limit: 2,
            to: TransactionKind::Call(Address::default()),
            value: 3,
            input: Bytes::from_static(&[1, 2]),
            access_list: Default::default(),
        });

        let signature = Signature {
            odd_y_parity: true,
            r: U256::default(),
            s: U256::default(),
        };

        let tx = TransactionSigned::from_transaction_and_signature(request, signature);

        let mut encoded = BytesMut::new();
        tx.encode(&mut encoded);
        assert_eq!(encoded.len(), tx.length());

        let decoded = TransactionSigned::decode(&mut &*encoded).unwrap();
        assert_eq!(decoded, tx);
    }

    #[test]
    fn decode_transaction_consumes_buffer() {
        let bytes = &mut &hex::decode("b87502f872041a8459682f008459682f0d8252089461815774383099e24810ab832a5b2a5425c154d58829a2241af62c000080c001a059e6b67f48fb32e7e570dfb11e042b5ad2e55e3ce3ce9cd989c7e06e07feeafda0016b83f4f980694ed2eee4d10667242b1f40dc406901b34125b008d334d47469").unwrap()[..];
        let _transaction_res = TransactionSigned::decode(bytes).unwrap();
        assert_eq!(
            bytes.len(),
            0,
            "did not consume all bytes in the buffer, {:?} remaining",
            bytes.len()
        );
    }

    #[test]
    fn decode_multiple_network_txs() {
        let bytes = hex::decode("f86b02843b9aca00830186a094d3e8763675e4c425df46cc3b5c0f6cbdac39604687038d7ea4c68000802ba00eb96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5aea03a456401896b1b6055311536bf00a718568c744d8c1f9df59879e8350220ca18").unwrap();
        let transaction = Transaction::Legacy(TxLegacy {
            chain_id: Some(4u64),
            nonce: 2,
            gas_price: 1000000000,
            gas_limit: 100000,
            to: TransactionKind::Call(
                Address::from_str("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap(),
            ),
            value: 1000000000000000,
            input: Bytes::default(),
        });
        let signature = Signature {
            odd_y_parity: false,
            r: U256::from_str("0xeb96ca19e8a77102767a41fc85a36afd5c61ccb09911cec5d3e86e193d9c5ae")
                .unwrap(),
            s: U256::from_str("0x3a456401896b1b6055311536bf00a718568c744d8c1f9df59879e8350220ca18")
                .unwrap(),
        };
        let hash =
            B256::from_str("0xa517b206d2223278f860ea017d3626cacad4f52ff51030dc9a96b432f17f8d34")
                .ok();
        test_decode_and_encode(bytes, transaction, signature, hash);

        let bytes = hex::decode("f86b01843b9aca00830186a094d3e8763675e4c425df46cc3b5c0f6cbdac3960468702769bb01b2a00802ba0e24d8bd32ad906d6f8b8d7741e08d1959df021698b19ee232feba15361587d0aa05406ad177223213df262cb66ccbb2f46bfdccfdfbbb5ffdda9e2c02d977631da").unwrap();
        let transaction = Transaction::Legacy(TxLegacy {
            chain_id: Some(4),
            nonce: 1u64,
            gas_price: 1000000000,
            gas_limit: 100000u64,
            to: TransactionKind::Call(Address::from_slice(
                &hex::decode("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 693361000000000u64.into(),
            input: Default::default(),
        });
        let signature = Signature {
            odd_y_parity: false,
            r: U256::from_str("0xe24d8bd32ad906d6f8b8d7741e08d1959df021698b19ee232feba15361587d0a")
                .unwrap(),
            s: U256::from_str("0x5406ad177223213df262cb66ccbb2f46bfdccfdfbbb5ffdda9e2c02d977631da")
                .unwrap(),
        };
        test_decode_and_encode(bytes, transaction, signature, None);

        let bytes = hex::decode("f86b0384773594008398968094d3e8763675e4c425df46cc3b5c0f6cbdac39604687038d7ea4c68000802ba0ce6834447c0a4193c40382e6c57ae33b241379c5418caac9cdc18d786fd12071a03ca3ae86580e94550d7c071e3a02eadb5a77830947c9225165cf9100901bee88").unwrap();
        let transaction = Transaction::Legacy(TxLegacy {
            chain_id: Some(4),
            nonce: 3,
            gas_price: 2000000000,
            gas_limit: 10000000,
            to: TransactionKind::Call(Address::from_slice(
                &hex::decode("d3e8763675e4c425df46cc3b5c0f6cbdac396046").unwrap()[..],
            )),
            value: 1000000000000000u64.into(),
            input: Bytes::default(),
        });
        let signature = Signature {
            odd_y_parity: false,
            r: U256::from_str("0xce6834447c0a4193c40382e6c57ae33b241379c5418caac9cdc18d786fd12071")
                .unwrap(),
            s: U256::from_str("0x3ca3ae86580e94550d7c071e3a02eadb5a77830947c9225165cf9100901bee88")
                .unwrap(),
        };
        test_decode_and_encode(bytes, transaction, signature, None);

        let bytes = hex::decode("b87502f872041a8459682f008459682f0d8252089461815774383099e24810ab832a5b2a5425c154d58829a2241af62c000080c001a059e6b67f48fb32e7e570dfb11e042b5ad2e55e3ce3ce9cd989c7e06e07feeafda0016b83f4f980694ed2eee4d10667242b1f40dc406901b34125b008d334d47469").unwrap();
        let transaction = Transaction::Eip1559(TxEip1559 {
            chain_id: 4,
            nonce: 26,
            max_priority_fee_per_gas: 1500000000,
            max_fee_per_gas: 1500000013,
            gas_limit: 21000,
            to: TransactionKind::Call(Address::from_slice(
                &hex::decode("61815774383099e24810ab832a5b2a5425c154d5").unwrap()[..],
            )),
            value: 3000000000000000000u64.into(),
            input: Default::default(),
            access_list: Default::default(),
        });
        let signature = Signature {
            odd_y_parity: true,
            r: U256::from_str("0x59e6b67f48fb32e7e570dfb11e042b5ad2e55e3ce3ce9cd989c7e06e07feeafd")
                .unwrap(),
            s: U256::from_str("0x016b83f4f980694ed2eee4d10667242b1f40dc406901b34125b008d334d47469")
                .unwrap(),
        };
        test_decode_and_encode(bytes, transaction, signature, None);

        let bytes = hex::decode("f8650f84832156008287fb94cf7f9e66af820a19257a2108375b180b0ec491678204d2802ca035b7bfeb9ad9ece2cbafaaf8e202e706b4cfaeb233f46198f00b44d4a566a981a0612638fb29427ca33b9a3be2a0a561beecfe0269655be160d35e72d366a6a860").unwrap();
        let transaction = Transaction::Legacy(TxLegacy {
            chain_id: Some(4),
            nonce: 15,
            gas_price: 2200000000,
            gas_limit: 34811,
            to: TransactionKind::Call(Address::from_slice(
                &hex::decode("cf7f9e66af820a19257a2108375b180b0ec49167").unwrap()[..],
            )),
            value: 1234u64.into(),
            input: Bytes::default(),
        });
        let signature = Signature {
            odd_y_parity: true,
            r: U256::from_str("0x35b7bfeb9ad9ece2cbafaaf8e202e706b4cfaeb233f46198f00b44d4a566a981")
                .unwrap(),
            s: U256::from_str("0x612638fb29427ca33b9a3be2a0a561beecfe0269655be160d35e72d366a6a860")
                .unwrap(),
        };
        test_decode_and_encode(bytes, transaction, signature, None);
    }

    fn test_decode_and_encode(
        bytes: Vec<u8>,
        transaction: Transaction,
        signature: Signature,
        hash: Option<B256>,
    ) {
        let expected = TransactionSigned::from_transaction_and_signature(transaction, signature);
        if let Some(hash) = hash {
            assert_eq!(hash, expected.hash);
        }
        assert_eq!(bytes.len(), expected.length());

        let decoded = TransactionSigned::decode(&mut &bytes[..]).unwrap();
        assert_eq!(expected, decoded);

        let mut encoded = BytesMut::new();
        expected.encode(&mut encoded);
        assert_eq!(bytes, encoded);
    }

    #[test]
    fn decode_raw_tx_and_recover_signer() {
        // transaction is from ropsten
        let hash: B256 =
            hex!("559fb34c4a7f115db26cbf8505389475caaab3df45f5c7a0faa4abfa3835306c").into();
        let signer: Address = hex!("641c5d790f862a58ec7abcfd644c0442e9c201b3").into();
        let raw = hex!("f88b8212b085028fa6ae00830f424094aad593da0c8116ef7d2d594dd6a63241bccfc26c80a48318b64b000000000000000000000000641c5d790f862a58ec7abcfd644c0442e9c201b32aa0a6ef9e170bca5ffb7ac05433b13b7043de667fbb0b4a5e45d3b54fb2d6efcc63a0037ec2c05c3d60c5f5f78244ce0a3859e3a18a36c61efb061b383507d3ce19d2");
        let tx = TransactionSigned::decode(&mut &raw[..]).unwrap();
        assert_eq!(tx.hash(), hash);
        assert_eq!(tx.recover_signer(), Some(signer),);
    }

    #[test]
    fn recover_signer_legacy() {
        let signer: Address = hex!("398137383b3d25c92898c656696e41950e47316b").into();
        let hash: B256 =
            hex!("bb3a336e3f823ec18197f1e13ee875700f08f03e2cab75f0d0b118dabb44cba0").into();

        let tx = Transaction::Legacy(TxLegacy {
            chain_id: Some(1),
            nonce: 0x18,
            gas_price: 0xfa56ea00,
            gas_limit: 119902,
            to: TransactionKind::Call( hex!("06012c8cf97bead5deae237070f9587f8e7a266d").into()),
            value: 0x1c6bf526340000u64.into(),
            input:  hex!("f7d8c88300000000000000000000000000000000000000000000000000000000000cee6100000000000000000000000000000000000000000000000000000000000ac3e1").into(),
        });

        let sig = Signature {
            r: U256::from_be_bytes(hex!(
                "2a378831cf81d99a3f06a18ae1b6ca366817ab4d88a70053c41d7a8f0368e031"
            )),
            s: U256::from_be_bytes(hex!(
                "450d831a05b6e418724436c05c155e0a1b7b921015d0fbc2f667aed709ac4fb5"
            )),
            odd_y_parity: false,
        };

        let signed_tx = TransactionSigned::from_transaction_and_signature(tx, sig);
        assert_eq!(signed_tx.hash(), hash);
        assert_eq!(signed_tx.recover_signer(), Some(signer),);
    }

    #[test]
    fn recover_signer_eip1559() {
        let signer: Address = hex!("dd6b8b3dc6b7ad97db52f08a275ff4483e024cea").into();
        let hash: B256 =
            hex!("0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0").into();

        let tx = Transaction::Eip1559( TxEip1559 {
            chain_id: 1,
            nonce: 0x42,
            gas_limit: 44386,
            to: TransactionKind::Call( hex!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6").into()),
            value: 0,
            input:  hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
            max_fee_per_gas: 0x4a817c800,
            max_priority_fee_per_gas: 0x3b9aca00,
            access_list: AccessList::default(),
        });

        let sig = Signature {
            r: U256::from_str("0x840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565")
                .unwrap(),
            s: U256::from_str("0x25e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1")
                .unwrap(),
            odd_y_parity: false,
        };

        let signed_tx = TransactionSigned::from_transaction_and_signature(tx, sig);
        assert_eq!(signed_tx.hash(), hash);
        assert_eq!(signed_tx.recover_signer(), Some(signer),);
    }

    #[test]
    fn test_envelop_encode() {
        // random tx: <https://etherscan.io/getRawTx?tx=0x9448608d36e721ef403c53b00546068a6474d6cbab6816c3926de449898e7bce>
        let input = Bytes::from_static(&hex!("02f871018302a90f808504890aef60826b6c94ddf4c5025d1a5742cf12f74eec246d4432c295e487e09c3bbcc12b2b80c080a0f21a4eacd0bf8fea9c5105c543be5a1d8c796516875710fafafdf16d16d8ee23a001280915021bb446d1973501a67f93d2b38894a514b976e7b46dc2fe54598d76"));
        let decoded = TransactionSigned::decode(&mut &input[..]).unwrap();

        let encoded = decoded.envelope_encoded();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_envelop_decode() {
        // random tx: <https://etherscan.io/getRawTx?tx=0x9448608d36e721ef403c53b00546068a6474d6cbab6816c3926de449898e7bce>
        let input = hex::decode("02f871018302a90f808504890aef60826b6c94ddf4c5025d1a5742cf12f74eec246d4432c295e487e09c3bbcc12b2b80c080a0f21a4eacd0bf8fea9c5105c543be5a1d8c796516875710fafafdf16d16d8ee23a001280915021bb446d1973501a67f93d2b38894a514b976e7b46dc2fe54598d76").unwrap();
        let decoded = TransactionSigned::decode_enveloped(input.clone().into()).unwrap();

        let encoded = decoded.envelope_encoded();
        assert_eq!(encoded, input);
    }

    #[test]
    fn test_decode_signed_ec_recovered_transaction() {
        // random tx: <https://etherscan.io/getRawTx?tx=0x9448608d36e721ef403c53b00546068a6474d6cbab6816c3926de449898e7bce>
        let input = hex::decode("02f871018302a90f808504890aef60826b6c94ddf4c5025d1a5742cf12f74eec246d4432c295e487e09c3bbcc12b2b80c080a0f21a4eacd0bf8fea9c5105c543be5a1d8c796516875710fafafdf16d16d8ee23a001280915021bb446d1973501a67f93d2b38894a514b976e7b46dc2fe54598d76").unwrap();
        let tx = TransactionSigned::decode(&mut &input[..]).unwrap();
        let recovered = tx.into_ecrecovered().unwrap();

        let mut encoded = BytesMut::new();
        recovered.encode(&mut encoded);

        let decoded = TransactionSignedEcRecovered::decode(&mut &encoded[..]).unwrap();
        assert_eq!(recovered, decoded)
    }

    #[test]
    fn test_decode_tx() {
        // some random transactions pulled from hive tests
        let s = "b86f02f86c0705843b9aca008506fc23ac00830124f89400000000000000000000000000000000000003160180c001a00293c713e2f1eab91c366621ff2f867e05ad7e99d4aa5d069aafeb9e1e8c9b6aa05ec6c0605ff20b57c90a6484ec3b0509e5923733d06f9b69bee9a2dabe4f1352";
        let tx = TransactionSigned::decode(&mut &hex::decode(s).unwrap()[..]).unwrap();
        let mut b = Vec::new();
        tx.encode(&mut b);
        assert_eq!(s, hex::encode(&b));

        let s = "f865048506fc23ac00830124f8940000000000000000000000000000000000000316018032a06b8fdfdcb84790816b7af85b19305f493665fe8b4e7c51ffdd7cc144cd776a60a028a09ab55def7b8d6602ba1c97a0ebbafe64ffc9c8e89520cec97a8edfb2ebe9";
        let tx = TransactionSigned::decode(&mut &hex::decode(s).unwrap()[..]).unwrap();
        let mut b = Vec::new();
        tx.encode(&mut b);
        assert_eq!(s, hex::encode(&b));
    }
}
