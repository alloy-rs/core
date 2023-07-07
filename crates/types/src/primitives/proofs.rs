//! Helper function for calculating Merkle proofs and hashes.

use super::{Header, Log};
use crate::{
    trie::{HashBuilder, Nibbles},
    GenesisAccount, ReceiptWithBloom, ReceiptWithBloomRef, TransactionSigned, Withdrawal,
};
use alloy_primitives::{b256, hex, keccak256, Address, Bytes, FixedBytes, B256};
use alloy_rlp::Encodable;
use bytes::{BufMut, BytesMut};
use hash_db::Hasher;
use plain_hasher::PlainHasher;
use std::collections::HashMap;
use triehash::sec_trie_root;

/// Keccak-256 hash of the RLP of an empty list, KEC("\xc0").
pub const EMPTY_LIST_HASH: B256 =
    b256!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347");

/// Root hash of an empty trie.
pub const EMPTY_ROOT: B256 =
    b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");

/// A [Hasher] that calculates a keccak256 hash of the given data.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct KeccakHasher;

impl Hasher for KeccakHasher {
    type Out = B256;
    type StdHasher = PlainHasher;

    const LENGTH: usize = 32;

    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x)
    }
}

/// Adjust the index of an item for rlp encoding.
pub const fn adjust_index_for_rlp(i: usize, len: usize) -> usize {
    if i > 0x7f {
        i
    } else if i == 0x7f || i + 1 == len {
        0
    } else {
        i + 1
    }
}

/// Compute a trie root of the collection of rlp encodable items.
pub fn ordered_trie_root<T: Encodable>(items: &[T]) -> B256 {
    ordered_trie_root_with_encoder(items, |item, buf| item.encode(buf))
}

/// Compute a trie root of the collection of items with a custom encoder.
pub fn ordered_trie_root_with_encoder<T, F>(items: &[T], mut encode: F) -> B256
where
    F: FnMut(&T, &mut dyn BufMut),
{
    let mut index_buffer = BytesMut::new();
    let mut value_buffer = BytesMut::new();

    let mut hb = HashBuilder::default();
    let items_len = items.len();
    for i in 0..items_len {
        let index = adjust_index_for_rlp(i, items_len);

        index_buffer.clear();
        index.encode(&mut index_buffer);

        value_buffer.clear();
        encode(&items[index], &mut value_buffer);

        hb.add_leaf(Nibbles::unpack(&index_buffer), &value_buffer);
    }

    hb.root()
}

/// Calculate a transaction root.
///
/// `(rlp(index), encoded(tx))` pairs.
pub fn calculate_transaction_root<T>(transactions: &[T]) -> B256
where
    T: AsRef<TransactionSigned>,
{
    ordered_trie_root_with_encoder(transactions, |tx: &T, buf| {
        tx.as_ref().encode_inner(buf, false)
    })
}

/// Calculates the root hash of the withdrawals.
pub fn calculate_withdrawals_root(withdrawals: &[Withdrawal]) -> B256 {
    ordered_trie_root(withdrawals)
}

/// Calculates the receipt root for a header.
pub fn calculate_receipt_root(receipts: &[ReceiptWithBloom]) -> B256 {
    ordered_trie_root_with_encoder(receipts, |r, buf| r.encode_inner(buf, false))
}

/// Calculates the receipt root for a header for the reference type of
/// [ReceiptWithBloom].
///
/// NOTE: Prefer [calculate_receipt_root] if you have log blooms memoized.
pub fn calculate_receipt_root_ref<T>(receipts: &[T]) -> B256
where
    for<'a> ReceiptWithBloomRef<'a>: From<&'a T>,
{
    ordered_trie_root_with_encoder(receipts, |r, buf| {
        ReceiptWithBloomRef::from(r).encode_inner(buf, false)
    })
}

/// Calculates the log root for headers.
pub fn calculate_log_root(logs: &[Log]) -> B256 {
    //https://github.com/ethereum/go-ethereum/blob/356bbe343a30789e77bb38f25983c8f2f2bfbb47/cmd/evm/internal/t8ntool/execution.go#L255
    let mut logs_rlp = Vec::new();
    alloy_rlp::encode_list(logs, &mut logs_rlp);
    keccak256(logs_rlp)
}

/// Calculates the root hash for ommer/uncle headers.
pub fn calculate_ommers_root(ommers: &[Header]) -> B256 {
    // RLP Encode
    let mut ommers_rlp = Vec::new();
    alloy_rlp::encode_list(ommers, &mut ommers_rlp);
    keccak256(ommers_rlp)
}

/// Calculates the root hash for the state, this corresponds to [geth's
/// `deriveHash`](https://github.com/ethereum/go-ethereum/blob/6c149fd4ad063f7c24d726a73bc0546badd1bc73/core/genesis.go#L119).
pub fn genesis_state_root(genesis_alloc: &HashMap<Address, GenesisAccount>) -> B256 {
    let encoded_accounts = genesis_alloc.iter().map(|(address, account)| {
        let mut acc_rlp = BytesMut::new();
        account.encode(&mut acc_rlp);
        (address, Bytes::from(acc_rlp.freeze()))
    });

    sec_trie_root::<KeccakHasher, _, _, _>(encoded_accounts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        primitives::{Block, Log},
        proofs::{calculate_receipt_root, calculate_transaction_root, genesis_state_root},
        GenesisAccount, Receipt, ReceiptWithBloom, TxType,
    };
    use alloy_primitives::{address, hex, Address, Bloom, B256, U256};
    use alloy_rlp::Decodable;
    use std::{collections::HashMap, str::FromStr};

    #[test]
    fn check_transaction_root() {
        let data = &hex!("f90262f901f9a092230ce5476ae868e98c7979cfc165a93f8b6ad1922acf2df62e340916efd49da01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa02307107a867056ca33b5087e77c4174f47625e48fb49f1c70ced34890ddd88f3a08151d548273f6683169524b66ca9fe338b9ce42bc3540046c828fd939ae23bcba0c598f69a5674cae9337261b669970e24abc0b46e6d284372a239ec8ccbf20b0ab901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000083020000018502540be40082a8618203e800a00000000000000000000000000000000000000000000000000000000000000000880000000000000000f863f861800a8405f5e10094100000000000000000000000000000000000000080801ba07e09e26678ed4fac08a249ebe8ed680bf9051a5e14ad223e4b2b9d26e0208f37a05f6e3f188e3e6eab7d7d3b6568f5eac7d687b08d307d3154ccd8c87b4630509bc0");
        let block_rlp = &mut data.as_slice();
        let block: Block = Block::decode(block_rlp).unwrap();

        let tx_root = calculate_transaction_root(&block.body);
        assert_eq!(block.transactions_root, tx_root, "Must be the same");
    }

    #[test]
    fn check_receipt_root() {
        let logs = vec![Log {
            address: Address::ZERO,
            topics: vec![],
            data: Default::default(),
        }];
        let bloom = Bloom::from(hex!("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001"));
        let receipt = ReceiptWithBloom {
            receipt: Receipt {
                tx_type: TxType::EIP2930,
                success: true,
                cumulative_gas_used: 102068,
                logs,
            },
            bloom,
        };
        let receipt = vec![receipt];
        let root = calculate_receipt_root(&receipt);
        assert_eq!(
            root,
            B256::from(hex!(
                "fe70ae4a136d98944951b2123859698d59ad251a381abc9960fa81cae3d0d4a0"
            ))
        );
    }

    #[test]
    fn check_withdrawals_root() {
        // Single withdrawal, amount 0
        // https://github.com/ethereum/tests/blob/9760400e667eba241265016b02644ef62ab55de2/BlockchainTests/EIPTests/bc4895-withdrawals/amountIs0.json
        let data = &hex!("f90238f90219a0151934ad9b654c50197f37018ee5ee9bb922dec0a1b5e24a6d679cb111cdb107a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa0046119afb1ab36aaa8f66088677ed96cd62762f6d3e65642898e189fbe702d51a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008001887fffffffffffffff8082079e42a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b42188000000000000000009a048a703da164234812273ea083e4ec3d09d028300cd325b46a6a75402e5a7ab95c0c0d9d8808094c94f5374fce5edbc8e2a8697c15331677e6ebf0b80");
        let block: Block = Block::decode(&mut data.as_slice()).unwrap();
        assert!(block.withdrawals.is_some());
        let withdrawals = block.withdrawals.as_ref().unwrap();
        assert_eq!(withdrawals.len(), 1);
        let withdrawals_root = calculate_withdrawals_root(withdrawals);
        assert_eq!(block.withdrawals_root, Some(withdrawals_root));

        // 4 withdrawals, identical indices
        // https://github.com/ethereum/tests/blob/9760400e667eba241265016b02644ef62ab55de2/BlockchainTests/EIPTests/bc4895-withdrawals/twoIdenticalIndex.json
        let data = &hex!("f9028cf90219a0151934ad9b654c50197f37018ee5ee9bb922dec0a1b5e24a6d679cb111cdb107a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa0ccf7b62d616c2ad7af862d67b9dcd2119a90cebbff8c3cd1e5d7fc99f8755774a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008001887fffffffffffffff8082079e42a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b42188000000000000000009a0a95b9a7b58a6b3cb4001eb0be67951c5517141cb0183a255b5cae027a7b10b36c0c0f86cda808094c94f5374fce5edbc8e2a8697c15331677e6ebf0b822710da028094c94f5374fce5edbc8e2a8697c15331677e6ebf0b822710da018094c94f5374fce5edbc8e2a8697c15331677e6ebf0b822710da028094c94f5374fce5edbc8e2a8697c15331677e6ebf0b822710");
        let block: Block = Block::decode(&mut data.as_slice()).unwrap();
        assert!(block.withdrawals.is_some());
        let withdrawals = block.withdrawals.as_ref().unwrap();
        assert_eq!(withdrawals.len(), 4);
        let withdrawals_root = calculate_withdrawals_root(withdrawals);
        assert_eq!(block.withdrawals_root, Some(withdrawals_root));
    }

    #[test]
    fn check_empty_state_root() {
        let genesis_alloc = HashMap::new();
        let root = genesis_state_root(&genesis_alloc);
        assert_eq!(root, EMPTY_ROOT);
    }

    #[test]
    fn test_simple_account_state_root() {
        // each fixture specifies an address and expected root hash - the address is
        // initialized with a maximum balance, and is the only account in the
        // state. these test cases are generated by using geth with a custom
        // genesis.json (with a single account that has max balance)
        let fixtures = vec![
            (
                address!("9fe4abd71ad081f091bd06dd1c16f7e92927561e"),
                b256!("4b35be4231841d212ce2fa43aedbddeadd6eb7d420195664f9f0d55629db8c32"),
            ),
            (
                address!("c2ba9d87f8be0ade00c60d3656c1188e008fbfa2"),
                b256!("e1389256c47d63df8856d7729dec9dc2dae074a7f0cbc49acad1cf7b29f7fe94"),
            ),
        ];

        for (test_addr, expected_root) in fixtures {
            let mut genesis_alloc = HashMap::new();
            genesis_alloc.insert(
                test_addr,
                GenesisAccount {
                    nonce: None,
                    balance: U256::MAX,
                    code: None,
                    storage: None,
                },
            );
            let root = genesis_state_root(&genesis_alloc);

            assert_eq!(root, expected_root);
        }
    }

    #[test]
    fn test_sepolia_state_root() {
        let expected_root =
            b256!("5eb6e371a698b8d68f665192350ffcecbbbf322916f4b51bd79bb6887da3f494");
        let alloc = HashMap::from([
            (
                address!("a2A6d93439144FFE4D27c9E088dCD8b783946263").into(),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("Bc11295936Aa79d594139de1B2e12629414F3BDB"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("7cF5b79bfe291A67AB02b393E456cCc4c266F753"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("aaec86394441f915bce3e6ab399977e9906f3b69"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("F47CaE1CF79ca6758Bfc787dbD21E6bdBe7112B8"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("d7eDDB78ED295B3C9629240E8924fb8D8874ddD8"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("8b7F0977Bb4f0fBE7076FA22bC24acA043583F5e"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("e2e2659028143784d557bcec6ff3a0721048880a"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("d9a5179f091d85051d3c982785efd1455cec8699"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("beef32ca5b9a198d27B4e02F4c70439fE60356Cf"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("0000006916a87b82333f4245046623b23794c65c"),
                GenesisAccount {
                    balance: U256::from_str("10000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("b21c33de1fab3fa15499c62b59fe0cc3250020d1"),
                GenesisAccount {
                    balance: U256::from_str("100000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("10F5d45854e038071485AC9e402308cF80D2d2fE"),
                GenesisAccount {
                    balance: U256::from_str("100000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("d7d76c58b3a519e9fA6Cc4D22dC017259BC49F1E"),
                GenesisAccount {
                    balance: U256::from_str("100000000000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
            (
                address!("799D329e5f583419167cD722962485926E338F4a"),
                GenesisAccount {
                    balance: U256::from_str("1000000000000000000").unwrap(),
                    ..Default::default()
                },
            ),
        ]);

        let root = genesis_state_root(&alloc);

        assert_eq!(root, expected_root);
    }
}
