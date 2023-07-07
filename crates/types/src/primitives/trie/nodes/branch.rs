use super::{super::TrieMask, rlp_node, CHILD_INDEX_RANGE};
use alloy_primitives::B256;
use alloy_rlp::{BufMut, EMPTY_STRING_CODE};
use serde::{Deserialize, Serialize};

/// A Branch node is only a pointer to the stack of nodes and is used to
/// create the RLP encoding of the node using masks which filter from
/// the stack of nodes.
#[derive(Clone, Debug)]
pub struct BranchNode<'a> {
    /// Rlp encoded children
    pub stack: &'a [Vec<u8>],
}

impl<'a> BranchNode<'a> {
    /// Create a new branch node from the stack of nodes.
    pub fn new(stack: &'a [Vec<u8>]) -> Self {
        Self { stack }
    }

    /// Given the hash and state mask of children present, return an iterator
    /// over the stack items that match the mask.
    pub fn children(
        &self,
        state_mask: TrieMask,
        hash_mask: TrieMask,
    ) -> impl Iterator<Item = B256> + '_ {
        let mut index = self.stack.len() - state_mask.count_ones() as usize;
        CHILD_INDEX_RANGE.filter_map(move |digit| {
            let mut child = None;
            if state_mask.is_bit_set(digit) {
                if hash_mask.is_bit_set(digit) {
                    child = Some(&self.stack[index]);
                }
                index += 1;
            }
            child.map(|child| B256::from_slice(&child[1..]))
        })
    }

    /// Returns the RLP encoding of the branch node given the state mask of
    /// children present.
    pub fn rlp(&self, state_mask: TrieMask, buf: &mut Vec<u8>) -> Vec<u8> {
        let first_child_idx = self.stack.len() - state_mask.count_ones() as usize;

        // Create the RLP header from the mask elements present.
        let mut i = first_child_idx;
        let header = CHILD_INDEX_RANGE.fold(
            alloy_rlp::Header {
                list: true,
                payload_length: 1,
            },
            |mut header, digit| {
                if state_mask.is_bit_set(digit) {
                    header.payload_length += self.stack[i].len();
                    i += 1;
                } else {
                    header.payload_length += 1;
                }
                header
            },
        );
        header.encode(buf);

        // Extend the RLP buffer with the present children
        let mut i = first_child_idx;
        CHILD_INDEX_RANGE.for_each(|idx| {
            if state_mask.is_bit_set(idx) {
                buf.extend_from_slice(&self.stack[i]);
                i += 1;
            } else {
                buf.put_u8(EMPTY_STRING_CODE)
            }
        });

        // Is this needed?
        buf.put_u8(EMPTY_STRING_CODE);

        rlp_node(buf)
    }
}

/// A struct representing a branch node in an Ethereum trie.
///
/// A branch node can have up to 16 children, each corresponding to one of the
/// possible nibble values (0 to 15) in the trie's path.
///
/// The masks in a BranchNode are used to efficiently represent and manage
/// information about the presence and types of its children. They are bitmasks,
/// where each bit corresponds to a nibble (half-byte, or 4 bits) value from 0
/// to 15.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct BranchNodeCompact {
    /// The bitmask indicating the presence of children at the respective nibble
    /// positions in the trie. If the bit at position i (counting from the
    /// right) is set (1), it indicates that a child exists for the nibble
    /// value i. If the bit is unset (0), it means there is no child
    /// for that nibble value.
    pub state_mask: TrieMask,
    /// The bitmask representing the internal (unhashed) children at the
    /// respective nibble positions in the trie. If the bit at position `i`
    /// (counting from the right) is set (1) and also present in the
    /// state_mask, it indicates that the corresponding child at the nibble
    /// value `i` is an internal child. If the bit is unset (0), it means
    /// the child is not an internal child.
    pub tree_mask: TrieMask,
    /// The bitmask representing the hashed children at the respective nibble
    /// positions in the trie. If the bit at position `i` (counting from the
    /// right) is set (1) and also present in the state_mask, it indicates
    /// that the corresponding child at the nibble value `i` is a hashed
    /// child. If the bit is unset (0), it means the child is not a
    /// hashed child.
    pub hash_mask: TrieMask,
    /// Collection of hashes associated with the children of the branch node.
    /// Each child hash is calculated by hashing two consecutive sub-branch
    /// roots.
    pub hashes: Vec<B256>,
    /// An optional root hash of the subtree rooted at this branch node.
    pub root_hash: Option<B256>,
}

impl BranchNodeCompact {
    /// Creates a new [BranchNodeCompact] from the given parameters.
    pub fn new(
        state_mask: impl Into<TrieMask>,
        tree_mask: impl Into<TrieMask>,
        hash_mask: impl Into<TrieMask>,
        hashes: Vec<B256>,
        root_hash: Option<B256>,
    ) -> Self {
        let (state_mask, tree_mask, hash_mask) =
            (state_mask.into(), tree_mask.into(), hash_mask.into());
        assert!(tree_mask.is_subset_of(&state_mask));
        assert!(hash_mask.is_subset_of(&state_mask));
        assert_eq!(hash_mask.count_ones() as usize, hashes.len());
        Self {
            state_mask,
            tree_mask,
            hash_mask,
            hashes,
            root_hash,
        }
    }

    /// Returns the hash associated with the given nibble.
    pub fn hash_for_nibble(&self, nibble: u8) -> B256 {
        let mask = *TrieMask::from_nibble(nibble) - 1;
        let index = (*self.hash_mask & mask).count_ones();
        self.hashes[index as usize]
    }
}
