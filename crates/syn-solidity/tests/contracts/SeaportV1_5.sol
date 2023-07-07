// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

enum OrderType {
    // 0: no partial fills, anyone can execute
    FULL_OPEN,

    // 1: partial fills supported, anyone can execute
    PARTIAL_OPEN,

    // 2: no partial fills, only offerer or zone can execute
    FULL_RESTRICTED,

    // 3: partial fills supported, only offerer or zone can execute
    PARTIAL_RESTRICTED,

    // 4: contract order type
    CONTRACT
}

enum BasicOrderType {
    // 0: no partial fills, anyone can execute
    ETH_TO_ERC721_FULL_OPEN,

    // 1: partial fills supported, anyone can execute
    ETH_TO_ERC721_PARTIAL_OPEN,

    // 2: no partial fills, only offerer or zone can execute
    ETH_TO_ERC721_FULL_RESTRICTED,

    // 3: partial fills supported, only offerer or zone can execute
    ETH_TO_ERC721_PARTIAL_RESTRICTED,

    // 4: no partial fills, anyone can execute
    ETH_TO_ERC1155_FULL_OPEN,

    // 5: partial fills supported, anyone can execute
    ETH_TO_ERC1155_PARTIAL_OPEN,

    // 6: no partial fills, only offerer or zone can execute
    ETH_TO_ERC1155_FULL_RESTRICTED,

    // 7: partial fills supported, only offerer or zone can execute
    ETH_TO_ERC1155_PARTIAL_RESTRICTED,

    // 8: no partial fills, anyone can execute
    ERC20_TO_ERC721_FULL_OPEN,

    // 9: partial fills supported, anyone can execute
    ERC20_TO_ERC721_PARTIAL_OPEN,

    // 10: no partial fills, only offerer or zone can execute
    ERC20_TO_ERC721_FULL_RESTRICTED,

    // 11: partial fills supported, only offerer or zone can execute
    ERC20_TO_ERC721_PARTIAL_RESTRICTED,

    // 12: no partial fills, anyone can execute
    ERC20_TO_ERC1155_FULL_OPEN,

    // 13: partial fills supported, anyone can execute
    ERC20_TO_ERC1155_PARTIAL_OPEN,

    // 14: no partial fills, only offerer or zone can execute
    ERC20_TO_ERC1155_FULL_RESTRICTED,

    // 15: partial fills supported, only offerer or zone can execute
    ERC20_TO_ERC1155_PARTIAL_RESTRICTED,

    // 16: no partial fills, anyone can execute
    ERC721_TO_ERC20_FULL_OPEN,

    // 17: partial fills supported, anyone can execute
    ERC721_TO_ERC20_PARTIAL_OPEN,

    // 18: no partial fills, only offerer or zone can execute
    ERC721_TO_ERC20_FULL_RESTRICTED,

    // 19: partial fills supported, only offerer or zone can execute
    ERC721_TO_ERC20_PARTIAL_RESTRICTED,

    // 20: no partial fills, anyone can execute
    ERC1155_TO_ERC20_FULL_OPEN,

    // 21: partial fills supported, anyone can execute
    ERC1155_TO_ERC20_PARTIAL_OPEN,

    // 22: no partial fills, only offerer or zone can execute
    ERC1155_TO_ERC20_FULL_RESTRICTED,

    // 23: partial fills supported, only offerer or zone can execute
    ERC1155_TO_ERC20_PARTIAL_RESTRICTED
}

enum BasicOrderRouteType {
    // 0: provide Ether (or other native token) to receive offered ERC721 item.
    ETH_TO_ERC721,

    // 1: provide Ether (or other native token) to receive offered ERC1155 item.
    ETH_TO_ERC1155,

    // 2: provide ERC20 item to receive offered ERC721 item.
    ERC20_TO_ERC721,

    // 3: provide ERC20 item to receive offered ERC1155 item.
    ERC20_TO_ERC1155,

    // 4: provide ERC721 item to receive offered ERC20 item.
    ERC721_TO_ERC20,

    // 5: provide ERC1155 item to receive offered ERC20 item.
    ERC1155_TO_ERC20
}

enum ItemType {
    // 0: ETH on mainnet, MATIC on polygon, etc.
    NATIVE,

    // 1: ERC20 items (ERC777 and ERC20 analogues could also technically work)
    ERC20,

    // 2: ERC721 items
    ERC721,

    // 3: ERC1155 items
    ERC1155,

    // 4: ERC721 items where a number of tokenIds are supported
    ERC721_WITH_CRITERIA,

    // 5: ERC1155 items where a number of ids are supported
    ERC1155_WITH_CRITERIA
}

enum Side {
    // 0: Items that can be spent
    OFFER,

    // 1: Items that must be received
    CONSIDERATION
}

type CalldataPointer is uint256;

type ReturndataPointer is uint256;

type MemoryPointer is uint256;

using CalldataPointerLib for CalldataPointer global;
using MemoryPointerLib for MemoryPointer global;
using ReturndataPointerLib for ReturndataPointer global;

using CalldataReaders for CalldataPointer global;
using ReturndataReaders for ReturndataPointer global;
using MemoryReaders for MemoryPointer global;
using MemoryWriters for MemoryPointer global;

CalldataPointer constant CalldataStart = CalldataPointer.wrap(0x04);
MemoryPointer constant FreeMemoryPPtr = MemoryPointer.wrap(0x40);
uint256 constant IdentityPrecompileAddress = 0x4;
uint256 constant OffsetOrLengthMask = 0xffffffff;
uint256 constant _OneWord = 0x20;
uint256 constant _FreeMemoryPointerSlot = 0x40;

/// @dev Allocates `size` bytes in memory by increasing the free memory pointer
///    and returns the memory pointer to the first byte of the allocated region.
// (Free functions cannot have visibility.)
// solhint-disable-next-line func-visibility
function malloc(uint256 size) pure returns (MemoryPointer mPtr) {
    assembly {
        mPtr := mload(_FreeMemoryPointerSlot)
        mstore(_FreeMemoryPointerSlot, add(mPtr, size))
    }
}

// (Free functions cannot have visibility.)
// solhint-disable-next-line func-visibility
function getFreeMemoryPointer() pure returns (MemoryPointer mPtr) {
    mPtr = FreeMemoryPPtr.readMemoryPointer();
}

// (Free functions cannot have visibility.)
// solhint-disable-next-line func-visibility
function setFreeMemoryPointer(MemoryPointer mPtr) pure {
    FreeMemoryPPtr.write(mPtr);
}

library CalldataPointerLib {
    function lt(
        CalldataPointer a,
        CalldataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := lt(a, b)
        }
    }

    function gt(
        CalldataPointer a,
        CalldataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := gt(a, b)
        }
    }

    function eq(
        CalldataPointer a,
        CalldataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := eq(a, b)
        }
    }

    function isNull(CalldataPointer a) internal pure returns (bool b) {
        assembly {
            b := iszero(a)
        }
    }

    /// @dev Resolves an offset stored at `cdPtr + headOffset` to a calldata.
    ///      pointer `cdPtr` must point to some parent object with a dynamic
    ///      type's head stored at `cdPtr + headOffset`.
    function pptr(
        CalldataPointer cdPtr,
        uint256 headOffset
    ) internal pure returns (CalldataPointer cdPtrChild) {
        cdPtrChild = cdPtr.offset(
            cdPtr.offset(headOffset).readUint256() & OffsetOrLengthMask
        );
    }

    /// @dev Resolves an offset stored at `cdPtr` to a calldata pointer.
    ///      `cdPtr` must point to some parent object with a dynamic type as its
    ///      first member, e.g. `struct { bytes data; }`
    function pptr(
        CalldataPointer cdPtr
    ) internal pure returns (CalldataPointer cdPtrChild) {
        cdPtrChild = cdPtr.offset(cdPtr.readUint256() & OffsetOrLengthMask);
    }

    /// @dev Returns the calldata pointer one word after `cdPtr`.
    function next(
        CalldataPointer cdPtr
    ) internal pure returns (CalldataPointer cdPtrNext) {
        assembly {
            cdPtrNext := add(cdPtr, _OneWord)
        }
    }

    /// @dev Returns the calldata pointer `_offset` bytes after `cdPtr`.
    function offset(
        CalldataPointer cdPtr,
        uint256 _offset
    ) internal pure returns (CalldataPointer cdPtrNext) {
        assembly {
            cdPtrNext := add(cdPtr, _offset)
        }
    }

    /// @dev Copies `size` bytes from calldata starting at `src` to memory at
    ///      `dst`.
    function copy(
        CalldataPointer src,
        MemoryPointer dst,
        uint256 size
    ) internal pure {
        assembly {
            calldatacopy(dst, src, size)
        }
    }
}

library ReturndataPointerLib {
    function lt(
        ReturndataPointer a,
        ReturndataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := lt(a, b)
        }
    }

    function gt(
        ReturndataPointer a,
        ReturndataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := gt(a, b)
        }
    }

    function eq(
        ReturndataPointer a,
        ReturndataPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := eq(a, b)
        }
    }

    function isNull(ReturndataPointer a) internal pure returns (bool b) {
        assembly {
            b := iszero(a)
        }
    }

    /// @dev Resolves an offset stored at `rdPtr + headOffset` to a returndata
    ///      pointer. `rdPtr` must point to some parent object with a dynamic
    ///      type's head stored at `rdPtr + headOffset`.
    function pptr(
        ReturndataPointer rdPtr,
        uint256 headOffset
    ) internal pure returns (ReturndataPointer rdPtrChild) {
        rdPtrChild = rdPtr.offset(
            rdPtr.offset(headOffset).readUint256() & OffsetOrLengthMask
        );
    }

    /// @dev Resolves an offset stored at `rdPtr` to a returndata pointer.
    ///    `rdPtr` must point to some parent object with a dynamic type as its
    ///    first member, e.g. `struct { bytes data; }`
    function pptr(
        ReturndataPointer rdPtr
    ) internal pure returns (ReturndataPointer rdPtrChild) {
        rdPtrChild = rdPtr.offset(rdPtr.readUint256() & OffsetOrLengthMask);
    }

    /// @dev Returns the returndata pointer one word after `cdPtr`.
    function next(
        ReturndataPointer rdPtr
    ) internal pure returns (ReturndataPointer rdPtrNext) {
        assembly {
            rdPtrNext := add(rdPtr, _OneWord)
        }
    }

    /// @dev Returns the returndata pointer `_offset` bytes after `cdPtr`.
    function offset(
        ReturndataPointer rdPtr,
        uint256 _offset
    ) internal pure returns (ReturndataPointer rdPtrNext) {
        assembly {
            rdPtrNext := add(rdPtr, _offset)
        }
    }

    /// @dev Copies `size` bytes from returndata starting at `src` to memory at
    /// `dst`.
    function copy(
        ReturndataPointer src,
        MemoryPointer dst,
        uint256 size
    ) internal pure {
        assembly {
            returndatacopy(dst, src, size)
        }
    }
}

library MemoryPointerLib {
    function copy(
        MemoryPointer src,
        MemoryPointer dst,
        uint256 size
    ) internal view {
        assembly {
            let success := staticcall(
                gas(),
                IdentityPrecompileAddress,
                src,
                size,
                dst,
                size
            )
            if or(iszero(returndatasize()), iszero(success)) {
                revert(0, 0)
            }
        }
    }

    function lt(
        MemoryPointer a,
        MemoryPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := lt(a, b)
        }
    }

    function gt(
        MemoryPointer a,
        MemoryPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := gt(a, b)
        }
    }

    function eq(
        MemoryPointer a,
        MemoryPointer b
    ) internal pure returns (bool c) {
        assembly {
            c := eq(a, b)
        }
    }

    function isNull(MemoryPointer a) internal pure returns (bool b) {
        assembly {
            b := iszero(a)
        }
    }

    function hash(
        MemoryPointer ptr,
        uint256 length
    ) internal pure returns (bytes32 _hash) {
        assembly {
            _hash := keccak256(ptr, length)
        }
    }

    /// @dev Returns the memory pointer one word after `mPtr`.
    function next(
        MemoryPointer mPtr
    ) internal pure returns (MemoryPointer mPtrNext) {
        assembly {
            mPtrNext := add(mPtr, _OneWord)
        }
    }

    /// @dev Returns the memory pointer `_offset` bytes after `mPtr`.
    function offset(
        MemoryPointer mPtr,
        uint256 _offset
    ) internal pure returns (MemoryPointer mPtrNext) {
        assembly {
            mPtrNext := add(mPtr, _offset)
        }
    }

    /// @dev Resolves a pointer at `mPtr + headOffset` to a memory
    ///    pointer. `mPtr` must point to some parent object with a dynamic
    ///    type's pointer stored at `mPtr + headOffset`.
    function pptr(
        MemoryPointer mPtr,
        uint256 headOffset
    ) internal pure returns (MemoryPointer mPtrChild) {
        mPtrChild = mPtr.offset(headOffset).readMemoryPointer();
    }

    /// @dev Resolves a pointer stored at `mPtr` to a memory pointer.
    ///    `mPtr` must point to some parent object with a dynamic type as its
    ///    first member, e.g. `struct { bytes data; }`
    function pptr(
        MemoryPointer mPtr
    ) internal pure returns (MemoryPointer mPtrChild) {
        mPtrChild = mPtr.readMemoryPointer();
    }
}

library CalldataReaders {
    /// @dev Reads the value at `cdPtr` and applies a mask to return only the
    ///    last 4 bytes.
    function readMaskedUint256(
        CalldataPointer cdPtr
    ) internal pure returns (uint256 value) {
        value = cdPtr.readUint256() & OffsetOrLengthMask;
    }

    /// @dev Reads the bool at `cdPtr` in calldata.
    function readBool(
        CalldataPointer cdPtr
    ) internal pure returns (bool value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the address at `cdPtr` in calldata.
    function readAddress(
        CalldataPointer cdPtr
    ) internal pure returns (address value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes1 at `cdPtr` in calldata.
    function readBytes1(
        CalldataPointer cdPtr
    ) internal pure returns (bytes1 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes2 at `cdPtr` in calldata.
    function readBytes2(
        CalldataPointer cdPtr
    ) internal pure returns (bytes2 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes3 at `cdPtr` in calldata.
    function readBytes3(
        CalldataPointer cdPtr
    ) internal pure returns (bytes3 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes4 at `cdPtr` in calldata.
    function readBytes4(
        CalldataPointer cdPtr
    ) internal pure returns (bytes4 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes5 at `cdPtr` in calldata.
    function readBytes5(
        CalldataPointer cdPtr
    ) internal pure returns (bytes5 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes6 at `cdPtr` in calldata.
    function readBytes6(
        CalldataPointer cdPtr
    ) internal pure returns (bytes6 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes7 at `cdPtr` in calldata.
    function readBytes7(
        CalldataPointer cdPtr
    ) internal pure returns (bytes7 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes8 at `cdPtr` in calldata.
    function readBytes8(
        CalldataPointer cdPtr
    ) internal pure returns (bytes8 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes9 at `cdPtr` in calldata.
    function readBytes9(
        CalldataPointer cdPtr
    ) internal pure returns (bytes9 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes10 at `cdPtr` in calldata.
    function readBytes10(
        CalldataPointer cdPtr
    ) internal pure returns (bytes10 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes11 at `cdPtr` in calldata.
    function readBytes11(
        CalldataPointer cdPtr
    ) internal pure returns (bytes11 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes12 at `cdPtr` in calldata.
    function readBytes12(
        CalldataPointer cdPtr
    ) internal pure returns (bytes12 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes13 at `cdPtr` in calldata.
    function readBytes13(
        CalldataPointer cdPtr
    ) internal pure returns (bytes13 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes14 at `cdPtr` in calldata.
    function readBytes14(
        CalldataPointer cdPtr
    ) internal pure returns (bytes14 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes15 at `cdPtr` in calldata.
    function readBytes15(
        CalldataPointer cdPtr
    ) internal pure returns (bytes15 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes16 at `cdPtr` in calldata.
    function readBytes16(
        CalldataPointer cdPtr
    ) internal pure returns (bytes16 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes17 at `cdPtr` in calldata.
    function readBytes17(
        CalldataPointer cdPtr
    ) internal pure returns (bytes17 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes18 at `cdPtr` in calldata.
    function readBytes18(
        CalldataPointer cdPtr
    ) internal pure returns (bytes18 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes19 at `cdPtr` in calldata.
    function readBytes19(
        CalldataPointer cdPtr
    ) internal pure returns (bytes19 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes20 at `cdPtr` in calldata.
    function readBytes20(
        CalldataPointer cdPtr
    ) internal pure returns (bytes20 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes21 at `cdPtr` in calldata.
    function readBytes21(
        CalldataPointer cdPtr
    ) internal pure returns (bytes21 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes22 at `cdPtr` in calldata.
    function readBytes22(
        CalldataPointer cdPtr
    ) internal pure returns (bytes22 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes23 at `cdPtr` in calldata.
    function readBytes23(
        CalldataPointer cdPtr
    ) internal pure returns (bytes23 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes24 at `cdPtr` in calldata.
    function readBytes24(
        CalldataPointer cdPtr
    ) internal pure returns (bytes24 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes25 at `cdPtr` in calldata.
    function readBytes25(
        CalldataPointer cdPtr
    ) internal pure returns (bytes25 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes26 at `cdPtr` in calldata.
    function readBytes26(
        CalldataPointer cdPtr
    ) internal pure returns (bytes26 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes27 at `cdPtr` in calldata.
    function readBytes27(
        CalldataPointer cdPtr
    ) internal pure returns (bytes27 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes28 at `cdPtr` in calldata.
    function readBytes28(
        CalldataPointer cdPtr
    ) internal pure returns (bytes28 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes29 at `cdPtr` in calldata.
    function readBytes29(
        CalldataPointer cdPtr
    ) internal pure returns (bytes29 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes30 at `cdPtr` in calldata.
    function readBytes30(
        CalldataPointer cdPtr
    ) internal pure returns (bytes30 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes31 at `cdPtr` in calldata.
    function readBytes31(
        CalldataPointer cdPtr
    ) internal pure returns (bytes31 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the bytes32 at `cdPtr` in calldata.
    function readBytes32(
        CalldataPointer cdPtr
    ) internal pure returns (bytes32 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint8 at `cdPtr` in calldata.
    function readUint8(
        CalldataPointer cdPtr
    ) internal pure returns (uint8 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint16 at `cdPtr` in calldata.
    function readUint16(
        CalldataPointer cdPtr
    ) internal pure returns (uint16 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint24 at `cdPtr` in calldata.
    function readUint24(
        CalldataPointer cdPtr
    ) internal pure returns (uint24 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint32 at `cdPtr` in calldata.
    function readUint32(
        CalldataPointer cdPtr
    ) internal pure returns (uint32 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint40 at `cdPtr` in calldata.
    function readUint40(
        CalldataPointer cdPtr
    ) internal pure returns (uint40 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint48 at `cdPtr` in calldata.
    function readUint48(
        CalldataPointer cdPtr
    ) internal pure returns (uint48 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint56 at `cdPtr` in calldata.
    function readUint56(
        CalldataPointer cdPtr
    ) internal pure returns (uint56 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint64 at `cdPtr` in calldata.
    function readUint64(
        CalldataPointer cdPtr
    ) internal pure returns (uint64 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint72 at `cdPtr` in calldata.
    function readUint72(
        CalldataPointer cdPtr
    ) internal pure returns (uint72 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint80 at `cdPtr` in calldata.
    function readUint80(
        CalldataPointer cdPtr
    ) internal pure returns (uint80 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint88 at `cdPtr` in calldata.
    function readUint88(
        CalldataPointer cdPtr
    ) internal pure returns (uint88 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint96 at `cdPtr` in calldata.
    function readUint96(
        CalldataPointer cdPtr
    ) internal pure returns (uint96 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint104 at `cdPtr` in calldata.
    function readUint104(
        CalldataPointer cdPtr
    ) internal pure returns (uint104 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint112 at `cdPtr` in calldata.
    function readUint112(
        CalldataPointer cdPtr
    ) internal pure returns (uint112 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint120 at `cdPtr` in calldata.
    function readUint120(
        CalldataPointer cdPtr
    ) internal pure returns (uint120 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint128 at `cdPtr` in calldata.
    function readUint128(
        CalldataPointer cdPtr
    ) internal pure returns (uint128 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint136 at `cdPtr` in calldata.
    function readUint136(
        CalldataPointer cdPtr
    ) internal pure returns (uint136 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint144 at `cdPtr` in calldata.
    function readUint144(
        CalldataPointer cdPtr
    ) internal pure returns (uint144 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint152 at `cdPtr` in calldata.
    function readUint152(
        CalldataPointer cdPtr
    ) internal pure returns (uint152 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint160 at `cdPtr` in calldata.
    function readUint160(
        CalldataPointer cdPtr
    ) internal pure returns (uint160 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint168 at `cdPtr` in calldata.
    function readUint168(
        CalldataPointer cdPtr
    ) internal pure returns (uint168 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint176 at `cdPtr` in calldata.
    function readUint176(
        CalldataPointer cdPtr
    ) internal pure returns (uint176 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint184 at `cdPtr` in calldata.
    function readUint184(
        CalldataPointer cdPtr
    ) internal pure returns (uint184 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint192 at `cdPtr` in calldata.
    function readUint192(
        CalldataPointer cdPtr
    ) internal pure returns (uint192 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint200 at `cdPtr` in calldata.
    function readUint200(
        CalldataPointer cdPtr
    ) internal pure returns (uint200 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint208 at `cdPtr` in calldata.
    function readUint208(
        CalldataPointer cdPtr
    ) internal pure returns (uint208 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint216 at `cdPtr` in calldata.
    function readUint216(
        CalldataPointer cdPtr
    ) internal pure returns (uint216 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint224 at `cdPtr` in calldata.
    function readUint224(
        CalldataPointer cdPtr
    ) internal pure returns (uint224 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint232 at `cdPtr` in calldata.
    function readUint232(
        CalldataPointer cdPtr
    ) internal pure returns (uint232 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint240 at `cdPtr` in calldata.
    function readUint240(
        CalldataPointer cdPtr
    ) internal pure returns (uint240 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint248 at `cdPtr` in calldata.
    function readUint248(
        CalldataPointer cdPtr
    ) internal pure returns (uint248 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the uint256 at `cdPtr` in calldata.
    function readUint256(
        CalldataPointer cdPtr
    ) internal pure returns (uint256 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int8 at `cdPtr` in calldata.
    function readInt8(
        CalldataPointer cdPtr
    ) internal pure returns (int8 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int16 at `cdPtr` in calldata.
    function readInt16(
        CalldataPointer cdPtr
    ) internal pure returns (int16 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int24 at `cdPtr` in calldata.
    function readInt24(
        CalldataPointer cdPtr
    ) internal pure returns (int24 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int32 at `cdPtr` in calldata.
    function readInt32(
        CalldataPointer cdPtr
    ) internal pure returns (int32 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int40 at `cdPtr` in calldata.
    function readInt40(
        CalldataPointer cdPtr
    ) internal pure returns (int40 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int48 at `cdPtr` in calldata.
    function readInt48(
        CalldataPointer cdPtr
    ) internal pure returns (int48 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int56 at `cdPtr` in calldata.
    function readInt56(
        CalldataPointer cdPtr
    ) internal pure returns (int56 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int64 at `cdPtr` in calldata.
    function readInt64(
        CalldataPointer cdPtr
    ) internal pure returns (int64 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int72 at `cdPtr` in calldata.
    function readInt72(
        CalldataPointer cdPtr
    ) internal pure returns (int72 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int80 at `cdPtr` in calldata.
    function readInt80(
        CalldataPointer cdPtr
    ) internal pure returns (int80 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int88 at `cdPtr` in calldata.
    function readInt88(
        CalldataPointer cdPtr
    ) internal pure returns (int88 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int96 at `cdPtr` in calldata.
    function readInt96(
        CalldataPointer cdPtr
    ) internal pure returns (int96 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int104 at `cdPtr` in calldata.
    function readInt104(
        CalldataPointer cdPtr
    ) internal pure returns (int104 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int112 at `cdPtr` in calldata.
    function readInt112(
        CalldataPointer cdPtr
    ) internal pure returns (int112 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int120 at `cdPtr` in calldata.
    function readInt120(
        CalldataPointer cdPtr
    ) internal pure returns (int120 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int128 at `cdPtr` in calldata.
    function readInt128(
        CalldataPointer cdPtr
    ) internal pure returns (int128 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int136 at `cdPtr` in calldata.
    function readInt136(
        CalldataPointer cdPtr
    ) internal pure returns (int136 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int144 at `cdPtr` in calldata.
    function readInt144(
        CalldataPointer cdPtr
    ) internal pure returns (int144 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int152 at `cdPtr` in calldata.
    function readInt152(
        CalldataPointer cdPtr
    ) internal pure returns (int152 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int160 at `cdPtr` in calldata.
    function readInt160(
        CalldataPointer cdPtr
    ) internal pure returns (int160 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int168 at `cdPtr` in calldata.
    function readInt168(
        CalldataPointer cdPtr
    ) internal pure returns (int168 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int176 at `cdPtr` in calldata.
    function readInt176(
        CalldataPointer cdPtr
    ) internal pure returns (int176 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int184 at `cdPtr` in calldata.
    function readInt184(
        CalldataPointer cdPtr
    ) internal pure returns (int184 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int192 at `cdPtr` in calldata.
    function readInt192(
        CalldataPointer cdPtr
    ) internal pure returns (int192 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int200 at `cdPtr` in calldata.
    function readInt200(
        CalldataPointer cdPtr
    ) internal pure returns (int200 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int208 at `cdPtr` in calldata.
    function readInt208(
        CalldataPointer cdPtr
    ) internal pure returns (int208 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int216 at `cdPtr` in calldata.
    function readInt216(
        CalldataPointer cdPtr
    ) internal pure returns (int216 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int224 at `cdPtr` in calldata.
    function readInt224(
        CalldataPointer cdPtr
    ) internal pure returns (int224 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int232 at `cdPtr` in calldata.
    function readInt232(
        CalldataPointer cdPtr
    ) internal pure returns (int232 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int240 at `cdPtr` in calldata.
    function readInt240(
        CalldataPointer cdPtr
    ) internal pure returns (int240 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int248 at `cdPtr` in calldata.
    function readInt248(
        CalldataPointer cdPtr
    ) internal pure returns (int248 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }

    /// @dev Reads the int256 at `cdPtr` in calldata.
    function readInt256(
        CalldataPointer cdPtr
    ) internal pure returns (int256 value) {
        assembly {
            value := calldataload(cdPtr)
        }
    }
}

library ReturndataReaders {
    /// @dev Reads value at `rdPtr` & applies a mask to return only last 4 bytes
    function readMaskedUint256(
        ReturndataPointer rdPtr
    ) internal pure returns (uint256 value) {
        value = rdPtr.readUint256() & OffsetOrLengthMask;
    }

    /// @dev Reads the bool at `rdPtr` in returndata.
    function readBool(
        ReturndataPointer rdPtr
    ) internal pure returns (bool value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the address at `rdPtr` in returndata.
    function readAddress(
        ReturndataPointer rdPtr
    ) internal pure returns (address value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes1 at `rdPtr` in returndata.
    function readBytes1(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes1 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes2 at `rdPtr` in returndata.
    function readBytes2(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes2 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes3 at `rdPtr` in returndata.
    function readBytes3(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes3 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes4 at `rdPtr` in returndata.
    function readBytes4(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes4 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes5 at `rdPtr` in returndata.
    function readBytes5(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes5 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes6 at `rdPtr` in returndata.
    function readBytes6(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes6 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes7 at `rdPtr` in returndata.
    function readBytes7(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes7 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes8 at `rdPtr` in returndata.
    function readBytes8(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes8 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes9 at `rdPtr` in returndata.
    function readBytes9(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes9 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes10 at `rdPtr` in returndata.
    function readBytes10(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes10 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes11 at `rdPtr` in returndata.
    function readBytes11(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes11 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes12 at `rdPtr` in returndata.
    function readBytes12(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes12 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes13 at `rdPtr` in returndata.
    function readBytes13(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes13 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes14 at `rdPtr` in returndata.
    function readBytes14(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes14 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes15 at `rdPtr` in returndata.
    function readBytes15(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes15 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes16 at `rdPtr` in returndata.
    function readBytes16(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes16 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes17 at `rdPtr` in returndata.
    function readBytes17(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes17 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes18 at `rdPtr` in returndata.
    function readBytes18(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes18 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes19 at `rdPtr` in returndata.
    function readBytes19(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes19 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes20 at `rdPtr` in returndata.
    function readBytes20(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes20 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes21 at `rdPtr` in returndata.
    function readBytes21(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes21 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes22 at `rdPtr` in returndata.
    function readBytes22(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes22 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes23 at `rdPtr` in returndata.
    function readBytes23(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes23 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes24 at `rdPtr` in returndata.
    function readBytes24(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes24 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes25 at `rdPtr` in returndata.
    function readBytes25(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes25 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes26 at `rdPtr` in returndata.
    function readBytes26(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes26 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes27 at `rdPtr` in returndata.
    function readBytes27(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes27 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes28 at `rdPtr` in returndata.
    function readBytes28(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes28 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes29 at `rdPtr` in returndata.
    function readBytes29(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes29 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes30 at `rdPtr` in returndata.
    function readBytes30(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes30 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes31 at `rdPtr` in returndata.
    function readBytes31(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes31 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the bytes32 at `rdPtr` in returndata.
    function readBytes32(
        ReturndataPointer rdPtr
    ) internal pure returns (bytes32 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint8 at `rdPtr` in returndata.
    function readUint8(
        ReturndataPointer rdPtr
    ) internal pure returns (uint8 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint16 at `rdPtr` in returndata.
    function readUint16(
        ReturndataPointer rdPtr
    ) internal pure returns (uint16 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint24 at `rdPtr` in returndata.
    function readUint24(
        ReturndataPointer rdPtr
    ) internal pure returns (uint24 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint32 at `rdPtr` in returndata.
    function readUint32(
        ReturndataPointer rdPtr
    ) internal pure returns (uint32 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint40 at `rdPtr` in returndata.
    function readUint40(
        ReturndataPointer rdPtr
    ) internal pure returns (uint40 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint48 at `rdPtr` in returndata.
    function readUint48(
        ReturndataPointer rdPtr
    ) internal pure returns (uint48 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint56 at `rdPtr` in returndata.
    function readUint56(
        ReturndataPointer rdPtr
    ) internal pure returns (uint56 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint64 at `rdPtr` in returndata.
    function readUint64(
        ReturndataPointer rdPtr
    ) internal pure returns (uint64 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint72 at `rdPtr` in returndata.
    function readUint72(
        ReturndataPointer rdPtr
    ) internal pure returns (uint72 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint80 at `rdPtr` in returndata.
    function readUint80(
        ReturndataPointer rdPtr
    ) internal pure returns (uint80 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint88 at `rdPtr` in returndata.
    function readUint88(
        ReturndataPointer rdPtr
    ) internal pure returns (uint88 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint96 at `rdPtr` in returndata.
    function readUint96(
        ReturndataPointer rdPtr
    ) internal pure returns (uint96 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint104 at `rdPtr` in returndata.
    function readUint104(
        ReturndataPointer rdPtr
    ) internal pure returns (uint104 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint112 at `rdPtr` in returndata.
    function readUint112(
        ReturndataPointer rdPtr
    ) internal pure returns (uint112 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint120 at `rdPtr` in returndata.
    function readUint120(
        ReturndataPointer rdPtr
    ) internal pure returns (uint120 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint128 at `rdPtr` in returndata.
    function readUint128(
        ReturndataPointer rdPtr
    ) internal pure returns (uint128 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint136 at `rdPtr` in returndata.
    function readUint136(
        ReturndataPointer rdPtr
    ) internal pure returns (uint136 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint144 at `rdPtr` in returndata.
    function readUint144(
        ReturndataPointer rdPtr
    ) internal pure returns (uint144 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint152 at `rdPtr` in returndata.
    function readUint152(
        ReturndataPointer rdPtr
    ) internal pure returns (uint152 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint160 at `rdPtr` in returndata.
    function readUint160(
        ReturndataPointer rdPtr
    ) internal pure returns (uint160 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint168 at `rdPtr` in returndata.
    function readUint168(
        ReturndataPointer rdPtr
    ) internal pure returns (uint168 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint176 at `rdPtr` in returndata.
    function readUint176(
        ReturndataPointer rdPtr
    ) internal pure returns (uint176 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint184 at `rdPtr` in returndata.
    function readUint184(
        ReturndataPointer rdPtr
    ) internal pure returns (uint184 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint192 at `rdPtr` in returndata.
    function readUint192(
        ReturndataPointer rdPtr
    ) internal pure returns (uint192 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint200 at `rdPtr` in returndata.
    function readUint200(
        ReturndataPointer rdPtr
    ) internal pure returns (uint200 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint208 at `rdPtr` in returndata.
    function readUint208(
        ReturndataPointer rdPtr
    ) internal pure returns (uint208 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint216 at `rdPtr` in returndata.
    function readUint216(
        ReturndataPointer rdPtr
    ) internal pure returns (uint216 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint224 at `rdPtr` in returndata.
    function readUint224(
        ReturndataPointer rdPtr
    ) internal pure returns (uint224 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint232 at `rdPtr` in returndata.
    function readUint232(
        ReturndataPointer rdPtr
    ) internal pure returns (uint232 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint240 at `rdPtr` in returndata.
    function readUint240(
        ReturndataPointer rdPtr
    ) internal pure returns (uint240 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint248 at `rdPtr` in returndata.
    function readUint248(
        ReturndataPointer rdPtr
    ) internal pure returns (uint248 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the uint256 at `rdPtr` in returndata.
    function readUint256(
        ReturndataPointer rdPtr
    ) internal pure returns (uint256 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int8 at `rdPtr` in returndata.
    function readInt8(
        ReturndataPointer rdPtr
    ) internal pure returns (int8 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int16 at `rdPtr` in returndata.
    function readInt16(
        ReturndataPointer rdPtr
    ) internal pure returns (int16 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int24 at `rdPtr` in returndata.
    function readInt24(
        ReturndataPointer rdPtr
    ) internal pure returns (int24 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int32 at `rdPtr` in returndata.
    function readInt32(
        ReturndataPointer rdPtr
    ) internal pure returns (int32 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int40 at `rdPtr` in returndata.
    function readInt40(
        ReturndataPointer rdPtr
    ) internal pure returns (int40 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int48 at `rdPtr` in returndata.
    function readInt48(
        ReturndataPointer rdPtr
    ) internal pure returns (int48 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int56 at `rdPtr` in returndata.
    function readInt56(
        ReturndataPointer rdPtr
    ) internal pure returns (int56 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int64 at `rdPtr` in returndata.
    function readInt64(
        ReturndataPointer rdPtr
    ) internal pure returns (int64 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int72 at `rdPtr` in returndata.
    function readInt72(
        ReturndataPointer rdPtr
    ) internal pure returns (int72 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int80 at `rdPtr` in returndata.
    function readInt80(
        ReturndataPointer rdPtr
    ) internal pure returns (int80 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int88 at `rdPtr` in returndata.
    function readInt88(
        ReturndataPointer rdPtr
    ) internal pure returns (int88 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int96 at `rdPtr` in returndata.
    function readInt96(
        ReturndataPointer rdPtr
    ) internal pure returns (int96 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int104 at `rdPtr` in returndata.
    function readInt104(
        ReturndataPointer rdPtr
    ) internal pure returns (int104 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int112 at `rdPtr` in returndata.
    function readInt112(
        ReturndataPointer rdPtr
    ) internal pure returns (int112 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int120 at `rdPtr` in returndata.
    function readInt120(
        ReturndataPointer rdPtr
    ) internal pure returns (int120 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int128 at `rdPtr` in returndata.
    function readInt128(
        ReturndataPointer rdPtr
    ) internal pure returns (int128 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int136 at `rdPtr` in returndata.
    function readInt136(
        ReturndataPointer rdPtr
    ) internal pure returns (int136 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int144 at `rdPtr` in returndata.
    function readInt144(
        ReturndataPointer rdPtr
    ) internal pure returns (int144 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int152 at `rdPtr` in returndata.
    function readInt152(
        ReturndataPointer rdPtr
    ) internal pure returns (int152 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int160 at `rdPtr` in returndata.
    function readInt160(
        ReturndataPointer rdPtr
    ) internal pure returns (int160 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int168 at `rdPtr` in returndata.
    function readInt168(
        ReturndataPointer rdPtr
    ) internal pure returns (int168 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int176 at `rdPtr` in returndata.
    function readInt176(
        ReturndataPointer rdPtr
    ) internal pure returns (int176 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int184 at `rdPtr` in returndata.
    function readInt184(
        ReturndataPointer rdPtr
    ) internal pure returns (int184 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int192 at `rdPtr` in returndata.
    function readInt192(
        ReturndataPointer rdPtr
    ) internal pure returns (int192 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int200 at `rdPtr` in returndata.
    function readInt200(
        ReturndataPointer rdPtr
    ) internal pure returns (int200 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int208 at `rdPtr` in returndata.
    function readInt208(
        ReturndataPointer rdPtr
    ) internal pure returns (int208 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int216 at `rdPtr` in returndata.
    function readInt216(
        ReturndataPointer rdPtr
    ) internal pure returns (int216 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int224 at `rdPtr` in returndata.
    function readInt224(
        ReturndataPointer rdPtr
    ) internal pure returns (int224 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int232 at `rdPtr` in returndata.
    function readInt232(
        ReturndataPointer rdPtr
    ) internal pure returns (int232 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int240 at `rdPtr` in returndata.
    function readInt240(
        ReturndataPointer rdPtr
    ) internal pure returns (int240 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int248 at `rdPtr` in returndata.
    function readInt248(
        ReturndataPointer rdPtr
    ) internal pure returns (int248 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }

    /// @dev Reads the int256 at `rdPtr` in returndata.
    function readInt256(
        ReturndataPointer rdPtr
    ) internal pure returns (int256 value) {
        assembly {
            returndatacopy(0, rdPtr, _OneWord)
            value := mload(0)
        }
    }
}

library MemoryReaders {
    /// @dev Reads the memory pointer at `mPtr` in memory.
    function readMemoryPointer(
        MemoryPointer mPtr
    ) internal pure returns (MemoryPointer value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads value at `mPtr` & applies a mask to return only last 4 bytes
    function readMaskedUint256(
        MemoryPointer mPtr
    ) internal pure returns (uint256 value) {
        value = mPtr.readUint256() & OffsetOrLengthMask;
    }

    /// @dev Reads the bool at `mPtr` in memory.
    function readBool(MemoryPointer mPtr) internal pure returns (bool value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the address at `mPtr` in memory.
    function readAddress(
        MemoryPointer mPtr
    ) internal pure returns (address value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes1 at `mPtr` in memory.
    function readBytes1(
        MemoryPointer mPtr
    ) internal pure returns (bytes1 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes2 at `mPtr` in memory.
    function readBytes2(
        MemoryPointer mPtr
    ) internal pure returns (bytes2 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes3 at `mPtr` in memory.
    function readBytes3(
        MemoryPointer mPtr
    ) internal pure returns (bytes3 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes4 at `mPtr` in memory.
    function readBytes4(
        MemoryPointer mPtr
    ) internal pure returns (bytes4 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes5 at `mPtr` in memory.
    function readBytes5(
        MemoryPointer mPtr
    ) internal pure returns (bytes5 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes6 at `mPtr` in memory.
    function readBytes6(
        MemoryPointer mPtr
    ) internal pure returns (bytes6 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes7 at `mPtr` in memory.
    function readBytes7(
        MemoryPointer mPtr
    ) internal pure returns (bytes7 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes8 at `mPtr` in memory.
    function readBytes8(
        MemoryPointer mPtr
    ) internal pure returns (bytes8 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes9 at `mPtr` in memory.
    function readBytes9(
        MemoryPointer mPtr
    ) internal pure returns (bytes9 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes10 at `mPtr` in memory.
    function readBytes10(
        MemoryPointer mPtr
    ) internal pure returns (bytes10 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes11 at `mPtr` in memory.
    function readBytes11(
        MemoryPointer mPtr
    ) internal pure returns (bytes11 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes12 at `mPtr` in memory.
    function readBytes12(
        MemoryPointer mPtr
    ) internal pure returns (bytes12 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes13 at `mPtr` in memory.
    function readBytes13(
        MemoryPointer mPtr
    ) internal pure returns (bytes13 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes14 at `mPtr` in memory.
    function readBytes14(
        MemoryPointer mPtr
    ) internal pure returns (bytes14 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes15 at `mPtr` in memory.
    function readBytes15(
        MemoryPointer mPtr
    ) internal pure returns (bytes15 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes16 at `mPtr` in memory.
    function readBytes16(
        MemoryPointer mPtr
    ) internal pure returns (bytes16 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes17 at `mPtr` in memory.
    function readBytes17(
        MemoryPointer mPtr
    ) internal pure returns (bytes17 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes18 at `mPtr` in memory.
    function readBytes18(
        MemoryPointer mPtr
    ) internal pure returns (bytes18 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes19 at `mPtr` in memory.
    function readBytes19(
        MemoryPointer mPtr
    ) internal pure returns (bytes19 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes20 at `mPtr` in memory.
    function readBytes20(
        MemoryPointer mPtr
    ) internal pure returns (bytes20 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes21 at `mPtr` in memory.
    function readBytes21(
        MemoryPointer mPtr
    ) internal pure returns (bytes21 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes22 at `mPtr` in memory.
    function readBytes22(
        MemoryPointer mPtr
    ) internal pure returns (bytes22 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes23 at `mPtr` in memory.
    function readBytes23(
        MemoryPointer mPtr
    ) internal pure returns (bytes23 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes24 at `mPtr` in memory.
    function readBytes24(
        MemoryPointer mPtr
    ) internal pure returns (bytes24 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes25 at `mPtr` in memory.
    function readBytes25(
        MemoryPointer mPtr
    ) internal pure returns (bytes25 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes26 at `mPtr` in memory.
    function readBytes26(
        MemoryPointer mPtr
    ) internal pure returns (bytes26 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes27 at `mPtr` in memory.
    function readBytes27(
        MemoryPointer mPtr
    ) internal pure returns (bytes27 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes28 at `mPtr` in memory.
    function readBytes28(
        MemoryPointer mPtr
    ) internal pure returns (bytes28 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes29 at `mPtr` in memory.
    function readBytes29(
        MemoryPointer mPtr
    ) internal pure returns (bytes29 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes30 at `mPtr` in memory.
    function readBytes30(
        MemoryPointer mPtr
    ) internal pure returns (bytes30 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes31 at `mPtr` in memory.
    function readBytes31(
        MemoryPointer mPtr
    ) internal pure returns (bytes31 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the bytes32 at `mPtr` in memory.
    function readBytes32(
        MemoryPointer mPtr
    ) internal pure returns (bytes32 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint8 at `mPtr` in memory.
    function readUint8(MemoryPointer mPtr) internal pure returns (uint8 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint16 at `mPtr` in memory.
    function readUint16(
        MemoryPointer mPtr
    ) internal pure returns (uint16 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint24 at `mPtr` in memory.
    function readUint24(
        MemoryPointer mPtr
    ) internal pure returns (uint24 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint32 at `mPtr` in memory.
    function readUint32(
        MemoryPointer mPtr
    ) internal pure returns (uint32 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint40 at `mPtr` in memory.
    function readUint40(
        MemoryPointer mPtr
    ) internal pure returns (uint40 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint48 at `mPtr` in memory.
    function readUint48(
        MemoryPointer mPtr
    ) internal pure returns (uint48 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint56 at `mPtr` in memory.
    function readUint56(
        MemoryPointer mPtr
    ) internal pure returns (uint56 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint64 at `mPtr` in memory.
    function readUint64(
        MemoryPointer mPtr
    ) internal pure returns (uint64 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint72 at `mPtr` in memory.
    function readUint72(
        MemoryPointer mPtr
    ) internal pure returns (uint72 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint80 at `mPtr` in memory.
    function readUint80(
        MemoryPointer mPtr
    ) internal pure returns (uint80 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint88 at `mPtr` in memory.
    function readUint88(
        MemoryPointer mPtr
    ) internal pure returns (uint88 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint96 at `mPtr` in memory.
    function readUint96(
        MemoryPointer mPtr
    ) internal pure returns (uint96 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint104 at `mPtr` in memory.
    function readUint104(
        MemoryPointer mPtr
    ) internal pure returns (uint104 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint112 at `mPtr` in memory.
    function readUint112(
        MemoryPointer mPtr
    ) internal pure returns (uint112 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint120 at `mPtr` in memory.
    function readUint120(
        MemoryPointer mPtr
    ) internal pure returns (uint120 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint128 at `mPtr` in memory.
    function readUint128(
        MemoryPointer mPtr
    ) internal pure returns (uint128 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint136 at `mPtr` in memory.
    function readUint136(
        MemoryPointer mPtr
    ) internal pure returns (uint136 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint144 at `mPtr` in memory.
    function readUint144(
        MemoryPointer mPtr
    ) internal pure returns (uint144 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint152 at `mPtr` in memory.
    function readUint152(
        MemoryPointer mPtr
    ) internal pure returns (uint152 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint160 at `mPtr` in memory.
    function readUint160(
        MemoryPointer mPtr
    ) internal pure returns (uint160 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint168 at `mPtr` in memory.
    function readUint168(
        MemoryPointer mPtr
    ) internal pure returns (uint168 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint176 at `mPtr` in memory.
    function readUint176(
        MemoryPointer mPtr
    ) internal pure returns (uint176 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint184 at `mPtr` in memory.
    function readUint184(
        MemoryPointer mPtr
    ) internal pure returns (uint184 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint192 at `mPtr` in memory.
    function readUint192(
        MemoryPointer mPtr
    ) internal pure returns (uint192 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint200 at `mPtr` in memory.
    function readUint200(
        MemoryPointer mPtr
    ) internal pure returns (uint200 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint208 at `mPtr` in memory.
    function readUint208(
        MemoryPointer mPtr
    ) internal pure returns (uint208 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint216 at `mPtr` in memory.
    function readUint216(
        MemoryPointer mPtr
    ) internal pure returns (uint216 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint224 at `mPtr` in memory.
    function readUint224(
        MemoryPointer mPtr
    ) internal pure returns (uint224 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint232 at `mPtr` in memory.
    function readUint232(
        MemoryPointer mPtr
    ) internal pure returns (uint232 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint240 at `mPtr` in memory.
    function readUint240(
        MemoryPointer mPtr
    ) internal pure returns (uint240 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint248 at `mPtr` in memory.
    function readUint248(
        MemoryPointer mPtr
    ) internal pure returns (uint248 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the uint256 at `mPtr` in memory.
    function readUint256(
        MemoryPointer mPtr
    ) internal pure returns (uint256 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int8 at `mPtr` in memory.
    function readInt8(MemoryPointer mPtr) internal pure returns (int8 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int16 at `mPtr` in memory.
    function readInt16(MemoryPointer mPtr) internal pure returns (int16 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int24 at `mPtr` in memory.
    function readInt24(MemoryPointer mPtr) internal pure returns (int24 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int32 at `mPtr` in memory.
    function readInt32(MemoryPointer mPtr) internal pure returns (int32 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int40 at `mPtr` in memory.
    function readInt40(MemoryPointer mPtr) internal pure returns (int40 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int48 at `mPtr` in memory.
    function readInt48(MemoryPointer mPtr) internal pure returns (int48 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int56 at `mPtr` in memory.
    function readInt56(MemoryPointer mPtr) internal pure returns (int56 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int64 at `mPtr` in memory.
    function readInt64(MemoryPointer mPtr) internal pure returns (int64 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int72 at `mPtr` in memory.
    function readInt72(MemoryPointer mPtr) internal pure returns (int72 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int80 at `mPtr` in memory.
    function readInt80(MemoryPointer mPtr) internal pure returns (int80 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int88 at `mPtr` in memory.
    function readInt88(MemoryPointer mPtr) internal pure returns (int88 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int96 at `mPtr` in memory.
    function readInt96(MemoryPointer mPtr) internal pure returns (int96 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int104 at `mPtr` in memory.
    function readInt104(
        MemoryPointer mPtr
    ) internal pure returns (int104 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int112 at `mPtr` in memory.
    function readInt112(
        MemoryPointer mPtr
    ) internal pure returns (int112 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int120 at `mPtr` in memory.
    function readInt120(
        MemoryPointer mPtr
    ) internal pure returns (int120 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int128 at `mPtr` in memory.
    function readInt128(
        MemoryPointer mPtr
    ) internal pure returns (int128 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int136 at `mPtr` in memory.
    function readInt136(
        MemoryPointer mPtr
    ) internal pure returns (int136 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int144 at `mPtr` in memory.
    function readInt144(
        MemoryPointer mPtr
    ) internal pure returns (int144 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int152 at `mPtr` in memory.
    function readInt152(
        MemoryPointer mPtr
    ) internal pure returns (int152 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int160 at `mPtr` in memory.
    function readInt160(
        MemoryPointer mPtr
    ) internal pure returns (int160 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int168 at `mPtr` in memory.
    function readInt168(
        MemoryPointer mPtr
    ) internal pure returns (int168 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int176 at `mPtr` in memory.
    function readInt176(
        MemoryPointer mPtr
    ) internal pure returns (int176 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int184 at `mPtr` in memory.
    function readInt184(
        MemoryPointer mPtr
    ) internal pure returns (int184 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int192 at `mPtr` in memory.
    function readInt192(
        MemoryPointer mPtr
    ) internal pure returns (int192 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int200 at `mPtr` in memory.
    function readInt200(
        MemoryPointer mPtr
    ) internal pure returns (int200 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int208 at `mPtr` in memory.
    function readInt208(
        MemoryPointer mPtr
    ) internal pure returns (int208 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int216 at `mPtr` in memory.
    function readInt216(
        MemoryPointer mPtr
    ) internal pure returns (int216 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int224 at `mPtr` in memory.
    function readInt224(
        MemoryPointer mPtr
    ) internal pure returns (int224 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int232 at `mPtr` in memory.
    function readInt232(
        MemoryPointer mPtr
    ) internal pure returns (int232 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int240 at `mPtr` in memory.
    function readInt240(
        MemoryPointer mPtr
    ) internal pure returns (int240 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int248 at `mPtr` in memory.
    function readInt248(
        MemoryPointer mPtr
    ) internal pure returns (int248 value) {
        assembly {
            value := mload(mPtr)
        }
    }

    /// @dev Reads the int256 at `mPtr` in memory.
    function readInt256(
        MemoryPointer mPtr
    ) internal pure returns (int256 value) {
        assembly {
            value := mload(mPtr)
        }
    }
}

library MemoryWriters {
    /// @dev Writes `valuePtr` to memory at `mPtr`.
    function write(MemoryPointer mPtr, MemoryPointer valuePtr) internal pure {
        assembly {
            mstore(mPtr, valuePtr)
        }
    }

    /// @dev Writes a boolean `value` to `mPtr` in memory.
    function write(MemoryPointer mPtr, bool value) internal pure {
        assembly {
            mstore(mPtr, value)
        }
    }

    /// @dev Writes an address `value` to `mPtr` in memory.
    function write(MemoryPointer mPtr, address value) internal pure {
        assembly {
            mstore(mPtr, value)
        }
    }

    /// @dev Writes a bytes32 `value` to `mPtr` in memory.
    /// Separate name to disambiguate literal write parameters.
    function writeBytes32(MemoryPointer mPtr, bytes32 value) internal pure {
        assembly {
            mstore(mPtr, value)
        }
    }

    /// @dev Writes a uint256 `value` to `mPtr` in memory.
    function write(MemoryPointer mPtr, uint256 value) internal pure {
        assembly {
            mstore(mPtr, value)
        }
    }

    /// @dev Writes an int256 `value` to `mPtr` in memory.
    /// Separate name to disambiguate literal write parameters.
    function writeInt(MemoryPointer mPtr, int256 value) internal pure {
        assembly {
            mstore(mPtr, value)
        }
    }
}

/**
 * @dev An order contains eleven components: an offerer, a zone (or account that
 *      can cancel the order or restrict who can fulfill the order depending on
 *      the type), the order type (specifying partial fill support as well as
 *      restricted order status), the start and end time, a hash that will be
 *      provided to the zone when validating restricted orders, a salt, a key
 *      corresponding to a given conduit, a counter, and an arbitrary number of
 *      offer items that can be spent along with consideration items that must
 *      be received by their respective recipient.
 */
struct OrderComponents {
    address offerer;
    address zone;
    OfferItem[] offer;
    ConsiderationItem[] consideration;
    OrderType orderType;
    uint256 startTime;
    uint256 endTime;
    bytes32 zoneHash;
    uint256 salt;
    bytes32 conduitKey;
    uint256 counter;
}

/**
 * @dev An offer item has five components: an item type (ETH or other native
 *      tokens, ERC20, ERC721, and ERC1155, as well as criteria-based ERC721 and
 *      ERC1155), a token address, a dual-purpose "identifierOrCriteria"
 *      component that will either represent a tokenId or a merkle root
 *      depending on the item type, and a start and end amount that support
 *      increasing or decreasing amounts over the duration of the respective
 *      order.
 */
struct OfferItem {
    ItemType itemType;
    address token;
    uint256 identifierOrCriteria;
    uint256 startAmount;
    uint256 endAmount;
}

/**
 * @dev A consideration item has the same five components as an offer item and
 *      an additional sixth component designating the required recipient of the
 *      item.
 */
struct ConsiderationItem {
    ItemType itemType;
    address token;
    uint256 identifierOrCriteria;
    uint256 startAmount;
    uint256 endAmount;
    address payable recipient;
}

/**
 * @dev A spent item is translated from a utilized offer item and has four
 *      components: an item type (ETH or other native tokens, ERC20, ERC721, and
 *      ERC1155), a token address, a tokenId, and an amount.
 */
struct SpentItem {
    ItemType itemType;
    address token;
    uint256 identifier;
    uint256 amount;
}

/**
 * @dev A received item is translated from a utilized consideration item and has
 *      the same four components as a spent item, as well as an additional fifth
 *      component designating the required recipient of the item.
 */
struct ReceivedItem {
    ItemType itemType;
    address token;
    uint256 identifier;
    uint256 amount;
    address payable recipient;
}

/**
 * @dev For basic orders involving ETH / native / ERC20 <=> ERC721 / ERC1155
 *      matching, a group of six functions may be called that only requires a
 *      subset of the usual order arguments. Note the use of a "basicOrderType"
 *      enum; this represents both the usual order type as well as the "route"
 *      of the basic order (a simple derivation function for the basic order
 *      type is `basicOrderType = orderType + (4 * basicOrderRoute)`.)
 */
struct BasicOrderParameters {
    // calldata offset
    address considerationToken; // 0x24
    uint256 considerationIdentifier; // 0x44
    uint256 considerationAmount; // 0x64
    address payable offerer; // 0x84
    address zone; // 0xa4
    address offerToken; // 0xc4
    uint256 offerIdentifier; // 0xe4
    uint256 offerAmount; // 0x104
    BasicOrderType basicOrderType; // 0x124
    uint256 startTime; // 0x144
    uint256 endTime; // 0x164
    bytes32 zoneHash; // 0x184
    uint256 salt; // 0x1a4
    bytes32 offererConduitKey; // 0x1c4
    bytes32 fulfillerConduitKey; // 0x1e4
    uint256 totalOriginalAdditionalRecipients; // 0x204
    AdditionalRecipient[] additionalRecipients; // 0x224
    bytes signature; // 0x244
    // Total length, excluding dynamic array data: 0x264 (580)
}

/**
 * @dev Basic orders can supply any number of additional recipients, with the
 *      implied assumption that they are supplied from the offered ETH (or other
 *      native token) or ERC20 token for the order.
 */
struct AdditionalRecipient {
    uint256 amount;
    address payable recipient;
}

/**
 * @dev The full set of order components, with the exception of the counter,
 *      must be supplied when fulfilling more sophisticated orders or groups of
 *      orders. The total number of original consideration items must also be
 *      supplied, as the caller may specify additional consideration items.
 */
struct OrderParameters {
    address offerer; // 0x00
    address zone; // 0x20
    OfferItem[] offer; // 0x40
    ConsiderationItem[] consideration; // 0x60
    OrderType orderType; // 0x80
    uint256 startTime; // 0xa0
    uint256 endTime; // 0xc0
    bytes32 zoneHash; // 0xe0
    uint256 salt; // 0x100
    bytes32 conduitKey; // 0x120
    uint256 totalOriginalConsiderationItems; // 0x140
    // offer.length                          // 0x160
}

/**
 * @dev Orders require a signature in addition to the other order parameters.
 */
struct Order {
    OrderParameters parameters;
    bytes signature;
}

/**
 * @dev Advanced orders include a numerator (i.e. a fraction to attempt to fill)
 *      and a denominator (the total size of the order) in addition to the
 *      signature and other order parameters. It also supports an optional field
 *      for supplying extra data; this data will be provided to the zone if the
 *      order type is restricted and the zone is not the caller, or will be
 *      provided to the offerer as context for contract order types.
 */
struct AdvancedOrder {
    OrderParameters parameters;
    uint120 numerator;
    uint120 denominator;
    bytes signature;
    bytes extraData;
}

/**
 * @dev Orders can be validated (either explicitly via `validate`, or as a
 *      consequence of a full or partial fill), specifically cancelled (they can
 *      also be cancelled in bulk via incrementing a per-zone counter), and
 *      partially or fully filled (with the fraction filled represented by a
 *      numerator and denominator).
 */
struct OrderStatus {
    bool isValidated;
    bool isCancelled;
    uint120 numerator;
    uint120 denominator;
}

/**
 * @dev A criteria resolver specifies an order, side (offer vs. consideration),
 *      and item index. It then provides a chosen identifier (i.e. tokenId)
 *      alongside a merkle proof demonstrating the identifier meets the required
 *      criteria.
 */
struct CriteriaResolver {
    uint256 orderIndex;
    Side side;
    uint256 index;
    uint256 identifier;
    bytes32[] criteriaProof;
}

/**
 * @dev A fulfillment is applied to a group of orders. It decrements a series of
 *      offer and consideration items, then generates a single execution
 *      element. A given fulfillment can be applied to as many offer and
 *      consideration items as desired, but must contain at least one offer and
 *      at least one consideration that match. The fulfillment must also remain
 *      consistent on all key parameters across all offer items (same offerer,
 *      token, type, tokenId, and conduit preference) as well as across all
 *      consideration items (token, type, tokenId, and recipient).
 */
struct Fulfillment {
    FulfillmentComponent[] offerComponents;
    FulfillmentComponent[] considerationComponents;
}

/**
 * @dev Each fulfillment component contains one index referencing a specific
 *      order and another referencing a specific offer or consideration item.
 */
struct FulfillmentComponent {
    uint256 orderIndex;
    uint256 itemIndex;
}

/**
 * @dev An execution is triggered once all consideration items have been zeroed
 *      out. It sends the item in question from the offerer to the item's
 *      recipient, optionally sourcing approvals from either this contract
 *      directly or from the offerer's chosen conduit if one is specified. An
 *      execution is not provided as an argument, but rather is derived via
 *      orders, criteria resolvers, and fulfillments (where the total number of
 *      executions will be less than or equal to the total number of indicated
 *      fulfillments) and returned as part of `matchOrders`.
 */
struct Execution {
    ReceivedItem item;
    address offerer;
    bytes32 conduitKey;
}

/**
 * @dev Restricted orders are validated post-execution by calling validateOrder
 *      on the zone. This struct provides context about the order fulfillment
 *      and any supplied extraData, as well as all order hashes fulfilled in a
 *      call to a match or fulfillAvailable method.
 */
struct ZoneParameters {
    bytes32 orderHash;
    address fulfiller;
    address offerer;
    SpentItem[] offer;
    ReceivedItem[] consideration;
    bytes extraData;
    bytes32[] orderHashes;
    uint256 startTime;
    uint256 endTime;
    bytes32 zoneHash;
}

/**
 * @dev Zones and contract offerers can communicate which schemas they implement
 *      along with any associated metadata related to each schema.
 */
struct Schema {
    uint256 id;
    bytes metadata;
}

using StructPointers for OrderComponents global;
using StructPointers for OfferItem global;
using StructPointers for ConsiderationItem global;
using StructPointers for SpentItem global;
using StructPointers for ReceivedItem global;
using StructPointers for BasicOrderParameters global;
using StructPointers for AdditionalRecipient global;
using StructPointers for OrderParameters global;
using StructPointers for Order global;
using StructPointers for AdvancedOrder global;
using StructPointers for OrderStatus global;
using StructPointers for CriteriaResolver global;
using StructPointers for Fulfillment global;
using StructPointers for FulfillmentComponent global;
using StructPointers for Execution global;
using StructPointers for ZoneParameters global;

/**
 * @dev This library provides a set of functions for converting structs to
 *      pointers.
 */
library StructPointers {
    /**
     * @dev Get a MemoryPointer from OrderComponents.
     *
     * @param obj The OrderComponents object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        OrderComponents memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from OrderComponents.
     *
     * @param obj The OrderComponents object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        OrderComponents calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from OfferItem.
     *
     * @param obj The OfferItem object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        OfferItem memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from OfferItem.
     *
     * @param obj The OfferItem object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        OfferItem calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from ConsiderationItem.
     *
     * @param obj The ConsiderationItem object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        ConsiderationItem memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from ConsiderationItem.
     *
     * @param obj The ConsiderationItem object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        ConsiderationItem calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from SpentItem.
     *
     * @param obj The SpentItem object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        SpentItem memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from SpentItem.
     *
     * @param obj The SpentItem object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        SpentItem calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from ReceivedItem.
     *
     * @param obj The ReceivedItem object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        ReceivedItem memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from ReceivedItem.
     *
     * @param obj The ReceivedItem object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        ReceivedItem calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from BasicOrderParameters.
     *
     * @param obj The BasicOrderParameters object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        BasicOrderParameters memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from BasicOrderParameters.
     *
     * @param obj The BasicOrderParameters object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        BasicOrderParameters calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from AdditionalRecipient.
     *
     * @param obj The AdditionalRecipient object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        AdditionalRecipient memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from AdditionalRecipient.
     *
     * @param obj The AdditionalRecipient object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        AdditionalRecipient calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from OrderParameters.
     *
     * @param obj The OrderParameters object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        OrderParameters memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from OrderParameters.
     *
     * @param obj The OrderParameters object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        OrderParameters calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from Order.
     *
     * @param obj The Order object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        Order memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from Order.
     *
     * @param obj The Order object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        Order calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from AdvancedOrder.
     *
     * @param obj The AdvancedOrder object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        AdvancedOrder memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from AdvancedOrder.
     *
     * @param obj The AdvancedOrder object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        AdvancedOrder calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from OrderStatus.
     *
     * @param obj The OrderStatus object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        OrderStatus memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from OrderStatus.
     *
     * @param obj The OrderStatus object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        OrderStatus calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from CriteriaResolver.
     *
     * @param obj The CriteriaResolver object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        CriteriaResolver memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from CriteriaResolver.
     *
     * @param obj The CriteriaResolver object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        CriteriaResolver calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from Fulfillment.
     *
     * @param obj The Fulfillment object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        Fulfillment memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from Fulfillment.
     *
     * @param obj The Fulfillment object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        Fulfillment calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from FulfillmentComponent.
     *
     * @param obj The FulfillmentComponent object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        FulfillmentComponent memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from FulfillmentComponent.
     *
     * @param obj The FulfillmentComponent object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        FulfillmentComponent calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from Execution.
     *
     * @param obj The Execution object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        Execution memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from Execution.
     *
     * @param obj The Execution object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        Execution calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a MemoryPointer from ZoneParameters.
     *
     * @param obj The ZoneParameters object.
     *
     * @return ptr The MemoryPointer.
     */
    function toMemoryPointer(
        ZoneParameters memory obj
    ) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Get a CalldataPointer from ZoneParameters.
     *
     * @param obj The ZoneParameters object.
     *
     * @return ptr The CalldataPointer.
     */
    function toCalldataPointer(
        ZoneParameters calldata obj
    ) internal pure returns (CalldataPointer ptr) {
        assembly {
            ptr := obj
        }
    }
}

/**
 * @title ConsiderationInterface
 * @author 0age
 * @custom:version 1.5
 * @notice Consideration is a generalized native token/ERC20/ERC721/ERC1155
 *         marketplace. It minimizes external calls to the greatest extent
 *         possible and provides lightweight methods for common routes as well
 *         as more flexible methods for composing advanced orders.
 *
 * @dev ConsiderationInterface contains all external function interfaces for
 *      Consideration.
 */
interface ConsiderationInterface {
    /**
     * @notice Fulfill an order offering an ERC721 token by supplying Ether (or
     *         the native token for the given chain) as consideration for the
     *         order. An arbitrary number of "additional recipients" may also be
     *         supplied which will each receive native tokens from the fulfiller
     *         as consideration.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer must first approve this contract (or
     *                   their preferred conduit if indicated by the order) for
     *                   their offered ERC721 token to be transferred.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillBasicOrder(
        BasicOrderParameters calldata parameters
    ) external payable returns (bool fulfilled);

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and
     *         consideration. Note that this function does not support
     *         criteria-based orders or partial filling of orders (though
     *         filling the remainder of a partially-filled order is supported).
     *
     * @param order               The order to fulfill. Note that both the
     *                            offerer and the fulfiller must first approve
     *                            this contract (or the corresponding conduit if
     *                            indicated) to transfer any relevant tokens on
     *                            their behalf and that contracts must implement
     *                            `onERC1155Received` to receive ERC1155 tokens
     *                            as consideration.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillOrder(
        Order calldata order,
        bytes32 fulfillerConduitKey
    ) external payable returns (bool fulfilled);

    /**
     * @notice Fill an order, fully or partially, with an arbitrary number of
     *         items for offer and consideration alongside criteria resolvers
     *         containing specific token identifiers and associated proofs.
     *
     * @param advancedOrder       The order to fulfill along with the fraction
     *                            of the order to attempt to fill. Note that
     *                            both the offerer and the fulfiller must first
     *                            approve this contract (or their preferred
     *                            conduit if indicated by the order) to transfer
     *                            any relevant tokens on their behalf and that
     *                            contracts must implement `onERC1155Received`
     *                            to receive ERC1155 tokens as consideration.
     *                            Also note that all offer and consideration
     *                            components must have no remainder after
     *                            multiplication of the respective amount with
     *                            the supplied fraction for the partial fill to
     *                            be considered valid.
     * @param criteriaResolvers   An array where each element contains a
     *                            reference to a specific offer or
     *                            consideration, a token identifier, and a proof
     *                            that the supplied token identifier is
     *                            contained in the merkle root held by the item
     *                            in question's criteria element. Note that an
     *                            empty criteria indicates that any
     *                            (transferable) token identifier on the token
     *                            in question is valid and that no associated
     *                            proof needs to be supplied.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     * @param recipient           The intended recipient for all received items,
     *                            with `address(0)` indicating that the caller
     *                            should receive the items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillAdvancedOrder(
        AdvancedOrder calldata advancedOrder,
        CriteriaResolver[] calldata criteriaResolvers,
        bytes32 fulfillerConduitKey,
        address recipient
    ) external payable returns (bool fulfilled);

    /**
     * @notice Attempt to fill a group of orders, each with an arbitrary number
     *         of items for offer and consideration. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *         Note that this function does not support criteria-based orders or
     *         partial filling of orders (though filling the remainder of a
     *         partially-filled order is supported).
     *
     * @param orders                    The orders to fulfill. Note that both
     *                                  the offerer and the fulfiller must first
     *                                  approve this contract (or the
     *                                  corresponding conduit if indicated) to
     *                                  transfer any relevant tokens on their
     *                                  behalf and that contracts must implement
     *                                  `onERC1155Received` to receive ERC1155
     *                                  tokens as consideration.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used, with
     *                                  direct approvals set on this contract.
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders. Note that unspent offer item amounts or
     *                         native tokens will not be reflected as part of
     *                         this array.
     */
    function fulfillAvailableOrders(
        Order[] calldata orders,
        FulfillmentComponent[][] calldata offerFulfillments,
        FulfillmentComponent[][] calldata considerationFulfillments,
        bytes32 fulfillerConduitKey,
        uint256 maximumFulfilled
    )
        external
        payable
        returns (bool[] memory availableOrders, Execution[] memory executions);

    /**
     * @notice Attempt to fill a group of orders, fully or partially, with an
     *         arbitrary number of items for offer and consideration per order
     *         alongside criteria resolvers containing specific token
     *         identifiers and associated proofs. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or their preferred conduit if
     *                                  indicated by the order) to transfer any
     *                                  relevant tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` to enable receipt of
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param criteriaResolvers         An array where each element contains a
     *                                  reference to a specific offer or
     *                                  consideration, a token identifier, and a
     *                                  proof that the supplied token identifier
     *                                  is contained in the merkle root held by
     *                                  the item in question's criteria element.
     *                                  Note that an empty criteria indicates
     *                                  that any (transferable) token
     *                                  identifier on the token in question is
     *                                  valid and that no associated proof needs
     *                                  to be supplied.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used, with
     *                                  direct approvals set on this contract.
     * @param recipient                 The intended recipient for all received
     *                                  items, with `address(0)` indicating that
     *                                  the caller should receive the items.
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders. Note that unspent offer item amounts or
     *                         native tokens will not be reflected as part of
     *                         this array.
     */
    function fulfillAvailableAdvancedOrders(
        AdvancedOrder[] calldata advancedOrders,
        CriteriaResolver[] calldata criteriaResolvers,
        FulfillmentComponent[][] calldata offerFulfillments,
        FulfillmentComponent[][] calldata considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient,
        uint256 maximumFulfilled
    )
        external
        payable
        returns (bool[] memory availableOrders, Execution[] memory executions);

    /**
     * @notice Match an arbitrary number of orders, each with an arbitrary
     *         number of items for offer and consideration along with a set of
     *         fulfillments allocating offer components to consideration
     *         components. Note that this function does not support
     *         criteria-based or partial filling of orders (though filling the
     *         remainder of a partially-filled order is supported). Any unspent
     *         offer item amounts or native tokens will be transferred to the
     *         caller.
     *
     * @param orders       The orders to match. Note that both the offerer and
     *                     fulfiller on each order must first approve this
     *                     contract (or their conduit if indicated by the order)
     *                     to transfer any relevant tokens on their behalf and
     *                     each consideration recipient must implement
     *                     `onERC1155Received` to enable ERC1155 token receipt.
     * @param fulfillments An array of elements allocating offer components to
     *                     consideration components. Note that each
     *                     consideration component must be fully met for the
     *                     match operation to be valid.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders. Note that unspent offer item amounts or
     *                    native tokens will not be reflected as part of this
     *                    array.
     */
    function matchOrders(
        Order[] calldata orders,
        Fulfillment[] calldata fulfillments
    ) external payable returns (Execution[] memory executions);

    /**
     * @notice Match an arbitrary number of full or partial orders, each with an
     *         arbitrary number of items for offer and consideration, supplying
     *         criteria resolvers containing specific token identifiers and
     *         associated proofs as well as fulfillments allocating offer
     *         components to consideration components. Any unspent offer item
     *         amounts will be transferred to the designated recipient (with the
     *         null address signifying to use the caller) and any unspent native
     *         tokens will be returned to the caller.
     *
     * @param orders            The advanced orders to match. Note that both the
     *                          offerer and fulfiller on each order must first
     *                          approve this contract (or a preferred conduit if
     *                          indicated by the order) to transfer any relevant
     *                          tokens on their behalf and each consideration
     *                          recipient must implement `onERC1155Received` in
     *                          order to receive ERC1155 tokens. Also note that
     *                          the offer and consideration components for each
     *                          order must have no remainder after multiplying
     *                          the respective amount with the supplied fraction
     *                          in order for the group of partial fills to be
     *                          considered valid.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          an empty root indicates that any (transferable)
     *                          token identifier is valid and that no associated
     *                          proof needs to be supplied.
     * @param fulfillments      An array of elements allocating offer components
     *                          to consideration components. Note that each
     *                          consideration component must be fully met in
     *                          order for the match operation to be valid.
     * @param recipient         The intended recipient for all unspent offer
     *                          item amounts, or the caller if the null address
     *                          is supplied.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders. Note that unspent offer item amounts or native
     *                    tokens will not be reflected as part of this array.
     */
    function matchAdvancedOrders(
        AdvancedOrder[] calldata orders,
        CriteriaResolver[] calldata criteriaResolvers,
        Fulfillment[] calldata fulfillments,
        address recipient
    ) external payable returns (Execution[] memory executions);

    /**
     * @notice Cancel an arbitrary number of orders. Note that only the offerer
     *         or the zone of a given order may cancel it. Callers should ensure
     *         that the intended order was cancelled by calling `getOrderStatus`
     *         and confirming that `isCancelled` returns `true`.
     *
     * @param orders The orders to cancel.
     *
     * @return cancelled A boolean indicating whether the supplied orders have
     *                   been successfully cancelled.
     */
    function cancel(
        OrderComponents[] calldata orders
    ) external returns (bool cancelled);

    /**
     * @notice Validate an arbitrary number of orders, thereby registering their
     *         signatures as valid and allowing the fulfiller to skip signature
     *         verification on fulfillment. Note that validated orders may still
     *         be unfulfillable due to invalid item amounts or other factors;
     *         callers should determine whether validated orders are fulfillable
     *         by simulating the fulfillment call prior to execution. Also note
     *         that anyone can validate a signed order, but only the offerer can
     *         validate an order without supplying a signature.
     *
     * @param orders The orders to validate.
     *
     * @return validated A boolean indicating whether the supplied orders have
     *                   been successfully validated.
     */
    function validate(
        Order[] calldata orders
    ) external returns (bool validated);

    /**
     * @notice Cancel all orders from a given offerer with a given zone in bulk
     *         by incrementing a counter. Note that only the offerer may
     *         increment the counter.
     *
     * @return newCounter The new counter.
     */
    function incrementCounter() external returns (uint256 newCounter);

    /**
     * @notice Fulfill an order offering an ERC721 token by supplying Ether (or
     *         the native token for the given chain) as consideration for the
     *         order. An arbitrary number of "additional recipients" may also be
     *         supplied which will each receive native tokens from the fulfiller
     *         as consideration. Note that this function costs less gas than
     *         `fulfillBasicOrder` due to the zero bytes in the function
     *         selector (0x00000000) which also results in earlier function
     *         dispatch.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer must first approve this contract (or
     *                   their preferred conduit if indicated by the order) for
     *                   their offered ERC721 token to be transferred.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillBasicOrder_efficient_6GL6yc(
        BasicOrderParameters calldata parameters
    ) external payable returns (bool fulfilled);

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(
        OrderComponents calldata order
    ) external view returns (bytes32 orderHash);

    /**
     * @notice Retrieve the status of a given order by hash, including whether
     *         the order has been cancelled or validated and the fraction of the
     *         order that has been filled.
     *
     * @param orderHash The order hash in question.
     *
     * @return isValidated A boolean indicating whether the order in question
     *                     has been validated (i.e. previously approved or
     *                     partially filled).
     * @return isCancelled A boolean indicating whether the order in question
     *                     has been cancelled.
     * @return totalFilled The total portion of the order that has been filled
     *                     (i.e. the "numerator").
     * @return totalSize   The total size of the order that is either filled or
     *                     unfilled (i.e. the "denominator").
     */
    function getOrderStatus(
        bytes32 orderHash
    )
        external
        view
        returns (
            bool isValidated,
            bool isCancelled,
            uint256 totalFilled,
            uint256 totalSize
        );

    /**
     * @notice Retrieve the current counter for a given offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return counter The current counter.
     */
    function getCounter(
        address offerer
    ) external view returns (uint256 counter);

    /**
     * @notice Retrieve configuration information for this contract.
     *
     * @return version           The contract version.
     * @return domainSeparator   The domain separator for this contract.
     * @return conduitController The conduit Controller set for this contract.
     */
    function information()
        external
        view
        returns (
            string memory version,
            bytes32 domainSeparator,
            address conduitController
        );

    function getContractOffererNonce(
        address contractOfferer
    ) external view returns (uint256 nonce);

    /**
     * @notice Retrieve the name of this contract.
     *
     * @return contractName The name of this contract.
     */
    function name() external view returns (string memory contractName);
}

uint256 constant Error_selector_offset = 0x1c;

/*
 *  error MissingFulfillmentComponentOnAggregation(uint8 side)
 *    - Defined in FulfillmentApplicationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: side
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant MissingFulfillmentComponentOnAggregation_error_selector = (
    0x375c24c1
);
uint256 constant MissingFulfillmentComponentOnAggregation_error_side_ptr = 0x20;
uint256 constant MissingFulfillmentComponentOnAggregation_error_length = 0x24;

/*
 *  error OfferAndConsiderationRequiredOnFulfillment()
 *    - Defined in FulfillmentApplicationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant OfferAndConsiderationRequiredOnFulfillment_error_selector = (
    0x98e9db6e
);
uint256 constant OfferAndConsiderationRequiredOnFulfillment_error_length = 0x04;

/*
 *  error MismatchedFulfillmentOfferAndConsiderationComponents(
 *      uint256 fulfillmentIndex
 *  )
 *    - Defined in FulfillmentApplicationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: fulfillmentIndex
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant MismatchedOfferAndConsiderationComponents_error_selector = (
    0xbced929d
);
uint256 constant MismatchedOfferAndConsiderationComponents_error_idx_ptr = 0x20;
uint256 constant MismatchedOfferAndConsiderationComponents_error_length = 0x24;

/*
 *  error InvalidFulfillmentComponentData()
 *    - Defined in FulfillmentApplicationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidFulfillmentComponentData_error_selector = 0x7fda7279;
uint256 constant InvalidFulfillmentComponentData_error_length = 0x04;

/*
 *  error InexactFraction()
 *    - Defined in AmountDerivationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InexactFraction_error_selector = 0xc63cf089;
uint256 constant InexactFraction_error_length = 0x04;

/*
 *  error OrderCriteriaResolverOutOfRange(uint8 side)
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: side
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant OrderCriteriaResolverOutOfRange_error_selector = 0x133c37c6;
uint256 constant OrderCriteriaResolverOutOfRange_error_side_ptr = 0x20;
uint256 constant OrderCriteriaResolverOutOfRange_error_length = 0x24;

/*
 *  error UnresolvedOfferCriteria(uint256 orderIndex, uint256 offerIndex)
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderIndex
 *    - 0x40: offerIndex
 * Revert buffer is memory[0x1c:0x60]
 */
uint256 constant UnresolvedOfferCriteria_error_selector = 0xd6929332;
uint256 constant UnresolvedOfferCriteria_error_orderIndex_ptr = 0x20;
uint256 constant UnresolvedOfferCriteria_error_offerIndex_ptr = 0x40;
uint256 constant UnresolvedOfferCriteria_error_length = 0x44;

/*
 *  error UnresolvedConsiderationCriteria(
 *      uint256 orderIndex,
 *      uint256 considerationIndex
 *  )
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderIndex
 *    - 0x40: considerationIndex
 * Revert buffer is memory[0x1c:0x60]
 */
uint256 constant UnresolvedConsiderationCriteria_error_selector = 0xa8930e9a;
uint256 constant UnresolvedConsiderationCriteria_error_orderIndex_ptr = 0x20;
uint256 constant UnresolvedConsiderationCriteria_error_considerationIdx_ptr = (
    0x40
);
uint256 constant UnresolvedConsiderationCriteria_error_length = 0x44;

/*
 *  error OfferCriteriaResolverOutOfRange()
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant OfferCriteriaResolverOutOfRange_error_selector = 0xbfb3f8ce;
// uint256 constant OfferCriteriaResolverOutOfRange_error_length = 0x04;

/*
 *  error ConsiderationCriteriaResolverOutOfRange()
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant ConsiderationCriteriaResolverOutOfRange_error_selector = (
    0x6088d7de
);
uint256 constant ConsiderationCriteriaResolverOutOfRange_err_selector = (
    0x6088d7de
);
// uint256 constant ConsiderationCriteriaResolverOutOfRange_error_length = 0x04;

/*
 *  error CriteriaNotEnabledForItem()
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant CriteriaNotEnabledForItem_error_selector = 0x94eb6af6;
uint256 constant CriteriaNotEnabledForItem_error_length = 0x04;

/*
 *  error InvalidProof()
 *    - Defined in CriteriaResolutionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidProof_error_selector = 0x09bde339;
uint256 constant InvalidProof_error_length = 0x04;

/*
 *  error InvalidRestrictedOrder(bytes32 orderHash)
 *    - Defined in ZoneInteractionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderHash
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant InvalidRestrictedOrder_error_selector = 0xfb5014fc;
uint256 constant InvalidRestrictedOrder_error_orderHash_ptr = 0x20;
uint256 constant InvalidRestrictedOrder_error_length = 0x24;

/*
 *  error InvalidContractOrder(bytes32 orderHash)
 *    - Defined in ZoneInteractionErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderHash
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant InvalidContractOrder_error_selector = 0x93979285;
uint256 constant InvalidContractOrder_error_orderHash_ptr = 0x20;
uint256 constant InvalidContractOrder_error_length = 0x24;

/*
 *  error BadSignatureV(uint8 v)
 *    - Defined in SignatureVerificationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: v
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant BadSignatureV_error_selector = 0x1f003d0a;
uint256 constant BadSignatureV_error_v_ptr = 0x20;
uint256 constant BadSignatureV_error_length = 0x24;

/*
 *  error InvalidSigner()
 *    - Defined in SignatureVerificationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidSigner_error_selector = 0x815e1d64;
uint256 constant InvalidSigner_error_length = 0x04;

/*
 *  error InvalidSignature()
 *    - Defined in SignatureVerificationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidSignature_error_selector = 0x8baa579f;
uint256 constant InvalidSignature_error_length = 0x04;

/*
 *  error BadContractSignature()
 *    - Defined in SignatureVerificationErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant BadContractSignature_error_selector = 0x4f7fb80d;
uint256 constant BadContractSignature_error_length = 0x04;

/*
 *  error InvalidERC721TransferAmount(uint256 amount)
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: amount
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant InvalidERC721TransferAmount_error_selector = 0x69f95827;
uint256 constant InvalidERC721TransferAmount_error_amount_ptr = 0x20;
uint256 constant InvalidERC721TransferAmount_error_length = 0x24;

/*
 *  error MissingItemAmount()
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant MissingItemAmount_error_selector = 0x91b3e514;
uint256 constant MissingItemAmount_error_length = 0x04;

/*
 *  error UnusedItemParameters()
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant UnusedItemParameters_error_selector = 0x6ab37ce7;
uint256 constant UnusedItemParameters_error_length = 0x04;

/*
 *  error NoReentrantCalls()
 *    - Defined in ReentrancyErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant NoReentrantCalls_error_selector = 0x7fa8a987;
uint256 constant NoReentrantCalls_error_length = 0x04;

/*
 *  error OrderAlreadyFilled(bytes32 orderHash)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderHash
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant OrderAlreadyFilled_error_selector = 0x10fda3e1;
uint256 constant OrderAlreadyFilled_error_orderHash_ptr = 0x20;
uint256 constant OrderAlreadyFilled_error_length = 0x24;

/*
 *  error InvalidTime(uint256 startTime, uint256 endTime)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: startTime
 *    - 0x40: endTime
 * Revert buffer is memory[0x1c:0x60]
 */
uint256 constant InvalidTime_error_selector = 0x21ccfeb7;
uint256 constant InvalidTime_error_startTime_ptr = 0x20;
uint256 constant InvalidTime_error_endTime_ptr = 0x40;
uint256 constant InvalidTime_error_length = 0x44;

/*
 *  error InvalidConduit(bytes32 conduitKey, address conduit)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: conduitKey
 *    - 0x40: conduit
 * Revert buffer is memory[0x1c:0x60]
 */
uint256 constant InvalidConduit_error_selector = 0x1cf99b26;
uint256 constant InvalidConduit_error_conduitKey_ptr = 0x20;
uint256 constant InvalidConduit_error_conduit_ptr = 0x40;
uint256 constant InvalidConduit_error_length = 0x44;

/*
 *  error MissingOriginalConsiderationItems()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant MissingOriginalConsiderationItems_error_selector = 0x466aa616;
uint256 constant MissingOriginalConsiderationItems_error_length = 0x04;

/*
 *  error InvalidCallToConduit(address conduit)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: conduit
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant InvalidCallToConduit_error_selector = 0xd13d53d4;
uint256 constant InvalidCallToConduit_error_conduit_ptr = 0x20;
uint256 constant InvalidCallToConduit_error_length = 0x24;

/*
 *  error ConsiderationNotMet(
 *      uint256 orderIndex,
 *      uint256 considerationIndex,
 *      uint256 shortfallAmount
 *  )
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderIndex
 *    - 0x40: considerationIndex
 *    - 0x60: shortfallAmount
 * Revert buffer is memory[0x1c:0x80]
 */
uint256 constant ConsiderationNotMet_error_selector = 0xa5f54208;
uint256 constant ConsiderationNotMet_error_orderIndex_ptr = 0x20;
uint256 constant ConsiderationNotMet_error_considerationIndex_ptr = 0x40;
uint256 constant ConsiderationNotMet_error_shortfallAmount_ptr = 0x60;
uint256 constant ConsiderationNotMet_error_length = 0x64;

/*
 *  error InsufficientNativeTokensSupplied()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InsufficientNativeTokensSupplied_error_selector = 0x8ffff980;
uint256 constant InsufficientNativeTokensSupplied_error_length = 0x04;

/*
 *  error NativeTokenTransferGenericFailure(address account, uint256 amount)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: account
 *    - 0x40: amount
 * Revert buffer is memory[0x1c:0x60]
 */
uint256 constant NativeTokenTransferGenericFailure_error_selector = 0xbc806b96;
uint256 constant NativeTokenTransferGenericFailure_error_account_ptr = 0x20;
uint256 constant NativeTokenTransferGenericFailure_error_amount_ptr = 0x40;
uint256 constant NativeTokenTransferGenericFailure_error_length = 0x44;

/*
 *  error PartialFillsNotEnabledForOrder()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant PartialFillsNotEnabledForOrder_error_selector = 0xa11b63ff;
uint256 constant PartialFillsNotEnabledForOrder_error_length = 0x04;

/*
 *  error OrderIsCancelled(bytes32 orderHash)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderHash
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant OrderIsCancelled_error_selector = 0x1a515574;
uint256 constant OrderIsCancelled_error_orderHash_ptr = 0x20;
uint256 constant OrderIsCancelled_error_length = 0x24;

/*
 *  error OrderPartiallyFilled(bytes32 orderHash)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: orderHash
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant OrderPartiallyFilled_error_selector = 0xee9e0e63;
uint256 constant OrderPartiallyFilled_error_orderHash_ptr = 0x20;
uint256 constant OrderPartiallyFilled_error_length = 0x24;

/*
 *  error CannotCancelOrder()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant CannotCancelOrder_error_selector = 0xfed398fc;
uint256 constant CannotCancelOrder_error_length = 0x04;

/*
 *  error BadFraction()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant BadFraction_error_selector = 0x5a052b32;
uint256 constant BadFraction_error_length = 0x04;

/*
 *  error InvalidMsgValue(uint256 value)
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: value
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant InvalidMsgValue_error_selector = 0xa61be9f0;
uint256 constant InvalidMsgValue_error_value_ptr = 0x20;
uint256 constant InvalidMsgValue_error_length = 0x24;

/*
 *  error InvalidBasicOrderParameterEncoding()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidBasicOrderParameterEncoding_error_selector = 0x39f3e3fd;
uint256 constant InvalidBasicOrderParameterEncoding_error_length = 0x04;

/*
 *  error NoSpecifiedOrdersAvailable()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant NoSpecifiedOrdersAvailable_error_selector = 0xd5da9a1b;
uint256 constant NoSpecifiedOrdersAvailable_error_length = 0x04;

/*
 *  error InvalidNativeOfferItem()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant InvalidNativeOfferItem_error_selector = 0x12d3f5a3;
uint256 constant InvalidNativeOfferItem_error_length = 0x04;

/*
 *  error ConsiderationLengthNotEqualToTotalOriginal()
 *    - Defined in ConsiderationEventsAndErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 * Revert buffer is memory[0x1c:0x20]
 */
uint256 constant ConsiderationLengthNotEqualToTotalOriginal_error_selector = (
    0x2165628a
);
uint256 constant ConsiderationLengthNotEqualToTotalOriginal_error_length = 0x04;

/*
 *  error Panic(uint256 code)
 *    - Built-in Solidity error
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: code
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant Panic_error_selector = 0x4e487b71;
uint256 constant Panic_error_code_ptr = 0x20;
uint256 constant Panic_error_length = 0x24;

uint256 constant Panic_arithmetic = 0x11;
// uint256 constant Panic_resource = 0x41;

/**
 * @dev Reverts the current transaction with a "BadFraction" error message.
 */
function _revertBadFraction() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, BadFraction_error_selector)

        // revert(abi.encodeWithSignature("BadFraction()"))
        revert(Error_selector_offset, BadFraction_error_length)
    }
}

/**
 * @dev Reverts the current transaction with a "ConsiderationNotMet" error
 *      message, including the provided order index, consideration index, and
 *      shortfall amount.
 *
 * @param orderIndex         The index of the order that did not meet the
 *                           consideration criteria.
 * @param considerationIndex The index of the consideration item that did not
 *                           meet its criteria.
 * @param shortfallAmount    The amount by which the consideration criteria were
 *                           not met.
 */
function _revertConsiderationNotMet(
    uint256 orderIndex,
    uint256 considerationIndex,
    uint256 shortfallAmount
) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, ConsiderationNotMet_error_selector)

        // Store arguments.
        mstore(ConsiderationNotMet_error_orderIndex_ptr, orderIndex)
        mstore(
            ConsiderationNotMet_error_considerationIndex_ptr,
            considerationIndex
        )
        mstore(ConsiderationNotMet_error_shortfallAmount_ptr, shortfallAmount)

        // revert(abi.encodeWithSignature(
        //     "ConsiderationNotMet(uint256,uint256,uint256)",
        //     orderIndex,
        //     considerationIndex,
        //     shortfallAmount
        // ))
        revert(Error_selector_offset, ConsiderationNotMet_error_length)
    }
}

/**
 * @dev Reverts the current transaction with a "CriteriaNotEnabledForItem" error
 *      message.
 */
function _revertCriteriaNotEnabledForItem() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, CriteriaNotEnabledForItem_error_selector)

        // revert(abi.encodeWithSignature("CriteriaNotEnabledForItem()"))
        revert(Error_selector_offset, CriteriaNotEnabledForItem_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an
 *      "InsufficientNativeTokensSupplied" error message.
 */
function _revertInsufficientNativeTokensSupplied() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InsufficientNativeTokensSupplied_error_selector)

        // revert(abi.encodeWithSignature("InsufficientNativeTokensSupplied()"))
        revert(
            Error_selector_offset,
            InsufficientNativeTokensSupplied_error_length
        )
    }
}

/**
 * @dev Reverts the current transaction with an
 *      "InvalidBasicOrderParameterEncoding" error message.
 */
function _revertInvalidBasicOrderParameterEncoding() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidBasicOrderParameterEncoding_error_selector)

        // revert(abi.encodeWithSignature(
        //     "InvalidBasicOrderParameterEncoding()"
        // ))
        revert(
            Error_selector_offset,
            InvalidBasicOrderParameterEncoding_error_length
        )
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidCallToConduit" error
 *      message, including the provided address of the conduit that was called
 *      improperly.
 *
 * @param conduit The address of the conduit that was called improperly.
 */
function _revertInvalidCallToConduit(address conduit) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidCallToConduit_error_selector)

        // Store argument.
        mstore(InvalidCallToConduit_error_conduit_ptr, conduit)

        // revert(abi.encodeWithSignature(
        //     "InvalidCallToConduit(address)",
        //     conduit
        // ))
        revert(Error_selector_offset, InvalidCallToConduit_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "CannotCancelOrder" error
 *      message.
 */
function _revertCannotCancelOrder() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, CannotCancelOrder_error_selector)

        // revert(abi.encodeWithSignature("CannotCancelOrder()"))
        revert(Error_selector_offset, CannotCancelOrder_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidConduit" error message,
 *      including the provided key and address of the invalid conduit.
 *
 * @param conduitKey    The key of the invalid conduit.
 * @param conduit       The address of the invalid conduit.
 */
function _revertInvalidConduit(bytes32 conduitKey, address conduit) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidConduit_error_selector)

        // Store arguments.
        mstore(InvalidConduit_error_conduitKey_ptr, conduitKey)
        mstore(InvalidConduit_error_conduit_ptr, conduit)

        // revert(abi.encodeWithSignature(
        //     "InvalidConduit(bytes32,address)",
        //     conduitKey,
        //     conduit
        // ))
        revert(Error_selector_offset, InvalidConduit_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidERC721TransferAmount"
 *      error message.
 *
 * @param amount The invalid amount.
 */
function _revertInvalidERC721TransferAmount(uint256 amount) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidERC721TransferAmount_error_selector)

        // Store argument.
        mstore(InvalidERC721TransferAmount_error_amount_ptr, amount)

        // revert(abi.encodeWithSignature(
        //     "InvalidERC721TransferAmount(uint256)",
        //     amount
        // ))
        revert(Error_selector_offset, InvalidERC721TransferAmount_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidMsgValue" error message,
 *      including the invalid value that was sent in the transaction's
 *      `msg.value` field.
 *
 * @param value The invalid value that was sent in the transaction's `msg.value`
 *              field.
 */
function _revertInvalidMsgValue(uint256 value) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidMsgValue_error_selector)

        // Store argument.
        mstore(InvalidMsgValue_error_value_ptr, value)

        // revert(abi.encodeWithSignature("InvalidMsgValue(uint256)", value))
        revert(Error_selector_offset, InvalidMsgValue_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidNativeOfferItem" error
 *      message.
 */
function _revertInvalidNativeOfferItem() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidNativeOfferItem_error_selector)

        // revert(abi.encodeWithSignature("InvalidNativeOfferItem()"))
        revert(Error_selector_offset, InvalidNativeOfferItem_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidProof" error message.
 */
function _revertInvalidProof() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidProof_error_selector)

        // revert(abi.encodeWithSignature("InvalidProof()"))
        revert(Error_selector_offset, InvalidProof_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidContractOrder" error
 *      message.
 *
 * @param orderHash The hash of the contract order that caused the error.
 */
function _revertInvalidContractOrder(bytes32 orderHash) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidContractOrder_error_selector)

        // Store arguments.
        mstore(InvalidContractOrder_error_orderHash_ptr, orderHash)

        // revert(abi.encodeWithSignature(
        //     "InvalidContractOrder(bytes32)",
        //     orderHash
        // ))
        revert(Error_selector_offset, InvalidContractOrder_error_length)
    }
}

/**
 * @dev Reverts the current transaction with an "InvalidTime" error message.
 *
 * @param startTime       The time at which the order becomes active.
 * @param endTime         The time at which the order becomes inactive.
 */
function _revertInvalidTime(uint256 startTime, uint256 endTime) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, InvalidTime_error_selector)

        // Store arguments.
        mstore(InvalidTime_error_startTime_ptr, startTime)
        mstore(InvalidTime_error_endTime_ptr, endTime)

        // revert(abi.encodeWithSignature(
        //     "InvalidTime(uint256,uint256)",
        //     startTime,
        //     endTime
        // ))
        revert(Error_selector_offset, InvalidTime_error_length)
    }
}

/**
 * @dev Reverts execution with a
 *      "MismatchedFulfillmentOfferAndConsiderationComponents" error message.
 *
 * @param fulfillmentIndex         The index of the fulfillment that caused the
 *                                 error.
 */
function _revertMismatchedFulfillmentOfferAndConsiderationComponents(
    uint256 fulfillmentIndex
) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, MismatchedOfferAndConsiderationComponents_error_selector)

        // Store fulfillment index argument.
        mstore(
            MismatchedOfferAndConsiderationComponents_error_idx_ptr,
            fulfillmentIndex
        )

        // revert(abi.encodeWithSignature(
        //     "MismatchedFulfillmentOfferAndConsiderationComponents(uint256)",
        //     fulfillmentIndex
        // ))
        revert(
            Error_selector_offset,
            MismatchedOfferAndConsiderationComponents_error_length
        )
    }
}

/**
 * @dev Reverts execution with a "MissingFulfillmentComponentOnAggregation"
 *       error message.
 *
 * @param side The side of the fulfillment component that is missing (0 for
 *             offer, 1 for consideration).
 *
 */
function _revertMissingFulfillmentComponentOnAggregation(Side side) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, MissingFulfillmentComponentOnAggregation_error_selector)

        // Store argument.
        mstore(MissingFulfillmentComponentOnAggregation_error_side_ptr, side)

        // revert(abi.encodeWithSignature(
        //     "MissingFulfillmentComponentOnAggregation(uint8)",
        //     side
        // ))
        revert(
            Error_selector_offset,
            MissingFulfillmentComponentOnAggregation_error_length
        )
    }
}

/**
 * @dev Reverts execution with a "MissingOriginalConsiderationItems" error
 *      message.
 */
function _revertMissingOriginalConsiderationItems() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, MissingOriginalConsiderationItems_error_selector)

        // revert(abi.encodeWithSignature(
        //     "MissingOriginalConsiderationItems()"
        // ))
        revert(
            Error_selector_offset,
            MissingOriginalConsiderationItems_error_length
        )
    }
}

/**
 * @dev Reverts execution with a "NoReentrantCalls" error message.
 */
function _revertNoReentrantCalls() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, NoReentrantCalls_error_selector)

        // revert(abi.encodeWithSignature("NoReentrantCalls()"))
        revert(Error_selector_offset, NoReentrantCalls_error_length)
    }
}

/**
 * @dev Reverts execution with a "NoSpecifiedOrdersAvailable" error message.
 */
function _revertNoSpecifiedOrdersAvailable() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, NoSpecifiedOrdersAvailable_error_selector)

        // revert(abi.encodeWithSignature("NoSpecifiedOrdersAvailable()"))
        revert(Error_selector_offset, NoSpecifiedOrdersAvailable_error_length)
    }
}

/**
 * @dev Reverts execution with a "OfferAndConsiderationRequiredOnFulfillment"
 *      error message.
 */
function _revertOfferAndConsiderationRequiredOnFulfillment() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, OfferAndConsiderationRequiredOnFulfillment_error_selector)

        // revert(abi.encodeWithSignature(
        //     "OfferAndConsiderationRequiredOnFulfillment()"
        // ))
        revert(
            Error_selector_offset,
            OfferAndConsiderationRequiredOnFulfillment_error_length
        )
    }
}

/**
 * @dev Reverts execution with an "OrderAlreadyFilled" error message.
 *
 * @param orderHash The hash of the order that has already been filled.
 */
function _revertOrderAlreadyFilled(bytes32 orderHash) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, OrderAlreadyFilled_error_selector)

        // Store argument.
        mstore(OrderAlreadyFilled_error_orderHash_ptr, orderHash)

        // revert(abi.encodeWithSignature(
        //     "OrderAlreadyFilled(bytes32)",
        //     orderHash
        // ))
        revert(Error_selector_offset, OrderAlreadyFilled_error_length)
    }
}

/**
 * @dev Reverts execution with an "OrderCriteriaResolverOutOfRange" error
 *      message.
 *
 * @param side The side of the criteria that is missing (0 for offer, 1 for
 *             consideration).
 *
 */
function _revertOrderCriteriaResolverOutOfRange(Side side) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, OrderCriteriaResolverOutOfRange_error_selector)

        // Store argument.
        mstore(OrderCriteriaResolverOutOfRange_error_side_ptr, side)

        // revert(abi.encodeWithSignature(
        //     "OrderCriteriaResolverOutOfRange(uint8)",
        //     side
        // ))
        revert(
            Error_selector_offset,
            OrderCriteriaResolverOutOfRange_error_length
        )
    }
}

/**
 * @dev Reverts execution with an "OrderIsCancelled" error message.
 *
 * @param orderHash The hash of the order that has already been cancelled.
 */
function _revertOrderIsCancelled(bytes32 orderHash) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, OrderIsCancelled_error_selector)

        // Store argument.
        mstore(OrderIsCancelled_error_orderHash_ptr, orderHash)

        // revert(abi.encodeWithSignature(
        //     "OrderIsCancelled(bytes32)",
        //     orderHash
        // ))
        revert(Error_selector_offset, OrderIsCancelled_error_length)
    }
}

/**
 * @dev Reverts execution with an "OrderPartiallyFilled" error message.
 *
 * @param orderHash The hash of the order that has already been partially
 *                  filled.
 */
function _revertOrderPartiallyFilled(bytes32 orderHash) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, OrderPartiallyFilled_error_selector)

        // Store argument.
        mstore(OrderPartiallyFilled_error_orderHash_ptr, orderHash)

        // revert(abi.encodeWithSignature(
        //     "OrderPartiallyFilled(bytes32)",
        //     orderHash
        // ))
        revert(Error_selector_offset, OrderPartiallyFilled_error_length)
    }
}

/**
 * @dev Reverts execution with a "PartialFillsNotEnabledForOrder" error message.
 */
function _revertPartialFillsNotEnabledForOrder() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, PartialFillsNotEnabledForOrder_error_selector)

        // revert(abi.encodeWithSignature("PartialFillsNotEnabledForOrder()"))
        revert(
            Error_selector_offset,
            PartialFillsNotEnabledForOrder_error_length
        )
    }
}

/**
 * @dev Reverts execution with an "UnresolvedConsiderationCriteria" error
 *      message.
 */
function _revertUnresolvedConsiderationCriteria(
    uint256 orderIndex,
    uint256 considerationIndex
) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, UnresolvedConsiderationCriteria_error_selector)

        // Store orderIndex and considerationIndex arguments.
        mstore(UnresolvedConsiderationCriteria_error_orderIndex_ptr, orderIndex)
        mstore(
            UnresolvedConsiderationCriteria_error_considerationIdx_ptr,
            considerationIndex
        )

        // revert(abi.encodeWithSignature(
        //     "UnresolvedConsiderationCriteria(uint256, uint256)",
        //     orderIndex,
        //     considerationIndex
        // ))
        revert(
            Error_selector_offset,
            UnresolvedConsiderationCriteria_error_length
        )
    }
}

/**
 * @dev Reverts execution with an "UnresolvedOfferCriteria" error message.
 */
function _revertUnresolvedOfferCriteria(
    uint256 orderIndex,
    uint256 offerIndex
) pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, UnresolvedOfferCriteria_error_selector)

        // Store arguments.
        mstore(UnresolvedOfferCriteria_error_orderIndex_ptr, orderIndex)
        mstore(UnresolvedOfferCriteria_error_offerIndex_ptr, offerIndex)

        // revert(abi.encodeWithSignature(
        //     "UnresolvedOfferCriteria(uint256, uint256)",
        //     orderIndex,
        //     offerIndex
        // ))
        revert(Error_selector_offset, UnresolvedOfferCriteria_error_length)
    }
}

/**
 * @dev Reverts execution with an "UnusedItemParameters" error message.
 */
function _revertUnusedItemParameters() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, UnusedItemParameters_error_selector)

        // revert(abi.encodeWithSignature("UnusedItemParameters()"))
        revert(Error_selector_offset, UnusedItemParameters_error_length)
    }
}

/**
 * @dev Reverts execution with a "ConsiderationLengthNotEqualToTotalOriginal"
 *      error message.
 */
function _revertConsiderationLengthNotEqualToTotalOriginal() pure {
    assembly {
        // Store left-padded selector with push4 (reduces bytecode),
        // mem[28:32] = selector
        mstore(0, ConsiderationLengthNotEqualToTotalOriginal_error_selector)

        // revert(abi.encodeWithSignature(
        //     "ConsiderationLengthNotEqualToTotalOriginal()"
        // ))
        revert(
            Error_selector_offset,
            ConsiderationLengthNotEqualToTotalOriginal_error_length
        )
    }
}

enum ConduitItemType {
    NATIVE, // unused
    ERC20,
    ERC721,
    ERC1155
}

/**
 * @dev A ConduitTransfer is a struct that contains the information needed for a
 *      conduit to transfer an item from one address to another.
 */
struct ConduitTransfer {
    ConduitItemType itemType;
    address token;
    address from;
    address to;
    uint256 identifier;
    uint256 amount;
}

/**
 * @dev A ConduitBatch1155Transfer is a struct that contains the information
 *      needed for a conduit to transfer a batch of ERC-1155 tokens from one
 *      address to another.
 */
struct ConduitBatch1155Transfer {
    address token;
    address from;
    address to;
    uint256[] ids;
    uint256[] amounts;
}

/**
 * @title ConduitInterface
 * @author 0age
 * @notice ConduitInterface contains all external function interfaces, events,
 *         and errors for conduit contracts.
 */
interface ConduitInterface {
    /**
     * @dev Revert with an error when attempting to execute transfers using a
     *      caller that does not have an open channel.
     */
    error ChannelClosed(address channel);

    /**
     * @dev Revert with an error when attempting to update a channel to the
     *      current status of that channel.
     */
    error ChannelStatusAlreadySet(address channel, bool isOpen);

    /**
     * @dev Revert with an error when attempting to execute a transfer for an
     *      item that does not have an ERC20/721/1155 item type.
     */
    error InvalidItemType();

    /**
     * @dev Revert with an error when attempting to update the status of a
     *      channel from a caller that is not the conduit controller.
     */
    error InvalidController();

    /**
     * @dev Emit an event whenever a channel is opened or closed.
     *
     * @param channel The channel that has been updated.
     * @param open    A boolean indicating whether the conduit is open or not.
     */
    event ChannelUpdated(address indexed channel, bool open);

    /**
     * @notice Execute a sequence of ERC20/721/1155 transfers. Only a caller
     *         with an open channel can call this function.
     *
     * @param transfers The ERC20/721/1155 transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function execute(
        ConduitTransfer[] calldata transfers
    ) external returns (bytes4 magicValue);

    /**
     * @notice Execute a sequence of batch 1155 transfers. Only a caller with an
     *         open channel can call this function.
     *
     * @param batch1155Transfers The 1155 batch transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function executeBatch1155(
        ConduitBatch1155Transfer[] calldata batch1155Transfers
    ) external returns (bytes4 magicValue);

    /**
     * @notice Execute a sequence of transfers, both single and batch 1155. Only
     *         a caller with an open channel can call this function.
     *
     * @param standardTransfers  The ERC20/721/1155 transfers to perform.
     * @param batch1155Transfers The 1155 batch transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function executeWithBatch1155(
        ConduitTransfer[] calldata standardTransfers,
        ConduitBatch1155Transfer[] calldata batch1155Transfers
    ) external returns (bytes4 magicValue);

    /**
     * @notice Open or close a given channel. Only callable by the controller.
     *
     * @param channel The channel to open or close.
     * @param isOpen  The status of the channel (either open or closed).
     */
    function updateChannel(address channel, bool isOpen) external;
}

/**
 * @title ConduitControllerInterface
 * @author 0age
 * @notice ConduitControllerInterface contains all external function interfaces,
 *         structs, events, and errors for the conduit controller.
 */
interface ConduitControllerInterface {
    /**
     * @dev Track the conduit key, current owner, new potential owner, and open
     *      channels for each deployed conduit.
     */
    struct ConduitProperties {
        bytes32 key;
        address owner;
        address potentialOwner;
        address[] channels;
        mapping(address => uint256) channelIndexesPlusOne;
    }

    /**
     * @dev Emit an event whenever a new conduit is created.
     *
     * @param conduit    The newly created conduit.
     * @param conduitKey The conduit key used to create the new conduit.
     */
    event NewConduit(address conduit, bytes32 conduitKey);

    /**
     * @dev Emit an event whenever conduit ownership is transferred.
     *
     * @param conduit       The conduit for which ownership has been
     *                      transferred.
     * @param previousOwner The previous owner of the conduit.
     * @param newOwner      The new owner of the conduit.
     */
    event OwnershipTransferred(
        address indexed conduit,
        address indexed previousOwner,
        address indexed newOwner
    );

    /**
     * @dev Emit an event whenever a conduit owner registers a new potential
     *      owner for that conduit.
     *
     * @param newPotentialOwner The new potential owner of the conduit.
     */
    event PotentialOwnerUpdated(address indexed newPotentialOwner);

    /**
     * @dev Revert with an error when attempting to create a new conduit using a
     *      conduit key where the first twenty bytes of the key do not match the
     *      address of the caller.
     */
    error InvalidCreator();

    /**
     * @dev Revert with an error when attempting to create a new conduit when no
     *      initial owner address is supplied.
     */
    error InvalidInitialOwner();

    /**
     * @dev Revert with an error when attempting to set a new potential owner
     *      that is already set.
     */
    error NewPotentialOwnerAlreadySet(
        address conduit,
        address newPotentialOwner
    );

    /**
     * @dev Revert with an error when attempting to cancel ownership transfer
     *      when no new potential owner is currently set.
     */
    error NoPotentialOwnerCurrentlySet(address conduit);

    /**
     * @dev Revert with an error when attempting to interact with a conduit that
     *      does not yet exist.
     */
    error NoConduit();

    /**
     * @dev Revert with an error when attempting to create a conduit that
     *      already exists.
     */
    error ConduitAlreadyExists(address conduit);

    /**
     * @dev Revert with an error when attempting to update channels or transfer
     *      ownership of a conduit when the caller is not the owner of the
     *      conduit in question.
     */
    error CallerIsNotOwner(address conduit);

    /**
     * @dev Revert with an error when attempting to register a new potential
     *      owner and supplying the null address.
     */
    error NewPotentialOwnerIsZeroAddress(address conduit);

    /**
     * @dev Revert with an error when attempting to claim ownership of a conduit
     *      with a caller that is not the current potential owner for the
     *      conduit in question.
     */
    error CallerIsNotNewPotentialOwner(address conduit);

    /**
     * @dev Revert with an error when attempting to retrieve a channel using an
     *      index that is out of range.
     */
    error ChannelOutOfRange(address conduit);

    /**
     * @notice Deploy a new conduit using a supplied conduit key and assigning
     *         an initial owner for the deployed conduit. Note that the first
     *         twenty bytes of the supplied conduit key must match the caller
     *         and that a new conduit cannot be created if one has already been
     *         deployed using the same conduit key.
     *
     * @param conduitKey   The conduit key used to deploy the conduit. Note that
     *                     the first twenty bytes of the conduit key must match
     *                     the caller of this contract.
     * @param initialOwner The initial owner to set for the new conduit.
     *
     * @return conduit The address of the newly deployed conduit.
     */
    function createConduit(
        bytes32 conduitKey,
        address initialOwner
    ) external returns (address conduit);

    /**
     * @notice Open or close a channel on a given conduit, thereby allowing the
     *         specified account to execute transfers against that conduit.
     *         Extreme care must be taken when updating channels, as malicious
     *         or vulnerable channels can transfer any ERC20, ERC721 and ERC1155
     *         tokens where the token holder has granted the conduit approval.
     *         Only the owner of the conduit in question may call this function.
     *
     * @param conduit The conduit for which to open or close the channel.
     * @param channel The channel to open or close on the conduit.
     * @param isOpen  A boolean indicating whether to open or close the channel.
     */
    function updateChannel(
        address conduit,
        address channel,
        bool isOpen
    ) external;

    /**
     * @notice Initiate conduit ownership transfer by assigning a new potential
     *         owner for the given conduit. Once set, the new potential owner
     *         may call `acceptOwnership` to claim ownership of the conduit.
     *         Only the owner of the conduit in question may call this function.
     *
     * @param conduit The conduit for which to initiate ownership transfer.
     * @param newPotentialOwner The new potential owner of the conduit.
     */
    function transferOwnership(
        address conduit,
        address newPotentialOwner
    ) external;

    /**
     * @notice Clear the currently set potential owner, if any, from a conduit.
     *         Only the owner of the conduit in question may call this function.
     *
     * @param conduit The conduit for which to cancel ownership transfer.
     */
    function cancelOwnershipTransfer(address conduit) external;

    /**
     * @notice Accept ownership of a supplied conduit. Only accounts that the
     *         current owner has set as the new potential owner may call this
     *         function.
     *
     * @param conduit The conduit for which to accept ownership.
     */
    function acceptOwnership(address conduit) external;

    /**
     * @notice Retrieve the current owner of a deployed conduit.
     *
     * @param conduit The conduit for which to retrieve the associated owner.
     *
     * @return owner The owner of the supplied conduit.
     */
    function ownerOf(address conduit) external view returns (address owner);

    /**
     * @notice Retrieve the conduit key for a deployed conduit via reverse
     *         lookup.
     *
     * @param conduit The conduit for which to retrieve the associated conduit
     *                key.
     *
     * @return conduitKey The conduit key used to deploy the supplied conduit.
     */
    function getKey(address conduit) external view returns (bytes32 conduitKey);

    /**
     * @notice Derive the conduit associated with a given conduit key and
     *         determine whether that conduit exists (i.e. whether it has been
     *         deployed).
     *
     * @param conduitKey The conduit key used to derive the conduit.
     *
     * @return conduit The derived address of the conduit.
     * @return exists  A boolean indicating whether the derived conduit has been
     *                 deployed or not.
     */
    function getConduit(
        bytes32 conduitKey
    ) external view returns (address conduit, bool exists);

    /**
     * @notice Retrieve the potential owner, if any, for a given conduit. The
     *         current owner may set a new potential owner via
     *         `transferOwnership` and that owner may then accept ownership of
     *         the conduit in question via `acceptOwnership`.
     *
     * @param conduit The conduit for which to retrieve the potential owner.
     *
     * @return potentialOwner The potential owner, if any, for the conduit.
     */
    function getPotentialOwner(
        address conduit
    ) external view returns (address potentialOwner);

    /**
     * @notice Retrieve the status (either open or closed) of a given channel on
     *         a conduit.
     *
     * @param conduit The conduit for which to retrieve the channel status.
     * @param channel The channel for which to retrieve the status.
     *
     * @return isOpen The status of the channel on the given conduit.
     */
    function getChannelStatus(
        address conduit,
        address channel
    ) external view returns (bool isOpen);

    /**
     * @notice Retrieve the total number of open channels for a given conduit.
     *
     * @param conduit The conduit for which to retrieve the total channel count.
     *
     * @return totalChannels The total number of open channels for the conduit.
     */
    function getTotalChannels(
        address conduit
    ) external view returns (uint256 totalChannels);

    /**
     * @notice Retrieve an open channel at a specific index for a given conduit.
     *         Note that the index of a channel can change as a result of other
     *         channels being closed on the conduit.
     *
     * @param conduit      The conduit for which to retrieve the open channel.
     * @param channelIndex The index of the channel in question.
     *
     * @return channel The open channel, if any, at the specified channel index.
     */
    function getChannel(
        address conduit,
        uint256 channelIndex
    ) external view returns (address channel);

    /**
     * @notice Retrieve all open channels for a given conduit. Note that calling
     *         this function for a conduit with many channels will revert with
     *         an out-of-gas error.
     *
     * @param conduit The conduit for which to retrieve open channels.
     *
     * @return channels An array of open channels on the given conduit.
     */
    function getChannels(
        address conduit
    ) external view returns (address[] memory channels);

    /**
     * @dev Retrieve the conduit creation code and runtime code hashes.
     */
    function getConduitCodeHashes()
        external
        view
        returns (bytes32 creationCodeHash, bytes32 runtimeCodeHash);
}

/**
 * @title ConsiderationEventsAndErrors
 * @author 0age
 * @notice ConsiderationEventsAndErrors contains all events and errors.
 */
interface ConsiderationEventsAndErrors {
    /**
     * @dev Emit an event whenever an order is successfully fulfilled.
     *
     * @param orderHash     The hash of the fulfilled order.
     * @param offerer       The offerer of the fulfilled order.
     * @param zone          The zone of the fulfilled order.
     * @param recipient     The recipient of each spent item on the fulfilled
     *                      order, or the null address if there is no specific
     *                      fulfiller (i.e. the order is part of a group of
     *                      orders). Defaults to the caller unless explicitly
     *                      specified otherwise by the fulfiller.
     * @param offer         The offer items spent as part of the order.
     * @param consideration The consideration items received as part of the
     *                      order along with the recipients of each item.
     */
    event OrderFulfilled(
        bytes32 orderHash,
        address indexed offerer,
        address indexed zone,
        address recipient,
        SpentItem[] offer,
        ReceivedItem[] consideration
    );

    /**
     * @dev Emit an event whenever an order is successfully cancelled.
     *
     * @param orderHash The hash of the cancelled order.
     * @param offerer   The offerer of the cancelled order.
     * @param zone      The zone of the cancelled order.
     */
    event OrderCancelled(
        bytes32 orderHash,
        address indexed offerer,
        address indexed zone
    );

    /**
     * @dev Emit an event whenever an order is explicitly validated. Note that
     *      this event will not be emitted on partial fills even though they do
     *      validate the order as part of partial fulfillment.
     *
     * @param orderHash        The hash of the validated order.
     * @param orderParameters  The parameters of the validated order.
     */
    event OrderValidated(bytes32 orderHash, OrderParameters orderParameters);

    /**
     * @dev Emit an event whenever one or more orders are matched using either
     *      matchOrders or matchAdvancedOrders.
     *
     * @param orderHashes The order hashes of the matched orders.
     */
    event OrdersMatched(bytes32[] orderHashes);

    /**
     * @dev Emit an event whenever a counter for a given offerer is incremented.
     *
     * @param newCounter The new counter for the offerer.
     * @param offerer    The offerer in question.
     */
    event CounterIncremented(uint256 newCounter, address indexed offerer);

    /**
     * @dev Revert with an error when attempting to fill an order that has
     *      already been fully filled.
     *
     * @param orderHash The order hash on which a fill was attempted.
     */
    error OrderAlreadyFilled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to fill an order outside the
     *      specified start time and end time.
     *
     * @param startTime The time at which the order becomes active.
     * @param endTime   The time at which the order becomes inactive.
     */
    error InvalidTime(uint256 startTime, uint256 endTime);

    /**
     * @dev Revert with an error when attempting to fill an order referencing an
     *      invalid conduit (i.e. one that has not been deployed).
     */
    error InvalidConduit(bytes32 conduitKey, address conduit);

    /**
     * @dev Revert with an error when an order is supplied for fulfillment with
     *      a consideration array that is shorter than the original array.
     */
    error MissingOriginalConsiderationItems();

    /**
     * @dev Revert with an error when an order is validated and the length of
     *      the consideration array is not equal to the supplied total original
     *      consideration items value. This error is also thrown when contract
     *      orders supply a total original consideration items value that does
     *      not match the supplied consideration array length.
     */
    error ConsiderationLengthNotEqualToTotalOriginal();

    /**
     * @dev Revert with an error when a call to a conduit fails with revert data
     *      that is too expensive to return.
     */
    error InvalidCallToConduit(address conduit);

    /**
     * @dev Revert with an error if a consideration amount has not been fully
     *      zeroed out after applying all fulfillments.
     *
     * @param orderIndex         The index of the order with the consideration
     *                           item with a shortfall.
     * @param considerationIndex The index of the consideration item on the
     *                           order.
     * @param shortfallAmount    The unfulfilled consideration amount.
     */
    error ConsiderationNotMet(
        uint256 orderIndex,
        uint256 considerationIndex,
        uint256 shortfallAmount
    );

    /**
     * @dev Revert with an error when insufficient native tokens are supplied as
     *      part of msg.value when fulfilling orders.
     */
    error InsufficientNativeTokensSupplied();

    /**
     * @dev Revert with an error when a native token transfer reverts.
     */
    error NativeTokenTransferGenericFailure(address account, uint256 amount);

    /**
     * @dev Revert with an error when a partial fill is attempted on an order
     *      that does not specify partial fill support in its order type.
     */
    error PartialFillsNotEnabledForOrder();

    /**
     * @dev Revert with an error when attempting to fill an order that has been
     *      cancelled.
     *
     * @param orderHash The hash of the cancelled order.
     */
    error OrderIsCancelled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to fill a basic order that has
     *      been partially filled.
     *
     * @param orderHash The hash of the partially used order.
     */
    error OrderPartiallyFilled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to cancel an order as a caller
     *      other than the indicated offerer or zone or when attempting to
     *      cancel a contract order.
     */
    error CannotCancelOrder();

    /**
     * @dev Revert with an error when supplying a fraction with a value of zero
     *      for the numerator or denominator, or one where the numerator exceeds
     *      the denominator.
     */
    error BadFraction();

    /**
     * @dev Revert with an error when a caller attempts to supply callvalue to a
     *      non-payable basic order route or does not supply any callvalue to a
     *      payable basic order route.
     */
    error InvalidMsgValue(uint256 value);

    /**
     * @dev Revert with an error when attempting to fill a basic order using
     *      calldata not produced by default ABI encoding.
     */
    error InvalidBasicOrderParameterEncoding();

    /**
     * @dev Revert with an error when attempting to fulfill any number of
     *      available orders when none are fulfillable.
     */
    error NoSpecifiedOrdersAvailable();

    /**
     * @dev Revert with an error when attempting to fulfill an order with an
     *      offer for a native token outside of matching orders.
     */
    error InvalidNativeOfferItem();
}

/*
 * -------------------------- Disambiguation & Other Notes ---------------------
 *    - The term "head" is used as it is in the documentation for ABI encoding,
 *      but only in reference to dynamic types, i.e. it always refers to the
 *      offset or pointer to the body of a dynamic type. In calldata, the head
 *      is always an offset (relative to the parent object), while in memory,
 *      the head is always the pointer to the body. More information found here:
 *      https://docs.soliditylang.org/en/v0.8.17/abi-spec.html#argument-encoding
 *        - Note that the length of an array is separate from and precedes the
 *          head of the array.
 *
 *    - The term "body" is used in place of the term "head" used in the ABI
 *      documentation. It refers to the start of the data for a dynamic type,
 *      e.g. the first word of a struct or the first word of the first element
 *      in an array.
 *
 *    - The term "pointer" is used to describe the absolute position of a value
 *      and never an offset relative to another value.
 *        - The suffix "_ptr" refers to a memory pointer.
 *        - The suffix "_cdPtr" refers to a calldata pointer.
 *
 *    - The term "offset" is used to describe the position of a value relative
 *      to some parent value. For example, OrderParameters_conduit_offset is the
 *      offset to the "conduit" value in the OrderParameters struct relative to
 *      the start of the body.
 *        - Note: Offsets are used to derive pointers.
 *
 *    - Some structs have pointers defined for all of their fields in this file.
 *      Lines which are commented out are fields that are not used in the
 *      codebase but have been left in for readability.
 */

// Declare constants for name, version, and reentrancy sentinel values.

// Name is right padded, so it touches the length which is left padded. This
// enables writing both values at once. Length goes at byte 95 in memory, and
// name fills bytes 96-109, so both values can be written left-padded to 77.
uint256 constant NameLengthPtr = 0x4D;
uint256 constant NameWithLength = 0x0d436F6E73696465726174696F6E;

uint256 constant information_version_offset = 0;
uint256 constant information_version_cd_offset = 0x60;
uint256 constant information_domainSeparator_offset = 0x20;
uint256 constant information_conduitController_offset = 0x40;
uint256 constant information_versionLengthPtr = 0x63;
uint256 constant information_versionWithLength = 0x03312e35; // 1.5
uint256 constant information_length = 0xa0;

uint256 constant _NOT_ENTERED = 1;
uint256 constant _ENTERED = 2;
uint256 constant _ENTERED_AND_ACCEPTING_NATIVE_TOKENS = 3;

uint256 constant Offset_fulfillAdvancedOrder_criteriaResolvers = 0x20;
uint256 constant Offset_fulfillAvailableOrders_offerFulfillments = 0x20;
uint256 constant Offset_fulfillAvailableOrders_considerationFulfillments = 0x40;
uint256 constant Offset_fulfillAvailableAdvancedOrders_criteriaResolvers = 0x20;
uint256 constant Offset_fulfillAvailableAdvancedOrders_offerFulfillments = 0x40;
uint256 constant Offset_fulfillAvailableAdvancedOrders_cnsdrationFlflmnts = (
    0x60
);

uint256 constant Offset_matchOrders_fulfillments = 0x20;

uint256 constant Offset_matchAdvancedOrders_criteriaResolvers = 0x20;
uint256 constant Offset_matchAdvancedOrders_fulfillments = 0x40;

// Common Offsets
// Offsets for identically positioned fields shared by:
// OfferItem, ConsiderationItem, SpentItem, ReceivedItem

uint256 constant Selector_length = 0x4;

uint256 constant Common_token_offset = 0x20;
uint256 constant Common_identifier_offset = 0x40;
uint256 constant Common_amount_offset = 0x60;
uint256 constant Common_endAmount_offset = 0x80;

uint256 constant SpentItem_size = 0x80;
uint256 constant SpentItem_size_shift = 0x7;

uint256 constant OfferItem_size = 0xa0;
uint256 constant OfferItem_size_with_length = 0xc0;

uint256 constant ReceivedItem_size_excluding_recipient = 0x80;
uint256 constant ReceivedItem_size = 0xa0;
uint256 constant ReceivedItem_amount_offset = 0x60;
uint256 constant ReceivedItem_recipient_offset = 0x80;

uint256 constant ReceivedItem_CommonParams_size = 0x60;

uint256 constant ConsiderationItem_size = 0xc0;
uint256 constant ConsiderationItem_size_with_length = 0xe0;

uint256 constant ConsiderationItem_recipient_offset = 0xa0;
// Store the same constant in an abbreviated format for a line length fix.
uint256 constant ConsiderItem_recipient_offset = 0xa0;

uint256 constant Execution_offerer_offset = 0x20;
uint256 constant Execution_conduit_offset = 0x40;

// uint256 constant OrderParameters_offerer_offset = 0x00;
uint256 constant OrderParameters_zone_offset = 0x20;
uint256 constant OrderParameters_offer_head_offset = 0x40;
uint256 constant OrderParameters_consideration_head_offset = 0x60;
// uint256 constant OrderParameters_orderType_offset = 0x80;
uint256 constant OrderParameters_startTime_offset = 0xa0;
uint256 constant OrderParameters_endTime_offset = 0xc0;
uint256 constant OrderParameters_zoneHash_offset = 0xe0;
// uint256 constant OrderParameters_salt_offset = 0x100;
uint256 constant OrderParameters_conduit_offset = 0x120;
uint256 constant OrderParameters_counter_offset = 0x140;

uint256 constant Fulfillment_itemIndex_offset = 0x20;

uint256 constant AdvancedOrder_head_size = 0xa0;
uint256 constant AdvancedOrder_numerator_offset = 0x20;
uint256 constant AdvancedOrder_denominator_offset = 0x40;
uint256 constant AdvancedOrder_signature_offset = 0x60;
uint256 constant AdvancedOrder_extraData_offset = 0x80;

uint256 constant OrderStatus_ValidatedAndNotCancelled = 1;
uint256 constant OrderStatus_filledNumerator_offset = 0x10;
uint256 constant OrderStatus_filledDenominator_offset = 0x88;

uint256 constant ThirtyOneBytes = 0x1f;
uint256 constant OneWord = 0x20;
uint256 constant TwoWords = 0x40;
uint256 constant ThreeWords = 0x60;
uint256 constant FourWords = 0x80;
uint256 constant FiveWords = 0xa0;

uint256 constant OneWordShift = 0x5;
uint256 constant TwoWordsShift = 0x6;

uint256 constant SixtyThreeBytes = 0x3f;
uint256 constant OnlyFullWordMask = 0xffffffe0;

uint256 constant FreeMemoryPointerSlot = 0x40;
uint256 constant ZeroSlot = 0x60;
uint256 constant DefaultFreeMemoryPointer = 0x80;

uint256 constant Slot0x80 = 0x80;
uint256 constant Slot0xA0 = 0xa0;

// uint256 constant BasicOrder_endAmount_cdPtr = 0x104;
uint256 constant BasicOrder_common_params_size = 0xa0;
uint256 constant BasicOrder_considerationHashesArray_ptr = 0x160;
uint256 constant BasicOrder_receivedItemByteMap = (
    0x0000010102030000000000000000000000000000000000000000000000000000
);
uint256 constant BasicOrder_offeredItemByteMap = (
    0x0203020301010000000000000000000000000000000000000000000000000000
);

bytes32 constant OrdersMatchedTopic0 = (
    0x4b9f2d36e1b4c93de62cc077b00b1a91d84b6c31b4a14e012718dcca230689e7
);

uint256 constant EIP712_Order_size = 0x180;
uint256 constant EIP712_OfferItem_size = 0xc0;
uint256 constant EIP712_ConsiderationItem_size = 0xe0;
uint256 constant AdditionalRecipient_size = 0x40;
uint256 constant AdditionalRecipient_size_shift = 0x6;

uint256 constant EIP712_DomainSeparator_offset = 0x02;
uint256 constant EIP712_OrderHash_offset = 0x22;
uint256 constant EIP712_DigestPayload_size = 0x42;

uint256 constant EIP712_domainData_nameHash_offset = 0x20;
uint256 constant EIP712_domainData_versionHash_offset = 0x40;
uint256 constant EIP712_domainData_chainId_offset = 0x60;
uint256 constant EIP712_domainData_verifyingContract_offset = 0x80;
uint256 constant EIP712_domainData_size = 0xa0;

// Minimum BulkOrder proof size: 64 bytes for signature + 3 for key + 32 for 1
// sibling. Maximum BulkOrder proof size: 65 bytes for signature + 3 for key +
// 768 for 24 siblings.

uint256 constant BulkOrderProof_minSize = 0x63;
uint256 constant BulkOrderProof_rangeSize = 0x2e2;
uint256 constant BulkOrderProof_lengthAdjustmentBeforeMask = 0x1d;
uint256 constant BulkOrderProof_lengthRangeAfterMask = 0x2;
uint256 constant BulkOrderProof_keyShift = 0xe8;
uint256 constant BulkOrderProof_keySize = 0x3;

uint256 constant BulkOrder_Typehash_Height_One = (
    0x3ca2711d29384747a8f61d60aad3c450405f7aaff5613541dee28df2d6986d32
);
uint256 constant BulkOrder_Typehash_Height_Two = (
    0xbf8e29b89f29ed9b529c154a63038ffca562f8d7cd1e2545dda53a1b582dde30
);
uint256 constant BulkOrder_Typehash_Height_Three = (
    0x53c6f6856e13104584dd0797ca2b2779202dc2597c6066a42e0d8fe990b0024d
);
uint256 constant BulkOrder_Typehash_Height_Four = (
    0xa02eb7ff164c884e5e2c336dc85f81c6a93329d8e9adf214b32729b894de2af1
);
uint256 constant BulkOrder_Typehash_Height_Five = (
    0x39c9d33c18e050dda0aeb9a8086fb16fc12d5d64536780e1da7405a800b0b9f6
);
uint256 constant BulkOrder_Typehash_Height_Six = (
    0x1c19f71958cdd8f081b4c31f7caf5c010b29d12950be2fa1c95070dc47e30b55
);
uint256 constant BulkOrder_Typehash_Height_Seven = (
    0xca74fab2fece9a1d58234a274220ad05ca096a92ef6a1ca1750b9d90c948955c
);
uint256 constant BulkOrder_Typehash_Height_Eight = (
    0x7ff98d9d4e55d876c5cfac10b43c04039522f3ddfb0ea9bfe70c68cfb5c7cc14
);
uint256 constant BulkOrder_Typehash_Height_Nine = (
    0xbed7be92d41c56f9e59ac7a6272185299b815ddfabc3f25deb51fe55fe2f9e8a
);
uint256 constant BulkOrder_Typehash_Height_Ten = (
    0xd1d97d1ef5eaa37a4ee5fbf234e6f6d64eb511eb562221cd7edfbdde0848da05
);
uint256 constant BulkOrder_Typehash_Height_Eleven = (
    0x896c3f349c4da741c19b37fec49ed2e44d738e775a21d9c9860a69d67a3dae53
);
uint256 constant BulkOrder_Typehash_Height_Twelve = (
    0xbb98d87cc12922b83759626c5f07d72266da9702d19ffad6a514c73a89002f5f
);
uint256 constant BulkOrder_Typehash_Height_Thirteen = (
    0xe6ae19322608dd1f8a8d56aab48ed9c28be489b689f4b6c91268563efc85f20e
);
uint256 constant BulkOrder_Typehash_Height_Fourteen = (
    0x6b5b04cbae4fcb1a9d78e7b2dfc51a36933d023cf6e347e03d517b472a852590
);
uint256 constant BulkOrder_Typehash_Height_Fifteen = (
    0xd1eb68309202b7106b891e109739dbbd334a1817fe5d6202c939e75cf5e35ca9
);
uint256 constant BulkOrder_Typehash_Height_Sixteen = (
    0x1da3eed3ecef6ebaa6e5023c057ec2c75150693fd0dac5c90f4a142f9879fde8
);
uint256 constant BulkOrder_Typehash_Height_Seventeen = (
    0xeee9a1392aa395c7002308119a58f2582777a75e54e0c1d5d5437bd2e8bf6222
);
uint256 constant BulkOrder_Typehash_Height_Eighteen = (
    0xc3939feff011e53ab8c35ca3370aad54c5df1fc2938cd62543174fa6e7d85877
);
uint256 constant BulkOrder_Typehash_Height_Nineteen = (
    0x0efca7572ac20f5ae84db0e2940674f7eca0a4726fa1060ffc2d18cef54b203d
);
uint256 constant BulkOrder_Typehash_Height_Twenty = (
    0x5a4f867d3d458dabecad65f6201ceeaba0096df2d0c491cc32e6ea4e64350017
);
uint256 constant BulkOrder_Typehash_Height_TwentyOne = (
    0x80987079d291feebf21c2230e69add0f283cee0b8be492ca8050b4185a2ff719
);
uint256 constant BulkOrder_Typehash_Height_TwentyTwo = (
    0x3bd8cff538aba49a9c374c806d277181e9651624b3e31111bc0624574f8bca1d
);
uint256 constant BulkOrder_Typehash_Height_TwentyThree = (
    0x5d6a3f098a0bc373f808c619b1bb4028208721b3c4f8d6bc8a874d659814eb76
);
uint256 constant BulkOrder_Typehash_Height_TwentyFour = (
    0x1d51df90cba8de7637ca3e8fe1e3511d1dc2f23487d05dbdecb781860c21ac1c
);

uint256 constant receivedItemsHash_ptr = 0x60;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  data for OrderFulfilled
 *
 *   event OrderFulfilled(
 *     bytes32 orderHash,
 *     address indexed offerer,
 *     address indexed zone,
 *     address fulfiller,
 *     SpentItem[] offer,
 *       > (itemType, token, id, amount)
 *     ReceivedItem[] consideration
 *       > (itemType, token, id, amount, recipient)
 *   )
 *
 *  - 0x00: orderHash
 *  - 0x20: fulfiller
 *  - 0x40: offer offset (0x80)
 *  - 0x60: consideration offset (0x120)
 *  - 0x80: offer.length (1)
 *  - 0xa0: offerItemType
 *  - 0xc0: offerToken
 *  - 0xe0: offerIdentifier
 *  - 0x100: offerAmount
 *  - 0x120: consideration.length (1 + additionalRecipients.length)
 *  - 0x140: considerationItemType
 *  - 0x160: considerationToken
 *  - 0x180: considerationIdentifier
 *  - 0x1a0: considerationAmount
 *  - 0x1c0: considerationRecipient
 *  - ...
 */

// Minimum length of the OrderFulfilled event data.
// Must be added to the size of the ReceivedItem array for additionalRecipients
// (0xa0 * additionalRecipients.length) to calculate full size of the buffer.
uint256 constant OrderFulfilled_baseSize = 0x1e0;
uint256 constant OrderFulfilled_selector = (
    0x9d9af8e38d66c62e2c12f0225249fd9d721c54b83f48d9352c97c6cacdcb6f31
);

// Minimum offset in memory to OrderFulfilled event data.
// Must be added to the size of the EIP712 hash array for additionalRecipients
// (32 * additionalRecipients.length) to calculate the pointer to event data.
uint256 constant OrderFulfilled_baseOffset = 0x180;
uint256 constant OrderFulfilled_consideration_length_baseOffset = 0x2a0;
uint256 constant OrderFulfilled_offer_length_baseOffset = 0x200;

// Related constants used for restricted order checks on basic orders.
uint256 constant OrderFulfilled_baseDataSize = 0x160;
// uint256 constant ValidateOrder_offerDataOffset = 0x184;
// uint256 constant RatifyOrder_offerDataOffset = 0xc4;

// uint256 constant OrderFulfilled_orderHash_offset = 0x00;
uint256 constant OrderFulfilled_fulfiller_offset = 0x20;
uint256 constant OrderFulfilled_offer_head_offset = 0x40;
uint256 constant OrderFulfilled_offer_body_offset = 0x80;
uint256 constant OrderFulfilled_consideration_head_offset = 0x60;
uint256 constant OrderFulfilled_consideration_body_offset = 0x120;

// BasicOrderParameters
uint256 constant BasicOrder_parameters_cdPtr = 0x04;
uint256 constant BasicOrder_considerationToken_cdPtr = 0x24;
uint256 constant BasicOrder_considerationIdentifier_cdPtr = 0x44;
uint256 constant BasicOrder_considerationAmount_cdPtr = 0x64;
uint256 constant BasicOrder_offerer_cdPtr = 0x84;
uint256 constant BasicOrder_zone_cdPtr = 0xa4;
uint256 constant BasicOrder_offerToken_cdPtr = 0xc4;
uint256 constant BasicOrder_offerIdentifier_cdPtr = 0xe4;
uint256 constant BasicOrder_offerAmount_cdPtr = 0x104;
uint256 constant BasicOrder_basicOrderType_cdPtr = 0x124;
uint256 constant BasicOrder_startTime_cdPtr = 0x144;
uint256 constant BasicOrder_endTime_cdPtr = 0x164;
// uint256 constant BasicOrder_zoneHash_cdPtr = 0x184;
// uint256 constant BasicOrder_salt_cdPtr = 0x1a4;
uint256 constant BasicOrder_offererConduit_cdPtr = 0x1c4;
uint256 constant BasicOrder_fulfillerConduit_cdPtr = 0x1e4;
uint256 constant BasicOrder_totalOriginalAdditionalRecipients_cdPtr = 0x204;
uint256 constant BasicOrder_additionalRecipients_head_cdPtr = 0x224;
uint256 constant BasicOrder_signature_cdPtr = 0x244;
uint256 constant BasicOrder_additionalRecipients_length_cdPtr = 0x264;
uint256 constant BasicOrder_additionalRecipients_data_cdPtr = 0x284;
uint256 constant BasicOrder_parameters_ptr = 0x20;
uint256 constant BasicOrder_basicOrderType_range = 0x18; // 24 values

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for ConsiderationItem
 *   - 0x80: ConsiderationItem EIP-712 typehash (constant)
 *   - 0xa0: itemType
 *   - 0xc0: token
 *   - 0xe0: identifier
 *   - 0x100: startAmount
 *   - 0x120: endAmount
 *   - 0x140: recipient
 */
uint256 constant BasicOrder_considerationItem_typeHash_ptr = 0x80; // memoryPtr
uint256 constant BasicOrder_considerationItem_itemType_ptr = 0xa0;
uint256 constant BasicOrder_considerationItem_token_ptr = 0xc0;
uint256 constant BasicOrder_considerationItem_identifier_ptr = 0xe0;
uint256 constant BasicOrder_considerationItem_startAmount_ptr = 0x100;
uint256 constant BasicOrder_considerationItem_endAmount_ptr = 0x120;
// uint256 constant BasicOrder_considerationItem_recipient_ptr = 0x140;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for OfferItem
 *   - 0x80:  OfferItem EIP-712 typehash (constant)
 *   - 0xa0:  itemType
 *   - 0xc0:  token
 *   - 0xe0:  identifier (reused for offeredItemsHash)
 *   - 0x100: startAmount
 *   - 0x120: endAmount
 */
uint256 constant BasicOrder_offerItem_typeHash_ptr = 0x80;
uint256 constant BasicOrder_offerItem_itemType_ptr = 0xa0;
uint256 constant BasicOrder_offerItem_token_ptr = 0xc0;
// uint256 constant BasicOrder_offerItem_identifier_ptr = 0xe0;
// uint256 constant BasicOrder_offerItem_startAmount_ptr = 0x100;
uint256 constant BasicOrder_offerItem_endAmount_ptr = 0x120;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for Order
 *   - 0x80:   Order EIP-712 typehash (constant)
 *   - 0xa0:   orderParameters.offerer
 *   - 0xc0:   orderParameters.zone
 *   - 0xe0:   keccak256(abi.encodePacked(offerHashes))
 *   - 0x100:  keccak256(abi.encodePacked(considerationHashes))
 *   - 0x120:  orderType
 *   - 0x140:  startTime
 *   - 0x160:  endTime
 *   - 0x180:  zoneHash
 *   - 0x1a0:  salt
 *   - 0x1c0:  conduit
 *   - 0x1e0:  _counters[orderParameters.offerer] (from storage)
 */
uint256 constant BasicOrder_order_typeHash_ptr = 0x80;
uint256 constant BasicOrder_order_offerer_ptr = 0xa0;
// uint256 constant BasicOrder_order_zone_ptr = 0xc0;
uint256 constant BasicOrder_order_offerHashes_ptr = 0xe0;
uint256 constant BasicOrder_order_considerationHashes_ptr = 0x100;
uint256 constant BasicOrder_order_orderType_ptr = 0x120;
uint256 constant BasicOrder_order_startTime_ptr = 0x140;
// uint256 constant BasicOrder_order_endTime_ptr = 0x160;
// uint256 constant BasicOrder_order_zoneHash_ptr = 0x180;
// uint256 constant BasicOrder_order_salt_ptr = 0x1a0;
// uint256 constant BasicOrder_order_conduitKey_ptr = 0x1c0;
uint256 constant BasicOrder_order_counter_ptr = 0x1e0;
uint256 constant BasicOrder_additionalRecipients_head_ptr = 0x240;
uint256 constant BasicOrder_signature_ptr = 0x260;
uint256 constant BasicOrder_startTimeThroughZoneHash_size = 0x60;

uint256 constant ContractOrder_orderHash_offerer_shift = 0x60;

uint256 constant Counter_blockhash_shift = 0x80;

// Signature-related
bytes32 constant EIP2098_allButHighestBitMask = (
    0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
);
bytes32 constant ECDSA_twentySeventhAndTwentyEighthBytesSet = (
    0x0000000000000000000000000000000000000000000000000000000101000000
);
uint256 constant ECDSA_MaxLength = 65;
uint256 constant ECDSA_signature_s_offset = 0x40;
uint256 constant ECDSA_signature_v_offset = 0x60;

bytes32 constant EIP1271_isValidSignature_selector = (
    0x1626ba7e00000000000000000000000000000000000000000000000000000000
);
uint256 constant EIP1271_isValidSignature_digest_negativeOffset = 0x40;
uint256 constant EIP1271_isValidSignature_selector_negativeOffset = 0x44;
uint256 constant EIP1271_isValidSignature_calldata_baseLength = 0x64;
uint256 constant EIP1271_isValidSignature_signature_head_offset = 0x40;

uint256 constant EIP_712_PREFIX = (
    0x1901000000000000000000000000000000000000000000000000000000000000
);

uint256 constant ExtraGasBuffer = 0x20;
uint256 constant CostPerWord = 0x3;
uint256 constant MemoryExpansionCoefficientShift = 0x9;

uint256 constant Create2AddressDerivation_ptr = 0x0b;
uint256 constant Create2AddressDerivation_length = 0x55;

uint256 constant MaskOverByteTwelve = (
    0x0000000000000000000000ff0000000000000000000000000000000000000000
);
uint256 constant MaskOverLastTwentyBytes = (
    0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff
);
uint256 constant AddressDirtyUpperBitThreshold = (
    0x0000000000000000000000010000000000000000000000000000000000000000
);
uint256 constant MaskOverFirstFourBytes = (
    0xffffffff00000000000000000000000000000000000000000000000000000000
);

uint256 constant Conduit_execute_signature = (
    0x4ce34aa200000000000000000000000000000000000000000000000000000000
);

uint256 constant MaxUint8 = 0xff;
uint256 constant MaxUint120 = 0xffffffffffffffffffffffffffffff;

uint256 constant Conduit_execute_ConduitTransfer_ptr = 0x20;
uint256 constant Conduit_execute_ConduitTransfer_length = 0x01;
uint256 constant Conduit_execute_ConduitTransfer_offset_ptr = 0x04;
uint256 constant Conduit_execute_ConduitTransfer_length_ptr = 0x24;
uint256 constant Conduit_execute_transferItemType_ptr = 0x44;
uint256 constant Conduit_execute_transferToken_ptr = 0x64;
uint256 constant Conduit_execute_transferFrom_ptr = 0x84;
uint256 constant Conduit_execute_transferTo_ptr = 0xa4;
uint256 constant Conduit_execute_transferIdentifier_ptr = 0xc4;
uint256 constant Conduit_execute_transferAmount_ptr = 0xe4;

uint256 constant OneConduitExecute_size = 0x104;

// Sentinel value to indicate that the conduit accumulator is not armed.
uint256 constant AccumulatorDisarmed = 0x20;
uint256 constant AccumulatorArmed = 0x40;
uint256 constant Accumulator_conduitKey_ptr = 0x20;
uint256 constant Accumulator_selector_ptr = 0x40;
uint256 constant Accumulator_array_offset_ptr = 0x44;
uint256 constant Accumulator_array_length_ptr = 0x64;
uint256 constant Accumulator_itemSizeOffsetDifference = 0x3c;
uint256 constant Accumulator_array_offset = 0x20;

uint256 constant Conduit_transferItem_size = 0xc0;
uint256 constant Conduit_transferItem_token_ptr = 0x20;
uint256 constant Conduit_transferItem_from_ptr = 0x40;
uint256 constant Conduit_transferItem_to_ptr = 0x60;
uint256 constant Conduit_transferItem_identifier_ptr = 0x80;
uint256 constant Conduit_transferItem_amount_ptr = 0xa0;

uint256 constant Ecrecover_precompile = 0x1;
uint256 constant Ecrecover_args_size = 0x80;
uint256 constant Signature_lower_v = 27;

// Bitmask that only gives a non-zero value if masked with a non-match selector.
uint256 constant NonMatchSelector_MagicMask = (
    0x4000000000000000000000000000000000000000000000000000000000
);

// First bit indicates that a NATIVE offer items has been used and the 231st bit
// indicates that a non match selector has been called.
uint256 constant NonMatchSelector_InvalidErrorValue = (
    0x4000000000000000000000000000000000000000000000000000000001
);

/**
 * @dev Selector and offsets for generateOrder
 *
 * function generateOrder(
 *   address fulfiller,
 *   SpentItem[] calldata minimumReceived,
 *   SpentItem[] calldata maximumSpent,
 *   bytes calldata context
 * )
 */
uint256 constant generateOrder_selector = 0x98919765;
uint256 constant generateOrder_selector_offset = 0x1c;
uint256 constant generateOrder_head_offset = 0x04;
uint256 constant generateOrder_minimumReceived_head_offset = 0x20;
uint256 constant generateOrder_maximumSpent_head_offset = 0x40;
uint256 constant generateOrder_context_head_offset = 0x60;
uint256 constant generateOrder_base_tail_offset = 0x80;
uint256 constant generateOrder_maximum_returndatasize = 0xffff;

uint256 constant ratifyOrder_selector = 0xf4dd92ce;
uint256 constant ratifyOrder_selector_offset = 0x1c;
uint256 constant ratifyOrder_head_offset = 0x04;
// uint256 constant ratifyOrder_offer_head_offset = 0x00;
uint256 constant ratifyOrder_consideration_head_offset = 0x20;
uint256 constant ratifyOrder_context_head_offset = 0x40;
uint256 constant ratifyOrder_orderHashes_head_offset = 0x60;
uint256 constant ratifyOrder_contractNonce_offset = 0x80;
uint256 constant ratifyOrder_base_tail_offset = 0xa0;

uint256 constant validateOrder_selector = 0x17b1f942;
uint256 constant validateOrder_selector_offset = 0x1c;
uint256 constant validateOrder_head_offset = 0x04;
uint256 constant validateOrder_zoneParameters_offset = 0x20;

// uint256 constant ZoneParameters_orderHash_offset = 0x00;
uint256 constant ZoneParameters_fulfiller_offset = 0x20;
uint256 constant ZoneParameters_offerer_offset = 0x40;
uint256 constant ZoneParameters_offer_head_offset = 0x60;
uint256 constant ZoneParameters_consideration_head_offset = 0x80;
uint256 constant ZoneParameters_extraData_head_offset = 0xa0;
uint256 constant ZoneParameters_orderHashes_head_offset = 0xc0;
uint256 constant ZoneParameters_startTime_offset = 0xe0;
uint256 constant ZoneParameters_endTime_offset = 0x100;
uint256 constant ZoneParameters_zoneHash_offset = 0x120;
uint256 constant ZoneParameters_base_tail_offset = 0x140;
uint256 constant ZoneParameters_selectorAndPointer_length = 0x24;
uint256 constant ZoneParameters_basicOrderFixedElements_length = 0x64;

// ConsiderationDecoder Constants
uint256 constant OrderParameters_head_size = 0x0160;
uint256 constant OrderParameters_totalOriginalConsiderationItems_offset = (
    0x0140
);
uint256 constant AdvancedOrderPlusOrderParameters_head_size = 0x0200;

uint256 constant Order_signature_offset = 0x20;
uint256 constant Order_head_size = 0x40;

uint256 constant AdvancedOrder_fixed_segment_0 = 0x40;

uint256 constant CriteriaResolver_head_size = 0xa0;
uint256 constant CriteriaResolver_fixed_segment_0 = 0x80;
uint256 constant CriteriaResolver_criteriaProof_offset = 0x80;

uint256 constant FulfillmentComponent_mem_tail_size = 0x40;
uint256 constant FulfillmentComponent_mem_tail_size_shift = 0x6;
uint256 constant Fulfillment_head_size = 0x40;
uint256 constant Fulfillment_considerationComponents_offset = 0x20;

uint256 constant OrderComponents_OrderParameters_common_head_size = 0x0140;

contract ConsiderationDecoder {
    /**
     * @dev Takes a bytes array from calldata and copies it into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the bytes array in
     *                    calldata which contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the bytes array in
     *                    memory which contains the length of the array.
     */
    function _decodeBytes(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        assembly {
            // Get the current free memory pointer.
            mPtrLength := mload(FreeMemoryPointerSlot)

            // Derive the size of the bytes array, rounding up to nearest word
            // and adding a word for the length field. Note: masking
            // `calldataload(cdPtrLength)` is redundant here.
            let size := add(and(add(calldataload(cdPtrLength), ThirtyOneBytes), OnlyFullWordMask), OneWord)

            // Copy bytes from calldata into memory based on pointers and size.
            calldatacopy(mPtrLength, cdPtrLength, size)

            // Store the masked value in memory. Note: the value of `size` is at
            // least 32, meaning the calldatacopy above will at least write to
            // `[mPtrLength, mPtrLength + 32)`.
            mstore(mPtrLength, and(calldataload(cdPtrLength), OffsetOrLengthMask))

            // Update free memory pointer based on the size of the bytes array.
            mstore(FreeMemoryPointerSlot, add(mPtrLength, size))
        }
    }

    /**
     * @dev Takes an offer array from calldata and copies it into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the offer array
     *                    in calldata which contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the offer array in
     *                    memory which contains the length of the array.
     */
    function _decodeOffer(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        assembly {
            // Retrieve length of array, masking to prevent potential overflow.
            let arrLength := and(calldataload(cdPtrLength), OffsetOrLengthMask)

            // Get the current free memory pointer.
            mPtrLength := mload(FreeMemoryPointerSlot)

            // Write the array length to memory.
            mstore(mPtrLength, arrLength)

            // Derive the head by adding one word to the length pointer.
            let mPtrHead := add(mPtrLength, OneWord)

            // Derive the tail by adding one word per element (note that structs
            // are written to memory with an offset per struct element).
            let mPtrTail := add(mPtrHead, shl(OneWordShift, arrLength))

            // Track the next tail, beginning with the initial tail value.
            let mPtrTailNext := mPtrTail

            // Copy all offer array data into memory at the tail pointer.
            calldatacopy(mPtrTail, add(cdPtrLength, OneWord), mul(arrLength, OfferItem_size))

            // Track the next head pointer, starting with initial head value.
            let mPtrHeadNext := mPtrHead

            // Iterate over each head pointer until it reaches the tail.
            for {} lt(mPtrHeadNext, mPtrTail) {} {
                // Write the next tail pointer to next head pointer in memory.
                mstore(mPtrHeadNext, mPtrTailNext)

                // Increment the next head pointer by one word.
                mPtrHeadNext := add(mPtrHeadNext, OneWord)

                // Increment the next tail pointer by the size of an offer item.
                mPtrTailNext := add(mPtrTailNext, OfferItem_size)
            }

            // Update free memory pointer to allocate memory up to end of tail.
            mstore(FreeMemoryPointerSlot, mPtrTailNext)
        }
    }

    /**
     * @dev Takes a consideration array from calldata and copies it into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the consideration
     *                    array in calldata which contains the length of the
     *                    array.
     *
     * @return mPtrLength A memory pointer to the start of the consideration
     *                    array in memory which contains the length of the
     *                    array.
     */
    function _decodeConsideration(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        assembly {
            // Retrieve length of array, masking to prevent potential overflow.
            let arrLength := and(calldataload(cdPtrLength), OffsetOrLengthMask)

            // Get the current free memory pointer.
            mPtrLength := mload(FreeMemoryPointerSlot)

            // Write the array length to memory.
            mstore(mPtrLength, arrLength)

            // Derive the head by adding one word to the length pointer.
            let mPtrHead := add(mPtrLength, OneWord)

            // Derive the tail by adding one word per element (note that structs
            // are written to memory with an offset per struct element).
            let mPtrTail := add(mPtrHead, shl(OneWordShift, arrLength))

            // Track the next tail, beginning with the initial tail value.
            let mPtrTailNext := mPtrTail

            // Copy all consideration array data into memory at tail pointer.
            calldatacopy(mPtrTail, add(cdPtrLength, OneWord), mul(arrLength, ConsiderationItem_size))

            // Track the next head pointer, starting with initial head value.
            let mPtrHeadNext := mPtrHead

            // Iterate over each head pointer until it reaches the tail.
            for {} lt(mPtrHeadNext, mPtrTail) {} {
                // Write the next tail pointer to next head pointer in memory.
                mstore(mPtrHeadNext, mPtrTailNext)

                // Increment the next head pointer by one word.
                mPtrHeadNext := add(mPtrHeadNext, OneWord)

                // Increment next tail pointer by size of a consideration item.
                mPtrTailNext := add(mPtrTailNext, ConsiderationItem_size)
            }

            // Update free memory pointer to allocate memory up to end of tail.
            mstore(FreeMemoryPointerSlot, mPtrTailNext)
        }
    }

    /**
     * @dev Takes a calldata pointer and memory pointer and copies a referenced
     *      OrderParameters struct and associated offer and consideration data
     *      to memory.
     *
     * @param cdPtr A calldata pointer for the OrderParameters struct.
     * @param mPtr A memory pointer to the OrderParameters struct head.
     */
    function _decodeOrderParametersTo(CalldataPointer cdPtr, MemoryPointer mPtr) internal pure {
        // Copy the full OrderParameters head from calldata to memory.
        cdPtr.copy(mPtr, OrderParameters_head_size);

        // Resolve the offer calldata offset, use that to decode and copy offer
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.offset(OrderParameters_offer_head_offset).write(
            _decodeOffer(cdPtr.pptr(OrderParameters_offer_head_offset))
        );

        // Resolve consideration calldata offset, use that to copy consideration
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.offset(OrderParameters_consideration_head_offset).write(
            _decodeConsideration(cdPtr.pptr(OrderParameters_consideration_head_offset))
        );
    }

    /**
     * @dev Takes a calldata pointer to an OrderParameters struct and copies the
     *      decoded struct to memory.
     *
     * @param cdPtr A calldata pointer for the OrderParameters struct.
     *
     * @return mPtr A memory pointer to the OrderParameters struct head.
     */
    function _decodeOrderParameters(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate required memory for the OrderParameters head (offer and
        // consideration are allocated independently).
        mPtr = malloc(OrderParameters_head_size);

        // Decode and copy the order parameters to the newly allocated memory.
        _decodeOrderParametersTo(cdPtr, mPtr);
    }

    /**
     * @dev Takes a calldata pointer to an Order struct and copies the decoded
     *      struct to memory.
     *
     * @param cdPtr A calldata pointer for the Order struct.
     *
     * @return mPtr A memory pointer to the Order struct head.
     */
    function _decodeOrder(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate required memory for the Order head (OrderParameters and
        // signature are allocated independently).
        mPtr = malloc(Order_head_size);

        // Resolve OrderParameters calldata offset, use it to decode and copy
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.write(_decodeOrderParameters(cdPtr.pptr()));

        // Resolve signature calldata offset, use that to decode and copy from
        // calldata, and write resultant memory offset to head in memory.
        mPtr.offset(Order_signature_offset).write(_decodeBytes(cdPtr.pptr(Order_signature_offset)));
    }

    /**
     * @dev Takes a calldata pointer to an AdvancedOrder struct and copies the
     *      decoded struct to memory.
     *
     * @param cdPtr A calldata pointer for the AdvancedOrder struct.
     *
     * @return mPtr A memory pointer to the AdvancedOrder struct head.
     */
    function _decodeAdvancedOrder(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate memory for AdvancedOrder head and OrderParameters head.
        mPtr = malloc(AdvancedOrderPlusOrderParameters_head_size);

        // Use numerator + denominator calldata offset to decode and copy
        // from calldata and write resultant memory offset to head in memory.
        cdPtr.offset(AdvancedOrder_numerator_offset).copy(
            mPtr.offset(AdvancedOrder_numerator_offset), AdvancedOrder_fixed_segment_0
        );

        // Get pointer to memory immediately after advanced order.
        MemoryPointer mPtrParameters = mPtr.offset(AdvancedOrder_head_size);

        // Write pptr for advanced order parameters to memory.
        mPtr.write(mPtrParameters);

        // Resolve OrderParameters calldata pointer & write to allocated region.
        _decodeOrderParametersTo(cdPtr.pptr(), mPtrParameters);

        // Resolve signature calldata offset, use that to decode and copy from
        // calldata, and write resultant memory offset to head in memory.
        mPtr.offset(AdvancedOrder_signature_offset).write(_decodeBytes(cdPtr.pptr(AdvancedOrder_signature_offset)));

        // Resolve extraData calldata offset, use that to decode and copy from
        // calldata, and write resultant memory offset to head in memory.
        mPtr.offset(AdvancedOrder_extraData_offset).write(_decodeBytes(cdPtr.pptr(AdvancedOrder_extraData_offset)));
    }

    /**
     * @dev Allocates a single word of empty bytes in memory and returns the
     *      pointer to that memory region.
     *
     * @return mPtr The memory pointer to the new empty word in memory.
     */
    function _getEmptyBytesOrArray() internal pure returns (MemoryPointer mPtr) {
        mPtr = malloc(OneWord);
        mPtr.write(0);
    }

    /**
     * @dev Takes a calldata pointer to an Order struct and copies the decoded
     *      struct to memory as an AdvancedOrder.
     *
     * @param cdPtr A calldata pointer for the Order struct.
     *
     * @return mPtr A memory pointer to the AdvancedOrder struct head.
     */
    function _decodeOrderAsAdvancedOrder(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate memory for AdvancedOrder head and OrderParameters head.
        mPtr = malloc(AdvancedOrderPlusOrderParameters_head_size);

        // Get pointer to memory immediately after advanced order.
        MemoryPointer mPtrParameters = mPtr.offset(AdvancedOrder_head_size);

        // Write pptr for advanced order parameters.
        mPtr.write(mPtrParameters);

        // Resolve OrderParameters calldata pointer & write to allocated region.
        _decodeOrderParametersTo(cdPtr.pptr(), mPtrParameters);

        // Write default Order numerator and denominator values (i.e. 1/1).
        mPtr.offset(AdvancedOrder_numerator_offset).write(1);
        mPtr.offset(AdvancedOrder_denominator_offset).write(1);

        // Resolve signature calldata offset, use that to decode and copy from
        // calldata, and write resultant memory offset to head in memory.
        mPtr.offset(AdvancedOrder_signature_offset).write(_decodeBytes(cdPtr.pptr(Order_signature_offset)));

        // Resolve extraData calldata offset, use that to decode and copy from
        // calldata, and write resultant memory offset to head in memory.
        mPtr.offset(AdvancedOrder_extraData_offset).write(_getEmptyBytesOrArray());
    }

    /**
     * @dev Takes a calldata pointer to an array of Order structs and copies the
     *      decoded array to memory as an array of AdvancedOrder structs.
     *
     * @param cdPtrLength A calldata pointer to the start of the orders array in
     *                    calldata which contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the array of advanced
     *                    orders in memory which contains length of the array.
     */
    function _decodeOrdersAsAdvancedOrders(CalldataPointer cdPtrLength)
        internal
        pure
        returns (MemoryPointer mPtrLength)
    {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve Order calldata offset, use it to decode and copy from
                // calldata, and write resultant AdvancedOrder offset to memory.
                mPtrHead.offset(offset).write(_decodeOrderAsAdvancedOrder(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes a calldata pointer to a criteria proof, or an array bytes32
     *      types, and copies the decoded proof to memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the criteria proof
     *                    in calldata which contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the criteria proof
     *                    in memory which contains length of the array.
     */
    function _decodeCriteriaProof(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive array size based on one word per array element and length.
            uint256 arrSize = (arrLength + 1) << OneWordShift;

            // Allocate memory equal to the array size.
            mPtrLength = malloc(arrSize);

            // Copy the array from calldata into memory.
            cdPtrLength.copy(mPtrLength, arrSize);
        }
    }

    /**
     * @dev Takes a calldata pointer to a CriteriaResolver struct and copies the
     *      decoded struct to memory.
     *
     * @param cdPtr A calldata pointer for the CriteriaResolver struct.
     *
     * @return mPtr A memory pointer to the CriteriaResolver struct head.
     */
    function _decodeCriteriaResolver(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate required memory for the CriteriaResolver head (the criteria
        // proof bytes32 array is allocated independently).
        mPtr = malloc(CriteriaResolver_head_size);

        // Decode and copy order index, side, index, and identifier from
        // calldata and write resultant memory offset to head in memory.
        cdPtr.copy(mPtr, CriteriaResolver_fixed_segment_0);

        // Resolve criteria proof calldata offset, use it to decode and copy
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.offset(CriteriaResolver_criteriaProof_offset).write(
            _decodeCriteriaProof(cdPtr.pptr(CriteriaResolver_criteriaProof_offset))
        );
    }

    /**
     * @dev Takes an array of criteria resolvers from calldata and copies it
     *      into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the criteria
     *                    resolver array in calldata which contains the length
     *                    of the array.
     *
     * @return mPtrLength A memory pointer to the start of the criteria resolver
     *                    array in memory which contains the length of the
     *                    array.
     */
    function _decodeCriteriaResolvers(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve CriteriaResolver calldata offset, use it to decode
                // and copy from calldata, and write resultant memory offset.
                mPtrHead.offset(offset).write(_decodeCriteriaResolver(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes an array of orders from calldata and copies it into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the orders array in
     *                    calldata which contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the orders array
     *                    in memory which contains the length of the array.
     */
    function _decodeOrders(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve Order calldata offset, use it to decode and copy
                // from calldata, and write resultant memory offset.
                mPtrHead.offset(offset).write(_decodeOrder(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes an array of fulfillment components from calldata and copies it
     *      into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the fulfillment
     *                    components array in calldata which contains the length
     *                    of the array.
     *
     * @return mPtrLength A memory pointer to the start of the fulfillment
     *                    components array in memory which contains the length
     *                    of the array.
     */
    function _decodeFulfillmentComponents(CalldataPointer cdPtrLength)
        internal
        pure
        returns (MemoryPointer mPtrLength)
    {
        assembly {
            let arrLength := and(calldataload(cdPtrLength), OffsetOrLengthMask)

            // Get the current free memory pointer.
            mPtrLength := mload(FreeMemoryPointerSlot)

            mstore(mPtrLength, arrLength)
            let mPtrHead := add(mPtrLength, OneWord)
            let mPtrTail := add(mPtrHead, shl(OneWordShift, arrLength))
            let mPtrTailNext := mPtrTail
            calldatacopy(mPtrTail, add(cdPtrLength, OneWord), shl(FulfillmentComponent_mem_tail_size_shift, arrLength))
            let mPtrHeadNext := mPtrHead
            for {} lt(mPtrHeadNext, mPtrTail) {} {
                mstore(mPtrHeadNext, mPtrTailNext)
                mPtrHeadNext := add(mPtrHeadNext, OneWord)
                mPtrTailNext := add(mPtrTailNext, FulfillmentComponent_mem_tail_size)
            }

            // Update the free memory pointer.
            mstore(FreeMemoryPointerSlot, mPtrTailNext)
        }
    }

    /**
     * @dev Takes a nested array of fulfillment components from calldata and
     *      copies it into memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the nested
     *                    fulfillment components array in calldata which
     *                    contains the length of the array.
     *
     * @return mPtrLength A memory pointer to the start of the nested
     *                    fulfillment components array in memory which
     *                    contains the length of the array.
     */
    function _decodeNestedFulfillmentComponents(CalldataPointer cdPtrLength)
        internal
        pure
        returns (MemoryPointer mPtrLength)
    {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve FulfillmentComponents array calldata offset, use it
                // to decode and copy from calldata, and write memory offset.
                mPtrHead.offset(offset).write(_decodeFulfillmentComponents(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes an array of advanced orders from calldata and copies it into
     *      memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the advanced orders
     *                    array in calldata which contains the length of the
     *                    array.
     *
     * @return mPtrLength A memory pointer to the start of the advanced orders
     *                    array in memory which contains the length of the
     *                    array.
     */
    function _decodeAdvancedOrders(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve AdvancedOrder calldata offset, use it to decode and
                // copy from calldata, and write resultant memory offset.
                mPtrHead.offset(offset).write(_decodeAdvancedOrder(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes a calldata pointer to a Fulfillment struct and copies the
     *      decoded struct to memory.
     *
     * @param cdPtr A calldata pointer for the Fulfillment struct.
     *
     * @return mPtr A memory pointer to the Fulfillment struct head.
     */
    function _decodeFulfillment(CalldataPointer cdPtr) internal pure returns (MemoryPointer mPtr) {
        // Allocate required memory for the Fulfillment head (the fulfillment
        // components arrays are allocated independently).
        mPtr = malloc(Fulfillment_head_size);

        // Resolve offerComponents calldata offset, use it to decode and copy
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.write(_decodeFulfillmentComponents(cdPtr.pptr()));

        // Resolve considerationComponents calldata offset, use it to decode and
        // copy from calldata, and write resultant memory offset to memory head.
        mPtr.offset(Fulfillment_considerationComponents_offset).write(
            _decodeFulfillmentComponents(cdPtr.pptr(Fulfillment_considerationComponents_offset))
        );
    }

    /**
     * @dev Takes an array of fulfillments from calldata and copies it into
     *      memory.
     *
     * @param cdPtrLength A calldata pointer to the start of the fulfillments
     *                    array in calldata which contains the length of the
     *                    array.
     *
     * @return mPtrLength A memory pointer to the start of the fulfillments
     *                    array in memory which contains the length of the
     *                    array.
     */
    function _decodeFulfillments(CalldataPointer cdPtrLength) internal pure returns (MemoryPointer mPtrLength) {
        // Retrieve length of array, masking to prevent potential overflow.
        uint256 arrLength = cdPtrLength.readMaskedUint256();

        unchecked {
            // Derive offset to the tail based on one word per array element.
            uint256 tailOffset = arrLength << OneWordShift;

            // Add one additional word for the length and allocate memory.
            mPtrLength = malloc(tailOffset + OneWord);

            // Write the length of the array to memory.
            mPtrLength.write(arrLength);

            // Advance to first memory & calldata pointers (e.g. after length).
            MemoryPointer mPtrHead = mPtrLength.next();
            CalldataPointer cdPtrHead = cdPtrLength.next();

            // Iterate over each pointer, word by word, until tail is reached.
            for (uint256 offset = 0; offset < tailOffset; offset += OneWord) {
                // Resolve Fulfillment calldata offset, use it to decode and
                // copy from calldata, and write resultant memory offset.
                mPtrHead.offset(offset).write(_decodeFulfillment(cdPtrHead.pptr(offset)));
            }
        }
    }

    /**
     * @dev Takes a calldata pointer to an OrderComponents struct and copies the
     *      decoded struct to memory as an OrderParameters struct (with the
     *      totalOriginalConsiderationItems value set equal to the length of the
     *      supplied consideration array).
     *
     * @param cdPtr A calldata pointer for the OrderComponents struct.
     *
     * @return mPtr A memory pointer to the OrderParameters struct head.
     */
    function _decodeOrderComponentsAsOrderParameters(CalldataPointer cdPtr)
        internal
        pure
        returns (MemoryPointer mPtr)
    {
        // Allocate memory for the OrderParameters head.
        mPtr = malloc(OrderParameters_head_size);

        // Copy the full OrderComponents head from calldata to memory.
        cdPtr.copy(mPtr, OrderComponents_OrderParameters_common_head_size);

        // Resolve the offer calldata offset, use that to decode and copy offer
        // from calldata, and write resultant memory offset to head in memory.
        mPtr.offset(OrderParameters_offer_head_offset).write(
            _decodeOffer(cdPtr.pptr(OrderParameters_offer_head_offset))
        );

        // Resolve consideration calldata offset, use that to copy consideration
        // from calldata, and write resultant memory offset to head in memory.
        MemoryPointer consideration = _decodeConsideration(cdPtr.pptr(OrderParameters_consideration_head_offset));
        mPtr.offset(OrderParameters_consideration_head_offset).write(consideration);

        // Write masked consideration length to totalOriginalConsiderationItems.
        mPtr.offset(OrderParameters_totalOriginalConsiderationItems_offset).write(consideration.readUint256());
    }

    /**
     * @dev Decodes the returndata from a call to generateOrder, or returns
     *      empty arrays and a boolean signifying that the returndata does not
     *      adhere to a valid encoding scheme if it cannot be decoded.
     *
     * @return invalidEncoding A boolean signifying whether the returndata has
     *                         an invalid encoding.
     * @return offer           The decoded offer array.
     * @return consideration   The decoded consideration array.
     */
    function _decodeGenerateOrderReturndata()
        internal
        pure
        returns (uint256 invalidEncoding, MemoryPointer offer, MemoryPointer consideration)
    {
        assembly {
            // Check that returndatasize is at least four words: offerOffset,
            // considerationOffset, offerLength, & considerationLength
            invalidEncoding := lt(returndatasize(), FourWords)

            let offsetOffer
            let offsetConsideration
            let offerLength
            let considerationLength

            // Proceed if enough returndata is present to continue evaluation.
            if iszero(invalidEncoding) {
                // Copy first two words of returndata (the offsets to offer and
                // consideration array lengths) to scratch space.
                returndatacopy(0, 0, TwoWords)
                offsetOffer := mload(0)
                offsetConsideration := mload(OneWord)

                // If valid length, check that offsets are within returndata.
                let invalidOfferOffset := gt(offsetOffer, returndatasize())
                let invalidConsiderationOffset := gt(offsetConsideration, returndatasize())

                // Only proceed if length (and thus encoding) is valid so far.
                invalidEncoding := or(invalidOfferOffset, invalidConsiderationOffset)
                if iszero(invalidEncoding) {
                    // Copy length of offer array to scratch space.
                    returndatacopy(0, offsetOffer, OneWord)
                    offerLength := mload(0)

                    // Copy length of consideration array to scratch space.
                    returndatacopy(OneWord, offsetConsideration, OneWord)
                    considerationLength := mload(OneWord)

                    {
                        // Calculate total size of offer & consideration arrays.
                        let totalOfferSize := shl(SpentItem_size_shift, offerLength)
                        let totalConsiderationSize := mul(ReceivedItem_size, considerationLength)

                        // Add 4 words to total size to cover the offset and
                        // length fields of the two arrays.
                        let totalSize := add(FourWords, add(totalOfferSize, totalConsiderationSize))
                        // Don't continue if returndatasize exceeds 65535 bytes
                        // or is greater than the calculated size.
                        invalidEncoding :=
                            or(
                                gt(or(offerLength, considerationLength), generateOrder_maximum_returndatasize),
                                gt(totalSize, returndatasize())
                            )

                        // Set first word of scratch space to 0 so length of
                        // offer/consideration are set to 0 on invalid encoding.
                        mstore(0, 0)
                    }
                }
            }

            if iszero(invalidEncoding) {
                offer := copySpentItemsAsOfferItems(add(offsetOffer, OneWord), offerLength)

                consideration :=
                    copyReceivedItemsAsConsiderationItems(add(offsetConsideration, OneWord), considerationLength)
            }

            function copySpentItemsAsOfferItems(rdPtrHead, length) -> mPtrLength {
                // Retrieve the current free memory pointer.
                mPtrLength := mload(FreeMemoryPointerSlot)

                // Allocate memory for the array.
                mstore(FreeMemoryPointerSlot, add(mPtrLength, add(OneWord, mul(length, OfferItem_size_with_length))))

                // Write the length of the array to the start of free memory.
                mstore(mPtrLength, length)

                // Use offset from length to minimize stack depth.
                let headOffsetFromLength := OneWord
                let headSizeWithLength := shl(OneWordShift, add(1, length))
                let mPtrTailNext := add(mPtrLength, headSizeWithLength)

                // Iterate over each element.
                for {} lt(headOffsetFromLength, headSizeWithLength) {} {
                    // Write the memory pointer to the accompanying head offset.
                    mstore(add(mPtrLength, headOffsetFromLength), mPtrTailNext)

                    // Copy itemType, token, identifier and amount.
                    returndatacopy(mPtrTailNext, rdPtrHead, SpentItem_size)

                    // Copy amount to endAmount.
                    mstore(add(mPtrTailNext, Common_endAmount_offset), mload(add(mPtrTailNext, Common_amount_offset)))

                    // Update read pointer, next tail pointer, and head offset.
                    rdPtrHead := add(rdPtrHead, SpentItem_size)
                    mPtrTailNext := add(mPtrTailNext, OfferItem_size)
                    headOffsetFromLength := add(headOffsetFromLength, OneWord)
                }
            }

            function copyReceivedItemsAsConsiderationItems(rdPtrHead, length) -> mPtrLength {
                // Retrieve the current free memory pointer.
                mPtrLength := mload(FreeMemoryPointerSlot)

                // Allocate memory for the array.
                mstore(
                    FreeMemoryPointerSlot,
                    add(mPtrLength, add(OneWord, mul(length, ConsiderationItem_size_with_length)))
                )

                // Write the length of the array to the start of free memory.
                mstore(mPtrLength, length)

                // Use offset from length to minimize stack depth.
                let headOffsetFromLength := OneWord
                let headSizeWithLength := shl(OneWordShift, add(1, length))
                let mPtrTailNext := add(mPtrLength, headSizeWithLength)

                // Iterate over each element.
                for {} lt(headOffsetFromLength, headSizeWithLength) {} {
                    // Write the memory pointer to the accompanying head offset.
                    mstore(add(mPtrLength, headOffsetFromLength), mPtrTailNext)

                    // Copy itemType, token, identifier and amount.
                    returndatacopy(mPtrTailNext, rdPtrHead, ReceivedItem_size_excluding_recipient)

                    // Copy amount and recipient.
                    returndatacopy(
                        add(mPtrTailNext, Common_endAmount_offset), add(rdPtrHead, Common_amount_offset), TwoWords
                    )

                    // Update read pointer, next tail pointer, and head offset.
                    rdPtrHead := add(rdPtrHead, ReceivedItem_size)
                    mPtrTailNext := add(mPtrTailNext, ConsiderationItem_size)
                    headOffsetFromLength := add(headOffsetFromLength, OneWord)
                }
            }
        }
    }

    /**
     * @dev Converts a function returning _decodeGenerateOrderReturndata types
     *      into a function returning offer and consideration types.
     *
     * @param inFn The input function, taking no arguments and returning an
     *             error buffer, spent item array, and received item array.
     *
     * @return outFn The output function, taking no arguments and returning an
     *               error buffer, offer array, and consideration array.
     */
    function _convertGetGeneratedOrderResult(
        function()
            internal
            pure
            returns (uint256, MemoryPointer, MemoryPointer) inFn
    )
        internal
        pure
        returns (
            function()
                                                internal
                                                pure
                                                returns (
                                                    uint256,
                                                    OfferItem[] memory,
                                                    ConsiderationItem[] memory
                                                ) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking ReceivedItem, address, bytes32, and bytes
     *      types (e.g. the _transfer function) into a function taking
     *      OfferItem, address, bytes32, and bytes types.
     *
     * @param inFn The input function, taking ReceivedItem, address, bytes32,
     *             and bytes types (e.g. the _transfer function).
     *
     * @return outFn The output function, taking OfferItem, address, bytes32,
     *               and bytes types.
     */
    function _toOfferItemInput(
        function(ReceivedItem memory, address, bytes32, bytes memory)
            internal inFn
    )
        internal
        pure
        returns (
            function(OfferItem memory, address, bytes32, bytes memory)
                                                internal outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking ReceivedItem, address, bytes32, and bytes
     *      types (e.g. the _transfer function) into a function taking
     *      ConsiderationItem, address, bytes32, and bytes types.
     *
     * @param inFn The input function, taking ReceivedItem, address, bytes32,
     *             and bytes types (e.g. the _transfer function).
     *
     * @return outFn The output function, taking ConsiderationItem, address,
     *               bytes32, and bytes types.
     */
    function _toConsiderationItemInput(
        function(ReceivedItem memory, address, bytes32, bytes memory)
            internal inFn
    )
        internal
        pure
        returns (
            function(ConsiderationItem memory, address, bytes32, bytes memory)
                                                internal outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      an OrderParameters type.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning an OrderParameters type.
     */
    function _toOrderParametersReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (OrderParameters memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      an AdvancedOrder type.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning an AdvancedOrder type.
     */
    function _toAdvancedOrderReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (AdvancedOrder memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      a dynamic array of CriteriaResolver types.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning a dynamic array of CriteriaResolver types.
     */
    function _toCriteriaResolversReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (CriteriaResolver[] memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      a dynamic array of Order types.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning a dynamic array of Order types.
     */
    function _toOrdersReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (Order[] memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      a nested dynamic array of dynamic arrays of FulfillmentComponent
     *      types.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning a nested dynamic array of dynamic arrays of
     *               FulfillmentComponent types.
     */
    function _toNestedFulfillmentComponentsReturnType(
        function(CalldataPointer) internal pure returns (MemoryPointer) inFn
    )
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (FulfillmentComponent[][] memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      a dynamic array of AdvancedOrder types.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning a dynamic array of AdvancedOrder types.
     */
    function _toAdvancedOrdersReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (AdvancedOrder[] memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Converts a function taking a calldata pointer and returning a memory
     *      pointer into a function taking that calldata pointer and returning
     *      a dynamic array of Fulfillment types.
     *
     * @param inFn The input function, taking an arbitrary calldata pointer and
     *             returning an arbitrary memory pointer.
     *
     * @return outFn The output function, taking an arbitrary calldata pointer
     *               and returning a dynamic array of Fulfillment types.
     */
    function _toFulfillmentsReturnType(function(CalldataPointer) internal pure returns (MemoryPointer) inFn)
        internal
        pure
        returns (
            function(CalldataPointer)
                                                internal
                                                pure
                                                returns (Fulfillment[] memory) outFn
        )
    {
        assembly {
            outFn := inFn
        }
    }

    /**
     * @dev Caches the endAmount in an offer item and replaces it with
     * a given recipient so that its memory may be reused as a temporary
     * ReceivedItem.
     *
     * @param offerItem The offer item.
     * @param recipient The recipient.
     *
     * @return originalEndAmount The original end amount.
     */
    function _replaceEndAmountWithRecipient(OfferItem memory offerItem, address recipient)
        internal
        pure
        returns (uint256 originalEndAmount)
    {
        assembly {
            // Derive the pointer to the end amount on the offer item.
            let endAmountPtr := add(offerItem, ReceivedItem_recipient_offset)

            // Retrieve the value of the end amount on the offer item.
            originalEndAmount := mload(endAmountPtr)

            // Write recipient to received item at the offer end amount pointer.
            mstore(endAmountPtr, recipient)
        }
    }
}

contract ConsiderationEncoder {
    /**
     * @dev Takes a bytes array and casts it to a memory pointer.
     *
     * @param obj A bytes array in memory.
     *
     * @return ptr A memory pointer to the start of the bytes array in memory.
     */
    function toMemoryPointer(bytes memory obj) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Takes an array of bytes32 types and casts it to a memory pointer.
     *
     * @param obj An array of bytes32 types in memory.
     *
     * @return ptr A memory pointer to the start of the array of bytes32 types
     *             in memory.
     */
    function toMemoryPointer(bytes32[] memory obj) internal pure returns (MemoryPointer ptr) {
        assembly {
            ptr := obj
        }
    }

    /**
     * @dev Takes a bytes array in memory and copies it to a new location in
     *      memory.
     *
     * @param src A memory pointer referencing the bytes array to be copied (and
     *            pointing to the length of the bytes array).
     * @param src A memory pointer referencing the location in memory to copy
     *            the bytes array to (and pointing to the length of the copied
     *            bytes array).
     *
     * @return size The size of the bytes array.
     */
    function _encodeBytes(MemoryPointer src, MemoryPointer dst) internal view returns (uint256 size) {
        unchecked {
            // Mask the length of the bytes array to protect against overflow
            // and round up to the nearest word.
            // Note: `size` also includes the 1 word that stores the length.
            size = (src.readUint256() + SixtyThreeBytes) & OnlyFullWordMask;

            // Copy the bytes array to the new memory location.
            src.copy(dst, size);
        }
    }

    /**
     * @dev Takes an OrderParameters struct and a context bytes array in memory
     *      and encodes it as `generateOrder` calldata.
     *
     * @param orderParameters The OrderParameters struct used to construct the
     *                        encoded `generateOrder` calldata.
     * @param context         The context bytes array used to construct the
     *                        encoded `generateOrder` calldata.
     *
     * @return dst  A memory pointer referencing the encoded `generateOrder`
     *              calldata.
     * @return size The size of the bytes array.
     */
    function _encodeGenerateOrder(OrderParameters memory orderParameters, bytes memory context)
        internal
        view
        returns (MemoryPointer dst, uint256 size)
    {
        // Get the memory pointer for the OrderParameters struct.
        MemoryPointer src = orderParameters.toMemoryPointer();

        // Get free memory pointer to write calldata to.
        dst = getFreeMemoryPointer();

        // Write generateOrder selector and get pointer to start of calldata.
        dst.write(generateOrder_selector);
        dst = dst.offset(generateOrder_selector_offset);

        // Get pointer to the beginning of the encoded data.
        MemoryPointer dstHead = dst.offset(generateOrder_head_offset);

        // Write `fulfiller` to calldata.
        dstHead.write(msg.sender);

        // Initialize tail offset, used to populate the minimumReceived array.
        uint256 tailOffset = generateOrder_base_tail_offset;

        // Write offset to minimumReceived.
        dstHead.offset(generateOrder_minimumReceived_head_offset).write(tailOffset);

        // Get memory pointer to `orderParameters.offer.length`.
        MemoryPointer srcOfferPointer = src.offset(OrderParameters_offer_head_offset).readMemoryPointer();

        // Encode the offer array as a `SpentItem[]`.
        uint256 minimumReceivedSize = _encodeSpentItems(srcOfferPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate maximumSpent array.
            tailOffset += minimumReceivedSize;
        }

        // Write offset to maximumSpent.
        dstHead.offset(generateOrder_maximumSpent_head_offset).write(tailOffset);

        // Get memory pointer to `orderParameters.consideration.length`.
        MemoryPointer srcConsiderationPointer =
            src.offset(OrderParameters_consideration_head_offset).readMemoryPointer();

        // Encode the consideration array as a `SpentItem[]`.
        uint256 maximumSpentSize = _encodeSpentItems(srcConsiderationPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate context array.
            tailOffset += maximumSpentSize;
        }

        // Write offset to context.
        dstHead.offset(generateOrder_context_head_offset).write(tailOffset);

        // Get memory pointer to context.
        MemoryPointer srcContext = toMemoryPointer(context);

        // Encode context as a bytes array.
        uint256 contextSize = _encodeBytes(srcContext, dstHead.offset(tailOffset));

        unchecked {
            // Increment the tail offset, now used to determine final size.
            tailOffset += contextSize;

            // Derive the final size by including the selector.
            size = Selector_length + tailOffset;
        }
    }

    /**
     * @dev Takes an order hash (e.g. offerer shifted 96 bits to the left XOR'd
     *      with the contract nonce in the case of contract orders), an
     *      OrderParameters struct, context bytes array, and an array of order
     *      hashes for each order included as part of the current fulfillment
     *      and encodes it as `ratifyOrder` calldata.
     *
     * @param orderHash       The order hash (e.g. shl(0x60, offerer) ^ nonce).
     * @param orderParameters The OrderParameters struct used to construct the
     *                        encoded `ratifyOrder` calldata.
     * @param context         The context bytes array used to construct the
     *                        encoded `ratifyOrder` calldata.
     * @param orderHashes     An array of bytes32 values representing the order
     *                        hashes of all orders included as part of the
     *                        current fulfillment.
     * @param shiftedOfferer  The offerer for the order, shifted 96 bits to the
     *                        left.
     *
     * @return dst  A memory pointer referencing the encoded `ratifyOrder`
     *              calldata.
     * @return size The size of the bytes array.
     */
    function _encodeRatifyOrder(
        bytes32 orderHash, // e.g. shl(0x60, offerer) ^ contract nonce
        OrderParameters memory orderParameters,
        bytes memory context, // encoded based on the schemaID
        bytes32[] memory orderHashes,
        uint256 shiftedOfferer
    ) internal view returns (MemoryPointer dst, uint256 size) {
        // Get free memory pointer to write calldata to. This isn't allocated as
        // it is only used for a single function call.
        dst = getFreeMemoryPointer();

        // Write ratifyOrder selector and get pointer to start of calldata.
        dst.write(ratifyOrder_selector);
        dst = dst.offset(ratifyOrder_selector_offset);

        // Get pointer to the beginning of the encoded data.
        MemoryPointer dstHead = dst.offset(ratifyOrder_head_offset);

        // Write contractNonce to calldata via xor(orderHash, shiftedOfferer).
        dstHead.offset(ratifyOrder_contractNonce_offset).write(uint256(orderHash) ^ shiftedOfferer);

        // Initialize tail offset, used to populate the offer array.
        uint256 tailOffset = ratifyOrder_base_tail_offset;
        MemoryPointer src = orderParameters.toMemoryPointer();

        // Write offset to `offer`.
        dstHead.write(tailOffset);

        // Get memory pointer to `orderParameters.offer.length`.
        MemoryPointer srcOfferPointer = src.offset(OrderParameters_offer_head_offset).readMemoryPointer();

        // Encode the offer array as a `SpentItem[]`.
        uint256 offerSize = _encodeSpentItems(srcOfferPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate consideration array.
            tailOffset += offerSize;
        }

        // Write offset to consideration.
        dstHead.offset(ratifyOrder_consideration_head_offset).write(tailOffset);

        // Get pointer to `orderParameters.consideration.length`.
        MemoryPointer srcConsiderationPointer =
            src.offset(OrderParameters_consideration_head_offset).readMemoryPointer();

        // Encode the consideration array as a `ReceivedItem[]`.
        uint256 considerationSize =
            _encodeConsiderationAsReceivedItems(srcConsiderationPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate context array.
            tailOffset += considerationSize;
        }

        // Write offset to context.
        dstHead.offset(ratifyOrder_context_head_offset).write(tailOffset);

        // Encode context.
        uint256 contextSize = _encodeBytes(toMemoryPointer(context), dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate orderHashes array.
            tailOffset += contextSize;
        }

        // Write offset to orderHashes.
        dstHead.offset(ratifyOrder_orderHashes_head_offset).write(tailOffset);

        // Encode orderHashes.
        uint256 orderHashesSize = _encodeOrderHashes(toMemoryPointer(orderHashes), dstHead.offset(tailOffset));

        unchecked {
            // Increment the tail offset, now used to determine final size.
            tailOffset += orderHashesSize;

            // Derive the final size by including the selector.
            size = Selector_length + tailOffset;
        }
    }

    /**
     * @dev Takes an order hash, OrderParameters struct, extraData bytes array,
     *      and array of order hashes for each order included as part of the
     *      current fulfillment and encodes it as `validateOrder` calldata.
     *      Note that future, new versions of this contract may end up writing
     *      to a memory region that might have been potentially dirtied by the
     *      accumulator. Since the book-keeping for the accumulator does not
     *      update the free memory pointer, it will be necessary to ensure that
     *      all bytes in the memory in the range [dst, dst+size) are fully
     *      updated/written to in this function.
     *
     * @param orderHash       The order hash.
     * @param orderParameters The OrderParameters struct used to construct the
     *                        encoded `validateOrder` calldata.
     * @param extraData       The extraData bytes array used to construct the
     *                        encoded `validateOrder` calldata.
     * @param orderHashes     An array of bytes32 values representing the order
     *                        hashes of all orders included as part of the
     *                        current fulfillment.
     *
     * @return dst  A memory pointer referencing the encoded `validateOrder`
     *              calldata.
     * @return size The size of the bytes array.
     */
    function _encodeValidateOrder(
        bytes32 orderHash,
        OrderParameters memory orderParameters,
        bytes memory extraData,
        bytes32[] memory orderHashes
    ) internal view returns (MemoryPointer dst, uint256 size) {
        // Get free memory pointer to write calldata to. This isn't allocated as
        // it is only used for a single function call.
        dst = getFreeMemoryPointer();

        // Write validateOrder selector and get pointer to start of calldata.
        dst.write(validateOrder_selector);
        dst = dst.offset(validateOrder_selector_offset);

        // Get pointer to the beginning of the encoded data.
        MemoryPointer dstHead = dst.offset(validateOrder_head_offset);

        // Write offset to zoneParameters to start of calldata.
        dstHead.write(validateOrder_zoneParameters_offset);

        // Reuse `dstHead` as pointer to zoneParameters.
        dstHead = dstHead.offset(validateOrder_zoneParameters_offset);

        // Write orderHash and fulfiller to zoneParameters.
        dstHead.writeBytes32(orderHash);
        dstHead.offset(ZoneParameters_fulfiller_offset).write(msg.sender);

        // Get the memory pointer to the order parameters struct.
        MemoryPointer src = orderParameters.toMemoryPointer();

        // Copy offerer, startTime, endTime and zoneHash to zoneParameters.
        dstHead.offset(ZoneParameters_offerer_offset).write(src.readUint256());
        dstHead.offset(ZoneParameters_startTime_offset).write(
            src.offset(OrderParameters_startTime_offset).readUint256()
        );
        dstHead.offset(ZoneParameters_endTime_offset).write(src.offset(OrderParameters_endTime_offset).readUint256());
        dstHead.offset(ZoneParameters_zoneHash_offset).write(src.offset(OrderParameters_zoneHash_offset).readUint256());

        // Initialize tail offset, used to populate the offer array.
        uint256 tailOffset = ZoneParameters_base_tail_offset;

        // Write offset to `offer`.
        dstHead.offset(ZoneParameters_offer_head_offset).write(tailOffset);

        // Get pointer to `orderParameters.offer.length`.
        MemoryPointer srcOfferPointer = src.offset(OrderParameters_offer_head_offset).readMemoryPointer();

        // Encode the offer array as a `SpentItem[]`.
        uint256 offerSize = _encodeSpentItems(srcOfferPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate consideration array.
            tailOffset += offerSize;
        }

        // Write offset to consideration.
        dstHead.offset(ZoneParameters_consideration_head_offset).write(tailOffset);

        // Get pointer to `orderParameters.consideration.length`.
        MemoryPointer srcConsiderationPointer =
            src.offset(OrderParameters_consideration_head_offset).readMemoryPointer();

        // Encode the consideration array as a `ReceivedItem[]`.
        uint256 considerationSize =
            _encodeConsiderationAsReceivedItems(srcConsiderationPointer, dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate extraData array.
            tailOffset += considerationSize;
        }

        // Write offset to extraData.
        dstHead.offset(ZoneParameters_extraData_head_offset).write(tailOffset);
        // Copy extraData.
        uint256 extraDataSize = _encodeBytes(toMemoryPointer(extraData), dstHead.offset(tailOffset));

        unchecked {
            // Increment tail offset, now used to populate orderHashes array.
            tailOffset += extraDataSize;
        }

        // Write offset to orderHashes.
        dstHead.offset(ZoneParameters_orderHashes_head_offset).write(tailOffset);

        // Encode the order hashes array.
        uint256 orderHashesSize = _encodeOrderHashes(toMemoryPointer(orderHashes), dstHead.offset(tailOffset));

        unchecked {
            // Increment the tail offset, now used to determine final size.
            tailOffset += orderHashesSize;

            // Derive final size including selector and ZoneParameters pointer.
            size = ZoneParameters_selectorAndPointer_length + tailOffset;
        }
    }

    /**
     * @dev Takes an order hash and BasicOrderParameters struct (from calldata)
     *      and encodes it as `validateOrder` calldata.
     *
     * @param orderHash  The order hash.
     * @param parameters The BasicOrderParameters struct used to construct the
     *                   encoded `validateOrder` calldata.
     *
     * @return dst  A memory pointer referencing the encoded `validateOrder`
     *              calldata.
     * @return size The size of the bytes array.
     */
    function _encodeValidateBasicOrder(bytes32 orderHash, BasicOrderParameters calldata parameters)
        internal
        view
        returns (MemoryPointer dst, uint256 size)
    {
        // Get free memory pointer to write calldata to. This isn't allocated as
        // it is only used for a single function call.
        dst = getFreeMemoryPointer();

        // Write validateOrder selector and get pointer to start of calldata.
        dst.write(validateOrder_selector);
        dst = dst.offset(validateOrder_selector_offset);

        // Get pointer to the beginning of the encoded data.
        MemoryPointer dstHead = dst.offset(validateOrder_head_offset);

        // Write offset to zoneParameters to start of calldata.
        dstHead.write(validateOrder_zoneParameters_offset);

        // Reuse `dstHead` as pointer to zoneParameters.
        dstHead = dstHead.offset(validateOrder_zoneParameters_offset);

        // Write offerer, orderHash and fulfiller to zoneParameters.
        dstHead.writeBytes32(orderHash);
        dstHead.offset(ZoneParameters_fulfiller_offset).write(msg.sender);
        dstHead.offset(ZoneParameters_offerer_offset).write(parameters.offerer);

        // Copy startTime, endTime and zoneHash to zoneParameters.
        CalldataPointer.wrap(BasicOrder_startTime_cdPtr).copy(
            dstHead.offset(ZoneParameters_startTime_offset), BasicOrder_startTimeThroughZoneHash_size
        );

        // Initialize tail offset, used for the offer + consideration arrays.
        uint256 tailOffset = ZoneParameters_base_tail_offset;

        // Write offset to offer from event data into target calldata.
        dstHead.offset(ZoneParameters_offer_head_offset).write(tailOffset);

        unchecked {
            // Write consideration offset next (located 5 words after offer).
            dstHead.offset(ZoneParameters_consideration_head_offset).write(tailOffset + BasicOrder_common_params_size);

            // Retrieve the offset to the length of additional recipients.
            uint256 additionalRecipientsLength =
                CalldataPointer.wrap(BasicOrder_additionalRecipients_length_cdPtr).readUint256();

            // Derive offset to event data using base offset & total recipients.
            uint256 offerDataOffset = OrderFulfilled_offer_length_baseOffset + additionalRecipientsLength * OneWord;

            // Derive size of offer and consideration data.
            // 2 words (lengths) + 4 (offer data) + 5 (consideration 1) + 5 * ar
            uint256 offerAndConsiderationSize =
                OrderFulfilled_baseDataSize + (additionalRecipientsLength * ReceivedItem_size);

            // Copy offer and consideration data from event data to calldata.
            MemoryPointer.wrap(offerDataOffset).copy(dstHead.offset(tailOffset), offerAndConsiderationSize);

            // Increment tail offset, now used to populate extraData array.
            tailOffset += offerAndConsiderationSize;
        }

        // Write empty bytes for extraData.
        dstHead.offset(ZoneParameters_extraData_head_offset).write(tailOffset);
        dstHead.offset(tailOffset).write(0);

        unchecked {
            // Increment tail offset, now used to populate orderHashes array.
            tailOffset += OneWord;
        }

        // Write offset to orderHashes.
        dstHead.offset(ZoneParameters_orderHashes_head_offset).write(tailOffset);

        // Write length = 1 to the orderHashes array.
        dstHead.offset(tailOffset).write(1);

        unchecked {
            // Write the single order hash to the orderHashes array.
            dstHead.offset(tailOffset + OneWord).writeBytes32(orderHash);

            // Final size: selector, ZoneParameters pointer, orderHashes & tail.
            size = ZoneParameters_basicOrderFixedElements_length + tailOffset;
        }
    }

    /**
     * @dev Takes a memory pointer to an array of bytes32 values representing
     *      the order hashes included as part of the fulfillment and a memory
     *      pointer to a location to copy it to, and copies the source data to
     *      the destination in memory.
     *
     * @param srcLength A memory pointer referencing the order hashes array to
     *                  be copied (and pointing to the length of the array).
     * @param dstLength A memory pointer referencing the location in memory to
     *                  copy the orderHashes array to (and pointing to the
     *                  length of the copied array).
     *
     * @return size The size of the order hashes array (including the length).
     */
    function _encodeOrderHashes(MemoryPointer srcLength, MemoryPointer dstLength)
        internal
        view
        returns (uint256 size)
    {
        // Read length of the array from source and write to destination.
        uint256 length = srcLength.readUint256();
        dstLength.write(length);

        unchecked {
            // Determine head & tail size as one word per element in the array.
            uint256 headAndTailSize = length << OneWordShift;

            // Copy the tail starting from the next element of the source to the
            // next element of the destination.
            srcLength.next().copy(dstLength.next(), headAndTailSize);

            // Set size to the length of the tail plus one word for length.
            size = headAndTailSize + OneWord;
        }
    }

    /**
     * @dev Takes a memory pointer to an offer or consideration array and a
     *      memory pointer to a location to copy it to, and copies the source
     *      data to the destination in memory as a SpentItem array.
     *
     * @param srcLength A memory pointer referencing the offer or consideration
     *                  array to be copied as a SpentItem array (and pointing to
     *                  the length of the original array).
     * @param dstLength A memory pointer referencing the location in memory to
     *                  copy the offer array to (and pointing to the length of
     *                  the copied array).
     *
     * @return size The size of the SpentItem array (including the length).
     */
    function _encodeSpentItems(MemoryPointer srcLength, MemoryPointer dstLength) internal pure returns (uint256 size) {
        assembly {
            // Read length of the array from source and write to destination.
            let length := mload(srcLength)
            mstore(dstLength, length)

            // Get pointer to first item's head position in the array,
            // containing the item's pointer in memory. The head pointer will be
            // incremented until it reaches the tail position (start of the
            // array data).
            let mPtrHead := add(srcLength, OneWord)

            // Position in memory to write next item for calldata. Since
            // SpentItem has a fixed length, the array elements do not contain
            // head elements in calldata, they are concatenated together after
            // the array length.
            let cdPtrData := add(dstLength, OneWord)

            // Pointer to end of array head in memory.
            let mPtrHeadEnd := add(mPtrHead, shl(OneWordShift, length))

            for {} lt(mPtrHead, mPtrHeadEnd) {} {
                // Read pointer to data for array element from head position.
                let mPtrTail := mload(mPtrHead)

                // Copy itemType, token, identifier, amount to calldata.
                mstore(cdPtrData, mload(mPtrTail))
                mstore(add(cdPtrData, Common_token_offset), mload(add(mPtrTail, Common_token_offset)))
                mstore(add(cdPtrData, Common_identifier_offset), mload(add(mPtrTail, Common_identifier_offset)))
                mstore(add(cdPtrData, Common_amount_offset), mload(add(mPtrTail, Common_amount_offset)))

                mPtrHead := add(mPtrHead, OneWord)
                cdPtrData := add(cdPtrData, SpentItem_size)
            }

            size := add(OneWord, shl(SpentItem_size_shift, length))
        }
    }

    /**
     * @dev Takes a memory pointer to an consideration array and a memory
     *      pointer to a location to copy it to, and copies the source data to
     *      the destination in memory as a ReceivedItem array.
     *
     * @param srcLength A memory pointer referencing the consideration array to
     *                  be copied as a ReceivedItem array (and pointing to the
     *                  length of the original array).
     * @param dstLength A memory pointer referencing the location in memory to
     *                  copy the consideration array to as a ReceivedItem array
     *                  (and pointing to the length of the new array).
     *
     * @return size The size of the ReceivedItem array (including the length).
     */
    function _encodeConsiderationAsReceivedItems(MemoryPointer srcLength, MemoryPointer dstLength)
        internal
        view
        returns (uint256 size)
    {
        unchecked {
            // Read length of the array from source and write to destination.
            uint256 length = srcLength.readUint256();
            dstLength.write(length);

            // Get pointer to first item's head position in the array,
            // containing the item's pointer in memory. The head pointer will be
            // incremented until it reaches the tail position (start of the
            // array data).
            MemoryPointer srcHead = srcLength.next();
            MemoryPointer srcHeadEnd = srcHead.offset(length << OneWordShift);

            // Position in memory to write next item for calldata. Since
            // ReceivedItem has a fixed length, the array elements do not
            // contain offsets in calldata, they are concatenated together after
            // the array length.
            MemoryPointer dstHead = dstLength.next();
            while (srcHead.lt(srcHeadEnd)) {
                MemoryPointer srcTail = srcHead.pptr();
                srcTail.copy(dstHead, ReceivedItem_size);
                srcHead = srcHead.next();
                dstHead = dstHead.offset(ReceivedItem_size);
            }

            size = OneWord + (length * ReceivedItem_size);
        }
    }
}

/**
 * @title ConsiderationBase
 * @author 0age
 * @notice ConsiderationBase contains immutable constants and constructor logic.
 */
contract ConsiderationBase is ConsiderationDecoder, ConsiderationEncoder, ConsiderationEventsAndErrors {
    // Precompute hashes, original chainId, and domain separator on deployment.
    bytes32 internal immutable _NAME_HASH;
    bytes32 internal immutable _VERSION_HASH;
    bytes32 internal immutable _EIP_712_DOMAIN_TYPEHASH;
    bytes32 internal immutable _OFFER_ITEM_TYPEHASH;
    bytes32 internal immutable _CONSIDERATION_ITEM_TYPEHASH;
    bytes32 internal immutable _ORDER_TYPEHASH;
    uint256 internal immutable _CHAIN_ID;
    bytes32 internal immutable _DOMAIN_SEPARATOR;

    // Allow for interaction with the conduit controller.
    ConduitControllerInterface internal immutable _CONDUIT_CONTROLLER;

    // Cache the conduit creation code hash used by the conduit controller.
    bytes32 internal immutable _CONDUIT_CREATION_CODE_HASH;

    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) {
        // Derive name and version hashes alongside required EIP-712 typehashes.
        (
            _NAME_HASH,
            _VERSION_HASH,
            _EIP_712_DOMAIN_TYPEHASH,
            _OFFER_ITEM_TYPEHASH,
            _CONSIDERATION_ITEM_TYPEHASH,
            _ORDER_TYPEHASH
        ) = _deriveTypehashes();

        // Store the current chainId and derive the current domain separator.
        _CHAIN_ID = block.chainid;
        _DOMAIN_SEPARATOR = _deriveDomainSeparator();

        // Set the supplied conduit controller.
        _CONDUIT_CONTROLLER = ConduitControllerInterface(conduitController);

        // Retrieve the conduit creation code hash from the supplied controller.
        (_CONDUIT_CREATION_CODE_HASH,) = (_CONDUIT_CONTROLLER.getConduitCodeHashes());
    }

    /**
     * @dev Internal view function to derive the EIP-712 domain separator.
     *
     * @return domainSeparator The derived domain separator.
     */
    function _deriveDomainSeparator() internal view returns (bytes32 domainSeparator) {
        bytes32 typehash = _EIP_712_DOMAIN_TYPEHASH;
        bytes32 nameHash = _NAME_HASH;
        bytes32 versionHash = _VERSION_HASH;

        // Leverage scratch space and other memory to perform an efficient hash.
        assembly {
            // Retrieve the free memory pointer; it will be replaced afterwards.
            let freeMemoryPointer := mload(FreeMemoryPointerSlot)

            // Retrieve value at 0x80; it will also be replaced afterwards.
            let slot0x80 := mload(Slot0x80)

            // Place typehash, name hash, and version hash at start of memory.
            mstore(0, typehash)
            mstore(EIP712_domainData_nameHash_offset, nameHash)
            mstore(EIP712_domainData_versionHash_offset, versionHash)

            // Place chainId in the next memory location.
            mstore(EIP712_domainData_chainId_offset, chainid())

            // Place the address of this contract in the next memory location.
            mstore(EIP712_domainData_verifyingContract_offset, address())

            // Hash relevant region of memory to derive the domain separator.
            domainSeparator := keccak256(0, EIP712_domainData_size)

            // Restore the free memory pointer.
            mstore(FreeMemoryPointerSlot, freeMemoryPointer)

            // Restore the zero slot to zero.
            mstore(ZeroSlot, 0)

            // Restore the value at 0x80.
            mstore(Slot0x80, slot0x80)
        }
    }

    /**
     * @dev Internal pure function to retrieve the default name of this
     *      contract and return.
     *
     * @return The name of this contract.
     */
    function _name() internal pure virtual returns (string memory) {
        // Return the name of the contract.
        assembly {
            // First element is the offset for the returned string. Offset the
            // value in memory by one word so that the free memory pointer will
            // be overwritten by the next write.
            mstore(OneWord, OneWord)

            // Name is right padded, so it touches the length which is left
            // padded. This enables writing both values at once. The free memory
            // pointer will be overwritten in the process.
            mstore(NameLengthPtr, NameWithLength)

            // Standard ABI encoding pads returned data to the nearest word. Use
            // the already empty zero slot memory region for this purpose and
            // return the final name string, offset by the original single word.
            return(OneWord, ThreeWords)
        }
    }

    /**
     * @dev Internal pure function to retrieve the default name of this contract
     *      as a string that can be used internally.
     *
     * @return The name of this contract.
     */
    function _nameString() internal pure virtual returns (string memory) {
        // Return the name of the contract.
        return "Consideration";
    }

    /**
     * @dev Internal pure function to derive required EIP-712 typehashes and
     *      other hashes during contract creation.
     *
     * @return nameHash                  The hash of the name of the contract.
     * @return versionHash               The hash of the version string of the
     *                                   contract.
     * @return eip712DomainTypehash      The primary EIP-712 domain typehash.
     * @return offerItemTypehash         The EIP-712 typehash for OfferItem
     *                                   types.
     * @return considerationItemTypehash The EIP-712 typehash for
     *                                   ConsiderationItem types.
     * @return orderTypehash             The EIP-712 typehash for Order types.
     */
    function _deriveTypehashes()
        internal
        pure
        returns (
            bytes32 nameHash,
            bytes32 versionHash,
            bytes32 eip712DomainTypehash,
            bytes32 offerItemTypehash,
            bytes32 considerationItemTypehash,
            bytes32 orderTypehash
        )
    {
        // Derive hash of the name of the contract.
        nameHash = keccak256(bytes(_nameString()));

        // Derive hash of the version string of the contract.
        versionHash = keccak256(bytes("1.5"));

        // Construct the OfferItem type string.
        bytes memory offerItemTypeString = bytes(
            "OfferItem(" "uint8 itemType," "address token," "uint256 identifierOrCriteria," "uint256 startAmount,"
            "uint256 endAmount" ")"
        );

        // Construct the ConsiderationItem type string.
        bytes memory considerationItemTypeString = bytes(
            "ConsiderationItem(" "uint8 itemType," "address token," "uint256 identifierOrCriteria,"
            "uint256 startAmount," "uint256 endAmount," "address recipient" ")"
        );

        // Construct the OrderComponents type string, not including the above.
        bytes memory orderComponentsPartialTypeString = bytes(
            "OrderComponents(" "address offerer," "address zone," "OfferItem[] offer,"
            "ConsiderationItem[] consideration," "uint8 orderType," "uint256 startTime," "uint256 endTime,"
            "bytes32 zoneHash," "uint256 salt," "bytes32 conduitKey," "uint256 counter" ")"
        );

        // Construct the primary EIP-712 domain type string.
        eip712DomainTypehash = keccak256(
            bytes("EIP712Domain(" "string name," "string version," "uint256 chainId," "address verifyingContract" ")")
        );

        // Derive the OfferItem type hash using the corresponding type string.
        offerItemTypehash = keccak256(offerItemTypeString);

        // Derive ConsiderationItem type hash using corresponding type string.
        considerationItemTypehash = keccak256(considerationItemTypeString);

        bytes memory orderTypeString =
            bytes.concat(orderComponentsPartialTypeString, considerationItemTypeString, offerItemTypeString);

        // Derive OrderItem type hash via combination of relevant type strings.
        orderTypehash = keccak256(orderTypeString);
    }

    /**
     * @dev Internal pure function to look up one of twenty-four potential bulk
     *      order typehash constants based on the height of the bulk order tree.
     *      Note that values between one and twenty-four are supported, which is
     *      enforced by _isValidBulkOrderSize.
     *
     * @param _treeHeight The height of the bulk order tree. The value must be
     *                    between one and twenty-four.
     *
     * @return _typeHash The EIP-712 typehash for the bulk order type with the
     *                   given height.
     */
    function _lookupBulkOrderTypehash(uint256 _treeHeight) internal pure returns (bytes32 _typeHash) {
        // Utilize assembly to efficiently retrieve correct bulk order typehash.
        assembly {
            // Use a Yul function to enable use of the `leave` keyword
            // to stop searching once the appropriate type hash is found.
            function lookupTypeHash(treeHeight) -> typeHash {
                // Handle tree heights one through eight.
                if lt(treeHeight, 9) {
                    // Handle tree heights one through four.
                    if lt(treeHeight, 5) {
                        // Handle tree heights one and two.
                        if lt(treeHeight, 3) {
                            // Utilize branchless logic to determine typehash.
                            typeHash :=
                                ternary(eq(treeHeight, 1), BulkOrder_Typehash_Height_One, BulkOrder_Typehash_Height_Two)

                            // Exit the function once typehash has been located.
                            leave
                        }

                        // Handle height three and four via branchless logic.
                        typeHash :=
                            ternary(eq(treeHeight, 3), BulkOrder_Typehash_Height_Three, BulkOrder_Typehash_Height_Four)

                        // Exit the function once typehash has been located.
                        leave
                    }

                    // Handle tree height five and six.
                    if lt(treeHeight, 7) {
                        // Utilize branchless logic to determine typehash.
                        typeHash :=
                            ternary(eq(treeHeight, 5), BulkOrder_Typehash_Height_Five, BulkOrder_Typehash_Height_Six)

                        // Exit the function once typehash has been located.
                        leave
                    }

                    // Handle height seven and eight via branchless logic.
                    typeHash :=
                        ternary(eq(treeHeight, 7), BulkOrder_Typehash_Height_Seven, BulkOrder_Typehash_Height_Eight)

                    // Exit the function once typehash has been located.
                    leave
                }

                // Handle tree height nine through sixteen.
                if lt(treeHeight, 17) {
                    // Handle tree height nine through twelve.
                    if lt(treeHeight, 13) {
                        // Handle tree height nine and ten.
                        if lt(treeHeight, 11) {
                            // Utilize branchless logic to determine typehash.
                            typeHash :=
                                ternary(eq(treeHeight, 9), BulkOrder_Typehash_Height_Nine, BulkOrder_Typehash_Height_Ten)

                            // Exit the function once typehash has been located.
                            leave
                        }

                        // Handle height eleven and twelve via branchless logic.
                        typeHash :=
                            ternary(eq(treeHeight, 11), BulkOrder_Typehash_Height_Eleven, BulkOrder_Typehash_Height_Twelve)

                        // Exit the function once typehash has been located.
                        leave
                    }

                    // Handle tree height thirteen and fourteen.
                    if lt(treeHeight, 15) {
                        // Utilize branchless logic to determine typehash.
                        typeHash :=
                            ternary(
                                eq(treeHeight, 13), BulkOrder_Typehash_Height_Thirteen, BulkOrder_Typehash_Height_Fourteen
                            )

                        // Exit the function once typehash has been located.
                        leave
                    }
                    // Handle height fifteen and sixteen via branchless logic.
                    typeHash :=
                        ternary(eq(treeHeight, 15), BulkOrder_Typehash_Height_Fifteen, BulkOrder_Typehash_Height_Sixteen)

                    // Exit the function once typehash has been located.
                    leave
                }

                // Handle tree height seventeen through twenty.
                if lt(treeHeight, 21) {
                    // Handle tree height seventeen and eighteen.
                    if lt(treeHeight, 19) {
                        // Utilize branchless logic to determine typehash.
                        typeHash :=
                            ternary(
                                eq(treeHeight, 17), BulkOrder_Typehash_Height_Seventeen, BulkOrder_Typehash_Height_Eighteen
                            )

                        // Exit the function once typehash has been located.
                        leave
                    }

                    // Handle height nineteen and twenty via branchless logic.
                    typeHash :=
                        ternary(eq(treeHeight, 19), BulkOrder_Typehash_Height_Nineteen, BulkOrder_Typehash_Height_Twenty)

                    // Exit the function once typehash has been located.
                    leave
                }

                // Handle tree height twenty-one and twenty-two.
                if lt(treeHeight, 23) {
                    // Utilize branchless logic to determine typehash.
                    typeHash :=
                        ternary(
                            eq(treeHeight, 21), BulkOrder_Typehash_Height_TwentyOne, BulkOrder_Typehash_Height_TwentyTwo
                        )

                    // Exit the function once typehash has been located.
                    leave
                }

                // Handle height twenty-three & twenty-four w/ branchless logic.
                typeHash :=
                    ternary(eq(treeHeight, 23), BulkOrder_Typehash_Height_TwentyThree, BulkOrder_Typehash_Height_TwentyFour)

                // Exit the function once typehash has been located.
                leave
            }

            // Implement ternary conditional using branchless logic.
            function ternary(cond, ifTrue, ifFalse) -> c {
                c := xor(ifFalse, mul(cond, xor(ifFalse, ifTrue)))
            }

            // Look up the typehash using the supplied tree height.
            _typeHash := lookupTypeHash(_treeHeight)
        }
    }
}

/**
 * @title GettersAndDerivers
 * @author 0age
 * @notice ConsiderationInternal contains pure and internal view functions
 *         related to getting or deriving various values.
 */
contract GettersAndDerivers is ConsiderationBase {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) ConsiderationBase(conduitController) {}

    /**
     * @dev Internal view function to derive the order hash for a given order.
     *      Note that only the original consideration items are included in the
     *      order hash, as additional consideration items may be supplied by the
     *      caller.
     *
     * @param orderParameters The parameters of the order to hash.
     * @param counter         The counter of the order to hash.
     *
     * @return orderHash The hash.
     */
    function _deriveOrderHash(OrderParameters memory orderParameters, uint256 counter)
        internal
        view
        returns (bytes32 orderHash)
    {
        // Get length of original consideration array and place it on the stack.
        uint256 originalConsiderationLength = (orderParameters.totalOriginalConsiderationItems);

        /*
         * Memory layout for an array of structs (dynamic or not) is similar
         * to ABI encoding of dynamic types, with a head segment followed by
         * a data segment. The main difference is that the head of an element
         * is a memory pointer rather than an offset.
         */

        // Declare a variable for the derived hash of the offer array.
        bytes32 offerHash;

        // Read offer item EIP-712 typehash from runtime code & place on stack.
        bytes32 typeHash = _OFFER_ITEM_TYPEHASH;

        // Utilize assembly so that memory regions can be reused across hashes.
        assembly {
            // Retrieve the free memory pointer and place on the stack.
            let hashArrPtr := mload(FreeMemoryPointerSlot)

            // Get the pointer to the offers array.
            let offerArrPtr := mload(add(orderParameters, OrderParameters_offer_head_offset))

            // Load the length.
            let offerLength := mload(offerArrPtr)

            // Set the pointer to the first offer's head.
            offerArrPtr := add(offerArrPtr, OneWord)

            // Iterate over the offer items.
            for { let i := 0 } lt(i, offerLength) { i := add(i, 1) } {
                // Read the pointer to the offer data and subtract one word
                // to get typeHash pointer.
                let ptr := sub(mload(offerArrPtr), OneWord)

                // Read the current value before the offer data.
                let value := mload(ptr)

                // Write the type hash to the previous word.
                mstore(ptr, typeHash)

                // Take the EIP712 hash and store it in the hash array.
                mstore(hashArrPtr, keccak256(ptr, EIP712_OfferItem_size))

                // Restore the previous word.
                mstore(ptr, value)

                // Increment the array pointers by one word.
                offerArrPtr := add(offerArrPtr, OneWord)
                hashArrPtr := add(hashArrPtr, OneWord)
            }

            // Derive the offer hash using the hashes of each item.
            offerHash := keccak256(mload(FreeMemoryPointerSlot), shl(OneWordShift, offerLength))
        }

        // Declare a variable for the derived hash of the consideration array.
        bytes32 considerationHash;

        // Read consideration item typehash from runtime code & place on stack.
        typeHash = _CONSIDERATION_ITEM_TYPEHASH;

        // Utilize assembly so that memory regions can be reused across hashes.
        assembly {
            // Retrieve the free memory pointer and place on the stack.
            let hashArrPtr := mload(FreeMemoryPointerSlot)

            // Get the pointer to the consideration array.
            let considerationArrPtr :=
                add(mload(add(orderParameters, OrderParameters_consideration_head_offset)), OneWord)

            // Iterate over the consideration items (not including tips).
            for { let i := 0 } lt(i, originalConsiderationLength) { i := add(i, 1) } {
                // Read the pointer to the consideration data and subtract one
                // word to get typeHash pointer.
                let ptr := sub(mload(considerationArrPtr), OneWord)

                // Read the current value before the consideration data.
                let value := mload(ptr)

                // Write the type hash to the previous word.
                mstore(ptr, typeHash)

                // Take the EIP712 hash and store it in the hash array.
                mstore(hashArrPtr, keccak256(ptr, EIP712_ConsiderationItem_size))

                // Restore the previous word.
                mstore(ptr, value)

                // Increment the array pointers by one word.
                considerationArrPtr := add(considerationArrPtr, OneWord)
                hashArrPtr := add(hashArrPtr, OneWord)
            }

            // Derive the consideration hash using the hashes of each item.
            considerationHash := keccak256(mload(FreeMemoryPointerSlot), shl(OneWordShift, originalConsiderationLength))
        }

        // Read order item EIP-712 typehash from runtime code & place on stack.
        typeHash = _ORDER_TYPEHASH;

        // Utilize assembly to access derived hashes & other arguments directly.
        assembly {
            // Retrieve pointer to the region located just behind parameters.
            let typeHashPtr := sub(orderParameters, OneWord)

            // Store the value at that pointer location to restore later.
            let previousValue := mload(typeHashPtr)

            // Store the order item EIP-712 typehash at the typehash location.
            mstore(typeHashPtr, typeHash)

            // Retrieve the pointer for the offer array head.
            let offerHeadPtr := add(orderParameters, OrderParameters_offer_head_offset)

            // Retrieve the data pointer referenced by the offer head.
            let offerDataPtr := mload(offerHeadPtr)

            // Store the offer hash at the retrieved memory location.
            mstore(offerHeadPtr, offerHash)

            // Retrieve the pointer for the consideration array head.
            let considerationHeadPtr := add(orderParameters, OrderParameters_consideration_head_offset)

            // Retrieve the data pointer referenced by the consideration head.
            let considerationDataPtr := mload(considerationHeadPtr)

            // Store the consideration hash at the retrieved memory location.
            mstore(considerationHeadPtr, considerationHash)

            // Retrieve the pointer for the counter.
            let counterPtr := add(orderParameters, OrderParameters_counter_offset)

            // Store the counter at the retrieved memory location.
            mstore(counterPtr, counter)

            // Derive the order hash using the full range of order parameters.
            orderHash := keccak256(typeHashPtr, EIP712_Order_size)

            // Restore the value previously held at typehash pointer location.
            mstore(typeHashPtr, previousValue)

            // Restore offer data pointer at the offer head pointer location.
            mstore(offerHeadPtr, offerDataPtr)

            // Restore consideration data pointer at the consideration head ptr.
            mstore(considerationHeadPtr, considerationDataPtr)

            // Restore consideration item length at the counter pointer.
            mstore(counterPtr, originalConsiderationLength)
        }
    }

    /**
     * @dev Internal view function to derive the address of a given conduit
     *      using a corresponding conduit key.
     *
     * @param conduitKey A bytes32 value indicating what corresponding conduit,
     *                   if any, to source token approvals from. This value is
     *                   the "salt" parameter supplied by the deployer (i.e. the
     *                   conduit controller) when deploying the given conduit.
     *
     * @return conduit The address of the conduit associated with the given
     *                 conduit key.
     */
    function _deriveConduit(bytes32 conduitKey) internal view returns (address conduit) {
        // Read conduit controller address from runtime and place on the stack.
        address conduitController = address(_CONDUIT_CONTROLLER);

        // Read conduit creation code hash from runtime and place on the stack.
        bytes32 conduitCreationCodeHash = _CONDUIT_CREATION_CODE_HASH;

        // Leverage scratch space to perform an efficient hash.
        assembly {
            // Retrieve the free memory pointer; it will be replaced afterwards.
            let freeMemoryPointer := mload(FreeMemoryPointerSlot)

            // Place the control character and the conduit controller in scratch
            // space; note that eleven bytes at the beginning are left unused.
            mstore(0, or(MaskOverByteTwelve, conduitController))

            // Place the conduit key in the next region of scratch space.
            mstore(OneWord, conduitKey)

            // Place conduit creation code hash in free memory pointer location.
            mstore(TwoWords, conduitCreationCodeHash)

            // Derive conduit by hashing and applying a mask over last 20 bytes.
            conduit :=
                and(
                    // Hash the relevant region.
                    keccak256(
                        // The region starts at memory pointer 11.
                        Create2AddressDerivation_ptr,
                        // The region is 85 bytes long (1 + 20 + 32 + 32).
                        Create2AddressDerivation_length
                    ),
                    // The address equals the last twenty bytes of the hash.
                    MaskOverLastTwentyBytes
                )

            // Restore the free memory pointer.
            mstore(FreeMemoryPointerSlot, freeMemoryPointer)
        }
    }

    /**
     * @dev Internal view function to get the EIP-712 domain separator. If the
     *      chainId matches the chainId set on deployment, the cached domain
     *      separator will be returned; otherwise, it will be derived from
     *      scratch.
     *
     * @return The domain separator.
     */
    function _domainSeparator() internal view returns (bytes32) {
        return block.chainid == _CHAIN_ID ? _DOMAIN_SEPARATOR : _deriveDomainSeparator();
    }

    /**
     * @dev Internal view function to retrieve configuration information for
     *      this contract.
     *
     * @return The contract version.
     * @return The domain separator for this contract.
     * @return The conduit Controller set for this contract.
     */
    function _information()
        internal
        view
        returns (string memory, /* version */ bytes32, /* domainSeparator */ address /* conduitController */ )
    {
        // Derive the domain separator.
        bytes32 domainSeparator = _domainSeparator();

        // Declare variable as immutables cannot be accessed within assembly.
        address conduitController = address(_CONDUIT_CONTROLLER);

        // Return the version, domain separator, and conduit controller.
        assembly {
            mstore(information_version_offset, information_version_cd_offset)
            mstore(information_domainSeparator_offset, domainSeparator)
            mstore(information_conduitController_offset, conduitController)
            mstore(information_versionLengthPtr, information_versionWithLength)
            return(information_version_offset, information_length)
        }
    }

    /**
     * @dev Internal pure function to efficiently derive an digest to sign for
     *      an order in accordance with EIP-712.
     *
     * @param domainSeparator The domain separator.
     * @param orderHash       The order hash.
     *
     * @return value The hash.
     */
    function _deriveEIP712Digest(bytes32 domainSeparator, bytes32 orderHash) internal pure returns (bytes32 value) {
        // Leverage scratch space to perform an efficient hash.
        assembly {
            // Place the EIP-712 prefix at the start of scratch space.
            mstore(0, EIP_712_PREFIX)

            // Place the domain separator in the next region of scratch space.
            mstore(EIP712_DomainSeparator_offset, domainSeparator)

            // Place the order hash in scratch space, spilling into the first
            // two bytes of the free memory pointer — this should never be set
            // as memory cannot be expanded to that size, and will be zeroed out
            // after the hash is performed.
            mstore(EIP712_OrderHash_offset, orderHash)

            // Hash the relevant region (65 bytes).
            value := keccak256(0, EIP712_DigestPayload_size)

            // Clear out the dirtied bits in the memory pointer.
            mstore(EIP712_OrderHash_offset, 0)
        }
    }
}

/**
 * @title TokenTransferrerErrors
 */
interface TokenTransferrerErrors {
    /**
     * @dev Revert with an error when an ERC721 transfer with amount other than
     *      one is attempted.
     *
     * @param amount The amount of the ERC721 tokens to transfer.
     */
    error InvalidERC721TransferAmount(uint256 amount);

    /**
     * @dev Revert with an error when attempting to fulfill an order where an
     *      item has an amount of zero.
     */
    error MissingItemAmount();

    /**
     * @dev Revert with an error when attempting to fulfill an order where an
     *      item has unused parameters. This includes both the token and the
     *      identifier parameters for native transfers as well as the identifier
     *      parameter for ERC20 transfers. Note that the conduit does not
     *      perform this check, leaving it up to the calling channel to enforce
     *      when desired.
     */
    error UnusedItemParameters();

    /**
     * @dev Revert with an error when an ERC20, ERC721, or ERC1155 token
     *      transfer reverts.
     *
     * @param token      The token for which the transfer was attempted.
     * @param from       The source of the attempted transfer.
     * @param to         The recipient of the attempted transfer.
     * @param identifier The identifier for the attempted transfer.
     * @param amount     The amount for the attempted transfer.
     */
    error TokenTransferGenericFailure(
        address token,
        address from,
        address to,
        uint256 identifier,
        uint256 amount
    );

    /**
     * @dev Revert with an error when a batch ERC1155 token transfer reverts.
     *
     * @param token       The token for which the transfer was attempted.
     * @param from        The source of the attempted transfer.
     * @param to          The recipient of the attempted transfer.
     * @param identifiers The identifiers for the attempted transfer.
     * @param amounts     The amounts for the attempted transfer.
     */
    error ERC1155BatchTransferGenericFailure(
        address token,
        address from,
        address to,
        uint256[] identifiers,
        uint256[] amounts
    );

    /**
     * @dev Revert with an error when an ERC20 token transfer returns a falsey
     *      value.
     *
     * @param token      The token for which the ERC20 transfer was attempted.
     * @param from       The source of the attempted ERC20 transfer.
     * @param to         The recipient of the attempted ERC20 transfer.
     * @param amount     The amount for the attempted ERC20 transfer.
     */
    error BadReturnValueFromERC20OnTransfer(
        address token,
        address from,
        address to,
        uint256 amount
    );

    /**
     * @dev Revert with an error when an account being called as an assumed
     *      contract does not have code and returns no data.
     *
     * @param account The account that should contain code.
     */
    error NoContract(address account);

    /**
     * @dev Revert with an error when attempting to execute an 1155 batch
     *      transfer using calldata not produced by default ABI encoding or with
     *      different lengths for ids and amounts arrays.
     */
    error Invalid1155BatchTransferEncoding();
}

/**
 * @title ReentrancyErrors
 * @author 0age
 * @notice ReentrancyErrors contains errors related to reentrancy.
 */
interface ReentrancyErrors {
    /**
     * @dev Revert with an error when a caller attempts to reenter a protected
     *      function.
     */
    error NoReentrantCalls();
}

/**
 * @title LowLevelHelpers
 * @author 0age
 * @notice LowLevelHelpers contains logic for performing various low-level
 *         operations.
 */
contract LowLevelHelpers {
    /**
     * @dev Internal view function to revert and pass along the revert reason if
     *      data was returned by the last call and that the size of that data
     *      does not exceed the currently allocated memory size.
     */
    function _revertWithReasonIfOneIsReturned() internal view {
        assembly {
            // If it returned a message, bubble it up as long as sufficient gas
            // remains to do so:
            if returndatasize() {
                // Ensure that sufficient gas is available to copy returndata
                // while expanding memory where necessary. Start by computing
                // the word size of returndata and allocated memory.
                let returnDataWords := shr(OneWordShift, add(returndatasize(), ThirtyOneBytes))

                // Note: use the free memory pointer in place of msize() to work
                // around a Yul warning that prevents accessing msize directly
                // when the IR pipeline is activated.
                let msizeWords := shr(OneWordShift, mload(FreeMemoryPointerSlot))

                // Next, compute the cost of the returndatacopy.
                let cost := mul(CostPerWord, returnDataWords)

                // Then, compute cost of new memory allocation.
                if gt(returnDataWords, msizeWords) {
                    cost :=
                        add(
                            cost,
                            add(
                                mul(sub(returnDataWords, msizeWords), CostPerWord),
                                shr(
                                    MemoryExpansionCoefficientShift,
                                    sub(mul(returnDataWords, returnDataWords), mul(msizeWords, msizeWords))
                                )
                            )
                        )
                }

                // Finally, add a small constant and compare to gas remaining;
                // bubble up the revert data if enough gas is still available.
                if lt(add(cost, ExtraGasBuffer), gas()) {
                    // Copy returndata to memory; overwrite existing memory.
                    returndatacopy(0, 0, returndatasize())

                    // Revert, specifying memory region with copied returndata.
                    revert(0, returndatasize())
                }
            }
        }
    }

    /**
     * @dev Internal view function to branchlessly select either the caller (if
     *      a supplied recipient is equal to zero) or the supplied recipient (if
     *      that recipient is a nonzero value).
     *
     * @param recipient The supplied recipient.
     *
     * @return updatedRecipient The updated recipient.
     */
    function _substituteCallerForEmptyRecipient(address recipient) internal view returns (address updatedRecipient) {
        // Utilize assembly to perform a branchless operation on the recipient.
        assembly {
            // Add caller to recipient if recipient equals 0; otherwise add 0.
            updatedRecipient := add(recipient, mul(iszero(recipient), caller()))
        }
    }

    /**
     * @dev Internal pure function to cast a `bool` value to a `uint256` value.
     *
     * @param b The `bool` value to cast.
     *
     * @return u The `uint256` value.
     */
    function _cast(bool b) internal pure returns (uint256 u) {
        assembly {
            u := b
        }
    }
}

/**
 * @title ReentrancyGuard
 * @author 0age
 * @notice ReentrancyGuard contains a storage variable and related functionality
 *         for protecting against reentrancy.
 */
contract ReentrancyGuard is ReentrancyErrors, LowLevelHelpers {
    // Prevent reentrant calls on protected functions.
    uint256 private _reentrancyGuard;

    /**
     * @dev Initialize the reentrancy guard during deployment.
     */
    constructor() {
        // Initialize the reentrancy guard in a cleared state.
        _reentrancyGuard = _NOT_ENTERED;
    }

    /**
     * @dev Internal function to ensure that a sentinel value for the reentrancy
     *      guard is not currently set and, if not, to set a sentinel value for
     *      the reentrancy guard based on whether or not native tokens may be
     *      received during execution or not.
     *
     * @param acceptNativeTokens A boolean indicating whether native tokens may
     *                           be received during execution or not.
     */
    function _setReentrancyGuard(bool acceptNativeTokens) internal {
        // Ensure that the reentrancy guard is not already set.
        _assertNonReentrant();

        // Set the reentrancy guard. A value of 2 indicates that native tokens
        // may not be accepted during execution, whereas a value of 3 indicates
        // that they will be accepted (with any remaining native tokens returned
        // to the caller).
        unchecked {
            _reentrancyGuard = _ENTERED + _cast(acceptNativeTokens);
        }
    }

    /**
     * @dev Internal function to unset the reentrancy guard sentinel value.
     */
    function _clearReentrancyGuard() internal {
        // Clear the reentrancy guard.
        _reentrancyGuard = _NOT_ENTERED;
    }

    /**
     * @dev Internal view function to ensure that a sentinel value for the
     *         reentrancy guard is not currently set.
     */
    function _assertNonReentrant() internal view {
        // Ensure that the reentrancy guard is not currently set.
        if (_reentrancyGuard != _NOT_ENTERED) {
            _revertNoReentrantCalls();
        }
    }

    /**
     * @dev Internal view function to ensure that the sentinel value indicating
     *      native tokens may be received during execution is currently set.
     */
    function _assertAcceptingNativeTokens() internal view {
        // Ensure that the reentrancy guard is not currently set.
        if (_reentrancyGuard != _ENTERED_AND_ACCEPTING_NATIVE_TOKENS) {
            _revertInvalidMsgValue(msg.value);
        }
    }
}

/**
 * @title CounterManager
 * @author 0age
 * @notice CounterManager contains a storage mapping and related functionality
 *         for retrieving and incrementing a per-offerer counter.
 */
contract CounterManager is ConsiderationEventsAndErrors, ReentrancyGuard {
    // Only orders signed using an offerer's current counter are fulfillable.
    mapping(address => uint256) private _counters;

    /**
     * @dev Internal function to cancel all orders from a given offerer in bulk
     *      by incrementing a counter by a large, quasi-random interval. Note
     *      that only the offerer may increment the counter. Note that the
     *      counter is incremented by a large, quasi-random interval, which
     *      makes it infeasible to "activate" signed orders by incrementing the
     *      counter.  This activation functionality can be achieved instead with
     *      restricted orders or contract orders.
     *
     * @return newCounter The new counter.
     */
    function _incrementCounter() internal returns (uint256 newCounter) {
        // Ensure that the reentrancy guard is not currently set.
        _assertNonReentrant();

        // Utilize assembly to access counters storage mapping directly. Skip
        // overflow check as counter cannot be incremented that far.
        assembly {
            // Use second half of previous block hash as a quasi-random number.
            let quasiRandomNumber := shr(Counter_blockhash_shift, blockhash(sub(number(), 1)))

            // Write the caller to scratch space.
            mstore(0, caller())

            // Write the storage slot for _counters to scratch space.
            mstore(OneWord, _counters.slot)

            // Derive the storage pointer for the counter value.
            let storagePointer := keccak256(0, TwoWords)

            // Derive new counter value using random number and original value.
            newCounter := add(quasiRandomNumber, sload(storagePointer))

            // Store the updated counter value.
            sstore(storagePointer, newCounter)
        }

        // Emit an event containing the new counter.
        emit CounterIncremented(newCounter, msg.sender);
    }

    /**
     * @dev Internal view function to retrieve the current counter for a given
     *      offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return currentCounter The current counter.
     */
    function _getCounter(address offerer) internal view returns (uint256 currentCounter) {
        // Return the counter for the supplied offerer.
        currentCounter = _counters[offerer];
    }
}

/**
 * @title Assertions
 * @author 0age
 * @notice Assertions contains logic for making various assertions that do not
 *         fit neatly within a dedicated semantic scope.
 */
contract Assertions is GettersAndDerivers, CounterManager, TokenTransferrerErrors {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) GettersAndDerivers(conduitController) {}

    /**
     * @dev Internal view function to ensure that the supplied consideration
     *      array length on a given set of order parameters is not less than the
     *      original consideration array length for that order and to retrieve
     *      the current counter for a given order's offerer and zone and use it
     *      to derive the order hash.
     *
     * @param orderParameters The parameters of the order to hash.
     *
     * @return The hash.
     */
    function _assertConsiderationLengthAndGetOrderHash(OrderParameters memory orderParameters)
        internal
        view
        returns (bytes32)
    {
        // Ensure supplied consideration array length is not less than original.
        _assertConsiderationLengthIsNotLessThanOriginalConsiderationLength(
            orderParameters.consideration.length, orderParameters.totalOriginalConsiderationItems
        );

        // Derive and return order hash using current counter for the offerer.
        return _deriveOrderHash(orderParameters, _getCounter(orderParameters.offerer));
    }

    /**
     * @dev Internal pure function to ensure that the supplied consideration
     *      array length for an order to be fulfilled is not less than the
     *      original consideration array length for that order.
     *
     * @param suppliedConsiderationItemTotal The number of consideration items
     *                                       supplied when fulfilling the order.
     * @param originalConsiderationItemTotal The number of consideration items
     *                                       supplied on initial order creation.
     */
    function _assertConsiderationLengthIsNotLessThanOriginalConsiderationLength(
        uint256 suppliedConsiderationItemTotal,
        uint256 originalConsiderationItemTotal
    ) internal pure {
        // Ensure supplied consideration array length is not less than original.
        if (suppliedConsiderationItemTotal < originalConsiderationItemTotal) {
            _revertMissingOriginalConsiderationItems();
        }
    }

    /**
     * @dev Internal pure function to ensure that a given item amount is not
     *      zero.
     *
     * @param amount The amount to check.
     */
    function _assertNonZeroAmount(uint256 amount) internal pure {
        assembly {
            if iszero(amount) {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, MissingItemAmount_error_selector)

                // revert(abi.encodeWithSignature("MissingItemAmount()"))
                revert(Error_selector_offset, MissingItemAmount_error_length)
            }
        }
    }

    /**
     * @dev Internal pure function to validate calldata offsets for dynamic
     *      types in BasicOrderParameters and other parameters. This ensures
     *      that functions using the calldata object normally will be using the
     *      same data as the assembly functions and that values that are bound
     *      to a given range are within that range. Note that no parameters are
     *      supplied as all basic order functions use the same calldata
     *      encoding.
     */
    function _assertValidBasicOrderParameters() internal pure {
        // Declare a boolean designating basic order parameter offset validity.
        bool validOffsets;

        // Utilize assembly in order to read offset data directly from calldata.
        assembly {
            /*
             * Checks:
             * 1. Order parameters struct offset == 0x20
             * 2. Additional recipients arr offset == 0x240
             * 3. Signature offset == 0x260 + (recipients.length * 0x40)
             * 4. BasicOrderType between 0 and 23 (i.e. < 24)
             * 5. Offerer, zone, offer token, and consideration token have no
             *    upper dirty bits — each argument is type(uint160).max or less
             */
            validOffsets :=
                and(
                    and(
                        and(
                            // Order parameters at cd 0x04 must have offset of 0x20.
                            eq(calldataload(BasicOrder_parameters_cdPtr), BasicOrder_parameters_ptr),
                            // Additional recipients (cd 0x224) arr offset == 0x240.
                            eq(
                                calldataload(BasicOrder_additionalRecipients_head_cdPtr),
                                BasicOrder_additionalRecipients_head_ptr
                            )
                        ),
                        // Signature offset == 0x260 + (recipients.length * 0x40).
                        eq(
                            // Load signature offset from calldata 0x244.
                            calldataload(BasicOrder_signature_cdPtr),
                            // Expected offset is start of recipients + len * 64.
                            add(
                                BasicOrder_signature_ptr,
                                shl(
                                    // Each additional recipient has length of 0x40.
                                    AdditionalRecipient_size_shift,
                                    // Additional recipients length at cd 0x264.
                                    calldataload(BasicOrder_additionalRecipients_length_cdPtr)
                                )
                            )
                        )
                    ),
                    and(
                        // Ensure BasicOrderType parameter is less than 0x18.
                        lt(
                            // BasicOrderType parameter at calldata offset 0x124.
                            calldataload(BasicOrder_basicOrderType_cdPtr),
                            // Value should be less than 24.
                            BasicOrder_basicOrderType_range
                        ),
                        // Ensure no dirty upper bits are present on offerer, zone,
                        // offer token, or consideration token.
                        lt(
                            or(
                                or(
                                    // Offerer parameter at calldata offset 0x84.
                                    calldataload(BasicOrder_offerer_cdPtr),
                                    // Zone parameter at calldata offset 0xa4.
                                    calldataload(BasicOrder_zone_cdPtr)
                                ),
                                or(
                                    // Offer token parameter at cd offset 0xc4.
                                    calldataload(BasicOrder_offerToken_cdPtr),
                                    // Consideration token parameter at offset 0x24.
                                    calldataload(BasicOrder_considerationToken_cdPtr)
                                )
                            ),
                            AddressDirtyUpperBitThreshold
                        )
                    )
                )
        }

        // Revert with an error if basic order parameter offsets are invalid.
        if (!validOffsets) {
            _revertInvalidBasicOrderParameterEncoding();
        }
    }
}

/**
 * @title SignatureVerificationErrors
 * @author 0age
 * @notice SignatureVerificationErrors contains all errors related to signature
 *         verification.
 */
interface SignatureVerificationErrors {
    /**
     * @dev Revert with an error when a signature that does not contain a v
     *      value of 27 or 28 has been supplied.
     *
     * @param v The invalid v value.
     */
    error BadSignatureV(uint8 v);

    /**
     * @dev Revert with an error when the signer recovered by the supplied
     *      signature does not match the offerer or an allowed EIP-1271 signer
     *      as specified by the offerer in the event they are a contract.
     */
    error InvalidSigner();

    /**
     * @dev Revert with an error when a signer cannot be recovered from the
     *      supplied signature.
     */
    error InvalidSignature();

    /**
     * @dev Revert with an error when an EIP-1271 call to an account fails.
     */
    error BadContractSignature();
}

/**
 * @title SignatureVerification
 * @author 0age
 * @notice SignatureVerification contains logic for verifying signatures.
 */
contract SignatureVerification is SignatureVerificationErrors, LowLevelHelpers {
    /**
     * @dev Internal view function to verify the signature of an order. An
     *      ERC-1271 fallback will be attempted if either the signature length
     *      is not 64 or 65 bytes or if the recovered signer does not match the
     *      supplied signer.
     *
     * @param signer                  The signer for the order.
     * @param digest                  The digest to verify signature against.
     * @param originalDigest          The original digest to verify signature
     *                                against.
     * @param originalSignatureLength The original signature length.
     * @param signature               A signature from the signer indicating
     *                                that the order has been approved.
     */
    function _assertValidSignature(
        address signer,
        bytes32 digest,
        bytes32 originalDigest,
        uint256 originalSignatureLength,
        bytes memory signature
    ) internal view {
        // Declare value for ecrecover equality or 1271 call success status.
        bool success;

        // Utilize assembly to perform optimized signature verification check.
        assembly {
            // Ensure that first word of scratch space is empty.
            mstore(0, 0)

            // Get the length of the signature.
            let signatureLength := mload(signature)

            // Get the pointer to the value preceding the signature length.
            // This will be used for temporary memory overrides - either the
            // signature head for isValidSignature or the digest for ecrecover.
            let wordBeforeSignaturePtr := sub(signature, OneWord)

            // Cache the current value behind the signature to restore it later.
            let cachedWordBeforeSignature := mload(wordBeforeSignaturePtr)

            // Declare lenDiff + recoveredSigner scope to manage stack pressure.
            {
                // Take the difference between the max ECDSA signature length
                // and the actual signature length. Overflow desired for any
                // values > 65. If the diff is not 0 or 1, it is not a valid
                // ECDSA signature - move on to EIP1271 check.
                let lenDiff := sub(ECDSA_MaxLength, signatureLength)

                // Declare variable for recovered signer.
                let recoveredSigner

                // If diff is 0 or 1, it may be an ECDSA signature.
                // Try to recover signer.
                if iszero(gt(lenDiff, 1)) {
                    // Read the signature `s` value.
                    let originalSignatureS := mload(add(signature, ECDSA_signature_s_offset))

                    // Read the first byte of the word after `s`. If the
                    // signature is 65 bytes, this will be the real `v` value.
                    // If not, it will need to be modified - doing it this way
                    // saves an extra condition.
                    let v := byte(0, mload(add(signature, ECDSA_signature_v_offset)))

                    // If lenDiff is 1, parse 64-byte signature as ECDSA.
                    if lenDiff {
                        // Extract yParity from highest bit of vs and add 27 to
                        // get v.
                        v := add(shr(MaxUint8, originalSignatureS), Signature_lower_v)

                        // Extract canonical s from vs, all but the highest bit.
                        // Temporarily overwrite the original `s` value in the
                        // signature.
                        mstore(
                            add(signature, ECDSA_signature_s_offset),
                            and(originalSignatureS, EIP2098_allButHighestBitMask)
                        )
                    }
                    // Temporarily overwrite the signature length with `v` to
                    // conform to the expected input for ecrecover.
                    mstore(signature, v)

                    // Temporarily overwrite the word before the length with
                    // `digest` to conform to the expected input for ecrecover.
                    mstore(wordBeforeSignaturePtr, digest)

                    // Attempt to recover the signer for the given signature. Do
                    // not check the call status as ecrecover will return a null
                    // address if the signature is invalid.
                    pop(
                        staticcall(
                            gas(),
                            Ecrecover_precompile, // Call ecrecover precompile.
                            wordBeforeSignaturePtr, // Use data memory location.
                            Ecrecover_args_size, // Size of digest, v, r, and s.
                            0, // Write result to scratch space.
                            OneWord // Provide size of returned result.
                        )
                    )

                    // Restore cached word before signature.
                    mstore(wordBeforeSignaturePtr, cachedWordBeforeSignature)

                    // Restore cached signature length.
                    mstore(signature, signatureLength)

                    // Restore cached signature `s` value.
                    mstore(add(signature, ECDSA_signature_s_offset), originalSignatureS)

                    // Read the recovered signer from the buffer given as return
                    // space for ecrecover.
                    recoveredSigner := mload(0)
                }

                // Set success to true if the signature provided was a valid
                // ECDSA signature and the signer is not the null address. Use
                // gt instead of direct as success is used outside of assembly.
                success := and(eq(signer, recoveredSigner), gt(signer, 0))
            }

            // If the signature was not verified with ecrecover, try EIP1271.
            if iszero(success) {
                // Reset the original signature length.
                mstore(signature, originalSignatureLength)

                // Temporarily overwrite the word before the signature length
                // and use it as the head of the signature input to
                // `isValidSignature`, which has a value of 64.
                mstore(wordBeforeSignaturePtr, EIP1271_isValidSignature_signature_head_offset)

                // Get pointer to use for the selector of `isValidSignature`.
                let selectorPtr := sub(signature, EIP1271_isValidSignature_selector_negativeOffset)

                // Cache the value currently stored at the selector pointer.
                let cachedWordOverwrittenBySelector := mload(selectorPtr)

                // Cache the value currently stored at the digest pointer.
                let cachedWordOverwrittenByDigest :=
                    mload(sub(signature, EIP1271_isValidSignature_digest_negativeOffset))

                // Write the selector first, since it overlaps the digest.
                mstore(selectorPtr, EIP1271_isValidSignature_selector)

                // Next, write the original digest.
                mstore(sub(signature, EIP1271_isValidSignature_digest_negativeOffset), originalDigest)

                // Call signer with `isValidSignature` to validate signature.
                success :=
                    staticcall(
                        gas(),
                        signer,
                        selectorPtr,
                        add(originalSignatureLength, EIP1271_isValidSignature_calldata_baseLength),
                        0,
                        OneWord
                    )

                // Determine if the signature is valid on successful calls.
                if success {
                    // If first word of scratch space does not contain EIP-1271
                    // signature selector, revert.
                    if iszero(eq(mload(0), EIP1271_isValidSignature_selector)) {
                        // Revert with bad 1271 signature if signer has code.
                        if extcodesize(signer) {
                            // Bad contract signature.
                            // Store left-padded selector with push4, mem[28:32]
                            mstore(0, BadContractSignature_error_selector)

                            // revert(abi.encodeWithSignature(
                            //     "BadContractSignature()"
                            // ))
                            revert(Error_selector_offset, BadContractSignature_error_length)
                        }

                        // Check if signature length was invalid.
                        if gt(sub(ECDSA_MaxLength, signatureLength), 1) {
                            // Revert with generic invalid signature error.
                            // Store left-padded selector with push4, mem[28:32]
                            mstore(0, InvalidSignature_error_selector)

                            // revert(abi.encodeWithSignature(
                            //     "InvalidSignature()"
                            // ))
                            revert(Error_selector_offset, InvalidSignature_error_length)
                        }

                        // Check if v was invalid.
                        if and(
                            eq(signatureLength, ECDSA_MaxLength),
                            iszero(
                                byte(
                                    byte(0, mload(add(signature, ECDSA_signature_v_offset))),
                                    ECDSA_twentySeventhAndTwentyEighthBytesSet
                                )
                            )
                        ) {
                            // Revert with invalid v value.
                            // Store left-padded selector with push4, mem[28:32]
                            mstore(0, BadSignatureV_error_selector)
                            mstore(BadSignatureV_error_v_ptr, byte(0, mload(add(signature, ECDSA_signature_v_offset))))

                            // revert(abi.encodeWithSignature(
                            //     "BadSignatureV(uint8)", v
                            // ))
                            revert(Error_selector_offset, BadSignatureV_error_length)
                        }

                        // Revert with generic invalid signer error message.
                        // Store left-padded selector with push4, mem[28:32]
                        mstore(0, InvalidSigner_error_selector)

                        // revert(abi.encodeWithSignature("InvalidSigner()"))
                        revert(Error_selector_offset, InvalidSigner_error_length)
                    }
                }

                // Restore the cached values overwritten by selector, digest and
                // signature head.
                mstore(wordBeforeSignaturePtr, cachedWordBeforeSignature)
                mstore(selectorPtr, cachedWordOverwrittenBySelector)
                mstore(sub(signature, EIP1271_isValidSignature_digest_negativeOffset), cachedWordOverwrittenByDigest)
            }
        }

        // If the call failed...
        if (!success) {
            // Revert and pass reason along if one was returned.
            _revertWithReasonIfOneIsReturned();

            // Otherwise, revert with error indicating bad contract signature.
            assembly {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, BadContractSignature_error_selector)
                // revert(abi.encodeWithSignature("BadContractSignature()"))
                revert(Error_selector_offset, BadContractSignature_error_length)
            }
        }
    }
}

/**
 * @title Verifiers
 * @author 0age
 * @notice Verifiers contains functions for performing verifications.
 */
contract Verifiers is Assertions, SignatureVerification {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) Assertions(conduitController) {}

    /**
     * @dev Internal view function to ensure that the current time falls within
     *      an order's valid timespan.
     *
     * @param startTime       The time at which the order becomes active.
     * @param endTime         The time at which the order becomes inactive.
     * @param revertOnInvalid A boolean indicating whether to revert if the
     *                        order is not active.
     *
     * @return valid A boolean indicating whether the order is active.
     */
    function _verifyTime(uint256 startTime, uint256 endTime, bool revertOnInvalid) internal view returns (bool valid) {
        // Mark as valid if order has started and has not already ended.
        assembly {
            valid := and(iszero(gt(startTime, timestamp())), gt(endTime, timestamp()))
        }

        // Only revert on invalid if revertOnInvalid has been supplied as true.
        if (revertOnInvalid && !valid) {
            _revertInvalidTime(startTime, endTime);
        }
    }

    /**
     * @dev Internal view function to verify the signature of an order. An
     *      ERC-1271 fallback will be attempted if either the signature length
     *      is not 64 or 65 bytes or if the recovered signer does not match the
     *      supplied offerer. Note that in cases where a 64 or 65 byte signature
     *      is supplied, only standard ECDSA signatures that recover to a
     *      non-zero address are supported.
     *
     * @param offerer   The offerer for the order.
     * @param orderHash The order hash.
     * @param signature A signature from the offerer indicating that the order
     *                  has been approved.
     */
    function _verifySignature(address offerer, bytes32 orderHash, bytes memory signature) internal view {
        // Determine whether the offerer is the caller.
        bool offererIsCaller;
        assembly {
            offererIsCaller := eq(offerer, caller())
        }

        // Skip signature verification if the offerer is the caller.
        if (offererIsCaller) {
            return;
        }

        // Derive the EIP-712 domain separator.
        bytes32 domainSeparator = _domainSeparator();

        // Derive original EIP-712 digest using domain separator and order hash.
        bytes32 originalDigest = _deriveEIP712Digest(domainSeparator, orderHash);

        // Read the length of the signature from memory and place on the stack.
        uint256 originalSignatureLength = signature.length;

        // Determine effective digest if signature has a valid bulk order size.
        bytes32 digest;
        if (_isValidBulkOrderSize(originalSignatureLength)) {
            // Rederive order hash and digest using bulk order proof.
            (orderHash) = _computeBulkOrderProof(signature, orderHash);
            digest = _deriveEIP712Digest(domainSeparator, orderHash);
        } else {
            // Supply the original digest as the effective digest.
            digest = originalDigest;
        }

        // Ensure that the signature for the digest is valid for the offerer.
        _assertValidSignature(offerer, digest, originalDigest, originalSignatureLength, signature);
    }

    /**
     * @dev Determines whether the specified bulk order size is valid.
     *
     * @param signatureLength The signature length of the bulk order to check.
     *
     * @return validLength True if bulk order size is valid, false otherwise.
     */
    function _isValidBulkOrderSize(uint256 signatureLength) internal pure returns (bool validLength) {
        // Utilize assembly to validate the length; the equivalent logic is
        // (64 + x) + 3 + 32y where (0 <= x <= 1) and (1 <= y <= 24).
        assembly {
            validLength :=
                and(
                    lt(sub(signatureLength, BulkOrderProof_minSize), BulkOrderProof_rangeSize),
                    lt(
                        and(add(signatureLength, BulkOrderProof_lengthAdjustmentBeforeMask), ThirtyOneBytes),
                        BulkOrderProof_lengthRangeAfterMask
                    )
                )
        }
    }

    /**
     * @dev Computes the bulk order hash for the specified proof and leaf. Note
     *      that if an index that exceeds the number of orders in the bulk order
     *      payload will instead "wrap around" and refer to an earlier index.
     *
     * @param proofAndSignature The proof and signature of the bulk order.
     * @param leaf              The leaf of the bulk order tree.
     *
     * @return bulkOrderHash The bulk order hash.
     */
    function _computeBulkOrderProof(bytes memory proofAndSignature, bytes32 leaf)
        internal
        pure
        returns (bytes32 bulkOrderHash)
    {
        // Declare arguments for the root hash and the height of the proof.
        bytes32 root;
        uint256 height;

        // Utilize assembly to efficiently derive the root hash using the proof.
        assembly {
            // Retrieve the length of the proof, key, and signature combined.
            let fullLength := mload(proofAndSignature)

            // If proofAndSignature has odd length, it is a compact signature
            // with 64 bytes.
            let signatureLength := sub(ECDSA_MaxLength, and(fullLength, 1))

            // Derive height (or depth of tree) with signature and proof length.
            height := shr(OneWordShift, sub(fullLength, signatureLength))

            // Update the length in memory to only include the signature.
            mstore(proofAndSignature, signatureLength)

            // Derive the pointer for the key using the signature length.
            let keyPtr := add(proofAndSignature, add(OneWord, signatureLength))

            // Retrieve the three-byte key using the derived pointer.
            let key := shr(BulkOrderProof_keyShift, mload(keyPtr))

            /// Retrieve pointer to first proof element by applying a constant
            // for the key size to the derived key pointer.
            let proof := add(keyPtr, BulkOrderProof_keySize)

            // Compute level 1.
            let scratchPtr1 := shl(OneWordShift, and(key, 1))
            mstore(scratchPtr1, leaf)
            mstore(xor(scratchPtr1, OneWord), mload(proof))

            // Compute remaining proofs.
            for { let i := 1 } lt(i, height) { i := add(i, 1) } {
                proof := add(proof, OneWord)
                let scratchPtr := shl(OneWordShift, and(shr(i, key), 1))
                mstore(scratchPtr, keccak256(0, TwoWords))
                mstore(xor(scratchPtr, OneWord), mload(proof))
            }

            // Compute root hash.
            root := keccak256(0, TwoWords)
        }

        // Retrieve appropriate typehash constant based on height.
        bytes32 rootTypeHash = _lookupBulkOrderTypehash(height);

        // Use the typehash and the root hash to derive final bulk order hash.
        assembly {
            mstore(0, rootTypeHash)
            mstore(OneWord, root)
            bulkOrderHash := keccak256(0, TwoWords)
        }
    }

    /**
     * @dev Internal view function to validate that a given order is fillable
     *      and not cancelled based on the order status.
     *
     * @param orderHash       The order hash.
     * @param orderStatus     The status of the order, including whether it has
     *                        been cancelled and the fraction filled.
     * @param onlyAllowUnused A boolean flag indicating whether partial fills
     *                        are supported by the calling function.
     * @param revertOnInvalid A boolean indicating whether to revert if the
     *                        order has been cancelled or filled beyond the
     *                        allowable amount.
     *
     * @return valid A boolean indicating whether the order is valid.
     */
    function _verifyOrderStatus(
        bytes32 orderHash,
        OrderStatus storage orderStatus,
        bool onlyAllowUnused,
        bool revertOnInvalid
    ) internal view returns (bool valid) {
        // Ensure that the order has not been cancelled.
        if (orderStatus.isCancelled) {
            // Only revert if revertOnInvalid has been supplied as true.
            if (revertOnInvalid) {
                _revertOrderIsCancelled(orderHash);
            }

            // Return false as the order status is invalid.
            return false;
        }

        // Read order status numerator from storage and place on stack.
        uint256 orderStatusNumerator = orderStatus.numerator;

        // If the order is not entirely unused...
        if (orderStatusNumerator != 0) {
            // ensure the order has not been partially filled when not allowed.
            if (onlyAllowUnused) {
                // Always revert on partial fills when onlyAllowUnused is true.
                _revertOrderPartiallyFilled(orderHash);
            }
            // Otherwise, ensure that order has not been entirely filled.
            else if (orderStatusNumerator >= orderStatus.denominator) {
                // Only revert if revertOnInvalid has been supplied as true.
                if (revertOnInvalid) {
                    _revertOrderAlreadyFilled(orderHash);
                }

                // Return false as the order status is invalid.
                return false;
            }
        }

        // Return true as the order status is valid.
        valid = true;
    }
}

/*
 * -------------------------- Disambiguation & Other Notes ---------------------
 *    - The term "head" is used as it is in the documentation for ABI encoding,
 *      but only in reference to dynamic types, i.e. it always refers to the
 *      offset or pointer to the body of a dynamic type. In calldata, the head
 *      is always an offset (relative to the parent object), while in memory,
 *      the head is always the pointer to the body. More information found here:
 *      https://docs.soliditylang.org/en/v0.8.17/abi-spec.html#argument-encoding
 *        - Note that the length of an array is separate from and precedes the
 *          head of the array.
 *
 *    - The term "body" is used in place of the term "head" used in the ABI
 *      documentation. It refers to the start of the data for a dynamic type,
 *      e.g. the first word of a struct or the first word of the first element
 *      in an array.
 *
 *    - The term "pointer" is used to describe the absolute position of a value
 *      and never an offset relative to another value.
 *        - The suffix "_ptr" refers to a memory pointer.
 *        - The suffix "_cdPtr" refers to a calldata pointer.
 *
 *    - The term "offset" is used to describe the position of a value relative
 *      to some parent value. For example, OrderParameters_conduit_offset is the
 *      offset to the "conduit" value in the OrderParameters struct relative to
 *      the start of the body.
 *        - Note: Offsets are used to derive pointers.
 *
 *    - Some structs have pointers defined for all of their fields in this file.
 *      Lines which are commented out are fields that are not used in the
 *      codebase but have been left in for readability.
 */

uint256 constant ThirtyOneBytes = 0x1f;
uint256 constant OneWord = 0x20;
uint256 constant TwoWords = 0x40;
uint256 constant ThreeWords = 0x60;

uint256 constant OneWordShift = 0x5;
uint256 constant TwoWordsShift = 0x6;

uint256 constant FreeMemoryPointerSlot = 0x40;
uint256 constant ZeroSlot = 0x60;
uint256 constant DefaultFreeMemoryPointer = 0x80;

uint256 constant Slot0x80 = 0x80;
uint256 constant Slot0xA0 = 0xa0;
uint256 constant Slot0xC0 = 0xc0;

uint256 constant Generic_error_selector_offset = 0x1c;

// abi.encodeWithSignature("transferFrom(address,address,uint256)")
uint256 constant ERC20_transferFrom_signature = (
    0x23b872dd00000000000000000000000000000000000000000000000000000000
);
uint256 constant ERC20_transferFrom_sig_ptr = 0x0;
uint256 constant ERC20_transferFrom_from_ptr = 0x04;
uint256 constant ERC20_transferFrom_to_ptr = 0x24;
uint256 constant ERC20_transferFrom_amount_ptr = 0x44;
uint256 constant ERC20_transferFrom_length = 0x64; // 4 + 32 * 3 == 100

// abi.encodeWithSignature(
//     "safeTransferFrom(address,address,uint256,uint256,bytes)"
// )
uint256 constant ERC1155_safeTransferFrom_signature = (
    0xf242432a00000000000000000000000000000000000000000000000000000000
);
uint256 constant ERC1155_safeTransferFrom_sig_ptr = 0x0;
uint256 constant ERC1155_safeTransferFrom_from_ptr = 0x04;
uint256 constant ERC1155_safeTransferFrom_to_ptr = 0x24;
uint256 constant ERC1155_safeTransferFrom_id_ptr = 0x44;
uint256 constant ERC1155_safeTransferFrom_amount_ptr = 0x64;
uint256 constant ERC1155_safeTransferFrom_data_offset_ptr = 0x84;
uint256 constant ERC1155_safeTransferFrom_data_length_ptr = 0xa4;
uint256 constant ERC1155_safeTransferFrom_length = 0xc4; // 4 + 32 * 6 == 196
uint256 constant ERC1155_safeTransferFrom_data_length_offset = 0xa0;

// abi.encodeWithSignature(
//     "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
// )
uint256 constant ERC1155_safeBatchTransferFrom_signature = (
    0x2eb2c2d600000000000000000000000000000000000000000000000000000000
);

// bytes4 constant ERC1155_safeBatchTransferFrom_selector = bytes4(
//     bytes32(ERC1155_safeBatchTransferFrom_signature)
// );

uint256 constant ERC721_transferFrom_signature = (
    0x23b872dd00000000000000000000000000000000000000000000000000000000
);
uint256 constant ERC721_transferFrom_sig_ptr = 0x0;
uint256 constant ERC721_transferFrom_from_ptr = 0x04;
uint256 constant ERC721_transferFrom_to_ptr = 0x24;
uint256 constant ERC721_transferFrom_id_ptr = 0x44;
uint256 constant ERC721_transferFrom_length = 0x64; // 4 + 32 * 3 == 100

/*
 *  error NoContract(address account)
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x00: account
 * Revert buffer is memory[0x1c:0x40]
 */
uint256 constant NoContract_error_selector = 0x5f15d672;
uint256 constant NoContract_error_account_ptr = 0x20;
uint256 constant NoContract_error_length = 0x24;

/*
 *  error TokenTransferGenericFailure(
 *      address token,
 *      address from,
 *      address to,
 *      uint256 identifier,
 *      uint256 amount
 *  )
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x20: token
 *    - 0x40: from
 *    - 0x60: to
 *    - 0x80: identifier
 *    - 0xa0: amount
 * Revert buffer is memory[0x1c:0xc0]
 */
uint256 constant TokenTransferGenericFailure_error_selector = 0xf486bc87;
uint256 constant TokenTransferGenericFailure_error_token_ptr = 0x20;
uint256 constant TokenTransferGenericFailure_error_from_ptr = 0x40;
uint256 constant TokenTransferGenericFailure_error_to_ptr = 0x60;
uint256 constant TokenTransferGenericFailure_error_identifier_ptr = 0x80;
uint256 constant TokenTransferGenericFailure_err_identifier_ptr = 0x80;
uint256 constant TokenTransferGenericFailure_error_amount_ptr = 0xa0;
uint256 constant TokenTransferGenericFailure_error_length = 0xa4;

uint256 constant ExtraGasBuffer = 0x20;
uint256 constant CostPerWord = 0x3;
uint256 constant MemoryExpansionCoefficientShift = 0x9;

// Values are offset by 32 bytes in order to write the token to the beginning
// in the event of a revert
uint256 constant BatchTransfer1155Params_ptr = 0x24;
uint256 constant BatchTransfer1155Params_ids_head_ptr = 0x64;
uint256 constant BatchTransfer1155Params_amounts_head_ptr = 0x84;
uint256 constant BatchTransfer1155Params_data_head_ptr = 0xa4;
uint256 constant BatchTransfer1155Params_data_length_basePtr = 0xc4;
uint256 constant BatchTransfer1155Params_calldata_baseSize = 0xc4;

uint256 constant BatchTransfer1155Params_ids_length_ptr = 0xc4;

uint256 constant BatchTransfer1155Params_ids_length_offset = 0xa0;
// uint256 constant BatchTransfer1155Params_amounts_length_baseOffset = 0xc0;
// uint256 constant BatchTransfer1155Params_data_length_baseOffset = 0xe0;

uint256 constant ConduitBatch1155Transfer_usable_head_size = 0x80;

uint256 constant ConduitBatch1155Transfer_from_offset = 0x20;
uint256 constant ConduitBatch1155Transfer_ids_head_offset = 0x60;
// uint256 constant ConduitBatch1155Transfer_amounts_head_offset = 0x80;
uint256 constant ConduitBatch1155Transfer_ids_length_offset = 0xa0;
uint256 constant ConduitBatch1155Transfer_amounts_length_baseOffset = 0xc0;
// uint256 constant ConduitBatch1155Transfer_calldata_baseSize = 0xc0;

// Note: abbreviated version of above constant to adhere to line length limit.
uint256 constant ConduitBatchTransfer_amounts_head_offset = 0x80;

uint256 constant Invalid1155BatchTransferEncoding_ptr = 0x00;
uint256 constant Invalid1155BatchTransferEncoding_length = 0x04;
uint256 constant Invalid1155BatchTransferEncoding_selector = (
    0xeba2084c00000000000000000000000000000000000000000000000000000000
);

uint256 constant ERC1155BatchTransferGenericFailure_error_signature = (
    0xafc445e200000000000000000000000000000000000000000000000000000000
);
uint256 constant ERC1155BatchTransferGenericFailure_token_ptr = 0x04;
uint256 constant ERC1155BatchTransferGenericFailure_ids_offset = 0xc0;

/*
 *  error BadReturnValueFromERC20OnTransfer(
 *      address token, address from, address to, uint256 amount
 *  )
 *    - Defined in TokenTransferrerErrors.sol
 *  Memory layout:
 *    - 0x00: Left-padded selector (data begins at 0x1c)
 *    - 0x00: token
 *    - 0x20: from
 *    - 0x40: to
 *    - 0x60: amount
 * Revert buffer is memory[0x1c:0xa0]
 */
uint256 constant BadReturnValueFromERC20OnTransfer_error_selector = 0x98891923;
uint256 constant BadReturnValueFromERC20OnTransfer_error_token_ptr = 0x20;
uint256 constant BadReturnValueFromERC20OnTransfer_error_from_ptr = 0x40;
uint256 constant BadReturnValueFromERC20OnTransfer_error_to_ptr = 0x60;
uint256 constant BadReturnValueFromERC20OnTransfer_error_amount_ptr = 0x80;
uint256 constant BadReturnValueFromERC20OnTransfer_error_length = 0x84;

/**
 * @title TokenTransferrer
 * @author 0age
 * @custom:coauthor d1ll0n
 * @custom:coauthor transmissions11
 * @notice TokenTransferrer is a library for performing optimized ERC20, ERC721,
 *         ERC1155, and batch ERC1155 transfers, used by both Seaport as well as
 *         by conduits deployed by the ConduitController. Use great caution when
 *         considering these functions for use in other codebases, as there are
 *         significant side effects and edge cases that need to be thoroughly
 *         understood and carefully addressed.
 */
contract TokenTransferrer is TokenTransferrerErrors {
    /**
     * @dev Internal function to transfer ERC20 tokens from a given originator
     *      to a given recipient. Sufficient approvals must be set on the
     *      contract performing the transfer.
     *
     * @param token      The ERC20 token to transfer.
     * @param from       The originator of the transfer.
     * @param to         The recipient of the transfer.
     * @param amount     The amount to transfer.
     */
    function _performERC20Transfer(address token, address from, address to, uint256 amount) internal {
        // Utilize assembly to perform an optimized ERC20 token transfer.
        assembly {
            // The free memory pointer memory slot will be used when populating
            // call data for the transfer; read the value and restore it later.
            let memPointer := mload(FreeMemoryPointerSlot)

            // Write call data into memory, starting with function selector.
            mstore(ERC20_transferFrom_sig_ptr, ERC20_transferFrom_signature)
            mstore(ERC20_transferFrom_from_ptr, from)
            mstore(ERC20_transferFrom_to_ptr, to)
            mstore(ERC20_transferFrom_amount_ptr, amount)

            // Make call & copy up to 32 bytes of return data to scratch space.
            // Scratch space does not need to be cleared ahead of time, as the
            // subsequent check will ensure that either at least a full word of
            // return data is received (in which case it will be overwritten) or
            // that no data is received (in which case scratch space will be
            // ignored) on a successful call to the given token.
            let callStatus := call(gas(), token, 0, ERC20_transferFrom_sig_ptr, ERC20_transferFrom_length, 0, OneWord)

            // Determine whether transfer was successful using status & result.
            let success :=
                and(
                    // Set success to whether the call reverted, if not check it
                    // either returned exactly 1 (can't just be non-zero data), or
                    // had no return data.
                    or(and(eq(mload(0), 1), gt(returndatasize(), 31)), iszero(returndatasize())),
                    callStatus
                )

            // Handle cases where either the transfer failed or no data was
            // returned. Group these, as most transfers will succeed with data.
            // Equivalent to `or(iszero(success), iszero(returndatasize()))`
            // but after it's inverted for JUMPI this expression is cheaper.
            if iszero(and(success, iszero(iszero(returndatasize())))) {
                // If the token has no code or the transfer failed: Equivalent
                // to `or(iszero(success), iszero(extcodesize(token)))` but
                // after it's inverted for JUMPI this expression is cheaper.
                if iszero(and(iszero(iszero(extcodesize(token))), success)) {
                    // If the transfer failed:
                    if iszero(success) {
                        // If it was due to a revert:
                        if iszero(callStatus) {
                            // If it returned a message, bubble it up as long as
                            // sufficient gas remains to do so:
                            if returndatasize() {
                                // Ensure that sufficient gas is available to
                                // copy returndata while expanding memory where
                                // necessary. Start by computing the word size
                                // of returndata and allocated memory. Round up
                                // to the nearest full word.
                                let returnDataWords := shr(OneWordShift, add(returndatasize(), ThirtyOneBytes))

                                // Note: use the free memory pointer in place of
                                // msize() to work around a Yul warning that
                                // prevents accessing msize directly when the IR
                                // pipeline is activated.
                                let msizeWords := shr(OneWordShift, memPointer)

                                // Next, compute the cost of the returndatacopy.
                                let cost := mul(CostPerWord, returnDataWords)

                                // Then, compute cost of new memory allocation.
                                if gt(returnDataWords, msizeWords) {
                                    cost :=
                                        add(
                                            cost,
                                            add(
                                                mul(sub(returnDataWords, msizeWords), CostPerWord),
                                                shr(
                                                    MemoryExpansionCoefficientShift,
                                                    sub(mul(returnDataWords, returnDataWords), mul(msizeWords, msizeWords))
                                                )
                                            )
                                        )
                                }

                                // Finally, add a small constant and compare to
                                // gas remaining; bubble up the revert data if
                                // enough gas is still available.
                                if lt(add(cost, ExtraGasBuffer), gas()) {
                                    // Copy returndata to memory; overwrite
                                    // existing memory.
                                    returndatacopy(0, 0, returndatasize())

                                    // Revert, specifying memory region with
                                    // copied returndata.
                                    revert(0, returndatasize())
                                }
                            }

                            // Store left-padded selector with push4, mem[28:32]
                            mstore(0, TokenTransferGenericFailure_error_selector)
                            mstore(TokenTransferGenericFailure_error_token_ptr, token)
                            mstore(TokenTransferGenericFailure_error_from_ptr, from)
                            mstore(TokenTransferGenericFailure_error_to_ptr, to)
                            mstore(TokenTransferGenericFailure_err_identifier_ptr, 0)
                            mstore(TokenTransferGenericFailure_error_amount_ptr, amount)

                            // revert(abi.encodeWithSignature(
                            //     "TokenTransferGenericFailure(
                            //         address,address,address,uint256,uint256
                            //     )", token, from, to, identifier, amount
                            // ))
                            revert(Generic_error_selector_offset, TokenTransferGenericFailure_error_length)
                        }

                        // Otherwise revert with a message about the token
                        // returning false or non-compliant return values.

                        // Store left-padded selector with push4, mem[28:32]
                        mstore(0, BadReturnValueFromERC20OnTransfer_error_selector)
                        mstore(BadReturnValueFromERC20OnTransfer_error_token_ptr, token)
                        mstore(BadReturnValueFromERC20OnTransfer_error_from_ptr, from)
                        mstore(BadReturnValueFromERC20OnTransfer_error_to_ptr, to)
                        mstore(BadReturnValueFromERC20OnTransfer_error_amount_ptr, amount)

                        // revert(abi.encodeWithSignature(
                        //     "BadReturnValueFromERC20OnTransfer(
                        //         address,address,address,uint256
                        //     )", token, from, to, amount
                        // ))
                        revert(Generic_error_selector_offset, BadReturnValueFromERC20OnTransfer_error_length)
                    }

                    // Otherwise, revert with error about token not having code:
                    // Store left-padded selector with push4, mem[28:32]
                    mstore(0, NoContract_error_selector)
                    mstore(NoContract_error_account_ptr, token)

                    // revert(abi.encodeWithSignature(
                    //      "NoContract(address)", account
                    // ))
                    revert(Generic_error_selector_offset, NoContract_error_length)
                }

                // Otherwise, the token just returned no data despite the call
                // having succeeded; no need to optimize for this as it's not
                // technically ERC20 compliant.
            }

            // Restore the original free memory pointer.
            mstore(FreeMemoryPointerSlot, memPointer)

            // Restore the zero slot to zero.
            mstore(ZeroSlot, 0)
        }
    }

    /**
     * @dev Internal function to transfer an ERC721 token from a given
     *      originator to a given recipient. Sufficient approvals must be set on
     *      the contract performing the transfer. Note that this function does
     *      not check whether the receiver can accept the ERC721 token (i.e. it
     *      does not use `safeTransferFrom`).
     *
     * @param token      The ERC721 token to transfer.
     * @param from       The originator of the transfer.
     * @param to         The recipient of the transfer.
     * @param identifier The tokenId to transfer.
     */
    function _performERC721Transfer(address token, address from, address to, uint256 identifier) internal {
        // Utilize assembly to perform an optimized ERC721 token transfer.
        assembly {
            // If the token has no code, revert.
            if iszero(extcodesize(token)) {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, NoContract_error_selector)
                mstore(NoContract_error_account_ptr, token)

                // revert(abi.encodeWithSignature(
                //     "NoContract(address)", account
                // ))
                revert(Generic_error_selector_offset, NoContract_error_length)
            }

            // The free memory pointer memory slot will be used when populating
            // call data for the transfer; read the value and restore it later.
            let memPointer := mload(FreeMemoryPointerSlot)

            // Write call data to memory starting with function selector.
            mstore(ERC721_transferFrom_sig_ptr, ERC721_transferFrom_signature)
            mstore(ERC721_transferFrom_from_ptr, from)
            mstore(ERC721_transferFrom_to_ptr, to)
            mstore(ERC721_transferFrom_id_ptr, identifier)

            // Perform the call, ignoring return data.
            let success := call(gas(), token, 0, ERC721_transferFrom_sig_ptr, ERC721_transferFrom_length, 0, 0)

            // If the transfer reverted:
            if iszero(success) {
                // If it returned a message, bubble it up as long as sufficient
                // gas remains to do so:
                if returndatasize() {
                    // Ensure that sufficient gas is available to copy
                    // returndata while expanding memory where necessary. Start
                    // by computing word size of returndata & allocated memory.
                    // Round up to the nearest full word.
                    let returnDataWords := shr(OneWordShift, add(returndatasize(), ThirtyOneBytes))

                    // Note: use the free memory pointer in place of msize() to
                    // work around a Yul warning that prevents accessing msize
                    // directly when the IR pipeline is activated.
                    let msizeWords := shr(OneWordShift, memPointer)

                    // Next, compute the cost of the returndatacopy.
                    let cost := mul(CostPerWord, returnDataWords)

                    // Then, compute cost of new memory allocation.
                    if gt(returnDataWords, msizeWords) {
                        cost :=
                            add(
                                cost,
                                add(
                                    mul(sub(returnDataWords, msizeWords), CostPerWord),
                                    shr(
                                        MemoryExpansionCoefficientShift,
                                        sub(mul(returnDataWords, returnDataWords), mul(msizeWords, msizeWords))
                                    )
                                )
                            )
                    }

                    // Finally, add a small constant and compare to gas
                    // remaining; bubble up the revert data if enough gas is
                    // still available.
                    if lt(add(cost, ExtraGasBuffer), gas()) {
                        // Copy returndata to memory; overwrite existing memory.
                        returndatacopy(0, 0, returndatasize())

                        // Revert, giving memory region with copied returndata.
                        revert(0, returndatasize())
                    }
                }

                // Otherwise revert with a generic error message.
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, TokenTransferGenericFailure_error_selector)
                mstore(TokenTransferGenericFailure_error_token_ptr, token)
                mstore(TokenTransferGenericFailure_error_from_ptr, from)
                mstore(TokenTransferGenericFailure_error_to_ptr, to)
                mstore(TokenTransferGenericFailure_error_identifier_ptr, identifier)
                mstore(TokenTransferGenericFailure_error_amount_ptr, 1)

                // revert(abi.encodeWithSignature(
                //     "TokenTransferGenericFailure(
                //         address,address,address,uint256,uint256
                //     )", token, from, to, identifier, amount
                // ))
                revert(Generic_error_selector_offset, TokenTransferGenericFailure_error_length)
            }

            // Restore the original free memory pointer.
            mstore(FreeMemoryPointerSlot, memPointer)

            // Restore the zero slot to zero.
            mstore(ZeroSlot, 0)
        }
    }

    /**
     * @dev Internal function to transfer ERC1155 tokens from a given
     *      originator to a given recipient. Sufficient approvals must be set on
     *      the contract performing the transfer and contract recipients must
     *      implement the ERC1155TokenReceiver interface to indicate that they
     *      are willing to accept the transfer.
     *
     * @param token      The ERC1155 token to transfer.
     * @param from       The originator of the transfer.
     * @param to         The recipient of the transfer.
     * @param identifier The id to transfer.
     * @param amount     The amount to transfer.
     */
    function _performERC1155Transfer(address token, address from, address to, uint256 identifier, uint256 amount)
        internal
    {
        // Utilize assembly to perform an optimized ERC1155 token transfer.
        assembly {
            // If the token has no code, revert.
            if iszero(extcodesize(token)) {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, NoContract_error_selector)
                mstore(NoContract_error_account_ptr, token)

                // revert(abi.encodeWithSignature(
                //     "NoContract(address)", account
                // ))
                revert(Generic_error_selector_offset, NoContract_error_length)
            }

            // The following memory slots will be used when populating call data
            // for the transfer; read the values and restore them later.
            let memPointer := mload(FreeMemoryPointerSlot)
            let slot0x80 := mload(Slot0x80)
            let slot0xA0 := mload(Slot0xA0)
            let slot0xC0 := mload(Slot0xC0)

            // Write call data into memory, beginning with function selector.
            mstore(ERC1155_safeTransferFrom_sig_ptr, ERC1155_safeTransferFrom_signature)
            mstore(ERC1155_safeTransferFrom_from_ptr, from)
            mstore(ERC1155_safeTransferFrom_to_ptr, to)
            mstore(ERC1155_safeTransferFrom_id_ptr, identifier)
            mstore(ERC1155_safeTransferFrom_amount_ptr, amount)
            mstore(ERC1155_safeTransferFrom_data_offset_ptr, ERC1155_safeTransferFrom_data_length_offset)
            mstore(ERC1155_safeTransferFrom_data_length_ptr, 0)

            // Perform the call, ignoring return data.
            let success :=
                call(gas(), token, 0, ERC1155_safeTransferFrom_sig_ptr, ERC1155_safeTransferFrom_length, 0, 0)

            // If the transfer reverted:
            if iszero(success) {
                // If it returned a message, bubble it up as long as sufficient
                // gas remains to do so:
                if returndatasize() {
                    // Ensure that sufficient gas is available to copy
                    // returndata while expanding memory where necessary. Start
                    // by computing word size of returndata & allocated memory.
                    // Round up to the nearest full word.
                    let returnDataWords := shr(OneWordShift, add(returndatasize(), ThirtyOneBytes))

                    // Note: use the free memory pointer in place of msize() to
                    // work around a Yul warning that prevents accessing msize
                    // directly when the IR pipeline is activated.
                    let msizeWords := shr(OneWordShift, memPointer)

                    // Next, compute the cost of the returndatacopy.
                    let cost := mul(CostPerWord, returnDataWords)

                    // Then, compute cost of new memory allocation.
                    if gt(returnDataWords, msizeWords) {
                        cost :=
                            add(
                                cost,
                                add(
                                    mul(sub(returnDataWords, msizeWords), CostPerWord),
                                    shr(
                                        MemoryExpansionCoefficientShift,
                                        sub(mul(returnDataWords, returnDataWords), mul(msizeWords, msizeWords))
                                    )
                                )
                            )
                    }

                    // Finally, add a small constant and compare to gas
                    // remaining; bubble up the revert data if enough gas is
                    // still available.
                    if lt(add(cost, ExtraGasBuffer), gas()) {
                        // Copy returndata to memory; overwrite existing memory.
                        returndatacopy(0, 0, returndatasize())

                        // Revert, giving memory region with copied returndata.
                        revert(0, returndatasize())
                    }
                }

                // Otherwise revert with a generic error message.

                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, TokenTransferGenericFailure_error_selector)
                mstore(TokenTransferGenericFailure_error_token_ptr, token)
                mstore(TokenTransferGenericFailure_error_from_ptr, from)
                mstore(TokenTransferGenericFailure_error_to_ptr, to)
                mstore(TokenTransferGenericFailure_error_identifier_ptr, identifier)
                mstore(TokenTransferGenericFailure_error_amount_ptr, amount)

                // revert(abi.encodeWithSignature(
                //     "TokenTransferGenericFailure(
                //         address,address,address,uint256,uint256
                //     )", token, from, to, identifier, amount
                // ))
                revert(Generic_error_selector_offset, TokenTransferGenericFailure_error_length)
            }

            mstore(Slot0x80, slot0x80) // Restore slot 0x80.
            mstore(Slot0xA0, slot0xA0) // Restore slot 0xA0.
            mstore(Slot0xC0, slot0xC0) // Restore slot 0xC0.

            // Restore the original free memory pointer.
            mstore(FreeMemoryPointerSlot, memPointer)

            // Restore the zero slot to zero.
            mstore(ZeroSlot, 0)
        }
    }

    /**
     * @dev Internal function to transfer ERC1155 tokens from a given
     *      originator to a given recipient. Sufficient approvals must be set on
     *      the contract performing the transfer and contract recipients must
     *      implement the ERC1155TokenReceiver interface to indicate that they
     *      are willing to accept the transfer. NOTE: this function is not
     *      memory-safe; it will overwrite existing memory, restore the free
     *      memory pointer to the default value, and overwrite the zero slot.
     *      This function should only be called once memory is no longer
     *      required and when uninitialized arrays are not utilized, and memory
     *      should be considered fully corrupted (aside from the existence of a
     *      default-value free memory pointer) after calling this function.
     *
     * @param batchTransfers The group of 1155 batch transfers to perform.
     */
    function _performERC1155BatchTransfers(ConduitBatch1155Transfer[] calldata batchTransfers) internal {
        // Utilize assembly to perform optimized batch 1155 transfers.
        assembly {
            let len := batchTransfers.length
            // Pointer to first head in the array, which is offset to the struct
            // at each index. This gets incremented after each loop to avoid
            // multiplying by 32 to get the offset for each element.
            let nextElementHeadPtr := batchTransfers.offset

            // Pointer to beginning of the head of the array. This is the
            // reference position each offset references. It's held static to
            // let each loop calculate the data position for an element.
            let arrayHeadPtr := nextElementHeadPtr

            // Write the function selector, which will be reused for each call:
            // safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)
            mstore(ConduitBatch1155Transfer_from_offset, ERC1155_safeBatchTransferFrom_signature)

            // Iterate over each batch transfer.
            for { let i := 0 } lt(i, len) { i := add(i, 1) } {
                // Read the offset to the beginning of the element and add
                // it to pointer to the beginning of the array head to get
                // the absolute position of the element in calldata.
                let elementPtr := add(arrayHeadPtr, calldataload(nextElementHeadPtr))

                // Retrieve the token from calldata.
                let token := calldataload(elementPtr)

                // If the token has no code, revert.
                if iszero(extcodesize(token)) {
                    // Store left-padded selector with push4, mem[28:32]
                    mstore(0, NoContract_error_selector)
                    mstore(NoContract_error_account_ptr, token)

                    // revert(abi.encodeWithSignature(
                    //     "NoContract(address)", account
                    // ))
                    revert(Generic_error_selector_offset, NoContract_error_length)
                }

                // Get the total number of supplied ids.
                let idsLength := calldataload(add(elementPtr, ConduitBatch1155Transfer_ids_length_offset))

                // Determine the expected offset for the amounts array.
                let expectedAmountsOffset :=
                    add(ConduitBatch1155Transfer_amounts_length_baseOffset, shl(OneWordShift, idsLength))

                // Validate struct encoding.
                let invalidEncoding :=
                    iszero(
                        and(
                            // ids.length == amounts.length
                            eq(idsLength, calldataload(add(elementPtr, expectedAmountsOffset))),
                            and(
                                // ids_offset == 0xa0
                                eq(
                                    calldataload(add(elementPtr, ConduitBatch1155Transfer_ids_head_offset)),
                                    ConduitBatch1155Transfer_ids_length_offset
                                ),
                                // amounts_offset == 0xc0 + ids.length*32
                                eq(
                                    calldataload(add(elementPtr, ConduitBatchTransfer_amounts_head_offset)),
                                    expectedAmountsOffset
                                )
                            )
                        )
                    )

                // Revert with an error if the encoding is not valid.
                if invalidEncoding {
                    // Store left-padded selector with push4, mem[28:32]
                    mstore(Invalid1155BatchTransferEncoding_ptr, Invalid1155BatchTransferEncoding_selector)

                    // revert(abi.encodeWithSignature(
                    //     "Invalid1155BatchTransferEncoding()"
                    // ))
                    revert(Invalid1155BatchTransferEncoding_ptr, Invalid1155BatchTransferEncoding_length)
                }

                // Update the offset position for the next loop
                nextElementHeadPtr := add(nextElementHeadPtr, OneWord)

                // Copy the first section of calldata (before dynamic values).
                calldatacopy(
                    BatchTransfer1155Params_ptr,
                    add(elementPtr, ConduitBatch1155Transfer_from_offset),
                    ConduitBatch1155Transfer_usable_head_size
                )

                // Determine size of calldata required for ids and amounts. Note
                // that the size includes both lengths as well as the data.
                let idsAndAmountsSize := add(TwoWords, shl(TwoWordsShift, idsLength))

                // Update the offset for the data array in memory.
                mstore(
                    BatchTransfer1155Params_data_head_ptr,
                    add(BatchTransfer1155Params_ids_length_offset, idsAndAmountsSize)
                )

                // Set the length of the data array in memory to zero.
                mstore(add(BatchTransfer1155Params_data_length_basePtr, idsAndAmountsSize), 0)

                // Determine the total calldata size for the call to transfer.
                let transferDataSize := add(BatchTransfer1155Params_calldata_baseSize, idsAndAmountsSize)

                // Copy second section of calldata (including dynamic values).
                calldatacopy(
                    BatchTransfer1155Params_ids_length_ptr,
                    add(elementPtr, ConduitBatch1155Transfer_ids_length_offset),
                    idsAndAmountsSize
                )

                // Perform the call to transfer 1155 tokens.
                let success :=
                    call(
                        gas(),
                        token,
                        0,
                        ConduitBatch1155Transfer_from_offset, // Data portion start.
                        transferDataSize, // Location of the length of callData.
                        0,
                        0
                    )

                // If the transfer reverted:
                if iszero(success) {
                    // If it returned a message, bubble it up as long as
                    // sufficient gas remains to do so:
                    if returndatasize() {
                        // Ensure that sufficient gas is available to copy
                        // returndata while expanding memory where necessary.
                        // Start by computing word size of returndata and
                        // allocated memory. Round up to the nearest full word.
                        let returnDataWords := shr(OneWordShift, add(returndatasize(), ThirtyOneBytes))

                        // Note: use transferDataSize in place of msize() to
                        // work around a Yul warning that prevents accessing
                        // msize directly when the IR pipeline is activated.
                        // The free memory pointer is not used here because
                        // this function does almost all memory management
                        // manually and does not update it, and transferDataSize
                        // should be the largest memory value used (unless a
                        // previous batch was larger).
                        let msizeWords := shr(OneWordShift, transferDataSize)

                        // Next, compute the cost of the returndatacopy.
                        let cost := mul(CostPerWord, returnDataWords)

                        // Then, compute cost of new memory allocation.
                        if gt(returnDataWords, msizeWords) {
                            cost :=
                                add(
                                    cost,
                                    add(
                                        mul(sub(returnDataWords, msizeWords), CostPerWord),
                                        shr(
                                            MemoryExpansionCoefficientShift,
                                            sub(mul(returnDataWords, returnDataWords), mul(msizeWords, msizeWords))
                                        )
                                    )
                                )
                        }

                        // Finally, add a small constant and compare to gas
                        // remaining; bubble up the revert data if enough gas is
                        // still available.
                        if lt(add(cost, ExtraGasBuffer), gas()) {
                            // Copy returndata to memory; overwrite existing.
                            returndatacopy(0, 0, returndatasize())

                            // Revert with memory region containing returndata.
                            revert(0, returndatasize())
                        }
                    }

                    // Set the error signature.
                    mstore(0, ERC1155BatchTransferGenericFailure_error_signature)

                    // Write the token.
                    mstore(ERC1155BatchTransferGenericFailure_token_ptr, token)

                    // Increase the offset to ids by 32.
                    mstore(BatchTransfer1155Params_ids_head_ptr, ERC1155BatchTransferGenericFailure_ids_offset)

                    // Increase the offset to amounts by 32.
                    mstore(
                        BatchTransfer1155Params_amounts_head_ptr,
                        add(OneWord, mload(BatchTransfer1155Params_amounts_head_ptr))
                    )

                    // Return modified region. The total size stays the same as
                    // `token` uses the same number of bytes as `data.length`.
                    revert(0, transferDataSize)
                }
            }

            // Reset the free memory pointer to the default value; memory must
            // be assumed to be dirtied and not reused from this point forward.
            // Also note that the zero slot is not reset to zero, meaning empty
            // arrays cannot be safely created or utilized until it is restored.
            mstore(FreeMemoryPointerSlot, DefaultFreeMemoryPointer)
        }
    }
}

/**
 * @title Executor
 * @author 0age
 * @notice Executor contains functions related to processing executions (i.e.
 *         transferring items, either directly or via conduits).
 */
contract Executor is Verifiers, TokenTransferrer {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) Verifiers(conduitController) {}

    /**
     * @dev Internal function to transfer a given item, either directly or via
     *      a corresponding conduit.
     *
     * @param item        The item to transfer, including an amount and a
     *                    recipient.
     * @param from        The account supplying the item.
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _transfer(ReceivedItem memory item, address from, bytes32 conduitKey, bytes memory accumulator) internal {
        // If the item type indicates Ether or a native token...
        if (item.itemType == ItemType.NATIVE) {
            // Ensure neither the token nor the identifier parameters are set.
            if ((uint160(item.token) | item.identifier) != 0) {
                _revertUnusedItemParameters();
            }

            // transfer the native tokens to the recipient.
            _transferNativeTokens(item.recipient, item.amount);
        } else if (item.itemType == ItemType.ERC20) {
            // Ensure that no identifier is supplied.
            if (item.identifier != 0) {
                _revertUnusedItemParameters();
            }

            // Transfer ERC20 tokens from the source to the recipient.
            _transferERC20(item.token, from, item.recipient, item.amount, conduitKey, accumulator);
        } else if (item.itemType == ItemType.ERC721) {
            // Transfer ERC721 token from the source to the recipient.
            _transferERC721(item.token, from, item.recipient, item.identifier, item.amount, conduitKey, accumulator);
        } else {
            // Transfer ERC1155 token from the source to the recipient.
            _transferERC1155(item.token, from, item.recipient, item.identifier, item.amount, conduitKey, accumulator);
        }
    }

    /**
     * @dev Internal function to transfer Ether or other native tokens to a
     *      given recipient.
     *
     * @param to     The recipient of the transfer.
     * @param amount The amount to transfer.
     */
    function _transferNativeTokens(address payable to, uint256 amount) internal {
        // Ensure that the supplied amount is non-zero.
        _assertNonZeroAmount(amount);

        // Declare a variable indicating whether the call was successful or not.
        bool success;

        assembly {
            // Transfer the native token and store if it succeeded or not.
            success := call(gas(), to, amount, 0, 0, 0, 0)
        }

        // If the call fails...
        if (!success) {
            // Revert and pass the revert reason along if one was returned.
            _revertWithReasonIfOneIsReturned();

            // Otherwise, revert with a generic error message.
            assembly {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, NativeTokenTransferGenericFailure_error_selector)

                // Write `to` and `amount` arguments.
                mstore(NativeTokenTransferGenericFailure_error_account_ptr, to)
                mstore(NativeTokenTransferGenericFailure_error_amount_ptr, amount)

                // revert(abi.encodeWithSignature(
                //     "NativeTokenTransferGenericFailure(address,uint256)",
                //     to,
                //     amount
                // ))
                revert(Error_selector_offset, NativeTokenTransferGenericFailure_error_length)
            }
        }
    }

    /**
     * @dev Internal function to transfer ERC20 tokens from a given originator
     *      to a given recipient using a given conduit if applicable. Sufficient
     *      approvals must be set on this contract or on a respective conduit.
     *
     * @param token       The ERC20 token to transfer.
     * @param from        The originator of the transfer.
     * @param to          The recipient of the transfer.
     * @param amount      The amount to transfer.
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _transferERC20(
        address token,
        address from,
        address to,
        uint256 amount,
        bytes32 conduitKey,
        bytes memory accumulator
    ) internal {
        // Ensure that the supplied amount is non-zero.
        _assertNonZeroAmount(amount);

        // Trigger accumulated transfers if the conduits differ.
        _triggerIfArmedAndNotAccumulatable(accumulator, conduitKey);

        // If no conduit has been specified...
        if (conduitKey == bytes32(0)) {
            // Perform the token transfer directly.
            _performERC20Transfer(token, from, to, amount);
        } else {
            // Insert the call to the conduit into the accumulator.
            _insert(conduitKey, accumulator, ConduitItemType.ERC20, token, from, to, uint256(0), amount);
        }
    }

    /**
     * @dev Internal function to transfer a single ERC721 token from a given
     *      originator to a given recipient. Sufficient approvals must be set,
     *      either on the respective conduit or on this contract itself.
     *
     * @param token       The ERC721 token to transfer.
     * @param from        The originator of the transfer.
     * @param to          The recipient of the transfer.
     * @param identifier  The tokenId to transfer.
     * @param amount      The amount to transfer (must be 1 for ERC721).
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _transferERC721(
        address token,
        address from,
        address to,
        uint256 identifier,
        uint256 amount,
        bytes32 conduitKey,
        bytes memory accumulator
    ) internal {
        // Trigger accumulated transfers if the conduits differ.
        _triggerIfArmedAndNotAccumulatable(accumulator, conduitKey);

        // If no conduit has been specified...
        if (conduitKey == bytes32(0)) {
            // Ensure that exactly one 721 item is being transferred.
            if (amount != 1) {
                _revertInvalidERC721TransferAmount(amount);
            }

            // Perform transfer via the token contract directly.
            _performERC721Transfer(token, from, to, identifier);
        } else {
            // Insert the call to the conduit into the accumulator.
            _insert(conduitKey, accumulator, ConduitItemType.ERC721, token, from, to, identifier, amount);
        }
    }

    /**
     * @dev Internal function to transfer ERC1155 tokens from a given originator
     *      to a given recipient. Sufficient approvals must be set, either on
     *      the respective conduit or on this contract itself.
     *
     * @param token       The ERC1155 token to transfer.
     * @param from        The originator of the transfer.
     * @param to          The recipient of the transfer.
     * @param identifier  The id to transfer.
     * @param amount      The amount to transfer.
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _transferERC1155(
        address token,
        address from,
        address to,
        uint256 identifier,
        uint256 amount,
        bytes32 conduitKey,
        bytes memory accumulator
    ) internal {
        // Ensure that the supplied amount is non-zero.
        _assertNonZeroAmount(amount);

        // Trigger accumulated transfers if the conduits differ.
        _triggerIfArmedAndNotAccumulatable(accumulator, conduitKey);

        // If no conduit has been specified...
        if (conduitKey == bytes32(0)) {
            // Perform transfer via the token contract directly.
            _performERC1155Transfer(token, from, to, identifier, amount);
        } else {
            // Insert the call to the conduit into the accumulator.
            _insert(conduitKey, accumulator, ConduitItemType.ERC1155, token, from, to, identifier, amount);
        }
    }

    /**
     * @dev Internal function to trigger a call to the conduit currently held by
     *      the accumulator if the accumulator contains item transfers (i.e. it
     *      is "armed") and the supplied conduit key does not match the key held
     *      by the accumulator.
     *
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     */
    function _triggerIfArmedAndNotAccumulatable(bytes memory accumulator, bytes32 conduitKey) internal {
        // Retrieve the current conduit key from the accumulator.
        bytes32 accumulatorConduitKey = _getAccumulatorConduitKey(accumulator);

        // Perform conduit call if the set key does not match the supplied key.
        if (accumulatorConduitKey != conduitKey) {
            _triggerIfArmed(accumulator);
        }
    }

    /**
     * @dev Internal function to trigger a call to the conduit currently held by
     *      the accumulator if the accumulator contains item transfers (i.e. it
     *      is "armed").
     *
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _triggerIfArmed(bytes memory accumulator) internal {
        // Exit if the accumulator is not "armed".
        if (accumulator.length != AccumulatorArmed) {
            return;
        }

        // Retrieve the current conduit key from the accumulator.
        bytes32 accumulatorConduitKey = _getAccumulatorConduitKey(accumulator);

        // Perform conduit call.
        _trigger(accumulatorConduitKey, accumulator);
    }

    /**
     * @dev Internal function to trigger a call to the conduit corresponding to
     *      a given conduit key, supplying all accumulated item transfers. The
     *      accumulator will be "disarmed" and reset in the process.
     *
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _trigger(bytes32 conduitKey, bytes memory accumulator) internal {
        // Declare variables for offset in memory & size of calldata to conduit.
        uint256 callDataOffset;
        uint256 callDataSize;

        // Call the conduit with all the accumulated transfers.
        assembly {
            // Call begins at third word; the first is length or "armed" status,
            // and the second is the current conduit key.
            callDataOffset := add(accumulator, TwoWords)

            // 68 + items * 192
            callDataSize :=
                add(
                    Accumulator_array_offset_ptr,
                    mul(mload(add(accumulator, Accumulator_array_length_ptr)), Conduit_transferItem_size)
                )
        }

        // Call conduit derived from conduit key & supply accumulated transfers.
        _callConduitUsingOffsets(conduitKey, callDataOffset, callDataSize);

        // Reset accumulator length to signal that it is now "disarmed".
        assembly {
            mstore(accumulator, AccumulatorDisarmed)
        }
    }

    /**
     * @dev Internal function to perform a call to the conduit corresponding to
     *      a given conduit key based on the offset and size of the calldata in
     *      question in memory.
     *
     * @param conduitKey     A bytes32 value indicating what corresponding
     *                       conduit, if any, to source token approvals from.
     *                       The zero hash signifies that no conduit should be
     *                       used, with direct approvals set on this contract.
     * @param callDataOffset The memory pointer where calldata is contained.
     * @param callDataSize   The size of calldata in memory.
     */
    function _callConduitUsingOffsets(bytes32 conduitKey, uint256 callDataOffset, uint256 callDataSize) internal {
        // Derive the address of the conduit using the conduit key.
        address conduit = _deriveConduit(conduitKey);

        bool success;
        bytes4 result;

        // call the conduit.
        assembly {
            // Ensure first word of scratch space is empty.
            mstore(0, 0)

            // Perform call, placing first word of return data in scratch space.
            success := call(gas(), conduit, 0, callDataOffset, callDataSize, 0, OneWord)

            // Take value from scratch space and place it on the stack.
            result := mload(0)
        }

        // If the call failed...
        if (!success) {
            // Pass along whatever revert reason was given by the conduit.
            _revertWithReasonIfOneIsReturned();

            // Otherwise, revert with a generic error.
            _revertInvalidCallToConduit(conduit);
        }

        // Ensure result was extracted and matches EIP-1271 magic value.
        if (result != ConduitInterface.execute.selector) {
            _revertInvalidConduit(conduitKey, conduit);
        }
    }

    /**
     * @dev Internal pure function to retrieve the current conduit key set for
     *      the accumulator.
     *
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     *
     * @return accumulatorConduitKey The conduit key currently set for the
     *                               accumulator.
     */
    function _getAccumulatorConduitKey(bytes memory accumulator)
        internal
        pure
        returns (bytes32 accumulatorConduitKey)
    {
        // Retrieve the current conduit key from the accumulator.
        assembly {
            accumulatorConduitKey := mload(add(accumulator, Accumulator_conduitKey_ptr))
        }
    }

    /**
     * @dev Internal pure function to place an item transfer into an accumulator
     *      that collects a series of transfers to execute against a given
     *      conduit in a single call.
     *
     * @param conduitKey  A bytes32 value indicating what corresponding conduit,
     *                    if any, to source token approvals from. The zero hash
     *                    signifies that no conduit should be used, with direct
     *                    approvals set on this contract.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     * @param itemType    The type of the item to transfer.
     * @param token       The token to transfer.
     * @param from        The originator of the transfer.
     * @param to          The recipient of the transfer.
     * @param identifier  The tokenId to transfer.
     * @param amount      The amount to transfer.
     */
    function _insert(
        bytes32 conduitKey,
        bytes memory accumulator,
        ConduitItemType itemType,
        address token,
        address from,
        address to,
        uint256 identifier,
        uint256 amount
    ) internal pure {
        uint256 elements;
        // "Arm" and prime accumulator if it's not already armed. The sentinel
        // value is held in the length of the accumulator array.
        if (accumulator.length == AccumulatorDisarmed) {
            elements = 1;
            bytes4 selector = ConduitInterface.execute.selector;
            assembly {
                mstore(accumulator, AccumulatorArmed) // "arm" the accumulator.
                mstore(add(accumulator, Accumulator_conduitKey_ptr), conduitKey)
                mstore(add(accumulator, Accumulator_selector_ptr), selector)
                mstore(add(accumulator, Accumulator_array_offset_ptr), Accumulator_array_offset)
                mstore(add(accumulator, Accumulator_array_length_ptr), elements)
            }
        } else {
            // Otherwise, increase the number of elements by one.
            assembly {
                elements := add(mload(add(accumulator, Accumulator_array_length_ptr)), 1)
                mstore(add(accumulator, Accumulator_array_length_ptr), elements)
            }
        }

        // Insert the item.
        assembly {
            let itemPointer :=
                sub(add(accumulator, mul(elements, Conduit_transferItem_size)), Accumulator_itemSizeOffsetDifference)
            mstore(itemPointer, itemType)
            mstore(add(itemPointer, Conduit_transferItem_token_ptr), token)
            mstore(add(itemPointer, Conduit_transferItem_from_ptr), from)
            mstore(add(itemPointer, Conduit_transferItem_to_ptr), to)
            mstore(add(itemPointer, Conduit_transferItem_identifier_ptr), identifier)
            mstore(add(itemPointer, Conduit_transferItem_amount_ptr), amount)
        }
    }
}

/**
 * @title ZoneInteractionErrors
 * @author 0age
 * @notice ZoneInteractionErrors contains errors related to zone interaction.
 */
interface ZoneInteractionErrors {
    /**
     * @dev Revert with an error when attempting to fill an order that specifies
     *      a restricted submitter as its order type when not submitted by
     *      either the offerer or the order's zone or approved as valid by the
     *      zone in question via a call to `isValidOrder`.
     *
     * @param orderHash The order hash for the invalid restricted order.
     */
    error InvalidRestrictedOrder(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to fill a contract order that
     *      fails to generate an order successfully, that does not adhere to the
     *      requirements for minimum spent or maximum received supplied by the
     *      fulfiller, or that fails the post-execution `ratifyOrder` check..
     *
     * @param orderHash The order hash for the invalid contract order.
     */
    error InvalidContractOrder(bytes32 orderHash);
}

/**
 * @title ZoneInteraction
 * @author 0age
 * @notice ZoneInteraction contains logic related to interacting with zones.
 */
contract ZoneInteraction is ConsiderationEncoder, ZoneInteractionErrors, LowLevelHelpers {
    /**
     * @dev Internal function to determine if an order has a restricted order
     *      type and, if so, to ensure that either the zone is the caller or
     *      that a call to `validateOrder` on the zone returns a magic value
     *      indicating that the order is currently valid. Note that contract
     *      orders are not accessible via the basic fulfillment method.
     *
     * @param orderHash  The hash of the order.
     * @param orderType  The order type.
     * @param parameters The parameters of the basic order.
     */
    function _assertRestrictedBasicOrderValidity(
        bytes32 orderHash,
        OrderType orderType,
        BasicOrderParameters calldata parameters
    ) internal {
        // Order type 2-3 require zone be caller or zone to approve.
        // Note that in cases where fulfiller == zone, the restricted order
        // validation will be skipped.
        if (_isRestrictedAndCallerNotZone(orderType, parameters.zone)) {
            // Encode the `validateOrder` call in memory.
            (MemoryPointer callData, uint256 size) = _encodeValidateBasicOrder(orderHash, parameters);

            // Perform `validateOrder` call and ensure magic value was returned.
            _callAndCheckStatus(parameters.zone, orderHash, callData, size, InvalidRestrictedOrder_error_selector);
        }
    }

    /**
     * @dev Internal function to determine the post-execution validity of
     *      restricted and contract orders. Restricted orders where the caller
     *      is not the zone must successfully call `validateOrder` with the
     *      correct magic value returned. Contract orders must successfully call
     *      `ratifyOrder` with the correct magic value returned.
     *
     * @param advancedOrder The advanced order in question.
     * @param orderHashes   The order hashes of each order included as part of
     *                      the current fulfillment.
     * @param orderHash     The hash of the order.
     */
    function _assertRestrictedAdvancedOrderValidity(
        AdvancedOrder memory advancedOrder,
        bytes32[] memory orderHashes,
        bytes32 orderHash
    ) internal {
        // Declare variables that will be assigned based on the order type.
        address target;
        uint256 errorSelector;
        MemoryPointer callData;
        uint256 size;

        // Retrieve the parameters of the order in question.
        OrderParameters memory parameters = advancedOrder.parameters;

        // OrderType 2-3 require zone to be caller or approve via validateOrder.
        if (_isRestrictedAndCallerNotZone(parameters.orderType, parameters.zone)) {
            // Encode the `validateOrder` call in memory.
            (callData, size) = _encodeValidateOrder(orderHash, parameters, advancedOrder.extraData, orderHashes);

            // Set the target to the zone.
            target = (parameters.toMemoryPointer().offset(OrderParameters_zone_offset).readAddress());

            // Set the restricted-order-specific error selector.
            errorSelector = InvalidRestrictedOrder_error_selector;
        } else if (parameters.orderType == OrderType.CONTRACT) {
            // Set the target to the offerer (note the offerer has no offset).
            target = parameters.toMemoryPointer().readAddress();

            // Shift the target 96 bits to the left.
            uint256 shiftedOfferer;
            assembly {
                shiftedOfferer := shl(ContractOrder_orderHash_offerer_shift, target)
            }

            // Encode the `ratifyOrder` call in memory.
            (callData, size) =
                _encodeRatifyOrder(orderHash, parameters, advancedOrder.extraData, orderHashes, shiftedOfferer);

            // Set the contract-order-specific error selector.
            errorSelector = InvalidContractOrder_error_selector;
        } else {
            return;
        }

        // Perform call and ensure a corresponding magic value was returned.
        _callAndCheckStatus(target, orderHash, callData, size, errorSelector);
    }

    /**
     * @dev Determines whether the specified order type is restricted and the
     *      caller is not the specified zone.
     *
     * @param orderType     The type of the order to check.
     * @param zone          The address of the zone to check against.
     *
     * @return mustValidate True if the order type is restricted and the caller
     *                      is not the specified zone, false otherwise.
     */
    function _isRestrictedAndCallerNotZone(OrderType orderType, address zone)
        internal
        view
        returns (bool mustValidate)
    {
        assembly {
            mustValidate :=
                and(
                    // Note that this check requires that there are no order types
                    // beyond the current set (0-4).  It will need to be modified if
                    // more order types are added.
                    and(lt(orderType, 4), gt(orderType, 1)),
                    iszero(eq(caller(), zone))
                )
        }
    }

    /**
     * @dev Calls the specified target with the given data and checks the status
     *      of the call. Revert reasons will be "bubbled up" if one is returned,
     *      otherwise reverting calls will throw a generic error based on the
     *      supplied error handler.
     *
     * @param target        The address of the contract to call.
     * @param orderHash     The hash of the order associated with the call.
     * @param callData      The data to pass to the contract call.
     * @param size          The size of calldata.
     * @param errorSelector The error handling function to call if the call
     *                      fails or the magic value does not match.
     */
    function _callAndCheckStatus(
        address target,
        bytes32 orderHash,
        MemoryPointer callData,
        uint256 size,
        uint256 errorSelector
    ) internal {
        bool success;
        bool magicMatch;
        assembly {
            // Get magic value from the selector at start of provided calldata.
            let magic := and(mload(callData), MaskOverFirstFourBytes)

            // Clear the start of scratch space.
            mstore(0, 0)

            // Perform call, placing result in the first word of scratch space.
            success := call(gas(), target, 0, callData, size, 0, OneWord)

            // Determine if returned magic value matches the calldata selector.
            magicMatch := eq(magic, mload(0))
        }

        // Revert if the call was not successful.
        if (!success) {
            // Revert and pass reason along if one was returned.
            _revertWithReasonIfOneIsReturned();

            // If no reason was returned, revert with supplied error selector.
            assembly {
                mstore(0, errorSelector)
                mstore(InvalidRestrictedOrder_error_orderHash_ptr, orderHash)
                // revert(abi.encodeWithSelector(
                //     "InvalidRestrictedOrder(bytes32)",
                //     orderHash
                // ))
                revert(Error_selector_offset, InvalidRestrictedOrder_error_length)
            }
        }

        // Revert if the correct magic value was not returned.
        if (!magicMatch) {
            // Revert with a generic error message.
            assembly {
                mstore(0, errorSelector)
                mstore(InvalidRestrictedOrder_error_orderHash_ptr, orderHash)

                // revert(abi.encodeWithSelector(
                //     "InvalidRestrictedOrder(bytes32)",
                //     orderHash
                // ))
                revert(Error_selector_offset, InvalidRestrictedOrder_error_length)
            }
        }
    }
}

/**
 * @title OrderValidator
 * @author 0age
 * @notice OrderValidator contains functionality related to validating orders
 *         and updating their status.
 */
contract OrderValidator is Executor, ZoneInteraction {
    // Track status of each order (validated, cancelled, and fraction filled).
    mapping(bytes32 => OrderStatus) private _orderStatus;

    // Track nonces for contract offerers.
    mapping(address => uint256) internal _contractNonces;

    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) Executor(conduitController) {}

    /**
     * @dev Internal function to verify and update the status of a basic order.
     *      Note that this function may only be safely called as part of basic
     *      orders, as it assumes a specific calldata encoding structure that
     *      must first be validated.
     *
     * @param orderHash The hash of the order.
     * @param signature A signature from the offerer indicating that the order
     *                  has been approved.
     */
    function _validateBasicOrderAndUpdateStatus(bytes32 orderHash, bytes calldata signature) internal {
        // Retrieve offerer directly using fixed calldata offset based on strict
        // basic parameter encoding.
        address offerer;
        assembly {
            offerer := calldataload(BasicOrder_offerer_cdPtr)
        }

        // Retrieve the order status for the given order hash.
        OrderStatus storage orderStatus = _orderStatus[orderHash];

        // Ensure order is fillable and is not cancelled.
        _verifyOrderStatus(
            orderHash,
            orderStatus,
            true, // Only allow unused orders when fulfilling basic orders.
            true // Signifies to revert if the order is invalid.
        );

        // If the order is not already validated, verify the supplied signature.
        if (!orderStatus.isValidated) {
            _verifySignature(offerer, orderHash, signature);
        }

        // Update order status as fully filled, packing struct values.
        orderStatus.isValidated = true;
        orderStatus.isCancelled = false;
        orderStatus.numerator = 1;
        orderStatus.denominator = 1;
    }

    /**
     * @dev Internal function to validate an order, determine what portion to
     *      fill, and update its status. The desired fill amount is supplied as
     *      a fraction, as is the returned amount to fill.
     *
     * @param advancedOrder     The order to fulfill as well as the fraction to
     *                          fill. Note that all offer and consideration
     *                          amounts must divide with no remainder in order
     *                          for a partial fill to be valid.
     * @param revertOnInvalid   A boolean indicating whether to revert if the
     *                          order is invalid due to the time or status.
     *
     * @return orderHash      The order hash.
     * @return numerator      A value indicating the portion of the order that
     *                        will be filled.
     * @return denominator    A value indicating the total size of the order.
     */
    function _validateOrderAndUpdateStatus(AdvancedOrder memory advancedOrder, bool revertOnInvalid)
        internal
        returns (bytes32 orderHash, uint256 numerator, uint256 denominator)
    {
        // Retrieve the parameters for the order.
        OrderParameters memory orderParameters = advancedOrder.parameters;

        // Ensure current timestamp falls between order start time and end time.
        if (!_verifyTime(orderParameters.startTime, orderParameters.endTime, revertOnInvalid)) {
            // Assuming an invalid time and no revert, return zeroed out values.
            return (bytes32(0), 0, 0);
        }

        // Read numerator and denominator from memory and place on the stack.
        // Note that overflowed values are masked.
        assembly {
            numerator := and(mload(add(advancedOrder, AdvancedOrder_numerator_offset)), MaxUint120)

            denominator := and(mload(add(advancedOrder, AdvancedOrder_denominator_offset)), MaxUint120)
        }

        // Declare variable for tracking the validity of the supplied fraction.
        bool invalidFraction;

        // If the order is a contract order, return the generated order.
        if (orderParameters.orderType == OrderType.CONTRACT) {
            // Ensure that the numerator and denominator are both equal to 1.
            assembly {
                // (1 ^ nd =/= 0) => (nd =/= 1) => (n =/= 1) || (d =/= 1)
                // It's important that the values are 120-bit masked before
                // multiplication is applied. Otherwise, the last implication
                // above is not correct (mod 2^256).
                invalidFraction := xor(mul(numerator, denominator), 1)
            }

            // Revert if the supplied numerator and denominator are not valid.
            if (invalidFraction) {
                _revertBadFraction();
            }

            // Return the generated order based on the order params and the
            // provided extra data. If revertOnInvalid is true, the function
            // will revert if the input is invalid.
            return _getGeneratedOrder(orderParameters, advancedOrder.extraData, revertOnInvalid);
        }

        // Ensure numerator does not exceed denominator and is not zero.
        assembly {
            invalidFraction := or(gt(numerator, denominator), iszero(numerator))
        }

        // Revert if the supplied numerator and denominator are not valid.
        if (invalidFraction) {
            _revertBadFraction();
        }

        // If attempting partial fill (n < d) check order type & ensure support.
        if (_doesNotSupportPartialFills(orderParameters.orderType, numerator, denominator)) {
            // Revert if partial fill was attempted on an unsupported order.
            _revertPartialFillsNotEnabledForOrder();
        }

        // Retrieve current counter & use it w/ parameters to derive order hash.
        orderHash = _assertConsiderationLengthAndGetOrderHash(orderParameters);

        // Retrieve the order status using the derived order hash.
        OrderStatus storage orderStatus = _orderStatus[orderHash];

        // Ensure order is fillable and is not cancelled.
        if (
            // Allow partially used orders to be filled.
            !_verifyOrderStatus(orderHash, orderStatus, false, revertOnInvalid)
        ) {
            // Assuming an invalid order status and no revert, return zero fill.
            return (orderHash, 0, 0);
        }

        // If the order is not already validated, verify the supplied signature.
        if (!orderStatus.isValidated) {
            _verifySignature(orderParameters.offerer, orderHash, advancedOrder.signature);
        }

        // Utilize assembly to determine the fraction to fill and update status.
        assembly {
            let orderStatusSlot := orderStatus.slot
            // Read filled amount as numerator and denominator and put on stack.
            let filledNumerator := sload(orderStatusSlot)
            let filledDenominator := shr(OrderStatus_filledDenominator_offset, filledNumerator)

            // "Loop" until the appropriate fill fraction has been determined.
            for {} 1 {} {
                // If no portion of the order has been filled yet...
                if iszero(filledDenominator) {
                    // fill the full supplied fraction.
                    filledNumerator := numerator

                    // Exit the "loop" early.
                    break
                }

                // Shift and mask to calculate the current filled numerator.
                filledNumerator := and(shr(OrderStatus_filledNumerator_offset, filledNumerator), MaxUint120)

                // If denominator of 1 supplied, fill entire remaining amount.
                if eq(denominator, 1) {
                    // Set the amount to fill to the remaining amount.
                    numerator := sub(filledDenominator, filledNumerator)

                    // Set the fill size to the current size.
                    denominator := filledDenominator

                    // Set the filled amount to the current size.
                    filledNumerator := filledDenominator

                    // Exit the "loop" early.
                    break
                }

                // If supplied denominator is equal to the current one:
                if eq(denominator, filledDenominator) {
                    // Increment the filled numerator by the new numerator.
                    filledNumerator := add(numerator, filledNumerator)

                    // Once adjusted, if current + supplied numerator exceeds
                    // the denominator:
                    let carry := mul(sub(filledNumerator, denominator), gt(filledNumerator, denominator))

                    // reduce the amount to fill by the excess.
                    numerator := sub(numerator, carry)

                    // Reduce the filled amount by the excess as well.
                    filledNumerator := sub(filledNumerator, carry)

                    // Exit the "loop" early.
                    break
                }

                // Otherwise, if supplied denominator differs from current one:
                // Scale the filled amount up by the supplied size.
                filledNumerator := mul(filledNumerator, denominator)

                // Scale the supplied amount and size up by the current size.
                numerator := mul(numerator, filledDenominator)
                denominator := mul(denominator, filledDenominator)

                // Increment the filled numerator by the new numerator.
                filledNumerator := add(numerator, filledNumerator)

                // Once adjusted, if current + supplied numerator exceeds
                // denominator:
                let carry := mul(sub(filledNumerator, denominator), gt(filledNumerator, denominator))

                // reduce the amount to fill by the excess.
                numerator := sub(numerator, carry)

                // Reduce the filled amount by the excess as well.
                filledNumerator := sub(filledNumerator, carry)

                // Check filledNumerator and denominator for uint120 overflow.
                if or(gt(filledNumerator, MaxUint120), gt(denominator, MaxUint120)) {
                    // Derive greatest common divisor using euclidean algorithm.
                    function gcd(_a, _b) -> out {
                        // "Loop" until only one non-zero value remains.
                        for {} _b {} {
                            // Assign the second value to a temporary variable.
                            let _c := _b

                            // Derive the modulus of the two values.
                            _b := mod(_a, _c)

                            // Set the first value to the temporary value.
                            _a := _c
                        }

                        // Return the remaining non-zero value.
                        out := _a
                    }

                    // Determine the amount to scale down the fill fractions.
                    let scaleDown := gcd(numerator, gcd(filledNumerator, denominator))

                    // Ensure that the divisor is at least one.
                    let safeScaleDown := add(scaleDown, iszero(scaleDown))

                    // Scale all fractional values down by gcd.
                    numerator := div(numerator, safeScaleDown)
                    filledNumerator := div(filledNumerator, safeScaleDown)
                    denominator := div(denominator, safeScaleDown)

                    // Perform the overflow check a second time.
                    if or(gt(filledNumerator, MaxUint120), gt(denominator, MaxUint120)) {
                        // Store the Panic error signature.
                        mstore(0, Panic_error_selector)
                        // Store the arithmetic (0x11) panic code.
                        mstore(Panic_error_code_ptr, Panic_arithmetic)

                        // revert(abi.encodeWithSignature(
                        //     "Panic(uint256)", 0x11
                        // ))
                        revert(Error_selector_offset, Panic_error_length)
                    }
                }

                // Exit the "loop" now that all evaluation is complete.
                break
            }

            // Update order status and fill amount, packing struct values.
            // [denominator: 15 bytes] [numerator: 15 bytes]
            // [isCancelled: 1 byte] [isValidated: 1 byte]
            sstore(
                orderStatusSlot,
                or(
                    OrderStatus_ValidatedAndNotCancelled,
                    or(
                        shl(OrderStatus_filledNumerator_offset, filledNumerator),
                        shl(OrderStatus_filledDenominator_offset, denominator)
                    )
                )
            )
        }
    }

    /**
     * @dev Internal pure function to check the compatibility of two offer
     *      or consideration items for contract orders.  Note that the itemType
     *      and identifier are reset in cases where criteria = 0 (collection-
     *      wide offers), which means that a contract offerer has full latitude
     *      to choose any identifier it wants mid-flight, in contrast to the
     *      normal behavior, where the fulfiller can pick which identifier to
     *      receive by providing a CriteriaResolver.
     *
     * @param originalItem The original offer or consideration item.
     * @param newItem      The new offer or consideration item.
     *
     * @return isInvalid Error buffer indicating if items are incompatible.
     */
    function _compareItems(MemoryPointer originalItem, MemoryPointer newItem)
        internal
        pure
        returns (uint256 isInvalid)
    {
        assembly {
            let itemType := mload(originalItem)
            let identifier := mload(add(originalItem, Common_identifier_offset))

            // Set returned identifier for criteria-based items w/ criteria = 0.
            if and(gt(itemType, 3), iszero(identifier)) {
                // replace item type
                itemType := sub(3, eq(itemType, 4))
                identifier := mload(add(newItem, Common_identifier_offset))
            }

            let originalAmount := mload(add(originalItem, Common_amount_offset))
            let newAmount := mload(add(newItem, Common_amount_offset))

            isInvalid :=
                iszero(
                    and(
                        // originalItem.token == newItem.token &&
                        // originalItem.itemType == newItem.itemType
                        and(
                            eq(mload(add(originalItem, Common_token_offset)), mload(add(newItem, Common_token_offset))),
                            eq(itemType, mload(newItem))
                        ),
                        // originalItem.identifier == newItem.identifier &&
                        // originalItem.startAmount == originalItem.endAmount
                        and(
                            eq(identifier, mload(add(newItem, Common_identifier_offset))),
                            eq(originalAmount, mload(add(originalItem, Common_endAmount_offset)))
                        )
                    )
                )
        }
    }

    /**
     * @dev Internal pure function to check the compatibility of two recipients
     *      on consideration items for contract orders. This check is skipped if
     *      no recipient is originally supplied.
     *
     * @param originalRecipient The original consideration item recipient.
     * @param newRecipient      The new consideration item recipient.
     *
     * @return isInvalid Error buffer indicating if recipients are incompatible.
     */
    function _checkRecipients(address originalRecipient, address newRecipient)
        internal
        pure
        returns (uint256 isInvalid)
    {
        assembly {
            isInvalid := iszero(or(iszero(originalRecipient), eq(newRecipient, originalRecipient)))
        }
    }

    /**
     * @dev Internal function to generate a contract order. When a
     *      collection-wide criteria-based item (criteria = 0) is provided as an
     *      input to a contract order, the contract offerer has full latitude to
     *      choose any identifier it wants mid-flight, which differs from the
     *      usual behavior.  For regular criteria-based orders with
     *      identifierOrCriteria = 0, the fulfiller can pick which identifier to
     *      receive by providing a CriteriaResolver. For contract offers with
     *      identifierOrCriteria = 0, Seaport does not expect a corresponding
     *      CriteriaResolver, and will revert if one is provided.
     *
     * @param orderParameters The parameters for the order.
     * @param context         The context for generating the order.
     * @param revertOnInvalid Whether to revert on invalid input.
     *
     * @return orderHash   The order hash.
     * @return numerator   The numerator.
     * @return denominator The denominator.
     */
    function _getGeneratedOrder(OrderParameters memory orderParameters, bytes memory context, bool revertOnInvalid)
        internal
        returns (bytes32 orderHash, uint256 numerator, uint256 denominator)
    {
        // Ensure that consideration array length is equal to the total original
        // consideration items value.
        if (orderParameters.consideration.length != orderParameters.totalOriginalConsiderationItems) {
            _revertConsiderationLengthNotEqualToTotalOriginal();
        }

        {
            address offerer = orderParameters.offerer;
            bool success;
            (MemoryPointer cdPtr, uint256 size) = _encodeGenerateOrder(orderParameters, context);
            assembly {
                success := call(gas(), offerer, 0, cdPtr, size, 0, 0)
            }

            {
                // Note: overflow impossible; nonce can't increment that high.
                uint256 contractNonce;
                unchecked {
                    // Note: nonce will be incremented even for skipped orders,
                    // and  even if generateOrder's return data does not satisfy
                    // all the constraints. This is the case when errorBuffer
                    // != 0 and revertOnInvalid == false.
                    contractNonce = _contractNonces[offerer]++;
                }

                assembly {
                    // Shift offerer address up 96 bytes and combine with nonce.
                    orderHash := xor(contractNonce, shl(ContractOrder_orderHash_offerer_shift, offerer))
                }
            }

            // Revert or skip if the call to generate the contract order failed.
            if (!success) {
                return _revertOrReturnEmpty(revertOnInvalid, orderHash);
            }
        }

        // From this point onward, do not allow for skipping orders as the
        // contract offerer may have modified state in expectation of any named
        // consideration items being sent to their designated recipients.

        // Decode the returned contract order and/or update the error buffer.
        (uint256 errorBuffer, OfferItem[] memory offer, ConsiderationItem[] memory consideration) =
            _convertGetGeneratedOrderResult(_decodeGenerateOrderReturndata)();

        // Revert if the returndata could not be decoded correctly.
        if (errorBuffer != 0) {
            _revertInvalidContractOrder(orderHash);
        }

        {
            // Designate lengths.
            uint256 originalOfferLength = orderParameters.offer.length;
            uint256 newOfferLength = offer.length;

            // Explicitly specified offer items cannot be removed.
            if (originalOfferLength > newOfferLength) {
                _revertInvalidContractOrder(orderHash);
            }

            // Iterate over each specified offer (e.g. minimumReceived) item.
            for (uint256 i = 0; i < originalOfferLength;) {
                // Retrieve the pointer to the originally supplied item.
                MemoryPointer mPtrOriginal = orderParameters.offer[i].toMemoryPointer();

                // Retrieve the pointer to the newly returned item.
                MemoryPointer mPtrNew = offer[i].toMemoryPointer();

                // Compare the items and update the error buffer accordingly.
                errorBuffer |= _cast(
                    mPtrOriginal.offset(Common_amount_offset).readUint256()
                        > mPtrNew.offset(Common_amount_offset).readUint256()
                ) | _compareItems(mPtrOriginal, mPtrNew);

                // Increment the array (cannot overflow as index starts at 0).
                unchecked {
                    ++i;
                }
            }

            // Assign the returned offer item in place of the original item.
            orderParameters.offer = offer;
        }

        {
            // Designate lengths & memory locations.
            ConsiderationItem[] memory originalConsiderationArray = (orderParameters.consideration);
            uint256 newConsiderationLength = consideration.length;

            // New consideration items cannot be created.
            if (newConsiderationLength > originalConsiderationArray.length) {
                _revertInvalidContractOrder(orderHash);
            }

            // Iterate over returned consideration & do not exceed maximumSpent.
            for (uint256 i = 0; i < newConsiderationLength;) {
                // Retrieve the pointer to the originally supplied item.
                MemoryPointer mPtrOriginal = originalConsiderationArray[i].toMemoryPointer();

                // Retrieve the pointer to the newly returned item.
                MemoryPointer mPtrNew = consideration[i].toMemoryPointer();

                // Compare the items and update the error buffer accordingly
                // and ensure that the recipients are equal when provided.
                errorBuffer |= _cast(
                    mPtrNew.offset(Common_amount_offset).readUint256()
                        > mPtrOriginal.offset(Common_amount_offset).readUint256()
                ) | _compareItems(mPtrOriginal, mPtrNew)
                    | _checkRecipients(
                        mPtrOriginal.offset(ConsiderItem_recipient_offset).readAddress(),
                        mPtrNew.offset(ConsiderItem_recipient_offset).readAddress()
                    );

                // Increment the array (cannot overflow as index starts at 0).
                unchecked {
                    ++i;
                }
            }

            // Assign returned consideration item in place of the original item.
            orderParameters.consideration = consideration;
        }

        // Revert if any item comparison failed.
        if (errorBuffer != 0) {
            _revertInvalidContractOrder(orderHash);
        }

        // Return order hash and full fill amount (numerator & denominator = 1).
        return (orderHash, 1, 1);
    }

    /**
     * @dev Internal function to cancel an arbitrary number of orders. Note that
     *      only the offerer or the zone of a given order may cancel it. Callers
     *      should ensure that the intended order was cancelled by calling
     *      `getOrderStatus` and confirming that `isCancelled` returns `true`.
     *      Also note that contract orders are not cancellable.
     *
     * @param orders The orders to cancel.
     *
     * @return cancelled A boolean indicating whether the supplied orders were
     *                   successfully cancelled.
     */
    function _cancel(OrderComponents[] calldata orders) internal returns (bool cancelled) {
        // Ensure that the reentrancy guard is not currently set.
        _assertNonReentrant();

        // Declare variables outside of the loop.
        OrderStatus storage orderStatus;

        // Declare a variable for tracking invariants in the loop.
        bool anyInvalidCallerOrContractOrder;

        // Skip overflow check as for loop is indexed starting at zero.
        unchecked {
            // Read length of the orders array from memory and place on stack.
            uint256 totalOrders = orders.length;

            // Iterate over each order.
            for (uint256 i = 0; i < totalOrders;) {
                // Retrieve the order.
                OrderComponents calldata order = orders[i];

                address offerer = order.offerer;
                address zone = order.zone;
                OrderType orderType = order.orderType;

                assembly {
                    // If caller is neither the offerer nor zone, or a contract
                    // order is present, flag anyInvalidCallerOrContractOrder.
                    anyInvalidCallerOrContractOrder :=
                        or(
                            anyInvalidCallerOrContractOrder,
                            // orderType == CONTRACT ||
                            // !(caller == offerer || caller == zone)
                            or(eq(orderType, 4), iszero(or(eq(caller(), offerer), eq(caller(), zone))))
                        )
                }

                bytes32 orderHash = _deriveOrderHash(
                    _toOrderParametersReturnType(_decodeOrderComponentsAsOrderParameters)(order.toCalldataPointer()),
                    order.counter
                );

                // Retrieve the order status using the derived order hash.
                orderStatus = _orderStatus[orderHash];

                // Update the order status as not valid and cancelled.
                orderStatus.isValidated = false;
                orderStatus.isCancelled = true;

                // Emit an event signifying that the order has been cancelled.
                emit OrderCancelled(orderHash, offerer, zone);

                // Increment counter inside body of loop for gas efficiency.
                ++i;
            }
        }

        if (anyInvalidCallerOrContractOrder) {
            _revertCannotCancelOrder();
        }

        // Return a boolean indicating that orders were successfully cancelled.
        cancelled = true;
    }

    /**
     * @dev Internal function to validate an arbitrary number of orders, thereby
     *      registering their signatures as valid and allowing the fulfiller to
     *      skip signature verification on fulfillment. Note that validated
     *      orders may still be unfulfillable due to invalid item amounts or
     *      other factors; callers should determine whether validated orders are
     *      fulfillable by simulating the fulfillment call prior to execution.
     *      Also note that anyone can validate a signed order, but only the
     *      offerer can validate an order without supplying a signature.
     *
     * @param orders The orders to validate.
     *
     * @return validated A boolean indicating whether the supplied orders were
     *                   successfully validated.
     */
    function _validate(Order[] memory orders) internal returns (bool validated) {
        // Ensure that the reentrancy guard is not currently set.
        _assertNonReentrant();

        // Declare variables outside of the loop.
        OrderStatus storage orderStatus;
        bytes32 orderHash;
        address offerer;

        // Skip overflow check as for loop is indexed starting at zero.
        unchecked {
            // Read length of the orders array from memory and place on stack.
            uint256 totalOrders = orders.length;

            // Iterate over each order.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Retrieve the order.
                Order memory order = orders[i];

                // Retrieve the order parameters.
                OrderParameters memory orderParameters = order.parameters;

                // Skip contract orders.
                if (orderParameters.orderType == OrderType.CONTRACT) {
                    continue;
                }

                // Move offerer from memory to the stack.
                offerer = orderParameters.offerer;

                // Get current counter & use it w/ params to derive order hash.
                orderHash = _assertConsiderationLengthAndGetOrderHash(orderParameters);

                // Retrieve the order status using the derived order hash.
                orderStatus = _orderStatus[orderHash];

                // Ensure order is fillable and retrieve the filled amount.
                _verifyOrderStatus(
                    orderHash,
                    orderStatus,
                    false, // Signifies that partially filled orders are valid.
                    true // Signifies to revert if the order is invalid.
                );

                // If the order has not already been validated...
                if (!orderStatus.isValidated) {
                    // Ensure that consideration array length is equal to the
                    // total original consideration items value.
                    if (orderParameters.consideration.length != orderParameters.totalOriginalConsiderationItems) {
                        _revertConsiderationLengthNotEqualToTotalOriginal();
                    }

                    // Verify the supplied signature.
                    _verifySignature(offerer, orderHash, order.signature);

                    // Update order status to mark the order as valid.
                    orderStatus.isValidated = true;

                    // Emit an event signifying the order has been validated.
                    emit OrderValidated(orderHash, orderParameters);
                }
            }
        }

        // Return a boolean indicating that orders were successfully validated.
        validated = true;
    }

    /**
     * @dev Internal view function to retrieve the status of a given order by
     *      hash, including whether the order has been cancelled or validated
     *      and the fraction of the order that has been filled.
     *
     * @param orderHash The order hash in question.
     *
     * @return isValidated A boolean indicating whether the order in question
     *                     has been validated (i.e. previously approved or
     *                     partially filled).
     * @return isCancelled A boolean indicating whether the order in question
     *                     has been cancelled.
     * @return totalFilled The total portion of the order that has been filled
     *                     (i.e. the "numerator").
     * @return totalSize   The total size of the order that is either filled or
     *                     unfilled (i.e. the "denominator").
     */
    function _getOrderStatus(bytes32 orderHash)
        internal
        view
        returns (bool isValidated, bool isCancelled, uint256 totalFilled, uint256 totalSize)
    {
        // Retrieve the order status using the order hash.
        OrderStatus storage orderStatus = _orderStatus[orderHash];

        // Return the fields on the order status.
        return (orderStatus.isValidated, orderStatus.isCancelled, orderStatus.numerator, orderStatus.denominator);
    }

    /**
     * @dev Internal pure function to either revert or return an empty tuple
     *      depending on the value of `revertOnInvalid`.
     *
     * @param revertOnInvalid   Whether to revert on invalid input.
     * @param contractOrderHash The contract order hash.
     *
     * @return orderHash   The order hash.
     * @return numerator   The numerator.
     * @return denominator The denominator.
     */
    function _revertOrReturnEmpty(bool revertOnInvalid, bytes32 contractOrderHash)
        internal
        pure
        returns (bytes32 orderHash, uint256 numerator, uint256 denominator)
    {
        if (revertOnInvalid) {
            _revertInvalidContractOrder(contractOrderHash);
        }

        return (contractOrderHash, 0, 0);
    }

    /**
     * @dev Internal pure function to check whether a given order type indicates
     *      that partial fills are not supported (e.g. only "full fills" are
     *      allowed for the order in question).
     *
     * @param orderType   The order type in question.
     * @param numerator   The numerator in question.
     * @param denominator The denominator in question.
     *
     * @return isFullOrder A boolean indicating whether the order type only
     *                     supports full fills.
     */
    function _doesNotSupportPartialFills(OrderType orderType, uint256 numerator, uint256 denominator)
        internal
        pure
        returns (bool isFullOrder)
    {
        // The "full" order types are even, while "partial" order types are odd.
        // Bitwise and by 1 is equivalent to modulo by 2, but 2 gas cheaper. The
        // check is only necessary if numerator is less than denominator.
        assembly {
            // Equivalent to `uint256(orderType) & 1 == 0`.
            isFullOrder := and(lt(numerator, denominator), iszero(and(orderType, 1)))
        }
    }
}

/**
 * @title BasicOrderFulfiller
 * @author 0age
 * @notice BasicOrderFulfiller contains functionality for fulfilling "basic"
 *         orders with minimal overhead. See documentation for details on what
 *         qualifies as a basic order.
 */
contract BasicOrderFulfiller is OrderValidator {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) OrderValidator(conduitController) {}

    /**
     * @dev Internal function to fulfill an order offering an ERC20, ERC721, or
     *      ERC1155 item by supplying Ether (or other native tokens), ERC20
     *      tokens, an ERC721 item, or an ERC1155 item as consideration. Six
     *      permutations are supported: Native token to ERC721, Native token to
     *      ERC1155, ERC20 to ERC721, ERC20 to ERC1155, ERC721 to ERC20, and
     *      ERC1155 to ERC20 (with native tokens supplied as msg.value). For an
     *      order to be eligible for fulfillment via this method, it must
     *      contain a single offer item (though that item may have a greater
     *      amount if the item is not an ERC721). An arbitrary number of
     *      "additional recipients" may also be supplied which will each receive
     *      native tokens or ERC20 items from the fulfiller as consideration.
     *      Refer to the documentation for a more comprehensive summary of how
     *      to utilize this method and what orders are compatible with it.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer and the fulfiller must first approve
     *                   this contract (or their chosen conduit if indicated)
     *                   before any tokens can be transferred. Also note that
     *                   contract recipients of ERC1155 consideration items must
     *                   implement `onERC1155Received` in order to receive those
     *                   items.
     *
     * @return A boolean indicating whether the order has been fulfilled.
     */
    function _validateAndFulfillBasicOrder(BasicOrderParameters calldata parameters) internal returns (bool) {
        // Declare enums for order type & route to extract from basicOrderType.
        BasicOrderRouteType route;
        OrderType orderType;

        // Declare additional recipient item type to derive from the route type.
        ItemType additionalRecipientsItemType;

        bytes32 orderHash;

        // Utilize assembly to extract the order type and the basic order route.
        assembly {
            // Read basicOrderType from calldata.
            let basicOrderType := calldataload(BasicOrder_basicOrderType_cdPtr)

            // Mask all but 2 least-significant bits to derive the order type.
            orderType := and(basicOrderType, 3)

            // Divide basicOrderType by four to derive the route.
            route := shr(2, basicOrderType)

            // If route > 1 additionalRecipient items are ERC20 (1) else native
            // token (0).
            additionalRecipientsItemType := gt(route, 1)
        }

        {
            // Declare temporary variable for enforcing payable status.
            bool correctPayableStatus;

            // Utilize assembly to compare the route to the callvalue.
            assembly {
                // route 0 and 1 are payable, otherwise route is not payable.
                correctPayableStatus := eq(additionalRecipientsItemType, iszero(callvalue()))
            }

            // Revert if msg.value has not been supplied as part of payable
            // routes or has been supplied as part of non-payable routes.
            if (!correctPayableStatus) {
                _revertInvalidMsgValue(msg.value);
            }
        }

        // Declare more arguments that will be derived from route and calldata.
        address additionalRecipientsToken;
        ItemType offeredItemType;
        bool offerTypeIsAdditionalRecipientsType;

        // Declare scope for received item type to manage stack pressure.
        {
            ItemType receivedItemType;

            // Utilize assembly to retrieve function arguments and cast types.
            assembly {
                // Check if offered item type == additional recipient item type.
                offerTypeIsAdditionalRecipientsType := gt(route, 3)

                // If route > 3 additionalRecipientsToken is at 0xc4 else 0x24.
                additionalRecipientsToken :=
                    calldataload(
                        add(
                            BasicOrder_considerationToken_cdPtr,
                            mul(offerTypeIsAdditionalRecipientsType, BasicOrder_common_params_size)
                        )
                    )

                // If route > 2, receivedItemType is route - 2. If route is 2,
                // the receivedItemType is ERC20 (1). Otherwise, it is native
                // token (0).
                receivedItemType := byte(route, BasicOrder_receivedItemByteMap)

                // If route > 3, offeredItemType is ERC20 (1). Route is 2 or 3,
                // offeredItemType = route. Route is 0 or 1, it is route + 2.
                offeredItemType := byte(route, BasicOrder_offeredItemByteMap)
            }

            // Derive & validate order using parameters and update order status.
            orderHash = _prepareBasicFulfillmentFromCalldata(
                parameters,
                orderType,
                receivedItemType,
                additionalRecipientsItemType,
                additionalRecipientsToken,
                offeredItemType
            );
        }

        // Declare conduitKey argument used by transfer functions.
        bytes32 conduitKey;

        // Utilize assembly to derive conduit (if relevant) based on route.
        assembly {
            // use offerer conduit for routes 0-3, fulfiller conduit otherwise.
            conduitKey :=
                calldataload(add(BasicOrder_offererConduit_cdPtr, shl(OneWordShift, offerTypeIsAdditionalRecipientsType)))
        }

        // Transfer tokens based on the route.
        if (additionalRecipientsItemType == ItemType.NATIVE) {
            // Ensure neither consideration token nor identifier are set. Note
            // that dirty upper bits in the consideration token will still cause
            // this error to be thrown.
            assembly {
                if or(
                    calldataload(BasicOrder_considerationToken_cdPtr),
                    calldataload(BasicOrder_considerationIdentifier_cdPtr)
                ) {
                    // Store left-padded selector with push4 (reduces bytecode),
                    // mem[28:32] = selector
                    mstore(0, UnusedItemParameters_error_selector)

                    // revert(abi.encodeWithSignature("UnusedItemParameters()"))
                    revert(Error_selector_offset, UnusedItemParameters_error_length)
                }
            }

            // Transfer the ERC721 or ERC1155 item, bypassing the accumulator.
            _transferIndividual721Or1155Item(offeredItemType, conduitKey);

            // Transfer native to recipients, return excess to caller & wrap up.
            _transferNativeTokensAndFinalize();
        } else {
            // Initialize an accumulator array. From this point forward, no new
            // memory regions can be safely allocated until the accumulator is
            // no longer being utilized, as the accumulator operates in an
            // open-ended fashion from this memory pointer; existing memory may
            // still be accessed and modified, however.
            bytes memory accumulator = new bytes(AccumulatorDisarmed);

            // Choose transfer method for ERC721 or ERC1155 item based on route.
            if (route == BasicOrderRouteType.ERC20_TO_ERC721) {
                // Transfer ERC721 to caller using offerer's conduit preference.
                _transferERC721(
                    parameters.offerToken,
                    parameters.offerer,
                    msg.sender,
                    parameters.offerIdentifier,
                    parameters.offerAmount,
                    conduitKey,
                    accumulator
                );
            } else if (route == BasicOrderRouteType.ERC20_TO_ERC1155) {
                // Transfer ERC1155 to caller with offerer's conduit preference.
                _transferERC1155(
                    parameters.offerToken,
                    parameters.offerer,
                    msg.sender,
                    parameters.offerIdentifier,
                    parameters.offerAmount,
                    conduitKey,
                    accumulator
                );
            } else if (route == BasicOrderRouteType.ERC721_TO_ERC20) {
                // Transfer ERC721 to offerer using caller's conduit preference.
                _transferERC721(
                    parameters.considerationToken,
                    msg.sender,
                    parameters.offerer,
                    parameters.considerationIdentifier,
                    parameters.considerationAmount,
                    conduitKey,
                    accumulator
                );
            } else {
                // route == BasicOrderRouteType.ERC1155_TO_ERC20

                // Transfer ERC1155 to offerer with caller's conduit preference.
                _transferERC1155(
                    parameters.considerationToken,
                    msg.sender,
                    parameters.offerer,
                    parameters.considerationIdentifier,
                    parameters.considerationAmount,
                    conduitKey,
                    accumulator
                );
            }

            // Transfer ERC20 tokens to all recipients and wrap up.
            _transferERC20AndFinalize(offerTypeIsAdditionalRecipientsType, accumulator);

            // Trigger any remaining accumulated transfers via call to conduit.
            _triggerIfArmed(accumulator);
        }

        // Determine whether order is restricted and, if so, that it is valid.
        _assertRestrictedBasicOrderValidity(orderHash, orderType, parameters);

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        return true;
    }

    /**
     * @dev Internal function to prepare fulfillment of a basic order with
     *      manual calldata and memory access. This calculates the order hash,
     *      emits an OrderFulfilled event, and asserts basic order validity.
     *      Note that calldata offsets must be validated as this function
     *      accesses constant calldata pointers for dynamic types that match
     *      default ABI encoding, but valid ABI encoding can use arbitrary
     *      offsets. Checking that the offsets were produced by default encoding
     *      will ensure that other functions using Solidity's calldata accessors
     *      (which calculate pointers from the stored offsets) are reading the
     *      same data as the order hash is derived from. Also note that this
     *      function accesses memory directly.
     *
     * @param parameters                   The parameters of the basic order.
     * @param orderType                    The order type.
     * @param receivedItemType             The item type of the initial
     *                                     consideration item on the order.
     * @param additionalRecipientsItemType The item type of any additional
     *                                     consideration item on the order.
     * @param additionalRecipientsToken    The ERC20 token contract address (if
     *                                     applicable) for any additional
     *                                     consideration item on the order.
     * @param offeredItemType              The item type of the offered item on
     *                                     the order.
     * @return orderHash The calculated order hash.
     */
    function _prepareBasicFulfillmentFromCalldata(
        BasicOrderParameters calldata parameters,
        OrderType orderType,
        ItemType receivedItemType,
        ItemType additionalRecipientsItemType,
        address additionalRecipientsToken,
        ItemType offeredItemType
    ) internal returns (bytes32 orderHash) {
        // Ensure this function cannot be triggered during a reentrant call.
        _setReentrancyGuard(false); // Native tokens rejected during execution.

        // Verify that calldata offsets for all dynamic types were produced by
        // default encoding. This ensures that the constants used for calldata
        // pointers to dynamic types are the same as those calculated by
        // Solidity using their offsets. Also verify that the basic order type
        // is within range.
        _assertValidBasicOrderParameters();

        // Check for invalid time and missing original consideration items.
        // Utilize assembly so that constant calldata pointers can be applied.
        assembly {
            // Ensure current timestamp is between order start time & end time.
            if or(
                gt(calldataload(BasicOrder_startTime_cdPtr), timestamp()),
                iszero(gt(calldataload(BasicOrder_endTime_cdPtr), timestamp()))
            ) {
                // Store left-padded selector with push4 (reduces bytecode),
                // mem[28:32] = selector
                mstore(0, InvalidTime_error_selector)

                // Store arguments.
                mstore(InvalidTime_error_startTime_ptr, calldataload(BasicOrder_startTime_cdPtr))
                mstore(InvalidTime_error_endTime_ptr, calldataload(BasicOrder_endTime_cdPtr))

                // revert(abi.encodeWithSignature(
                //     "InvalidTime(uint256,uint256)",
                //     startTime,
                //     endTime
                // ))
                revert(Error_selector_offset, InvalidTime_error_length)
            }

            // Ensure consideration array length isn't less than total original.
            if lt(
                calldataload(BasicOrder_additionalRecipients_length_cdPtr),
                calldataload(BasicOrder_totalOriginalAdditionalRecipients_cdPtr)
            ) {
                // Store left-padded selector with push4 (reduces bytecode),
                // mem[28:32] = selector
                mstore(0, MissingOriginalConsiderationItems_error_selector)

                // revert(abi.encodeWithSignature(
                //     "MissingOriginalConsiderationItems()"
                // ))
                revert(Error_selector_offset, MissingOriginalConsiderationItems_error_length)
            }
        }

        {
            /**
             * First, handle consideration items. Memory Layout:
             *  0x60: final hash of the array of consideration item hashes
             *  0x80-0x160: reused space for EIP712 hashing of each item
             *   - 0x80: ConsiderationItem EIP-712 typehash (constant)
             *   - 0xa0: itemType
             *   - 0xc0: token
             *   - 0xe0: identifier
             *   - 0x100: startAmount
             *   - 0x120: endAmount
             *   - 0x140: recipient
             *  0x160-END_ARR: array of consideration item hashes
             *   - 0x160: primary consideration item EIP712 hash
             *   - 0x180-END_ARR: additional recipient item EIP712 hashes
             *  END_ARR: beginning of data for OrderFulfilled event
             *   - END_ARR + 0x120: length of ReceivedItem array
             *   - END_ARR + 0x140: beginning of data for first ReceivedItem
             * (Note: END_ARR = 0x180 + RECIPIENTS_LENGTH * 0x20)
             */

            // Load consideration item typehash from runtime and place on stack.
            bytes32 typeHash = _CONSIDERATION_ITEM_TYPEHASH;

            // Utilize assembly to enable reuse of memory regions and use
            // constant pointers when possible.
            assembly {
                /*
                 * 1. Calculate the EIP712 ConsiderationItem hash for the
                 * primary consideration item of the basic order.
                 */

                // Write ConsiderationItem type hash and item type to memory.
                mstore(BasicOrder_considerationItem_typeHash_ptr, typeHash)
                mstore(BasicOrder_considerationItem_itemType_ptr, receivedItemType)

                // Copy calldata region with (token, identifier, amount) from
                // BasicOrderParameters to ConsiderationItem. The
                // considerationAmount is written to startAmount and endAmount
                // as basic orders do not have dynamic amounts.
                calldatacopy(BasicOrder_considerationItem_token_ptr, BasicOrder_considerationToken_cdPtr, ThreeWords)

                // Copy calldata region with considerationAmount and offerer
                // from BasicOrderParameters to endAmount and recipient in
                // ConsiderationItem.
                calldatacopy(BasicOrder_considerationItem_endAmount_ptr, BasicOrder_considerationAmount_cdPtr, TwoWords)

                // Calculate EIP712 ConsiderationItem hash and store it in the
                // array of EIP712 consideration hashes.
                mstore(
                    BasicOrder_considerationHashesArray_ptr,
                    keccak256(BasicOrder_considerationItem_typeHash_ptr, EIP712_ConsiderationItem_size)
                )

                /*
                 * 2. Write a ReceivedItem struct for the primary consideration
                 * item to the consideration array in OrderFulfilled.
                 */

                // Get the length of the additional recipients array.
                let totalAdditionalRecipients := calldataload(BasicOrder_additionalRecipients_length_cdPtr)

                // Calculate pointer to length of OrderFulfilled consideration
                // array.
                let eventConsiderationArrPtr :=
                    add(OrderFulfilled_consideration_length_baseOffset, shl(OneWordShift, totalAdditionalRecipients))

                // Set the length of the consideration array to the number of
                // additional recipients, plus one for the primary consideration
                // item.
                mstore(eventConsiderationArrPtr, add(totalAdditionalRecipients, 1))

                // Overwrite the consideration array pointer so it points to the
                // body of the first element
                eventConsiderationArrPtr := add(eventConsiderationArrPtr, OneWord)

                // Set itemType at start of the ReceivedItem memory region.
                mstore(eventConsiderationArrPtr, receivedItemType)

                // Copy calldata region (token, identifier, amount & recipient)
                // from BasicOrderParameters to ReceivedItem memory.
                calldatacopy(
                    add(eventConsiderationArrPtr, Common_token_offset), BasicOrder_considerationToken_cdPtr, FourWords
                )

                /*
                 * 3. Calculate EIP712 ConsiderationItem hashes for original
                 * additional recipients and add a ReceivedItem for each to the
                 * consideration array in the OrderFulfilled event. The original
                 * additional recipients are all the consideration items signed
                 * by the offerer aside from the primary consideration items of
                 * the order. Uses memory region from 0x80-0x160 as a buffer for
                 * calculating EIP712 ConsiderationItem hashes.
                 */

                // Put pointer to consideration hashes array on the stack.
                // This will be updated as each additional recipient is hashed
                let considerationHashesPtr := BasicOrder_considerationHashesArray_ptr

                // Write item type, token, & identifier for additional recipient
                // to memory region for hashing EIP712 ConsiderationItem; these
                // values will be reused for each recipient.
                mstore(BasicOrder_considerationItem_itemType_ptr, additionalRecipientsItemType)
                mstore(BasicOrder_considerationItem_token_ptr, additionalRecipientsToken)
                mstore(BasicOrder_considerationItem_identifier_ptr, 0)

                // Declare a stack variable where all additional recipients will
                // be combined to guard against providing dirty upper bits.
                let combinedAdditionalRecipients

                // Read length of the additionalRecipients array from calldata
                // and iterate.
                totalAdditionalRecipients := calldataload(BasicOrder_totalOriginalAdditionalRecipients_cdPtr)
                let i := 0
                for {} lt(i, totalAdditionalRecipients) { i := add(i, 1) } {
                    /*
                     * Calculate EIP712 ConsiderationItem hash for recipient.
                     */

                    // Retrieve calldata pointer for additional recipient.
                    let additionalRecipientCdPtr :=
                        add(BasicOrder_additionalRecipients_data_cdPtr, mul(AdditionalRecipient_size, i))

                    // Copy startAmount from calldata to the ConsiderationItem
                    // struct.
                    calldatacopy(BasicOrder_considerationItem_startAmount_ptr, additionalRecipientCdPtr, OneWord)

                    // Copy endAmount and recipient from calldata to the
                    // ConsiderationItem struct.
                    calldatacopy(
                        BasicOrder_considerationItem_endAmount_ptr, additionalRecipientCdPtr, AdditionalRecipient_size
                    )

                    // Include the recipient as part of combined recipients.
                    combinedAdditionalRecipients :=
                        or(combinedAdditionalRecipients, calldataload(add(additionalRecipientCdPtr, OneWord)))

                    // Add 1 word to the pointer as part of each loop to reduce
                    // operations needed to get local offset into the array.
                    considerationHashesPtr := add(considerationHashesPtr, OneWord)

                    // Calculate EIP712 ConsiderationItem hash and store it in
                    // the array of consideration hashes.
                    mstore(
                        considerationHashesPtr,
                        keccak256(BasicOrder_considerationItem_typeHash_ptr, EIP712_ConsiderationItem_size)
                    )

                    /*
                     * Write ReceivedItem to OrderFulfilled data.
                     */

                    // At this point, eventConsiderationArrPtr points to the
                    // beginning of the ReceivedItem struct of the previous
                    // element in the array. Increase it by the size of the
                    // struct to arrive at the pointer for the current element.
                    eventConsiderationArrPtr := add(eventConsiderationArrPtr, ReceivedItem_size)

                    // Write itemType to the ReceivedItem struct.
                    mstore(eventConsiderationArrPtr, additionalRecipientsItemType)

                    // Write token to the next word of the ReceivedItem struct.
                    mstore(add(eventConsiderationArrPtr, OneWord), additionalRecipientsToken)

                    // Copy endAmount & recipient words to ReceivedItem struct.
                    calldatacopy(
                        add(eventConsiderationArrPtr, ReceivedItem_amount_offset), additionalRecipientCdPtr, TwoWords
                    )
                }

                /*
                 * 4. Hash packed array of ConsiderationItem EIP712 hashes:
                 *   `keccak256(abi.encodePacked(receivedItemHashes))`
                 * Note that it is set at 0x60 — all other memory begins at
                 * 0x80. 0x60 is the "zero slot" and will be restored at the end
                 * of the assembly section and before required by the compiler.
                 */
                mstore(
                    receivedItemsHash_ptr,
                    keccak256(
                        BasicOrder_considerationHashesArray_ptr, shl(OneWordShift, add(totalAdditionalRecipients, 1))
                    )
                )

                /*
                 * 5. Add a ReceivedItem for each tip to the consideration array
                 * in the OrderFulfilled event. The tips are all the
                 * consideration items that were not signed by the offerer and
                 * were provided by the fulfiller.
                 */

                // Overwrite length to length of the additionalRecipients array.
                totalAdditionalRecipients := calldataload(BasicOrder_additionalRecipients_length_cdPtr)

                for {} lt(i, totalAdditionalRecipients) { i := add(i, 1) } {
                    // Retrieve calldata pointer for additional recipient.
                    let additionalRecipientCdPtr :=
                        add(BasicOrder_additionalRecipients_data_cdPtr, mul(AdditionalRecipient_size, i))

                    // At this point, eventConsiderationArrPtr points to the
                    // beginning of the ReceivedItem struct of the previous
                    // element in the array. Increase it by the size of the
                    // struct to arrive at the pointer for the current element.
                    eventConsiderationArrPtr := add(eventConsiderationArrPtr, ReceivedItem_size)

                    // Write itemType to the ReceivedItem struct.
                    mstore(eventConsiderationArrPtr, additionalRecipientsItemType)

                    // Write token to the next word of the ReceivedItem struct.
                    mstore(add(eventConsiderationArrPtr, OneWord), additionalRecipientsToken)

                    // Copy endAmount & recipient words to ReceivedItem struct.
                    calldatacopy(
                        add(eventConsiderationArrPtr, ReceivedItem_amount_offset), additionalRecipientCdPtr, TwoWords
                    )

                    // Include the recipient as part of combined recipients.
                    combinedAdditionalRecipients :=
                        or(combinedAdditionalRecipients, calldataload(add(additionalRecipientCdPtr, OneWord)))
                }

                // Ensure no dirty upper bits on combined additional recipients.
                if gt(combinedAdditionalRecipients, MaskOverLastTwentyBytes) {
                    // Store left-padded selector with push4 (reduces bytecode),
                    // mem[28:32] = selector
                    mstore(0, InvalidBasicOrderParameterEncoding_error_selector)

                    // revert(abi.encodeWithSignature(
                    //     "InvalidBasicOrderParameterEncoding()"
                    // ))
                    revert(Error_selector_offset, InvalidBasicOrderParameterEncoding_error_length)
                }
            }
        }

        {
            /**
             * Next, handle offered items. Memory Layout:
             *  EIP712 data for OfferItem
             *   - 0x80:  OfferItem EIP-712 typehash (constant)
             *   - 0xa0:  itemType
             *   - 0xc0:  token
             *   - 0xe0:  identifier (reused for offeredItemsHash)
             *   - 0x100: startAmount
             *   - 0x120: endAmount
             */

            // Place offer item typehash on the stack.
            bytes32 typeHash = _OFFER_ITEM_TYPEHASH;

            // Utilize assembly to enable reuse of memory regions when possible.
            assembly {
                /*
                 * 1. Calculate OfferItem EIP712 hash
                 */

                // Write the OfferItem typeHash to memory.
                mstore(BasicOrder_offerItem_typeHash_ptr, typeHash)

                // Write the OfferItem item type to memory.
                mstore(BasicOrder_offerItem_itemType_ptr, offeredItemType)

                // Copy calldata region with (offerToken, offerIdentifier,
                // offerAmount) from OrderParameters to (token, identifier,
                // startAmount) in OfferItem struct. The offerAmount is written
                // to startAmount and endAmount as basic orders do not have
                // dynamic amounts.
                calldatacopy(BasicOrder_offerItem_token_ptr, BasicOrder_offerToken_cdPtr, ThreeWords)

                // Copy offerAmount from calldata to endAmount in OfferItem
                // struct.
                calldatacopy(BasicOrder_offerItem_endAmount_ptr, BasicOrder_offerAmount_cdPtr, OneWord)

                // Compute EIP712 OfferItem hash, write result to scratch space:
                //   `keccak256(abi.encode(offeredItem))`
                mstore(0, keccak256(BasicOrder_offerItem_typeHash_ptr, EIP712_OfferItem_size))

                /*
                 * 2. Calculate hash of array of EIP712 hashes and write the
                 * result to the corresponding OfferItem struct:
                 *   `keccak256(abi.encodePacked(offerItemHashes))`
                 */
                mstore(BasicOrder_order_offerHashes_ptr, keccak256(0, OneWord))

                /*
                 * 3. Write SpentItem to offer array in OrderFulfilled event.
                 */
                let eventConsiderationArrPtr :=
                    add(
                        OrderFulfilled_offer_length_baseOffset,
                        shl(OneWordShift, calldataload(BasicOrder_additionalRecipients_length_cdPtr))
                    )

                // Set a length of 1 for the offer array.
                mstore(eventConsiderationArrPtr, 1)

                // Write itemType to the SpentItem struct.
                mstore(add(eventConsiderationArrPtr, OneWord), offeredItemType)

                // Copy calldata region with (offerToken, offerIdentifier,
                // offerAmount) from OrderParameters to (token, identifier,
                // amount) in SpentItem struct.
                calldatacopy(
                    add(eventConsiderationArrPtr, AdditionalRecipient_size), BasicOrder_offerToken_cdPtr, ThreeWords
                )
            }
        }

        {
            /**
             * Once consideration items and offer items have been handled,
             * derive the final order hash. Memory Layout:
             *  0x80-0x1c0: EIP712 data for order
             *   - 0x80:   Order EIP-712 typehash (constant)
             *   - 0xa0:   orderParameters.offerer
             *   - 0xc0:   orderParameters.zone
             *   - 0xe0:   keccak256(abi.encodePacked(offerHashes))
             *   - 0x100:  keccak256(abi.encodePacked(considerationHashes))
             *   - 0x120:  orderParameters.basicOrderType (% 4 = orderType)
             *   - 0x140:  orderParameters.startTime
             *   - 0x160:  orderParameters.endTime
             *   - 0x180:  orderParameters.zoneHash
             *   - 0x1a0:  orderParameters.salt
             *   - 0x1c0:  orderParameters.conduitKey
             *   - 0x1e0:  _counters[orderParameters.offerer] (from storage)
             */

            // Read the offerer from calldata and place on the stack.
            address offerer;
            assembly {
                offerer := calldataload(BasicOrder_offerer_cdPtr)
            }

            // Read offerer's current counter from storage and place on stack.
            uint256 counter = _getCounter(offerer);

            // Load order typehash from runtime code and place on stack.
            bytes32 typeHash = _ORDER_TYPEHASH;

            assembly {
                // Set the OrderItem typeHash in memory.
                mstore(BasicOrder_order_typeHash_ptr, typeHash)

                // Copy offerer and zone from OrderParameters in calldata to the
                // Order struct.
                calldatacopy(BasicOrder_order_offerer_ptr, BasicOrder_offerer_cdPtr, TwoWords)

                // Copy receivedItemsHash from zero slot to the Order struct.
                mstore(BasicOrder_order_considerationHashes_ptr, mload(receivedItemsHash_ptr))

                // Write the supplied orderType to the Order struct.
                mstore(BasicOrder_order_orderType_ptr, orderType)

                // Copy startTime, endTime, zoneHash, salt & conduit from
                // calldata to the Order struct.
                calldatacopy(BasicOrder_order_startTime_ptr, BasicOrder_startTime_cdPtr, FiveWords)

                // Write offerer's counter, retrieved from storage, to struct.
                mstore(BasicOrder_order_counter_ptr, counter)

                // Compute the EIP712 Order hash.
                orderHash := keccak256(BasicOrder_order_typeHash_ptr, EIP712_Order_size)
            }
        }

        assembly {
            /**
             * After the order hash has been derived, emit OrderFulfilled event:
             *   event OrderFulfilled(
             *     bytes32 orderHash,
             *     address indexed offerer,
             *     address indexed zone,
             *     address fulfiller,
             *     SpentItem[] offer,
             *       > (itemType, token, id, amount)
             *     ReceivedItem[] consideration
             *       > (itemType, token, id, amount, recipient)
             *   )
             * topic0 - OrderFulfilled event signature
             * topic1 - offerer
             * topic2 - zone
             * data:
             *  - 0x00: orderHash
             *  - 0x20: fulfiller
             *  - 0x40: offer arr ptr (0x80)
             *  - 0x60: consideration arr ptr (0x120)
             *  - 0x80: offer arr len (1)
             *  - 0xa0: offer.itemType
             *  - 0xc0: offer.token
             *  - 0xe0: offer.identifier
             *  - 0x100: offer.amount
             *  - 0x120: 1 + recipients.length
             *  - 0x140: recipient 0
             */

            // Derive pointer to start of OrderFulfilled event data.
            let eventDataPtr :=
                add(
                    OrderFulfilled_baseOffset, shl(OneWordShift, calldataload(BasicOrder_additionalRecipients_length_cdPtr))
                )

            // Write the order hash to the head of the event's data region.
            mstore(eventDataPtr, orderHash)

            // Write the fulfiller (i.e. the caller) next for receiver argument.
            mstore(add(eventDataPtr, OrderFulfilled_fulfiller_offset), caller())

            // Write the SpentItem and ReceivedItem array offsets (constants).
            mstore(
                // SpentItem array offset
                add(eventDataPtr, OrderFulfilled_offer_head_offset),
                OrderFulfilled_offer_body_offset
            )
            mstore(
                // ReceivedItem array offset
                add(eventDataPtr, OrderFulfilled_consideration_head_offset),
                OrderFulfilled_consideration_body_offset
            )

            // Derive total data size including SpentItem and ReceivedItem data.
            // SpentItem portion is already included in the baseSize constant,
            // as there can only be one element in the array.
            let dataSize :=
                add(
                    OrderFulfilled_baseSize,
                    mul(calldataload(BasicOrder_additionalRecipients_length_cdPtr), ReceivedItem_size)
                )

            // Emit OrderFulfilled log with three topics (the event signature
            // as well as the two indexed arguments, the offerer and the zone).
            log3(
                // Supply the pointer for event data in memory.
                eventDataPtr,
                // Supply the size of event data in memory.
                dataSize,
                // Supply the OrderFulfilled event signature.
                OrderFulfilled_selector,
                // Supply the first topic (the offerer).
                calldataload(BasicOrder_offerer_cdPtr),
                // Supply the second topic (the zone).
                calldataload(BasicOrder_zone_cdPtr)
            )

            // Restore the zero slot.
            mstore(ZeroSlot, 0)

            // Update the free memory pointer so that event data is persisted.
            mstore(FreeMemoryPointerSlot, add(eventDataPtr, dataSize))
        }

        // Verify and update the status of the derived order.
        _validateBasicOrderAndUpdateStatus(orderHash, parameters.signature);

        // Return the derived order hash.
        return orderHash;
    }

    /**
     * @dev Internal function to transfer an individual ERC721 or ERC1155 item
     *      from a given originator to a given recipient. The accumulator will
     *      be bypassed, meaning that this function should be utilized in cases
     *      where multiple item transfers can be accumulated into a single
     *      conduit call. Sufficient approvals must be set, either on the
     *      respective conduit or on this contract. Note that this function may
     *      only be safely called as part of basic orders, as it assumes a
     *      specific calldata encoding structure that must first be validated.
     *
     * @param itemType   The type of item to transfer, either ERC721 or ERC1155.
     * @param conduitKey A bytes32 value indicating what corresponding conduit,
     *                   if any, to source token approvals from. The zero hash
     *                   signifies that no conduit should be used, with direct
     *                   approvals set on this contract.
     */
    function _transferIndividual721Or1155Item(ItemType itemType, bytes32 conduitKey) internal {
        // Retrieve token, from, identifier, and amount from calldata using
        // fixed calldata offsets based on strict basic parameter encoding.
        address token;
        address from;
        uint256 identifier;
        uint256 amount;
        assembly {
            token := calldataload(BasicOrder_offerToken_cdPtr)
            from := calldataload(BasicOrder_offerer_cdPtr)
            identifier := calldataload(BasicOrder_offerIdentifier_cdPtr)
            amount := calldataload(BasicOrder_offerAmount_cdPtr)
        }

        // Determine if the transfer is to be performed via a conduit.
        if (conduitKey != bytes32(0)) {
            // Use free memory pointer as calldata offset for the conduit call.
            uint256 callDataOffset;

            // Utilize assembly to place each argument in free memory.
            assembly {
                // Retrieve the free memory pointer and use it as the offset.
                callDataOffset := mload(FreeMemoryPointerSlot)

                // Write ConduitInterface.execute.selector to memory.
                mstore(callDataOffset, Conduit_execute_signature)

                // Write the offset to the ConduitTransfer array in memory.
                mstore(
                    add(callDataOffset, Conduit_execute_ConduitTransfer_offset_ptr), Conduit_execute_ConduitTransfer_ptr
                )

                // Write the length of the ConduitTransfer array to memory.
                mstore(
                    add(callDataOffset, Conduit_execute_ConduitTransfer_length_ptr),
                    Conduit_execute_ConduitTransfer_length
                )

                // Write the item type to memory.
                mstore(add(callDataOffset, Conduit_execute_transferItemType_ptr), itemType)

                // Write the token to memory.
                mstore(add(callDataOffset, Conduit_execute_transferToken_ptr), token)

                // Write the transfer source to memory.
                mstore(add(callDataOffset, Conduit_execute_transferFrom_ptr), from)

                // Write the transfer recipient (the caller) to memory.
                mstore(add(callDataOffset, Conduit_execute_transferTo_ptr), caller())

                // Write the token identifier to memory.
                mstore(add(callDataOffset, Conduit_execute_transferIdentifier_ptr), identifier)

                // Write the transfer amount to memory.
                mstore(add(callDataOffset, Conduit_execute_transferAmount_ptr), amount)
            }

            // Perform the call to the conduit.
            _callConduitUsingOffsets(conduitKey, callDataOffset, OneConduitExecute_size);
        } else {
            // Otherwise, determine whether it is an ERC721 or ERC1155 item.
            if (itemType == ItemType.ERC721) {
                // Ensure that exactly one 721 item is being transferred.
                if (amount != 1) {
                    _revertInvalidERC721TransferAmount(amount);
                }

                // Perform transfer to caller via the token contract directly.
                _performERC721Transfer(token, from, msg.sender, identifier);
            } else {
                // Perform transfer to caller via the token contract directly.
                _performERC1155Transfer(token, from, msg.sender, identifier, amount);
            }
        }
    }

    /**
     * @dev Internal function to transfer Ether (or other native tokens) to a
     *      given recipient as part of basic order fulfillment. Note that
     *      conduits are not utilized for native tokens as the transferred
     *      amount must be provided as msg.value. Also note that this function
     *      may only be safely called as part of basic orders, as it assumes a
     *      specific calldata encoding structure that must first be validated.
     */
    function _transferNativeTokensAndFinalize() internal {
        // Put native token value supplied by the caller on the stack.
        uint256 nativeTokensRemaining = msg.value;

        // Retrieve consideration amount, offerer, and total size of additional
        // recipients data from calldata using fixed offsets and place on stack.
        uint256 amount;
        address payable to;
        uint256 totalAdditionalRecipientsDataSize;
        assembly {
            amount := calldataload(BasicOrder_considerationAmount_cdPtr)
            to := calldataload(BasicOrder_offerer_cdPtr)
            totalAdditionalRecipientsDataSize :=
                shl(AdditionalRecipient_size_shift, calldataload(BasicOrder_additionalRecipients_length_cdPtr))
        }

        uint256 additionalRecipientAmount;
        address payable recipient;

        // Skip overflow check as for loop is indexed starting at zero.
        unchecked {
            // Iterate over additional recipient data by two-word element.
            for (uint256 i = 0; i < totalAdditionalRecipientsDataSize; i += AdditionalRecipient_size) {
                assembly {
                    // Retrieve calldata pointer for additional recipient.
                    let additionalRecipientCdPtr := add(BasicOrder_additionalRecipients_data_cdPtr, i)

                    additionalRecipientAmount := calldataload(additionalRecipientCdPtr)
                    recipient := calldataload(add(OneWord, additionalRecipientCdPtr))
                }

                // Ensure that sufficient native tokens are available.
                if (additionalRecipientAmount > nativeTokensRemaining) {
                    _revertInsufficientNativeTokensSupplied();
                }

                // Reduce native token value available. Skip underflow check as
                // subtracted value is confirmed above as less than remaining.
                nativeTokensRemaining -= additionalRecipientAmount;

                // Transfer native tokens to the additional recipient.
                _transferNativeTokens(recipient, additionalRecipientAmount);
            }
        }

        // Ensure that sufficient native tokens are still available.
        if (amount > nativeTokensRemaining) {
            _revertInsufficientNativeTokensSupplied();
        }

        // Transfer native tokens to the offerer.
        _transferNativeTokens(to, amount);

        // If any native tokens remain after transfers, return to the caller.
        if (nativeTokensRemaining > amount) {
            // Skip underflow check as nativeTokensRemaining > amount.
            unchecked {
                // Transfer remaining native tokens to the caller.
                _transferNativeTokens(payable(msg.sender), nativeTokensRemaining - amount);
            }
        }
    }

    /**
     * @dev Internal function to transfer ERC20 tokens to a given recipient as
     *      part of basic order fulfillment. Note that this function may only be
     *      safely called as part of basic orders, as it assumes a specific
     *      calldata encoding structure that must first be validated. Also note
     *      that basic order parameters are retrieved using fixed offsets, this
     *      requires that strict basic order encoding has already been verified.
     *
     * @param fromOfferer A boolean indicating whether to decrement amount from
     *                    the offered amount.
     * @param accumulator An open-ended array that collects transfers to execute
     *                    against a given conduit in a single call.
     */
    function _transferERC20AndFinalize(bool fromOfferer, bytes memory accumulator) internal {
        // Declare from and to variables determined by fromOfferer value.
        address from;
        address to;

        // Declare token and amount variables determined by fromOfferer value.
        address token;
        uint256 amount;

        // Declare and check identifier variable within an isolated scope.
        {
            // Declare identifier variable determined by fromOfferer value.
            uint256 identifier;

            // Set ERC20 token transfer variables based on fromOfferer boolean.
            if (fromOfferer) {
                // Use offerer as from value, msg.sender as to value, and offer
                // token, identifier, & amount values if token is from offerer.
                assembly {
                    from := calldataload(BasicOrder_offerer_cdPtr)
                    to := caller()
                    token := calldataload(BasicOrder_offerToken_cdPtr)
                    identifier := calldataload(BasicOrder_offerIdentifier_cdPtr)
                    amount := calldataload(BasicOrder_offerAmount_cdPtr)
                }
            } else {
                // Otherwise, use msg.sender as from value, offerer as to value,
                // and consideration token, identifier, and amount values.
                assembly {
                    from := caller()
                    to := calldataload(BasicOrder_offerer_cdPtr)
                    token := calldataload(BasicOrder_considerationToken_cdPtr)
                    identifier := calldataload(BasicOrder_considerationIdentifier_cdPtr)
                    amount := calldataload(BasicOrder_considerationAmount_cdPtr)
                }
            }

            // Ensure that no identifier is supplied.
            if (identifier != 0) {
                _revertUnusedItemParameters();
            }
        }

        // Determine the appropriate conduit to utilize.
        bytes32 conduitKey;

        // Utilize assembly to derive conduit (if relevant) based on route.
        assembly {
            // Use offerer conduit if fromOfferer, fulfiller conduit otherwise.
            conduitKey := calldataload(sub(BasicOrder_fulfillerConduit_cdPtr, shl(OneWordShift, fromOfferer)))
        }

        // Retrieve total size of additional recipients data and place on stack.
        uint256 totalAdditionalRecipientsDataSize;
        assembly {
            totalAdditionalRecipientsDataSize :=
                shl(AdditionalRecipient_size_shift, calldataload(BasicOrder_additionalRecipients_length_cdPtr))
        }

        uint256 additionalRecipientAmount;
        address recipient;

        // Iterate over each additional recipient.
        for (uint256 i = 0; i < totalAdditionalRecipientsDataSize;) {
            assembly {
                // Retrieve calldata pointer for additional recipient.
                let additionalRecipientCdPtr := add(BasicOrder_additionalRecipients_data_cdPtr, i)

                additionalRecipientAmount := calldataload(additionalRecipientCdPtr)
                recipient := calldataload(add(OneWord, additionalRecipientCdPtr))
            }

            // Decrement the amount to transfer to fulfiller if indicated.
            if (fromOfferer) {
                amount -= additionalRecipientAmount;
            }

            // Transfer ERC20 tokens to additional recipient given approval.
            _transferERC20(token, from, recipient, additionalRecipientAmount, conduitKey, accumulator);

            // Skip overflow check as for loop is indexed starting at zero.
            unchecked {
                i += AdditionalRecipient_size;
            }
        }

        // Transfer ERC20 token amount (from account must have proper approval).
        _transferERC20(token, from, to, amount, conduitKey, accumulator);
    }
}

/**
 * @title CriteriaResolutionErrors
 * @author 0age
 * @notice CriteriaResolutionErrors contains all errors related to criteria
 *         resolution.
 */
interface CriteriaResolutionErrors {
    /**
     * @dev Revert with an error when providing a criteria resolver that refers
     *      to an order that has not been supplied.
     *
     * @param side The side of the order that was not supplied.
     */
    error OrderCriteriaResolverOutOfRange(Side side);

    /**
     * @dev Revert with an error if an offer item still has unresolved criteria
     *      after applying all criteria resolvers.
     *
     * @param orderIndex The index of the order that contains the offer item.
     * @param offerIndex The index of the offer item that still has unresolved
     *                   criteria.
     */
    error UnresolvedOfferCriteria(uint256 orderIndex, uint256 offerIndex);

    /**
     * @dev Revert with an error if a consideration item still has unresolved
     *      criteria after applying all criteria resolvers.
     *
     * @param orderIndex         The index of the order that contains the
     *                           consideration item.
     * @param considerationIndex The index of the consideration item that still
     *                           has unresolved criteria.
     */
    error UnresolvedConsiderationCriteria(
        uint256 orderIndex,
        uint256 considerationIndex
    );

    /**
     * @dev Revert with an error when providing a criteria resolver that refers
     *      to an order with an offer item that has not been supplied.
     */
    error OfferCriteriaResolverOutOfRange();

    /**
     * @dev Revert with an error when providing a criteria resolver that refers
     *      to an order with a consideration item that has not been supplied.
     */
    error ConsiderationCriteriaResolverOutOfRange();

    /**
     * @dev Revert with an error when providing a criteria resolver that refers
     *      to an order with an item that does not expect a criteria to be
     *      resolved.
     */
    error CriteriaNotEnabledForItem();

    /**
     * @dev Revert with an error when providing a criteria resolver that
     *      contains an invalid proof with respect to the given item and
     *      chosen identifier.
     */
    error InvalidProof();
}

/**
 * @title CriteriaResolution
 * @author 0age
 * @notice CriteriaResolution contains a collection of pure functions related to
 *         resolving criteria-based items.
 */
contract CriteriaResolution is CriteriaResolutionErrors {
    /**
     * @dev Internal pure function to apply criteria resolvers containing
     *      specific token identifiers and associated proofs to order items.
     *
     * @param advancedOrders     The orders to apply criteria resolvers to.
     * @param criteriaResolvers  An array where each element contains a
     *                           reference to a specific order as well as that
     *                           order's offer or consideration, a token
     *                           identifier, and a proof that the supplied token
     *                           identifier is contained in the order's merkle
     *                           root. Note that a root of zero indicates that
     *                           any transferable token identifier is valid and
     *                           that no proof needs to be supplied.
     */
    function _applyCriteriaResolvers(AdvancedOrder[] memory advancedOrders, CriteriaResolver[] memory criteriaResolvers)
        internal
        pure
    {
        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Retrieve length of criteria resolvers array and place on stack.
            uint256 totalCriteriaResolvers = criteriaResolvers.length;

            // Retrieve length of orders array and place on stack.
            uint256 totalAdvancedOrders = advancedOrders.length;

            // Iterate over each criteria resolver.
            for (uint256 i = 0; i < totalCriteriaResolvers; ++i) {
                // Retrieve the criteria resolver.
                CriteriaResolver memory criteriaResolver = (criteriaResolvers[i]);

                // Read the order index from memory and place it on the stack.
                uint256 orderIndex = criteriaResolver.orderIndex;

                // Ensure that the order index is in range.
                if (orderIndex >= totalAdvancedOrders) {
                    _revertOrderCriteriaResolverOutOfRange(criteriaResolver.side);
                }

                // Retrieve the referenced advanced order.
                AdvancedOrder memory advancedOrder = advancedOrders[orderIndex];

                // Skip criteria resolution for order if not fulfilled.
                if (advancedOrder.numerator == 0) {
                    continue;
                }

                // Retrieve the parameters for the order.
                OrderParameters memory orderParameters = (advancedOrder.parameters);

                {
                    // Get a pointer to the list of items to give to
                    // _updateCriteriaItem. If the resolver refers to a
                    // consideration item, this array pointer will be replaced
                    // with the consideration array.
                    OfferItem[] memory items = orderParameters.offer;

                    // Read component index from memory and place it on stack.
                    uint256 componentIndex = criteriaResolver.index;

                    // Get error selector for `OfferCriteriaResolverOutOfRange`.
                    uint256 errorSelector = (OfferCriteriaResolverOutOfRange_error_selector);

                    // If the resolver refers to a consideration item...
                    if (criteriaResolver.side != Side.OFFER) {
                        // Get the pointer to `orderParameters.consideration`
                        // Using the array directly has a significant impact on
                        // the optimized compiler output.
                        MemoryPointer considerationPtr =
                            orderParameters.toMemoryPointer().pptr(OrderParameters_consideration_head_offset);

                        // Replace the items pointer with a pointer to the
                        // consideration array.
                        assembly {
                            items := considerationPtr
                        }

                        // Replace the error selector with the selector for
                        // `ConsiderationCriteriaResolverOutOfRange`.
                        errorSelector = (ConsiderationCriteriaResolverOutOfRange_err_selector);
                    }

                    // Ensure that the component index is in range.
                    if (componentIndex >= items.length) {
                        assembly {
                            // Revert with either
                            // `OfferCriteriaResolverOutOfRange()` or
                            // `ConsiderationCriteriaResolverOutOfRange()`,
                            // depending on whether the resolver refers to a
                            // consideration item.
                            mstore(0, errorSelector)
                            // revert(abi.encodeWithSignature(
                            //    "OfferCriteriaResolverOutOfRange()"
                            // ))
                            // or
                            // revert(abi.encodeWithSignature(
                            //    "ConsiderationCriteriaResolverOutOfRange()"
                            // ))
                            revert(Error_selector_offset, Selector_length)
                        }
                    }

                    // Apply the criteria resolver to the item in question.
                    _updateCriteriaItem(items, componentIndex, criteriaResolver);
                }
            }

            // Iterate over each advanced order.
            for (uint256 i = 0; i < totalAdvancedOrders; ++i) {
                // Retrieve the advanced order.
                AdvancedOrder memory advancedOrder = advancedOrders[i];

                // Skip criteria resolution for order if not fulfilled.
                if (advancedOrder.numerator == 0) {
                    continue;
                }

                // Retrieve the parameters for the order.
                OrderParameters memory orderParameters = (advancedOrder.parameters);

                // Read consideration length from memory and place on stack.
                uint256 totalItems = orderParameters.consideration.length;

                // Iterate over each consideration item on the order.
                for (uint256 j = 0; j < totalItems; ++j) {
                    // Ensure item type no longer indicates criteria usage.
                    if (_isItemWithCriteria(orderParameters.consideration[j].itemType)) {
                        _revertUnresolvedConsiderationCriteria(i, j);
                    }
                }

                // Read offer length from memory and place on stack.
                totalItems = orderParameters.offer.length;

                // Iterate over each offer item on the order.
                for (uint256 j = 0; j < totalItems; ++j) {
                    // Ensure item type no longer indicates criteria usage.
                    if (_isItemWithCriteria(orderParameters.offer[j].itemType)) {
                        _revertUnresolvedOfferCriteria(i, j);
                    }
                }
            }
        }
    }

    /**
     * @dev Internal pure function to update a criteria item.
     *
     * @param offer             The offer containing the item to update.
     * @param componentIndex    The index of the item to update.
     * @param criteriaResolver  The criteria resolver to use to update the item.
     */
    function _updateCriteriaItem(
        OfferItem[] memory offer,
        uint256 componentIndex,
        CriteriaResolver memory criteriaResolver
    ) internal pure {
        // Retrieve relevant item using the component index.
        OfferItem memory offerItem = offer[componentIndex];

        // Read item type and criteria from memory & place on stack.
        ItemType itemType = offerItem.itemType;

        // Ensure the specified item type indicates criteria usage.
        if (!_isItemWithCriteria(itemType)) {
            _revertCriteriaNotEnabledForItem();
        }

        uint256 identifierOrCriteria = offerItem.identifierOrCriteria;

        // If criteria is not 0 (i.e. a collection-wide criteria-based item)...
        if (identifierOrCriteria != uint256(0)) {
            // Verify identifier inclusion in criteria root using proof.
            _verifyProof(criteriaResolver.identifier, identifierOrCriteria, criteriaResolver.criteriaProof);
        } else if (criteriaResolver.criteriaProof.length != 0) {
            // Revert if non-empty proof is supplied for a collection-wide item.
            _revertInvalidProof();
        }

        // Update item type to remove criteria usage.
        // Use assembly to operate on ItemType enum as a number.
        ItemType newItemType;
        assembly {
            // Item type 4 becomes 2 and item type 5 becomes 3.
            newItemType := sub(3, eq(itemType, 4))
        }
        offerItem.itemType = newItemType;

        // Update identifier w/ supplied identifier.
        offerItem.identifierOrCriteria = criteriaResolver.identifier;
    }

    /**
     * @dev Internal pure function to check whether a given item type represents
     *      a criteria-based ERC721 or ERC1155 item (e.g. an item that can be
     *      resolved to one of a number of different identifiers at the time of
     *      order fulfillment).
     *
     * @param itemType The item type in question.
     *
     * @return withCriteria A boolean indicating that the item type in question
     *                      represents a criteria-based item.
     */
    function _isItemWithCriteria(ItemType itemType) internal pure returns (bool withCriteria) {
        // ERC721WithCriteria is ItemType 4. ERC1155WithCriteria is ItemType 5.
        assembly {
            withCriteria := gt(itemType, 3)
        }
    }

    /**
     * @dev Internal pure function to ensure that a given element is contained
     *      in a merkle root via a supplied proof.
     *
     * @param leaf  The element for which to prove inclusion.
     * @param root  The merkle root that inclusion will be proved against.
     * @param proof The merkle proof.
     */
    function _verifyProof(uint256 leaf, uint256 root, bytes32[] memory proof) internal pure {
        // Declare a variable that will be used to determine proof validity.
        bool isValid;

        // Utilize assembly to efficiently verify the proof against the root.
        assembly {
            // Store the leaf at the beginning of scratch space.
            mstore(0, leaf)

            // Derive the hash of the leaf to use as the initial proof element.
            let computedHash := keccak256(0, OneWord)

            // Get memory start location of the first element in proof array.
            let data := add(proof, OneWord)

            // Iterate over each proof element to compute the root hash.
            for {
                // Left shift by 5 is equivalent to multiplying by 0x20.
                let end := add(data, shl(OneWordShift, mload(proof)))
            } lt(data, end) {
                // Increment by one word at a time.
                data := add(data, OneWord)
            } {
                // Get the proof element.
                let loadedData := mload(data)

                // Sort proof elements and place them in scratch space.
                // Slot of `computedHash` in scratch space.
                // If the condition is true: 0x20, otherwise: 0x00.
                let scratch := shl(OneWordShift, gt(computedHash, loadedData))

                // Store elements to hash contiguously in scratch space. Scratch
                // space is 64 bytes (0x00 - 0x3f) & both elements are 32 bytes.
                mstore(scratch, computedHash)
                mstore(xor(scratch, OneWord), loadedData)

                // Derive the updated hash.
                computedHash := keccak256(0, TwoWords)
            }

            // Compare the final hash to the supplied root.
            isValid := eq(computedHash, root)
        }

        // Revert if computed hash does not equal supplied root.
        if (!isValid) {
            _revertInvalidProof();
        }
    }
}

/**
 * @title AmountDerivationErrors
 * @author 0age
 * @notice AmountDerivationErrors contains errors related to amount derivation.
 */
interface AmountDerivationErrors {
    /**
     * @dev Revert with an error when attempting to apply a fraction as part of
     *      a partial fill that does not divide the target amount cleanly.
     */
    error InexactFraction();
}

/**
 * @title AmountDeriver
 * @author 0age
 * @notice AmountDeriver contains view and pure functions related to deriving
 *         item amounts based on partial fill quantity and on linear
 *         interpolation based on current time when the start amount and end
 *         amount differ.
 */
contract AmountDeriver is AmountDerivationErrors {
    /**
     * @dev Internal view function to derive the current amount of a given item
     *      based on the current price, the starting price, and the ending
     *      price. If the start and end prices differ, the current price will be
     *      interpolated on a linear basis. Note that this function expects that
     *      the startTime parameter of orderParameters is not greater than the
     *      current block timestamp and that the endTime parameter is greater
     *      than the current block timestamp. If this condition is not upheld,
     *      duration / elapsed / remaining variables will underflow.
     *
     * @param startAmount The starting amount of the item.
     * @param endAmount   The ending amount of the item.
     * @param startTime   The starting time of the order.
     * @param endTime     The end time of the order.
     * @param roundUp     A boolean indicating whether the resultant amount
     *                    should be rounded up or down.
     *
     * @return amount The current amount.
     */
    function _locateCurrentAmount(
        uint256 startAmount,
        uint256 endAmount,
        uint256 startTime,
        uint256 endTime,
        bool roundUp
    ) internal view returns (uint256 amount) {
        // Only modify end amount if it doesn't already equal start amount.
        if (startAmount != endAmount) {
            // Declare variables to derive in the subsequent unchecked scope.
            uint256 duration;
            uint256 elapsed;
            uint256 remaining;

            // Skip underflow checks as startTime <= block.timestamp < endTime.
            unchecked {
                // Derive the duration for the order and place it on the stack.
                duration = endTime - startTime;

                // Derive time elapsed since the order started & place on stack.
                elapsed = block.timestamp - startTime;

                // Derive time remaining until order expires and place on stack.
                remaining = duration - elapsed;
            }

            // Aggregate new amounts weighted by time with rounding factor.
            uint256 totalBeforeDivision = ((startAmount * remaining) + (endAmount * elapsed));

            // Use assembly to combine operations and skip divide-by-zero check.
            assembly {
                // Multiply by iszero(iszero(totalBeforeDivision)) to ensure
                // amount is set to zero if totalBeforeDivision is zero,
                // as intermediate overflow can occur if it is zero.
                amount :=
                    mul(
                        iszero(iszero(totalBeforeDivision)),
                        // Subtract 1 from the numerator and add 1 to the result if
                        // roundUp is true to get the proper rounding direction.
                        // Division is performed with no zero check as duration
                        // cannot be zero as long as startTime < endTime.
                        add(div(sub(totalBeforeDivision, roundUp), duration), roundUp)
                    )
            }

            // Return the current amount.
            return amount;
        }

        // Return the original amount as startAmount == endAmount.
        return endAmount;
    }

    /**
     * @dev Internal pure function to return a fraction of a given value and to
     *      ensure the resultant value does not have any fractional component.
     *      Note that this function assumes that zero will never be supplied as
     *      the denominator parameter; invalid / undefined behavior will result
     *      should a denominator of zero be provided.
     *
     * @param numerator   A value indicating the portion of the order that
     *                    should be filled.
     * @param denominator A value indicating the total size of the order. Note
     *                    that this value cannot be equal to zero.
     * @param value       The value for which to compute the fraction.
     *
     * @return newValue The value after applying the fraction.
     */
    function _getFraction(uint256 numerator, uint256 denominator, uint256 value)
        internal
        pure
        returns (uint256 newValue)
    {
        // Return value early in cases where the fraction resolves to 1.
        if (numerator == denominator) {
            return value;
        }

        // Ensure fraction can be applied to the value with no remainder. Note
        // that the denominator cannot be zero.
        assembly {
            // Ensure new value contains no remainder via mulmod operator.
            // Credit to @hrkrshnn + @axic for proposing this optimal solution.
            if mulmod(value, numerator, denominator) {
                // Store left-padded selector with push4, mem[28:32] = selector
                mstore(0, InexactFraction_error_selector)

                // revert(abi.encodeWithSignature("InexactFraction()"))
                revert(Error_selector_offset, InexactFraction_error_length)
            }
        }

        // Multiply the numerator by the value and ensure no overflow occurs.
        uint256 valueTimesNumerator = value * numerator;

        // Divide and check for remainder. Note that denominator cannot be zero.
        assembly {
            // Perform division without zero check.
            newValue := div(valueTimesNumerator, denominator)
        }
    }

    /**
     * @dev Internal view function to apply a fraction to a consideration
     * or offer item.
     *
     * @param startAmount     The starting amount of the item.
     * @param endAmount       The ending amount of the item.
     * @param numerator       A value indicating the portion of the order that
     *                        should be filled.
     * @param denominator     A value indicating the total size of the order.
     * @param startTime       The starting time of the order.
     * @param endTime         The end time of the order.
     * @param roundUp         A boolean indicating whether the resultant
     *                        amount should be rounded up or down.
     *
     * @return amount The received item to transfer with the final amount.
     */
    function _applyFraction(
        uint256 startAmount,
        uint256 endAmount,
        uint256 numerator,
        uint256 denominator,
        uint256 startTime,
        uint256 endTime,
        bool roundUp
    ) internal view returns (uint256 amount) {
        // If start amount equals end amount, apply fraction to end amount.
        if (startAmount == endAmount) {
            // Apply fraction to end amount.
            amount = _getFraction(numerator, denominator, endAmount);
        } else {
            // Otherwise, apply fraction to both and interpolated final amount.
            amount = _locateCurrentAmount(
                _getFraction(numerator, denominator, startAmount),
                _getFraction(numerator, denominator, endAmount),
                startTime,
                endTime,
                roundUp
            );
        }
    }
}

/**
 * @title OrderFulfiller
 * @author 0age
 * @notice OrderFulfiller contains logic related to order fulfillment where a
 *         single order is being fulfilled and where basic order fulfillment is
 *         not available as an option.
 */
contract OrderFulfiller is BasicOrderFulfiller, CriteriaResolution, AmountDeriver {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) BasicOrderFulfiller(conduitController) {}

    /**
     * @dev Internal function to validate an order and update its status, adjust
     *      prices based on current time, apply criteria resolvers, determine
     *      what portion to fill, and transfer relevant tokens.
     *
     * @param advancedOrder       The order to fulfill as well as the fraction
     *                            to fill. Note that all offer and consideration
     *                            components must divide with no remainder for
     *                            the partial fill to be valid.
     * @param criteriaResolvers   An array where each element contains a
     *                            reference to a specific offer or
     *                            consideration, a token identifier, and a proof
     *                            that the supplied token identifier is
     *                            contained in the order's merkle root. Note
     *                            that a criteria of zero indicates that any
     *                            (transferable) token identifier is valid and
     *                            that no proof needs to be supplied.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     * @param recipient           The intended recipient for all received items.
     *
     * @return A boolean indicating whether the order has been fulfilled.
     */
    function _validateAndFulfillAdvancedOrder(
        AdvancedOrder memory advancedOrder,
        CriteriaResolver[] memory criteriaResolvers,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal returns (bool) {
        // Ensure this function cannot be triggered during a reentrant call.
        _setReentrancyGuard(
            // Native tokens accepted during execution for contract order types.
            advancedOrder.parameters.orderType == OrderType.CONTRACT
        );

        // Validate order, update status, and determine fraction to fill.
        (bytes32 orderHash, uint256 fillNumerator, uint256 fillDenominator) =
            _validateOrderAndUpdateStatus(advancedOrder, true);

        // Create an array with length 1 containing the order.
        AdvancedOrder[] memory advancedOrders = new AdvancedOrder[](1);

        // Populate the order as the first and only element of the new array.
        advancedOrders[0] = advancedOrder;

        // Apply criteria resolvers using generated orders and details arrays.
        _applyCriteriaResolvers(advancedOrders, criteriaResolvers);

        // Retrieve the order parameters after applying criteria resolvers.
        OrderParameters memory orderParameters = advancedOrders[0].parameters;

        // Perform each item transfer with the appropriate fractional amount.
        _applyFractionsAndTransferEach(orderParameters, fillNumerator, fillDenominator, fulfillerConduitKey, recipient);

        // Declare empty bytes32 array and populate with the order hash.
        bytes32[] memory orderHashes = new bytes32[](1);
        orderHashes[0] = orderHash;

        // Ensure restricted orders have a valid submitter or pass a zone check.
        _assertRestrictedAdvancedOrderValidity(advancedOrders[0], orderHashes, orderHash);

        // Emit an event signifying that the order has been fulfilled.
        _emitOrderFulfilledEvent(
            orderHash,
            orderParameters.offerer,
            orderParameters.zone,
            recipient,
            orderParameters.offer,
            orderParameters.consideration
        );

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        return true;
    }

    /**
     * @dev Internal function to transfer each item contained in a given single
     *      order fulfillment after applying a respective fraction to the amount
     *      being transferred.
     *
     * @param orderParameters     The parameters for the fulfilled order.
     * @param numerator           A value indicating the portion of the order
     *                            that should be filled.
     * @param denominator         A value indicating the total order size.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     * @param recipient           The intended recipient for all received items.
     */
    function _applyFractionsAndTransferEach(
        OrderParameters memory orderParameters,
        uint256 numerator,
        uint256 denominator,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal {
        // Read start time & end time from order parameters and place on stack.
        uint256 startTime = orderParameters.startTime;
        uint256 endTime = orderParameters.endTime;

        // Initialize an accumulator array. From this point forward, no new
        // memory regions can be safely allocated until the accumulator is no
        // longer being utilized, as the accumulator operates in an open-ended
        // fashion from this memory pointer; existing memory may still be
        // accessed and modified, however.
        bytes memory accumulator = new bytes(AccumulatorDisarmed);

        // As of solidity 0.6.0, inline assembly cannot directly access function
        // definitions, but can still access locally scoped function variables.
        // This means that a local variable to reference the internal function
        // definition (using the same type), along with a local variable with
        // the desired type, must first be created. Then, the original function
        // pointer can be recast to the desired type.

        /**
         * Repurpose existing OfferItem memory regions on the offer array for
         * the order by overriding the _transfer function pointer to accept a
         * modified OfferItem argument in place of the usual ReceivedItem:
         *
         *   ========= OfferItem ==========   ====== ReceivedItem ======
         *   ItemType itemType; ------------> ItemType itemType;
         *   address token; ----------------> address token;
         *   uint256 identifierOrCriteria; -> uint256 identifier;
         *   uint256 startAmount; ----------> uint256 amount;
         *   uint256 endAmount; ------------> address recipient;
         */

        // Declare a nested scope to minimize stack depth.
        unchecked {
            // Read offer array length from memory and place on stack.
            uint256 totalOfferItems = orderParameters.offer.length;

            // Create a variable to indicate whether the order has any
            // native offer items
            uint256 anyNativeItems;

            // Iterate over each offer on the order.
            // Skip overflow check as for loop is indexed starting at zero.
            for (uint256 i = 0; i < totalOfferItems; ++i) {
                // Retrieve the offer item.
                OfferItem memory offerItem = orderParameters.offer[i];

                // Offer items for the native token can not be received outside
                // of a match order function except as part of a contract order.
                {
                    ItemType itemType = offerItem.itemType;
                    assembly {
                        anyNativeItems := or(anyNativeItems, iszero(itemType))
                    }
                }

                // Declare an additional nested scope to minimize stack depth.
                {
                    // Apply fill fraction to get offer item amount to transfer.
                    uint256 amount = _applyFraction(
                        offerItem.startAmount, offerItem.endAmount, numerator, denominator, startTime, endTime, false
                    );

                    // Utilize assembly to set overloaded offerItem arguments.
                    assembly {
                        // Write new fractional amount to startAmount as amount.
                        mstore(add(offerItem, ReceivedItem_amount_offset), amount)

                        // Write recipient to endAmount.
                        mstore(add(offerItem, ReceivedItem_recipient_offset), recipient)
                    }
                }

                // Transfer the item from the offerer to the recipient.
                _toOfferItemInput(_transfer)(
                    offerItem, orderParameters.offerer, orderParameters.conduitKey, accumulator
                );
            }

            // If a non-contract order has native offer items, throw with an
            // `InvalidNativeOfferItem` custom error.
            {
                OrderType orderType = orderParameters.orderType;
                uint256 invalidNativeOfferItem;
                assembly {
                    invalidNativeOfferItem :=
                        and(
                            // Note that this check requires that there are no order
                            // types beyond the current set (0-4).  It will need to
                            // be modified if more order types are added.
                            lt(orderType, 4),
                            anyNativeItems
                        )
                }
                if (invalidNativeOfferItem != 0) {
                    _revertInvalidNativeOfferItem();
                }
            }
        }

        // Declare a variable for the available native token balance.
        uint256 nativeTokenBalance;

        /**
         * Repurpose existing ConsiderationItem memory regions on the
         * consideration array for the order by overriding the _transfer
         * function pointer to accept a modified ConsiderationItem argument in
         * place of the usual ReceivedItem:
         *
         *   ====== ConsiderationItem =====   ====== ReceivedItem ======
         *   ItemType itemType; ------------> ItemType itemType;
         *   address token; ----------------> address token;
         *   uint256 identifierOrCriteria;--> uint256 identifier;
         *   uint256 startAmount; ----------> uint256 amount;
         *   uint256 endAmount;        /----> address recipient;
         *   address recipient; ------/
         */

        // Declare a nested scope to minimize stack depth.
        unchecked {
            // Read consideration array length from memory and place on stack.
            uint256 totalConsiderationItems = orderParameters.consideration.length;

            // Iterate over each consideration item on the order.
            // Skip overflow check as for loop is indexed starting at zero.
            for (uint256 i = 0; i < totalConsiderationItems; ++i) {
                // Retrieve the consideration item.
                ConsiderationItem memory considerationItem = (orderParameters.consideration[i]);

                // Apply fraction & derive considerationItem amount to transfer.
                uint256 amount = _applyFraction(
                    considerationItem.startAmount,
                    considerationItem.endAmount,
                    numerator,
                    denominator,
                    startTime,
                    endTime,
                    true
                );

                // Use assembly to set overloaded considerationItem arguments.
                assembly {
                    // Write derived fractional amount to startAmount as amount.
                    mstore(add(considerationItem, ReceivedItem_amount_offset), amount)

                    // Write original recipient to endAmount as recipient.
                    mstore(
                        add(considerationItem, ReceivedItem_recipient_offset),
                        mload(add(considerationItem, ConsiderationItem_recipient_offset))
                    )
                }

                if (considerationItem.itemType == ItemType.NATIVE) {
                    // Get the current available balance of native tokens.
                    assembly {
                        nativeTokenBalance := selfbalance()
                    }

                    // Ensure that sufficient native tokens are still available.
                    if (amount > nativeTokenBalance) {
                        _revertInsufficientNativeTokensSupplied();
                    }
                }

                // Transfer item from caller to recipient specified by the item.
                _toConsiderationItemInput(_transfer)(considerationItem, msg.sender, fulfillerConduitKey, accumulator);
            }
        }

        // Trigger any remaining accumulated transfers via call to the conduit.
        _triggerIfArmed(accumulator);

        // Determine whether any native token balance remains.
        assembly {
            nativeTokenBalance := selfbalance()
        }

        // Return any remaining native token balance to the caller.
        if (nativeTokenBalance != 0) {
            _transferNativeTokens(payable(msg.sender), nativeTokenBalance);
        }
    }

    /**
     * @dev Internal function to emit an OrderFulfilled event. OfferItems are
     *      translated into SpentItems and ConsiderationItems are translated
     *      into ReceivedItems.
     *
     * @param orderHash     The order hash.
     * @param offerer       The offerer for the order.
     * @param zone          The zone for the order.
     * @param recipient     The recipient of the order, or the null address if
     *                      the order was fulfilled via order matching.
     * @param offer         The offer items for the order.
     * @param consideration The consideration items for the order.
     */
    function _emitOrderFulfilledEvent(
        bytes32 orderHash,
        address offerer,
        address zone,
        address recipient,
        OfferItem[] memory offer,
        ConsiderationItem[] memory consideration
    ) internal {
        // Cast already-modified offer memory region as spent items.
        SpentItem[] memory spentItems;
        assembly {
            spentItems := offer
        }

        // Cast already-modified consideration memory region as received items.
        ReceivedItem[] memory receivedItems;
        assembly {
            receivedItems := consideration
        }

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(orderHash, offerer, zone, recipient, spentItems, receivedItems);
    }
}

/**
 * @title FulfillmentApplicationErrors
 * @author 0age
 * @notice FulfillmentApplicationErrors contains errors related to fulfillment
 *         application and aggregation.
 */
interface FulfillmentApplicationErrors {
    /**
     * @dev Revert with an error when a fulfillment is provided that does not
     *      declare at least one component as part of a call to fulfill
     *      available orders.
     */
    error MissingFulfillmentComponentOnAggregation(Side side);

    /**
     * @dev Revert with an error when a fulfillment is provided that does not
     *      declare at least one offer component and at least one consideration
     *      component.
     */
    error OfferAndConsiderationRequiredOnFulfillment();

    /**
     * @dev Revert with an error when the initial offer item named by a
     *      fulfillment component does not match the type, token, identifier,
     *      or conduit preference of the initial consideration item.
     *
     * @param fulfillmentIndex The index of the fulfillment component that
     *                         does not match the initial offer item.
     */
    error MismatchedFulfillmentOfferAndConsiderationComponents(
        uint256 fulfillmentIndex
    );

    /**
     * @dev Revert with an error when an order or item index are out of range
     *      or a fulfillment component does not match the type, token,
     *      identifier, or conduit preference of the initial consideration item.
     */
    error InvalidFulfillmentComponentData();
}

/**
 * @title FulfillmentApplier
 * @author 0age
 * @notice FulfillmentApplier contains logic related to applying fulfillments,
 *         both as part of order matching (where offer items are matched to
 *         consideration items) as well as fulfilling available orders (where
 *         order items and consideration items are independently aggregated).
 */
contract FulfillmentApplier is FulfillmentApplicationErrors {
    /**
     * @dev Internal pure function to match offer items to consideration items
     *      on a group of orders via a supplied fulfillment.
     *
     * @param advancedOrders          The orders to match.
     * @param offerComponents         An array designating offer components to
     *                                match to consideration components.
     * @param considerationComponents An array designating consideration
     *                                components to match to offer components.
     *                                Note that each consideration amount must
     *                                be zero in order for the match operation
     *                                to be valid.
     * @param fulfillmentIndex        The index of the fulfillment being
     *                                applied.
     *
     * @return execution The transfer performed as a result of the fulfillment.
     */
    function _applyFulfillment(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] memory offerComponents,
        FulfillmentComponent[] memory considerationComponents,
        uint256 fulfillmentIndex
    ) internal pure returns (Execution memory execution) {
        // Ensure 1+ of both offer and consideration components are supplied.
        if (offerComponents.length == 0 || considerationComponents.length == 0) {
            _revertOfferAndConsiderationRequiredOnFulfillment();
        }

        // Declare a new Execution struct.
        Execution memory considerationExecution;

        // Validate & aggregate consideration items to new Execution object.
        _aggregateValidFulfillmentConsiderationItems(advancedOrders, considerationComponents, considerationExecution);

        // Retrieve the consideration item from the execution struct.
        ReceivedItem memory considerationItem = considerationExecution.item;

        // Skip aggregating offer items if no consideration items are available.
        if (considerationItem.amount == 0) {
            // Set the offerer and recipient to null address and the item type
            // to a non-native item type if the execution amount is zero. This
            // will cause the execution item to be skipped.
            considerationExecution.offerer = address(0);
            considerationExecution.item.recipient = payable(0);
            considerationExecution.item.itemType = ItemType.ERC20;
            return considerationExecution;
        }

        // Recipient does not need to be specified because it will always be set
        // to that of the consideration.
        // Validate & aggregate offer items to Execution object.
        _aggregateValidFulfillmentOfferItems(advancedOrders, offerComponents, execution);

        // Ensure offer & consideration item types, tokens, & identifiers match.
        // (a != b || c != d || e != f) == (((a ^ b) | (c ^ d) | (e ^ f)) != 0),
        // but the second expression requires less gas to evaluate.
        if (
            (
                (uint8(execution.item.itemType) ^ uint8(considerationItem.itemType))
                    | (uint160(execution.item.token) ^ uint160(considerationItem.token))
                    | (execution.item.identifier ^ considerationItem.identifier)
            ) != 0
        ) {
            _revertMismatchedFulfillmentOfferAndConsiderationComponents(fulfillmentIndex);
        }

        // If total consideration amount exceeds the offer amount...
        if (considerationItem.amount > execution.item.amount) {
            // Retrieve the first consideration component from the fulfillment.
            FulfillmentComponent memory targetComponent = (considerationComponents[0]);

            // Skip underflow check as the conditional being true implies that
            // considerationItem.amount > execution.item.amount.
            unchecked {
                // Add excess consideration item amount to original order array.
                advancedOrders[targetComponent.orderIndex].parameters.consideration[targetComponent.itemIndex]
                    .startAmount = (considerationItem.amount - execution.item.amount);
            }
        } else {
            // Retrieve the first offer component from the fulfillment.
            FulfillmentComponent memory targetComponent = offerComponents[0];

            // Skip underflow check as the conditional being false implies that
            // execution.item.amount >= considerationItem.amount.
            unchecked {
                // Add excess offer item amount to the original array of orders.
                advancedOrders[targetComponent.orderIndex].parameters.offer[targetComponent.itemIndex].startAmount =
                    (execution.item.amount - considerationItem.amount);
            }

            // Reduce total offer amount to equal the consideration amount.
            execution.item.amount = considerationItem.amount;
        }

        // Reuse consideration recipient.
        execution.item.recipient = considerationItem.recipient;

        // Return the final execution that will be triggered for relevant items.
        return execution; // Execution(considerationItem, offerer, conduitKey);
    }

    /**
     * @dev Internal view function to aggregate offer or consideration items
     *      from a group of orders into a single execution via a supplied array
     *      of fulfillment components. Items that are not available to aggregate
     *      will not be included in the aggregated execution.
     *
     * @param advancedOrders        The orders to aggregate.
     * @param side                  The side (i.e. offer or consideration).
     * @param fulfillmentComponents An array designating item components to
     *                              aggregate if part of an available order.
     * @param fulfillerConduitKey   A bytes32 value indicating what conduit, if
     *                              any, to source the fulfiller's token
     *                              approvals from. The zero hash signifies that
     *                              no conduit should be used, with approvals
     *                              set directly on this contract.
     * @param recipient             The intended recipient for all received
     *                              items.
     *
     * @return execution The transfer performed as a result of the fulfillment.
     */
    function _aggregateAvailable(
        AdvancedOrder[] memory advancedOrders,
        Side side,
        FulfillmentComponent[] memory fulfillmentComponents,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal view returns (Execution memory execution) {
        // Skip overflow / underflow checks; conditions checked or unreachable.
        unchecked {
            // Retrieve fulfillment components array length and place on stack.
            // Ensure at least one fulfillment component has been supplied.
            if (fulfillmentComponents.length == 0) {
                _revertMissingFulfillmentComponentOnAggregation(side);
            }

            // Retrieve the received item on the execution being returned.
            ReceivedItem memory item = execution.item;

            // If the fulfillment components are offer components...
            if (side == Side.OFFER) {
                // Set the supplied recipient on the execution item.
                item.recipient = payable(recipient);

                // Return execution for aggregated items provided by offerer.
                _aggregateValidFulfillmentOfferItems(advancedOrders, fulfillmentComponents, execution);
            } else {
                // Otherwise, fulfillment components are consideration
                // components. Return execution for aggregated items provided by
                // the fulfiller.
                _aggregateValidFulfillmentConsiderationItems(advancedOrders, fulfillmentComponents, execution);

                // Set the caller as the offerer on the execution.
                execution.offerer = msg.sender;

                // Set fulfiller conduit key as the conduit key on execution.
                execution.conduitKey = fulfillerConduitKey;
            }

            // Set the offerer and recipient to null address and the item type
            // to a non-native item type if the execution amount is zero. This
            // will cause the execution item to be skipped.
            if (item.amount == 0) {
                execution.offerer = address(0);
                item.recipient = payable(0);
                item.itemType = ItemType.ERC20;
            }
        }
    }

    /**
     * @dev Internal pure function to aggregate a group of offer items using
     *      supplied directives on which component items are candidates for
     *      aggregation, skipping items on orders that are not available.
     *
     * @param advancedOrders  The orders to aggregate offer items from.
     * @param offerComponents An array of FulfillmentComponent structs
     *                        indicating the order index and item index of each
     *                        candidate offer item for aggregation.
     * @param execution       The execution to apply the aggregation to.
     */
    function _aggregateValidFulfillmentOfferItems(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] memory offerComponents,
        Execution memory execution
    ) internal pure {
        assembly {
            // Declare a variable for the final aggregated item amount.
            let amount

            // Declare a variable to track errors encountered with amount.
            let errorBuffer

            // Declare a variable for the hash of itemType, token, & identifier.
            let dataHash

            // Iterate over each offer component.
            for {
                // Create variable to track position in offerComponents head.
                let fulfillmentHeadPtr := offerComponents

                // Get position one word past last element in head of array.
                let endPtr := add(offerComponents, shl(OneWordShift, mload(offerComponents)))
            } lt(fulfillmentHeadPtr, endPtr) {} {
                // Increment position in considerationComponents head.
                fulfillmentHeadPtr := add(fulfillmentHeadPtr, OneWord)

                // Retrieve the order index using the fulfillment pointer.
                let orderIndex := mload(mload(fulfillmentHeadPtr))

                // Ensure that the order index is not out of range.
                if iszero(lt(orderIndex, mload(advancedOrders))) { throwInvalidFulfillmentComponentData() }

                // Read advancedOrders[orderIndex] pointer from its array head.
                let orderPtr :=
                    mload(
                        // Calculate head position of advancedOrders[orderIndex].
                        add(add(advancedOrders, OneWord), shl(OneWordShift, orderIndex))
                    )

                // Read the pointer to OrderParameters from the AdvancedOrder.
                let paramsPtr := mload(orderPtr)

                // Retrieve item index using an offset of fulfillment pointer.
                let itemIndex := mload(add(mload(fulfillmentHeadPtr), Fulfillment_itemIndex_offset))

                let offerItemPtr
                {
                    // Load the offer array pointer.
                    let offerArrPtr := mload(add(paramsPtr, OrderParameters_offer_head_offset))

                    // If the offer item index is out of range or the numerator
                    // is zero, skip this item.
                    if or(
                        iszero(lt(itemIndex, mload(offerArrPtr))),
                        iszero(mload(add(orderPtr, AdvancedOrder_numerator_offset)))
                    ) { continue }

                    // Retrieve offer item pointer using the item index.
                    offerItemPtr :=
                        mload(
                            add(
                                // Get pointer to beginning of receivedItem.
                                add(offerArrPtr, OneWord),
                                // Calculate offset to pointer for desired order.
                                shl(OneWordShift, itemIndex)
                            )
                        )
                }

                // Declare a separate scope for the amount update.
                {
                    // Retrieve amount pointer using consideration item pointer.
                    let amountPtr := add(offerItemPtr, Common_amount_offset)

                    // Add offer item amount to execution amount.
                    let newAmount := add(amount, mload(amountPtr))

                    // Update error buffer:
                    // 1 = zero amount, 2 = overflow, 3 = both.
                    errorBuffer := or(errorBuffer, or(shl(1, lt(newAmount, amount)), iszero(mload(amountPtr))))

                    // Update the amount to the new, summed amount.
                    amount := newAmount

                    // Zero out amount on original item to indicate it is spent.
                    mstore(amountPtr, 0)
                }

                // Retrieve ReceivedItem pointer from Execution.
                let receivedItem := mload(execution)

                // Check if this is the first valid fulfillment item.
                switch iszero(dataHash)
                case 1 {
                    // On first valid item, populate the received item in memory
                    // for later comparison.

                    // Set the item type on the received item.
                    mstore(receivedItem, mload(offerItemPtr))

                    // Set the token on the received item.
                    mstore(add(receivedItem, Common_token_offset), mload(add(offerItemPtr, Common_token_offset)))

                    // Set the identifier on the received item.
                    mstore(
                        add(receivedItem, Common_identifier_offset), mload(add(offerItemPtr, Common_identifier_offset))
                    )

                    // Set offerer on returned execution using order pointer.
                    mstore(add(execution, Execution_offerer_offset), mload(paramsPtr))

                    // Set execution conduitKey via order pointer offset.
                    mstore(
                        add(execution, Execution_conduit_offset), mload(add(paramsPtr, OrderParameters_conduit_offset))
                    )

                    // Calculate the hash of (itemType, token, identifier).
                    dataHash := keccak256(receivedItem, ReceivedItem_CommonParams_size)

                    // If component index > 0, swap component pointer with
                    // pointer to first component so that any remainder after
                    // fulfillment can be added back to the first item.
                    let firstFulfillmentHeadPtr := add(offerComponents, OneWord)
                    if xor(firstFulfillmentHeadPtr, fulfillmentHeadPtr) {
                        let firstFulfillmentPtr := mload(firstFulfillmentHeadPtr)
                        let fulfillmentPtr := mload(fulfillmentHeadPtr)
                        mstore(firstFulfillmentHeadPtr, fulfillmentPtr)
                    }
                }
                default {
                    // Compare every subsequent item to the first.
                    if or(
                        or(
                            // The offerer must match on both items.
                            xor(mload(paramsPtr), mload(add(execution, Execution_offerer_offset))),
                            // The conduit key must match on both items.
                            xor(
                                mload(add(paramsPtr, OrderParameters_conduit_offset)),
                                mload(add(execution, Execution_conduit_offset))
                            )
                        ),
                        // The itemType, token, and identifier must match.
                        xor(dataHash, keccak256(offerItemPtr, ReceivedItem_CommonParams_size))
                    ) {
                        // Throw if any of the requirements are not met.
                        throwInvalidFulfillmentComponentData()
                    }
                }
            }

            // Write final amount to execution.
            mstore(add(mload(execution), Common_amount_offset), amount)

            // Determine whether the error buffer contains a nonzero error code.
            if errorBuffer {
                // If errorBuffer is 1, an item had an amount of zero.
                if eq(errorBuffer, 1) {
                    // Store left-padded selector with push4 (reduces bytecode)
                    // mem[28:32] = selector
                    mstore(0, MissingItemAmount_error_selector)

                    // revert(abi.encodeWithSignature("MissingItemAmount()"))
                    revert(Error_selector_offset, MissingItemAmount_error_length)
                }

                // If errorBuffer is not 1 or 0, the sum overflowed.
                // Panic!
                throwOverflow()
            }

            // Declare function for reverts on invalid fulfillment data.
            function throwInvalidFulfillmentComponentData() {
                // Store left-padded selector (uses push4 and reduces code size)
                mstore(0, InvalidFulfillmentComponentData_error_selector)

                // revert(abi.encodeWithSignature(
                //     "InvalidFulfillmentComponentData()"
                // ))
                revert(Error_selector_offset, InvalidFulfillmentComponentData_error_length)
            }

            // Declare function for reverts due to arithmetic overflows.
            function throwOverflow() {
                // Store the Panic error signature.
                mstore(0, Panic_error_selector)
                // Store the arithmetic (0x11) panic code.
                mstore(Panic_error_code_ptr, Panic_arithmetic)
                // revert(abi.encodeWithSignature("Panic(uint256)", 0x11))
                revert(Error_selector_offset, Panic_error_length)
            }
        }
    }

    /**
     * @dev Internal pure function to aggregate a group of consideration items
     *      using supplied directives on which component items are candidates
     *      for aggregation, skipping items on orders that are not available.
     *      Note that this function depends on memory layout affected by an
     *      earlier call to _validateOrdersAndPrepareToFulfill.  The memory for
     *      the consideration arrays needs to be updated before calling
     *      _aggregateValidFulfillmentConsiderationItems.
     *      _validateOrdersAndPrepareToFulfill is called in _matchAdvancedOrders
     *      and _fulfillAvailableAdvancedOrders in the current version.
     *
     * @param advancedOrders          The orders to aggregate consideration
     *                                items from.
     * @param considerationComponents An array of FulfillmentComponent structs
     *                                indicating the order index and item index
     *                                of each candidate consideration item for
     *                                aggregation.
     * @param execution               The execution to apply the aggregation to.
     */
    function _aggregateValidFulfillmentConsiderationItems(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] memory considerationComponents,
        Execution memory execution
    ) internal pure {
        // Utilize assembly in order to efficiently aggregate the items.
        assembly {
            // Declare a variable for the final aggregated item amount.
            let amount

            // Create variable to track errors encountered with amount.
            let errorBuffer

            // Declare variable for hash(itemType, token, identifier, recipient)
            let dataHash

            // Iterate over each consideration component.
            for {
                // Track position in considerationComponents head.
                let fulfillmentHeadPtr := considerationComponents

                // Get position one word past last element in head of array.
                let endPtr := add(considerationComponents, shl(OneWordShift, mload(considerationComponents)))
            } lt(fulfillmentHeadPtr, endPtr) {} {
                // Increment position in considerationComponents head.
                fulfillmentHeadPtr := add(fulfillmentHeadPtr, OneWord)

                // Retrieve the order index using the fulfillment pointer.
                let orderIndex := mload(mload(fulfillmentHeadPtr))

                // Ensure that the order index is not out of range.
                if iszero(lt(orderIndex, mload(advancedOrders))) { throwInvalidFulfillmentComponentData() }

                // Read advancedOrders[orderIndex] pointer from its array head.
                let orderPtr :=
                    mload(
                        // Calculate head position of advancedOrders[orderIndex].
                        add(add(advancedOrders, OneWord), shl(OneWordShift, orderIndex))
                    )

                // Retrieve item index using an offset of fulfillment pointer.
                let itemIndex := mload(add(mload(fulfillmentHeadPtr), Fulfillment_itemIndex_offset))

                let considerationItemPtr
                {
                    // Load consideration array pointer.
                    let considerationArrPtr :=
                        mload(
                            add(
                                // Read OrderParameters pointer from AdvancedOrder.
                                mload(orderPtr),
                                OrderParameters_consideration_head_offset
                            )
                        )

                    // If the consideration item index is out of range or the
                    // numerator is zero, skip this item.
                    if or(
                        iszero(lt(itemIndex, mload(considerationArrPtr))),
                        iszero(mload(add(orderPtr, AdvancedOrder_numerator_offset)))
                    ) { continue }

                    // Retrieve consideration item pointer using the item index.
                    considerationItemPtr :=
                        mload(
                            add(
                                // Get pointer to beginning of receivedItem.
                                add(considerationArrPtr, OneWord),
                                // Calculate offset to pointer for desired order.
                                shl(OneWordShift, itemIndex)
                            )
                        )
                }

                // Declare a separate scope for the amount update.
                {
                    // Retrieve amount pointer using consideration item pointer.
                    let amountPtr := add(considerationItemPtr, Common_amount_offset)

                    // Add consideration item amount to execution amount.
                    let newAmount := add(amount, mload(amountPtr))

                    // Update error buffer:
                    // 1 = zero amount, 2 = overflow, 3 = both.
                    errorBuffer := or(errorBuffer, or(shl(1, lt(newAmount, amount)), iszero(mload(amountPtr))))

                    // Update the amount to the new, summed amount.
                    amount := newAmount

                    // Zero out original item amount to indicate it is credited.
                    mstore(amountPtr, 0)
                }

                // Retrieve ReceivedItem pointer from Execution.
                let receivedItem := mload(execution)

                switch iszero(dataHash)
                case 1 {
                    // On first valid item, populate the received item in
                    // memory for later comparison.

                    // Set the item type on the received item.
                    mstore(receivedItem, mload(considerationItemPtr))

                    // Set the token on the received item.
                    mstore(
                        add(receivedItem, Common_token_offset), mload(add(considerationItemPtr, Common_token_offset))
                    )

                    // Set the identifier on the received item.
                    mstore(
                        add(receivedItem, Common_identifier_offset),
                        mload(add(considerationItemPtr, Common_identifier_offset))
                    )

                    // Set the recipient on the received item. Note that this
                    // depends on the memory layout established by the
                    // _validateOrdersAndPrepareToFulfill function.
                    mstore(
                        add(receivedItem, ReceivedItem_recipient_offset),
                        mload(add(considerationItemPtr, ReceivedItem_recipient_offset))
                    )

                    // Calculate the hash of (itemType, token, identifier,
                    // recipient). This is run after amount is set to zero, so
                    // there will be one blank word after identifier included in
                    // the hash buffer.
                    dataHash := keccak256(considerationItemPtr, ReceivedItem_size)

                    // If component index > 0, swap component pointer with
                    // pointer to first component so that any remainder after
                    // fulfillment can be added back to the first item.
                    let firstFulfillmentHeadPtr := add(considerationComponents, OneWord)
                    if xor(firstFulfillmentHeadPtr, fulfillmentHeadPtr) {
                        let firstFulfillmentPtr := mload(firstFulfillmentHeadPtr)
                        let fulfillmentPtr := mload(fulfillmentHeadPtr)
                        mstore(firstFulfillmentHeadPtr, fulfillmentPtr)
                    }
                }
                default {
                    // Compare every subsequent item to the first; the item
                    // type, token, identifier and recipient must match.
                    if xor(
                        dataHash,
                        // Calculate the hash of (itemType, token, identifier,
                        // recipient). This is run after amount is set to zero,
                        // so there will be one blank word after identifier
                        // included in the hash buffer.
                        keccak256(considerationItemPtr, ReceivedItem_size)
                    ) {
                        // Throw if any of the requirements are not met.
                        throwInvalidFulfillmentComponentData()
                    }
                }
            }

            // Retrieve ReceivedItem pointer from Execution.
            let receivedItem := mload(execution)

            // Write final amount to execution.
            mstore(add(receivedItem, Common_amount_offset), amount)

            // Determine whether the error buffer contains a nonzero error code.
            if errorBuffer {
                // If errorBuffer is 1, an item had an amount of zero.
                if eq(errorBuffer, 1) {
                    // Store left-padded selector with push4, mem[28:32]
                    mstore(0, MissingItemAmount_error_selector)

                    // revert(abi.encodeWithSignature("MissingItemAmount()"))
                    revert(Error_selector_offset, MissingItemAmount_error_length)
                }

                // If errorBuffer is not 1 or 0, `amount` overflowed.
                // Panic!
                throwOverflow()
            }

            // Declare function for reverts on invalid fulfillment data.
            function throwInvalidFulfillmentComponentData() {
                // Store the InvalidFulfillmentComponentData error signature.
                mstore(0, InvalidFulfillmentComponentData_error_selector)

                // revert(abi.encodeWithSignature(
                //     "InvalidFulfillmentComponentData()"
                // ))
                revert(Error_selector_offset, InvalidFulfillmentComponentData_error_length)
            }

            // Declare function for reverts due to arithmetic overflows.
            function throwOverflow() {
                // Store the Panic error signature.
                mstore(0, Panic_error_selector)
                // Store the arithmetic (0x11) panic code.
                mstore(Panic_error_code_ptr, Panic_arithmetic)
                // revert(abi.encodeWithSignature("Panic(uint256)", 0x11))
                revert(Error_selector_offset, Panic_error_length)
            }
        }
    }
}

/**
 * @title OrderCombiner
 * @author 0age
 * @notice OrderCombiner contains logic for fulfilling combinations of orders,
 *         either by matching offer items to consideration items or by
 *         fulfilling orders where available.
 */
contract OrderCombiner is OrderFulfiller, FulfillmentApplier {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) OrderFulfiller(conduitController) {}

    /**
     * @notice Internal function to attempt to fill a group of orders, fully or
     *         partially, with an arbitrary number of items for offer and
     *         consideration per order alongside criteria resolvers containing
     *         specific token identifiers and associated proofs. Any order that
     *         is not currently active, has already been fully filled, or has
     *         been cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or a conduit if indicated by
     *                                  the order) to transfer any relevant
     *                                  tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` in order to receive
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param criteriaResolvers         An array where each element contains a
     *                                  reference to a specific offer or
     *                                  consideration, a token identifier, and a
     *                                  proof that the supplied token identifier
     *                                  is contained in the merkle root held by
     *                                  the item in question's criteria element.
     *                                  Note that an empty criteria indicates
     *                                  that any (transferable) token
     *                                  identifier on the token in question is
     *                                  valid and that no associated proof needs
     *                                  to be supplied.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used (and
     *                                  direct approvals set on Consideration).
     * @param recipient                 The intended recipient for all received
     *                                  items.
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function _fulfillAvailableAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        FulfillmentComponent[][] memory offerFulfillments,
        FulfillmentComponent[][] memory considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient,
        uint256 maximumFulfilled
    ) internal returns (bool[] memory, /* availableOrders */ Execution[] memory /* executions */ ) {
        // Validate orders, apply amounts, & determine if they use conduits.
        (bytes32[] memory orderHashes, bool containsNonOpen) = _validateOrdersAndPrepareToFulfill(
            advancedOrders,
            criteriaResolvers,
            false, // Signifies that invalid orders should NOT revert.
            maximumFulfilled,
            recipient
        );

        // Aggregate used offer and consideration items and execute transfers.
        return _executeAvailableFulfillments(
            advancedOrders,
            offerFulfillments,
            considerationFulfillments,
            fulfillerConduitKey,
            recipient,
            orderHashes,
            containsNonOpen
        );
    }

    /**
     * @dev Internal function to validate a group of orders, update their
     *      statuses, reduce amounts by their previously filled fractions, apply
     *      criteria resolvers, and emit OrderFulfilled events. Note that this
     *      function needs to be called before
     *      _aggregateValidFulfillmentConsiderationItems to set the memory
     *      layout that _aggregateValidFulfillmentConsiderationItems depends on.
     *
     * @param advancedOrders    The advanced orders to validate and reduce by
     *                          their previously filled amounts.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          a root of zero indicates that any transferable
     *                          token identifier is valid and that no proof
     *                          needs to be supplied.
     * @param revertOnInvalid   A boolean indicating whether to revert on any
     *                          order being invalid; setting this to false will
     *                          instead cause the invalid order to be skipped.
     * @param maximumFulfilled  The maximum number of orders to fulfill.
     * @param recipient         The intended recipient for all items that do not
     *                          already have a designated recipient and are not
     *                          already used as part of a provided fulfillment.
     *
     * @return orderHashes     The hashes of the orders being fulfilled.
     * @return containsNonOpen A boolean indicating whether any restricted or
     *                         contract orders are present within the provided
     *                         array of advanced orders.
     */
    function _validateOrdersAndPrepareToFulfill(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        bool revertOnInvalid,
        uint256 maximumFulfilled,
        address recipient
    ) internal returns (bytes32[] memory orderHashes, bool containsNonOpen) {
        // Ensure this function cannot be triggered during a reentrant call.
        _setReentrancyGuard(true); // Native tokens accepted during execution.

        // Declare an error buffer indicating status of any native offer items.
        // Native tokens may only be provided as part of contract orders or when
        // fulfilling via matchOrders or matchAdvancedOrders; if bits indicating
        // these conditions are not met have been set, throw.
        uint256 invalidNativeOfferItemErrorBuffer;

        // Use assembly to set the value for the second bit of the error buffer.
        assembly {
            /**
             * Use the 231st bit of the error buffer to indicate whether the
             * current function is not matchAdvancedOrders or matchOrders.
             *
             * sig                                func
             * -----------------------------------------------------------------
             * 1010100000010111010001000 0 000100 matchOrders
             * 1111001011010001001010110 0 010010 matchAdvancedOrders
             * 1110110110011000101001010 1 110100 fulfillAvailableOrders
             * 1000011100100000000110110 1 000001 fulfillAvailableAdvancedOrders
             *                           ^ 7th bit
             */
            invalidNativeOfferItemErrorBuffer := and(NonMatchSelector_MagicMask, calldataload(0))
        }

        // Declare variables for later use.
        AdvancedOrder memory advancedOrder;
        uint256 terminalMemoryOffset;

        unchecked {
            // Read length of orders array and place on the stack.
            uint256 totalOrders = advancedOrders.length;

            // Track the order hash for each order being fulfilled.
            orderHashes = new bytes32[](totalOrders);

            // Determine the memory offset to terminate on during loops.
            terminalMemoryOffset = (totalOrders + 1) << OneWordShift;
        }

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Declare inner variables.
            OfferItem[] memory offer;
            ConsiderationItem[] memory consideration;

            // Iterate over each order.
            for (uint256 i = OneWord; i < terminalMemoryOffset; i += OneWord) {
                // Retrieve order using assembly to bypass out-of-range check.
                assembly {
                    advancedOrder := mload(add(advancedOrders, i))
                }

                // Determine if max number orders have already been fulfilled.
                if (maximumFulfilled == 0) {
                    // Mark fill fraction as zero as the order will not be used.
                    advancedOrder.numerator = 0;

                    // Continue iterating through the remaining orders.
                    continue;
                }

                // Validate it, update status, and determine fraction to fill.
                (bytes32 orderHash, uint256 numerator, uint256 denominator) =
                    _validateOrderAndUpdateStatus(advancedOrder, revertOnInvalid);

                // Do not track hash or adjust prices if order is not fulfilled.
                if (numerator == 0) {
                    // Mark fill fraction as zero if the order is not fulfilled.
                    advancedOrder.numerator = 0;

                    // Continue iterating through the remaining orders.
                    continue;
                }

                // Otherwise, track the order hash in question.
                assembly {
                    mstore(add(orderHashes, i), orderHash)
                }

                // Decrement the number of fulfilled orders.
                // Skip underflow check as the condition before
                // implies that maximumFulfilled > 0.
                --maximumFulfilled;

                // Place the start time for the order on the stack.
                uint256 startTime = advancedOrder.parameters.startTime;

                // Place the end time for the order on the stack.
                uint256 endTime = advancedOrder.parameters.endTime;

                // Retrieve array of offer items for the order in question.
                offer = advancedOrder.parameters.offer;

                // Read length of offer array and place on the stack.
                uint256 totalOfferItems = offer.length;

                {
                    // Determine the order type, used to check for eligibility
                    // for native token offer items as well as for the presence
                    // of restricted and contract orders (or non-open orders).
                    OrderType orderType = advancedOrder.parameters.orderType;

                    // Utilize assembly to efficiently check for order types.
                    // Note that these checks expect that there are no order
                    // types beyond the current set (0-4) and will need to be
                    // modified if more order types are added.
                    assembly {
                        // Declare a variable indicating if the order is not a
                        // contract order. Cache in scratch space to avoid stack
                        // depth errors.
                        let isNonContract := lt(orderType, 4)
                        mstore(0, isNonContract)

                        // Update the variable indicating if the order is not an
                        // open order, remaining set if it has been set already.
                        containsNonOpen := or(containsNonOpen, gt(orderType, 1))
                    }
                }

                // Iterate over each offer item on the order.
                for (uint256 j = 0; j < totalOfferItems; ++j) {
                    // Retrieve the offer item.
                    OfferItem memory offerItem = offer[j];

                    // If the offer item is for the native token and the order
                    // type is not a contract order type, set the first bit of
                    // the error buffer to true.
                    assembly {
                        invalidNativeOfferItemErrorBuffer :=
                            or(invalidNativeOfferItemErrorBuffer, lt(mload(offerItem), mload(0)))
                    }

                    // Apply order fill fraction to offer item end amount.
                    uint256 endAmount = _getFraction(numerator, denominator, offerItem.endAmount);

                    // Reuse same fraction if start and end amounts are equal.
                    if (offerItem.startAmount == offerItem.endAmount) {
                        // Apply derived amount to both start and end amount.
                        offerItem.startAmount = endAmount;
                    } else {
                        // Apply order fill fraction to offer item start amount.
                        offerItem.startAmount = _getFraction(numerator, denominator, offerItem.startAmount);
                    }

                    // Adjust offer amount using current time; round down.
                    uint256 currentAmount = _locateCurrentAmount(
                        offerItem.startAmount,
                        endAmount,
                        startTime,
                        endTime,
                        false // round down
                    );

                    // Update amounts in memory to match the current amount.
                    // Note that the end amount is used to track spent amounts.
                    offerItem.startAmount = currentAmount;
                    offerItem.endAmount = currentAmount;
                }

                // Retrieve array of consideration items for order in question.
                consideration = (advancedOrder.parameters.consideration);

                // Read length of consideration array and place on the stack.
                uint256 totalConsiderationItems = consideration.length;

                // Iterate over each consideration item on the order.
                for (uint256 j = 0; j < totalConsiderationItems; ++j) {
                    // Retrieve the consideration item.
                    ConsiderationItem memory considerationItem = (consideration[j]);

                    // Apply fraction to consideration item end amount.
                    uint256 endAmount = _getFraction(numerator, denominator, considerationItem.endAmount);

                    // Reuse same fraction if start and end amounts are equal.
                    if (considerationItem.startAmount == considerationItem.endAmount) {
                        // Apply derived amount to both start and end amount.
                        considerationItem.startAmount = endAmount;
                    } else {
                        // Apply fraction to consideration item start amount.
                        considerationItem.startAmount =
                            _getFraction(numerator, denominator, considerationItem.startAmount);
                    }

                    // Adjust consideration amount using current time; round up.
                    uint256 currentAmount = (
                        _locateCurrentAmount(
                            considerationItem.startAmount,
                            endAmount,
                            startTime,
                            endTime,
                            true // round up
                        )
                    );

                    considerationItem.startAmount = currentAmount;

                    // Utilize assembly to manually "shift" the recipient value,
                    // then to copy the start amount to the recipient.
                    // Note that this sets up the memory layout that is
                    // subsequently relied upon by
                    // _aggregateValidFulfillmentConsiderationItems.
                    assembly {
                        // Derive the pointer to the recipient using the item
                        // pointer along with the offset to the recipient.
                        let considerationItemRecipientPtr :=
                            add(
                                considerationItem,
                                ConsiderationItem_recipient_offset // recipient
                            )

                        // Write recipient to endAmount, as endAmount is not
                        // used from this point on and can be repurposed to fit
                        // the layout of a ReceivedItem.
                        mstore(
                            add(
                                considerationItem,
                                ReceivedItem_recipient_offset // old endAmount
                            ),
                            mload(considerationItemRecipientPtr)
                        )

                        // Write startAmount to recipient, as recipient is not
                        // used from this point on and can be repurposed to
                        // track received amounts.
                        mstore(considerationItemRecipientPtr, currentAmount)
                    }
                }
            }
        }

        // If the first bit is set, a native offer item was encountered on an
        // order that is not a contract order. If the 231st bit is set in the
        // error buffer, the current function is not matchOrders or
        // matchAdvancedOrders. If the value is 1 + (1 << 230), then both the
        // 1st and 231st bits were set; in that case, revert with an error.
        if (invalidNativeOfferItemErrorBuffer == NonMatchSelector_InvalidErrorValue) {
            _revertInvalidNativeOfferItem();
        }

        // Apply criteria resolvers to each order as applicable.
        _applyCriteriaResolvers(advancedOrders, criteriaResolvers);

        // Emit an event for each order signifying that it has been fulfilled.
        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            bytes32 orderHash;

            // Iterate over each order.
            for (uint256 i = OneWord; i < terminalMemoryOffset; i += OneWord) {
                assembly {
                    orderHash := mload(add(orderHashes, i))
                }

                // Do not emit an event if no order hash is present.
                if (orderHash == bytes32(0)) {
                    continue;
                }

                // Retrieve order using assembly to bypass out-of-range check.
                assembly {
                    advancedOrder := mload(add(advancedOrders, i))
                }

                // Retrieve parameters for the order in question.
                OrderParameters memory orderParameters = (advancedOrder.parameters);

                // Emit an OrderFulfilled event.
                _emitOrderFulfilledEvent(
                    orderHash,
                    orderParameters.offerer,
                    orderParameters.zone,
                    recipient,
                    orderParameters.offer,
                    orderParameters.consideration
                );
            }
        }
    }

    /**
     * @dev Internal function to fulfill a group of validated orders, fully or
     *      partially, with an arbitrary number of items for offer and
     *      consideration per order and to execute transfers. Any order that is
     *      not currently active, has already been fully filled, or has been
     *      cancelled will be omitted. Remaining offer and consideration items
     *      will then be aggregated where possible as indicated by the supplied
     *      offer and consideration component arrays and aggregated items will
     *      be transferred to the fulfiller or to each intended recipient,
     *      respectively. Note that a failing item transfer or an issue with
     *      order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or the conduit if indicated by
     *                                  the order) to transfer any relevant
     *                                  tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` in order to receive
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used, with
     *                                  direct approvals set on Consideration.
     * @param recipient                 The intended recipient for all items
     *                                  that do not already have a designated
     *                                  recipient and are not already used as
     *                                  part of a provided fulfillment.
     * @param orderHashes               An array of order hashes for each order.
     * @param containsNonOpen           A boolean indicating whether any
     *                                  restricted or contract orders are
     *                                  present within the provided array of
     *                                  advanced orders.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function _executeAvailableFulfillments(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[][] memory offerFulfillments,
        FulfillmentComponent[][] memory considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient,
        bytes32[] memory orderHashes,
        bool containsNonOpen
    ) internal returns (bool[] memory availableOrders, Execution[] memory executions) {
        // Retrieve length of offer fulfillments array and place on the stack.
        uint256 totalOfferFulfillments = offerFulfillments.length;

        // Retrieve length of consideration fulfillments array & place on stack.
        uint256 totalConsiderationFulfillments = (considerationFulfillments.length);

        // Allocate an execution for each offer and consideration fulfillment.
        executions = new Execution[](
            totalOfferFulfillments + totalConsiderationFulfillments
        );

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Track number of filtered executions.
            uint256 totalFilteredExecutions = 0;

            // Iterate over each offer fulfillment.
            for (uint256 i = 0; i < totalOfferFulfillments;) {
                // Derive aggregated execution corresponding with fulfillment.
                Execution memory execution = _aggregateAvailable(
                    advancedOrders, Side.OFFER, offerFulfillments[i], fulfillerConduitKey, recipient
                );

                // If the execution is filterable...
                if (_isFilterableExecution(execution)) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[i - totalFilteredExecutions] = execution;
                }

                // Increment iterator.
                ++i;
            }

            // Iterate over each consideration fulfillment.
            for (uint256 i = 0; i < totalConsiderationFulfillments;) {
                // Derive aggregated execution corresponding with fulfillment.
                Execution memory execution = _aggregateAvailable(
                    advancedOrders,
                    Side.CONSIDERATION,
                    considerationFulfillments[i],
                    fulfillerConduitKey,
                    address(0) // unused
                );

                // If the execution is filterable...
                if (_isFilterableExecution(execution)) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[i + totalOfferFulfillments - totalFilteredExecutions] = execution;
                }

                // Increment iterator.
                ++i;
            }

            // If some number of executions have been filtered...
            if (totalFilteredExecutions != 0) {
                // reduce the total length of the executions array.
                assembly {
                    mstore(executions, sub(mload(executions), totalFilteredExecutions))
                }
            }
        }

        // Revert if no orders are available.
        if (executions.length == 0) {
            _revertNoSpecifiedOrdersAvailable();
        }

        // Perform final checks and return.
        availableOrders =
            _performFinalChecksAndExecuteOrders(advancedOrders, executions, orderHashes, recipient, containsNonOpen);

        return (availableOrders, executions);
    }

    /**
     * @dev Internal function to perform a final check that each consideration
     *      item for an arbitrary number of fulfilled orders has been met and to
     *      trigger associated executions, transferring the respective items.
     *
     * @param advancedOrders  The orders to check and perform executions for.
     * @param executions      An array of elements indicating the sequence of
     *                        transfers to perform when fulfilling the given
     *                        orders.
     * @param orderHashes     An array of order hashes for each order.
     * @param recipient       The intended recipient for all items that do not
     *                        already have a designated recipient and are not
     *                        used as part of a provided fulfillment.
     * @param containsNonOpen A boolean indicating whether any restricted or
     *                        contract orders are present within the provided
     *                        array of advanced orders.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     */
    function _performFinalChecksAndExecuteOrders(
        AdvancedOrder[] memory advancedOrders,
        Execution[] memory executions,
        bytes32[] memory orderHashes,
        address recipient,
        bool containsNonOpen
    ) internal returns (bool[] memory /* availableOrders */ ) {
        // Retrieve the length of the advanced orders array and place on stack.
        uint256 totalOrders = advancedOrders.length;

        // Initialize array for tracking available orders.
        bool[] memory availableOrders = new bool[](totalOrders);

        // Initialize an accumulator array. From this point forward, no new
        // memory regions can be safely allocated until the accumulator is no
        // longer being utilized, as the accumulator operates in an open-ended
        // fashion from this memory pointer; existing memory may still be
        // accessed and modified, however.
        bytes memory accumulator = new bytes(AccumulatorDisarmed);

        {
            // Declare a variable for the available native token balance.
            uint256 nativeTokenBalance;

            // Retrieve the length of the executions array and place on stack.
            uint256 totalExecutions = executions.length;

            // Iterate over each execution.
            for (uint256 i = 0; i < totalExecutions;) {
                // Retrieve the execution and the associated received item.
                Execution memory execution = executions[i];
                ReceivedItem memory item = execution.item;

                // If execution transfers native tokens, reduce value available.
                if (item.itemType == ItemType.NATIVE) {
                    // Get the current available balance of native tokens.
                    assembly {
                        nativeTokenBalance := selfbalance()
                    }

                    // Ensure that sufficient native tokens are still available.
                    if (item.amount > nativeTokenBalance) {
                        _revertInsufficientNativeTokensSupplied();
                    }
                }

                // Transfer the item specified by the execution.
                _transfer(item, execution.offerer, execution.conduitKey, accumulator);

                // Skip overflow check as for loop is indexed starting at zero.
                unchecked {
                    ++i;
                }
            }
        }

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Iterate over each order.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Retrieve the order in question.
                AdvancedOrder memory advancedOrder = advancedOrders[i];

                // Skip the order in question if not being not fulfilled.
                if (advancedOrder.numerator == 0) {
                    // Explicitly set availableOrders at the given index to
                    // guard against the possibility of dirtied memory.
                    availableOrders[i] = false;
                    continue;
                }

                // Mark the order as available.
                availableOrders[i] = true;

                // Retrieve the order parameters.
                OrderParameters memory parameters = advancedOrder.parameters;

                {
                    // Retrieve offer items.
                    OfferItem[] memory offer = parameters.offer;

                    // Read length of offer array & place on the stack.
                    uint256 totalOfferItems = offer.length;

                    // Iterate over each offer item to restore it.
                    for (uint256 j = 0; j < totalOfferItems; ++j) {
                        // Retrieve the offer item in question.
                        OfferItem memory offerItem = offer[j];

                        // Transfer to recipient if unspent amount is not zero.
                        // Note that the transfer will not be reflected in the
                        // executions array.
                        if (offerItem.startAmount != 0) {
                            // Replace the endAmount parameter with the recipient to
                            // make offerItem compatible with the ReceivedItem input
                            // to _transfer and cache the original endAmount so it
                            // can be restored after the transfer.
                            uint256 originalEndAmount = _replaceEndAmountWithRecipient(offerItem, recipient);

                            // Transfer excess offer item amount to recipient.
                            _toOfferItemInput(_transfer)(
                                offerItem, parameters.offerer, parameters.conduitKey, accumulator
                            );

                            // Restore the original endAmount in offerItem.
                            assembly {
                                mstore(add(offerItem, ReceivedItem_recipient_offset), originalEndAmount)
                            }
                        }

                        // Restore original amount on the offer item.
                        offerItem.startAmount = offerItem.endAmount;
                    }
                }

                {
                    // Read consideration items & ensure they are fulfilled.
                    ConsiderationItem[] memory consideration = (parameters.consideration);

                    // Read length of consideration array & place on stack.
                    uint256 totalConsiderationItems = consideration.length;

                    // Iterate over each consideration item.
                    for (uint256 j = 0; j < totalConsiderationItems; ++j) {
                        ConsiderationItem memory considerationItem = (consideration[j]);

                        // Retrieve remaining amount on consideration item.
                        uint256 unmetAmount = considerationItem.startAmount;

                        // Revert if the remaining amount is not zero.
                        if (unmetAmount != 0) {
                            _revertConsiderationNotMet(i, j, unmetAmount);
                        }

                        // Utilize assembly to restore the original value.
                        assembly {
                            // Write recipient to startAmount.
                            mstore(
                                add(considerationItem, ReceivedItem_amount_offset),
                                mload(add(considerationItem, ConsiderationItem_recipient_offset))
                            )
                        }
                    }
                }
            }
        }

        // Trigger any accumulated transfers via call to the conduit.
        _triggerIfArmed(accumulator);

        // Determine whether any native token balance remains.
        uint256 remainingNativeTokenBalance;
        assembly {
            remainingNativeTokenBalance := selfbalance()
        }

        // Return any remaining native token balance to the caller.
        if (remainingNativeTokenBalance != 0) {
            _transferNativeTokens(payable(msg.sender), remainingNativeTokenBalance);
        }

        // If any restricted or contract orders are present in the group of
        // orders being fulfilled, perform any validateOrder or ratifyOrder
        // calls after all executions and related transfers are complete.
        if (containsNonOpen) {
            // Iterate over each order a second time.
            for (uint256 i = 0; i < totalOrders;) {
                // Ensure the order in question is being fulfilled.
                if (availableOrders[i]) {
                    // Check restricted orders and contract orders.
                    _assertRestrictedAdvancedOrderValidity(advancedOrders[i], orderHashes, orderHashes[i]);
                }

                // Skip overflow checks as for loop is indexed starting at zero.
                unchecked {
                    ++i;
                }
            }
        }

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        // Return the array containing available orders.
        return availableOrders;
    }

    /**
     * @dev Internal function to emit an OrdersMatched event using the same
     *      memory region as the existing order hash array.
     *
     * @param orderHashes An array of order hashes to include as an argument for
     *                    the OrdersMatched event.
     */
    function _emitOrdersMatched(bytes32[] memory orderHashes) internal {
        assembly {
            // Load the array length from memory.
            let length := mload(orderHashes)

            // Get the full size of the event data - one word for the offset,
            // one for the array length and one per hash.
            let dataSize := add(TwoWords, shl(OneWordShift, length))

            // Get pointer to start of data, reusing word before array length
            // for the offset.
            let dataPointer := sub(orderHashes, OneWord)

            // Cache the existing word in memory at the offset pointer.
            let cache := mload(dataPointer)

            // Write an offset of 32.
            mstore(dataPointer, OneWord)

            // Emit the OrdersMatched event.
            log1(dataPointer, dataSize, OrdersMatchedTopic0)

            // Restore the cached word.
            mstore(dataPointer, cache)
        }
    }

    /**
     * @dev Internal function to match an arbitrary number of full or partial
     *      orders, each with an arbitrary number of items for offer and
     *      consideration, supplying criteria resolvers containing specific
     *      token identifiers and associated proofs as well as fulfillments
     *      allocating offer components to consideration components.
     *
     * @param advancedOrders    The advanced orders to match. Note that both the
     *                          offerer and fulfiller on each order must first
     *                          approve this contract (or their conduit if
     *                          indicated by the order) to transfer any relevant
     *                          tokens on their behalf and each consideration
     *                          recipient must implement `onERC1155Received` in
     *                          order to receive ERC1155 tokens. Also note that
     *                          the offer and consideration components for each
     *                          order must have no remainder after multiplying
     *                          the respective amount with the supplied fraction
     *                          in order for the group of partial fills to be
     *                          considered valid.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          an empty root indicates that any (transferable)
     *                          token identifier is valid and that no associated
     *                          proof needs to be supplied.
     * @param fulfillments      An array of elements allocating offer components
     *                          to consideration components. Note that each
     *                          consideration component must be fully met in
     *                          order for the match operation to be valid.
     * @param recipient         The intended recipient for all unspent offer
     *                          item amounts.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function _matchAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        Fulfillment[] memory fulfillments,
        address recipient
    ) internal returns (Execution[] memory /* executions */ ) {
        // Validate orders, update order status, and determine item amounts.
        (bytes32[] memory orderHashes, bool containsNonOpen) = _validateOrdersAndPrepareToFulfill(
            advancedOrders,
            criteriaResolvers,
            true, // Signifies that invalid orders should revert.
            advancedOrders.length,
            recipient
        );

        // Emit OrdersMatched event, providing an array of matched order hashes.
        _emitOrdersMatched(orderHashes);

        // Fulfill the orders using the supplied fulfillments and recipient.
        return _fulfillAdvancedOrders(advancedOrders, fulfillments, orderHashes, recipient, containsNonOpen);
    }

    /**
     * @dev Internal function to fulfill an arbitrary number of orders, either
     *      full or partial, after validating, adjusting amounts, and applying
     *      criteria resolvers.
     *
     * @param advancedOrders  The orders to match, including a fraction to
     *                        attempt to fill for each order.
     * @param fulfillments    An array of elements allocating offer components
     *                        to consideration components. Note that the final
     *                        amount of each consideration component must be
     *                        zero for a match operation to be considered valid.
     * @param orderHashes     An array of order hashes for each order.
     * @param recipient       The intended recipient for all items that do not
     *                        already have a designated recipient and are not
     *                        used as part of a provided fulfillment.
     * @param containsNonOpen A boolean indicating whether any restricted or
     *                        contract orders are present within the provided
     *                        array of advanced orders.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function _fulfillAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        Fulfillment[] memory fulfillments,
        bytes32[] memory orderHashes,
        address recipient,
        bool containsNonOpen
    ) internal returns (Execution[] memory executions) {
        // Retrieve fulfillments array length and place on the stack.
        uint256 totalFulfillments = fulfillments.length;

        // Allocate executions by fulfillment and apply them to each execution.
        executions = new Execution[](totalFulfillments);

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Track number of filtered executions.
            uint256 totalFilteredExecutions = 0;

            // Iterate over each fulfillment.
            for (uint256 i = 0; i < totalFulfillments; ++i) {
                /// Retrieve the fulfillment in question.
                Fulfillment memory fulfillment = fulfillments[i];

                // Derive the execution corresponding with the fulfillment.
                Execution memory execution = _applyFulfillment(
                    advancedOrders, fulfillment.offerComponents, fulfillment.considerationComponents, i
                );

                // If the execution is filterable...
                if (_isFilterableExecution(execution)) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[i - totalFilteredExecutions] = execution;
                }
            }

            // If some number of executions have been filtered...
            if (totalFilteredExecutions != 0) {
                // reduce the total length of the executions array.
                assembly {
                    mstore(executions, sub(mload(executions), totalFilteredExecutions))
                }
            }
        }

        // Perform final checks and execute orders.
        _performFinalChecksAndExecuteOrders(advancedOrders, executions, orderHashes, recipient, containsNonOpen);

        // Return the executions array.
        return executions;
    }

    /**
     * @dev Internal pure function to determine whether a given execution is
     *      filterable and may be removed from the executions array. The offerer
     *      and the recipient must be the same address and the item type cannot
     *      indicate a native token transfer.
     *
     * @param execution The execution to check for filterability.
     *
     * @return filterable A boolean indicating whether the execution in question
     *                    can be filtered from the executions array.
     */
    function _isFilterableExecution(Execution memory execution) internal pure returns (bool filterable) {
        // Utilize assembly to efficiently determine if execution is filterable.
        assembly {
            // Retrieve the received item referenced by the execution.
            let item := mload(execution)

            // Determine whether the execution is filterable.
            filterable :=
                and(
                    // Determine if offerer and recipient are the same address.
                    eq(
                        // Retrieve the recipient's address from the received item.
                        mload(add(item, ReceivedItem_recipient_offset)),
                        // Retrieve the offerer's address from the execution.
                        mload(add(execution, Execution_offerer_offset))
                    ),
                    // Determine if received item's item type is non-zero, thereby
                    // indicating that the execution does not involve native tokens.
                    iszero(iszero(mload(item)))
                )
        }
    }
}

/**
 * @title Consideration
 * @author 0age (0age.eth)
 * @custom:coauthor d1ll0n (d1ll0n.eth)
 * @custom:coauthor transmissions11 (t11s.eth)
 * @custom:coauthor James Wenzel (emo.eth)
 * @custom:version 1.5
 * @notice Consideration is a generalized native token/ERC20/ERC721/ERC1155
 *         marketplace that provides lightweight methods for common routes as
 *         well as more flexible methods for composing advanced orders or groups
 *         of orders. Each order contains an arbitrary number of items that may
 *         be spent (the "offer") along with an arbitrary number of items that
 *         must be received back by the indicated recipients (the
 *         "consideration").
 */
contract Consideration is ConsiderationInterface, OrderCombiner {
    /**
     * @notice Derive and set hashes, reference chainId, and associated domain
     *         separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) OrderCombiner(conduitController) {}

    /**
     * @notice Accept native token transfers during execution that may then be
     *         used to facilitate native token transfers, where any tokens that
     *         remain will be transferred to the caller. Native tokens are only
     *         acceptable mid-fulfillment (and not during basic fulfillment).
     */
    receive() external payable {
        // Ensure the reentrancy guard is currently set to accept native tokens.
        _assertAcceptingNativeTokens();
    }

    /**
     * @notice Fulfill an order offering an ERC20, ERC721, or ERC1155 item by
     *         supplying Ether (or other native tokens), ERC20 tokens, an ERC721
     *         item, or an ERC1155 item as consideration. Six permutations are
     *         supported: Native token to ERC721, Native token to ERC1155, ERC20
     *         to ERC721, ERC20 to ERC1155, ERC721 to ERC20, and ERC1155 to
     *         ERC20 (with native tokens supplied as msg.value). For an order to
     *         be eligible for fulfillment via this method, it must contain a
     *         single offer item (though that item may have a greater amount if
     *         the item is not an ERC721). An arbitrary number of "additional
     *         recipients" may also be supplied which will each receive native
     *         tokens or ERC20 items from the fulfiller as consideration. Refer
     *         to the documentation for a more comprehensive summary of how to
     *         utilize this method and what orders are compatible with it.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer and the fulfiller must first approve
     *                   this contract (or their chosen conduit if indicated)
     *                   before any tokens can be transferred. Also note that
     *                   contract recipients of ERC1155 consideration items must
     *                   implement `onERC1155Received` to receive those items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillBasicOrder(BasicOrderParameters calldata parameters)
        external
        payable
        override
        returns (bool fulfilled)
    {
        // Validate and fulfill the basic order.
        fulfilled = _validateAndFulfillBasicOrder(parameters);
    }

    /**
     * @notice Fulfill an order offering an ERC20, ERC721, or ERC1155 item by
     *         supplying Ether (or other native tokens), ERC20 tokens, an ERC721
     *         item, or an ERC1155 item as consideration. Six permutations are
     *         supported: Native token to ERC721, Native token to ERC1155, ERC20
     *         to ERC721, ERC20 to ERC1155, ERC721 to ERC20, and ERC1155 to
     *         ERC20 (with native tokens supplied as msg.value). For an order to
     *         be eligible for fulfillment via this method, it must contain a
     *         single offer item (though that item may have a greater amount if
     *         the item is not an ERC721). An arbitrary number of "additional
     *         recipients" may also be supplied which will each receive native
     *         tokens or ERC20 items from the fulfiller as consideration. Refer
     *         to the documentation for a more comprehensive summary of how to
     *         utilize this method and what orders are compatible with it. Note
     *         that this function costs less gas than `fulfillBasicOrder` due to
     *         the zero bytes in the function selector (0x00000000) which also
     *         results in earlier function dispatch.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer and the fulfiller must first approve
     *                   this contract (or their chosen conduit if indicated)
     *                   before any tokens can be transferred. Also note that
     *                   contract recipients of ERC1155 consideration items must
     *                   implement `onERC1155Received` to receive those items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillBasicOrder_efficient_6GL6yc(BasicOrderParameters calldata parameters)
        external
        payable
        override
        returns (bool fulfilled)
    {
        // Validate and fulfill the basic order.
        fulfilled = _validateAndFulfillBasicOrder(parameters);
    }

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and
     *         consideration. Note that this function does not support
     *         criteria-based orders or partial filling of orders (though
     *         filling the remainder of a partially-filled order is supported).
     *
     * @custom:param order        The order to fulfill. Note that both the
     *                            offerer and the fulfiller must first approve
     *                            this contract (or the corresponding conduit if
     *                            indicated) to transfer any relevant tokens on
     *                            their behalf and that contracts must implement
     *                            `onERC1155Received` to receive ERC1155 tokens
     *                            as consideration.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used (and direct approvals set on
     *                            this contract).
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillOrder(
        /**
         * @custom:name order
         */
        Order calldata,
        bytes32 fulfillerConduitKey
    ) external payable override returns (bool fulfilled) {
        // Convert order to "advanced" order, then validate and fulfill it.
        fulfilled = _validateAndFulfillAdvancedOrder(
            _toAdvancedOrderReturnType(_decodeOrderAsAdvancedOrder)(CalldataStart.pptr()),
            new CriteriaResolver[](0), // No criteria resolvers supplied.
            fulfillerConduitKey,
            msg.sender
        );
    }

    /**
     * @notice Fill an order, fully or partially, with an arbitrary number of
     *         items for offer and consideration alongside criteria resolvers
     *         containing specific token identifiers and associated proofs.
     *
     * @custom:param advancedOrder     The order to fulfill along with the
     *                                 fraction of the order to attempt to fill.
     *                                 Note that both the offerer and the
     *                                 fulfiller must first approve this
     *                                 contract (or their conduit if indicated
     *                                 by the order) to transfer any relevant
     *                                 tokens on their behalf and that contracts
     *                                 must implement `onERC1155Received` to
     *                                 receive ERC1155 tokens as consideration.
     *                                 Also note that all offer and
     *                                 consideration components must have no
     *                                 remainder after multiplication of the
     *                                 respective amount with the supplied
     *                                 fraction for the partial fill to be
     *                                 considered valid.
     * @custom:param criteriaResolvers An array where each element contains a
     *                                 reference to a specific offer or
     *                                 consideration, a token identifier, and a
     *                                 proof that the supplied token identifier
     *                                 is contained in the merkle root held by
     *                                 the item in question's criteria element.
     *                                 Note that an empty criteria indicates
     *                                 that any (transferable) token identifier
     *                                 on the token in question is valid and
     *                                 that no associated proof needs to be
     *                                 supplied.
     * @param fulfillerConduitKey      A bytes32 value indicating what conduit,
     *                                 if any, to source the fulfiller's token
     *                                 approvals from. The zero hash signifies
     *                                 that no conduit should be used (and
     *                                 direct approvals set on this contract).
     * @param recipient                The intended recipient for all received
     *                                 items, with `address(0)` indicating that
     *                                 the caller should receive the items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillAdvancedOrder(
        /**
         * @custom:name advancedOrder
         */
        AdvancedOrder calldata,
        /**
         * @custom:name criteriaResolvers
         */
        CriteriaResolver[] calldata,
        bytes32 fulfillerConduitKey,
        address recipient
    ) external payable override returns (bool fulfilled) {
        // Validate and fulfill the order.
        fulfilled = _validateAndFulfillAdvancedOrder(
            _toAdvancedOrderReturnType(_decodeAdvancedOrder)(CalldataStart.pptr()),
            _toCriteriaResolversReturnType(_decodeCriteriaResolvers)(
                CalldataStart.pptr(Offset_fulfillAdvancedOrder_criteriaResolvers)
            ),
            fulfillerConduitKey,
            _substituteCallerForEmptyRecipient(recipient)
        );
    }

    /**
     * @notice Attempt to fill a group of orders, each with an arbitrary number
     *         of items for offer and consideration. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *         Note that this function does not support criteria-based orders or
     *         partial filling of orders (though filling the remainder of a
     *         partially-filled order is supported).
     *
     * @custom:param orders                    The orders to fulfill. Note that
     *                                         both the offerer and the
     *                                         fulfiller must first approve this
     *                                         contract (or the corresponding
     *                                         conduit if indicated) to transfer
     *                                         any relevant tokens on their
     *                                         behalf and that contracts must
     *                                         implement `onERC1155Received` to
     *                                         receive ERC1155 tokens as
     *                                         consideration.
     * @custom:param offerFulfillments         An array of FulfillmentComponent
     *                                         arrays indicating which offer
     *                                         items to attempt to aggregate
     *                                         when preparing executions. Note
     *                                         that any offer items not included
     *                                         as part of a fulfillment will be
     *                                         sent unaggregated to the caller.
     * @custom:param considerationFulfillments An array of FulfillmentComponent
     *                                         arrays indicating which
     *                                         consideration items to attempt to
     *                                         aggregate when preparing
     *                                         executions.
     * @param fulfillerConduitKey              A bytes32 value indicating what
     *                                         conduit, if any, to source the
     *                                         fulfiller's token approvals from.
     *                                         The zero hash signifies that no
     *                                         conduit should be used (and
     *                                         direct approvals set on this
     *                                         contract).
     * @param maximumFulfilled                 The maximum number of orders to
     *                                         fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function fulfillAvailableOrders(
        /**
         * @custom:name orders
         */
        Order[] calldata,
        /**
         * @custom:name offerFulfillments
         */
        FulfillmentComponent[][] calldata,
        /**
         * @custom:name considerationFulfillments
         */
        FulfillmentComponent[][] calldata,
        bytes32 fulfillerConduitKey,
        uint256 maximumFulfilled
    ) external payable override returns (bool[] memory, /* availableOrders */ Execution[] memory /* executions */ ) {
        // Convert orders to "advanced" orders and fulfill all available orders.
        return _fulfillAvailableAdvancedOrders(
            _toAdvancedOrdersReturnType(_decodeOrdersAsAdvancedOrders)(CalldataStart.pptr()), // Convert to advanced orders.
            new CriteriaResolver[](0), // No criteria resolvers supplied.
            _toNestedFulfillmentComponentsReturnType(_decodeNestedFulfillmentComponents)(
                CalldataStart.pptr(Offset_fulfillAvailableOrders_offerFulfillments)
            ),
            _toNestedFulfillmentComponentsReturnType(_decodeNestedFulfillmentComponents)(
                CalldataStart.pptr(Offset_fulfillAvailableOrders_considerationFulfillments)
            ),
            fulfillerConduitKey,
            msg.sender,
            maximumFulfilled
        );
    }

    /**
     * @notice Attempt to fill a group of orders, fully or partially, with an
     *         arbitrary number of items for offer and consideration per order
     *         alongside criteria resolvers containing specific token
     *         identifiers and associated proofs. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *
     * @custom:param advancedOrders            The orders to fulfill along with
     *                                         the fraction of those orders to
     *                                         attempt to fill. Note that both
     *                                         the offerer and the fulfiller
     *                                         must first approve this contract
     *                                         (or their conduit if indicated by
     *                                         the order) to transfer any
     *                                         relevant tokens on their behalf
     *                                         and that contracts must implement
     *                                         `onERC1155Received` to receive
     *                                         ERC1155 tokens as consideration.
     *                                         Also note that all offer and
     *                                         consideration components must
     *                                         have no remainder after
     *                                         multiplication of the respective
     *                                         amount with the supplied fraction
     *                                         for an order's partial fill
     *                                         amount to be considered valid.
     * @custom:param criteriaResolvers         An array where each element
     *                                         contains a reference to a
     *                                         specific offer or consideration,
     *                                         a token identifier, and a proof
     *                                         that the supplied token
     *                                         identifier is contained in the
     *                                         merkle root held by the item in
     *                                         question's criteria element. Note
     *                                         that an empty criteria indicates
     *                                         that any (transferable) token
     *                                         identifier on the token in
     *                                         question is valid and that no
     *                                         associated proof needs to be
     *                                         supplied.
     * @custom:param offerFulfillments         An array of FulfillmentComponent
     *                                         arrays indicating which offer
     *                                         items to attempt to aggregate
     *                                         when preparing executions. Note
     *                                         that any offer items not included
     *                                         as part of a fulfillment will be
     *                                         sent unaggregated to the caller.
     * @custom:param considerationFulfillments An array of FulfillmentComponent
     *                                         arrays indicating which
     *                                         consideration items to attempt to
     *                                         aggregate when preparing
     *                                         executions.
     * @param fulfillerConduitKey              A bytes32 value indicating what
     *                                         conduit, if any, to source the
     *                                         fulfiller's token approvals from.
     *                                         The zero hash signifies that no
     *                                         conduit should be used (and
     *                                         direct approvals set on this
     *                                         contract).
     * @param recipient                        The intended recipient for all
     *                                         received items, with `address(0)`
     *                                         indicating that the caller should
     *                                         receive the offer items.
     * @param maximumFulfilled                 The maximum number of orders to
     *                                         fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function fulfillAvailableAdvancedOrders(
        /**
         * @custom:name advancedOrders
         */
        AdvancedOrder[] calldata,
        /**
         * @custom:name criteriaResolvers
         */
        CriteriaResolver[] calldata,
        /**
         * @custom:name offerFulfillments
         */
        FulfillmentComponent[][] calldata,
        /**
         * @custom:name considerationFulfillments
         */
        FulfillmentComponent[][] calldata,
        bytes32 fulfillerConduitKey,
        address recipient,
        uint256 maximumFulfilled
    ) external payable override returns (bool[] memory, /* availableOrders */ Execution[] memory /* executions */ ) {
        // Fulfill all available orders.
        return _fulfillAvailableAdvancedOrders(
            _toAdvancedOrdersReturnType(_decodeAdvancedOrders)(CalldataStart.pptr()),
            _toCriteriaResolversReturnType(_decodeCriteriaResolvers)(
                CalldataStart.pptr(Offset_fulfillAvailableAdvancedOrders_criteriaResolvers)
            ),
            _toNestedFulfillmentComponentsReturnType(_decodeNestedFulfillmentComponents)(
                CalldataStart.pptr(Offset_fulfillAvailableAdvancedOrders_offerFulfillments)
            ),
            _toNestedFulfillmentComponentsReturnType(_decodeNestedFulfillmentComponents)(
                CalldataStart.pptr(Offset_fulfillAvailableAdvancedOrders_cnsdrationFlflmnts)
            ),
            fulfillerConduitKey,
            _substituteCallerForEmptyRecipient(recipient),
            maximumFulfilled
        );
    }

    /**
     * @notice Match an arbitrary number of orders, each with an arbitrary
     *         number of items for offer and consideration along with a set of
     *         fulfillments allocating offer components to consideration
     *         components. Note that this function does not support
     *         criteria-based or partial filling of orders (though filling the
     *         remainder of a partially-filled order is supported). Any unspent
     *         offer item amounts or native tokens will be transferred to the
     *         caller.
     *
     * @custom:param orders       The orders to match. Note that both the
     *                            offerer and fulfiller on each order must first
     *                            approve this contract (or their conduit if
     *                            indicated by the order) to transfer any
     *                            relevant tokens on their behalf and each
     *                            consideration recipient must implement
     *                            `onERC1155Received` to receive ERC1155 tokens.
     * @custom:param fulfillments An array of elements allocating offer
     *                            components to consideration components. Note
     *                            that each consideration component must be
     *                            fully met for the match operation to be valid,
     *                            and that any unspent offer items will be sent
     *                            unaggregated to the caller.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders. Note that unspent offer item amounts or native
     *                    tokens will not be reflected as part of this array.
     */
    function matchOrders(
        /**
         * @custom:name orders
         */
        Order[] calldata,
        /**
         * @custom:name fulfillments
         */
        Fulfillment[] calldata
    ) external payable override returns (Execution[] memory /* executions */ ) {
        // Convert to advanced, validate, and match orders using fulfillments.
        return _matchAdvancedOrders(
            _toAdvancedOrdersReturnType(_decodeOrdersAsAdvancedOrders)(CalldataStart.pptr()),
            new CriteriaResolver[](0), // No criteria resolvers supplied.
            _toFulfillmentsReturnType(_decodeFulfillments)(CalldataStart.pptr(Offset_matchOrders_fulfillments)),
            msg.sender
        );
    }

    /**
     * @notice Match an arbitrary number of full, partial, or contract orders,
     *         each with an arbitrary number of items for offer and
     *         consideration, supplying criteria resolvers containing specific
     *         token identifiers and associated proofs as well as fulfillments
     *         allocating offer components to consideration components. Any
     *         unspent offer item amounts will be transferred to the designated
     *         recipient (with the null address signifying to use the caller)
     *         and any unspent native tokens will be returned to the caller.
     *
     * @custom:param advancedOrders    The advanced orders to match. Note that
     *                                 both the offerer and fulfiller on each
     *                                 order must first approve this contract
     *                                 (or their conduit if indicated by the
     *                                 order) to transfer any relevant tokens on
     *                                 their behalf and each consideration
     *                                 recipient must implement
     *                                 `onERC1155Received` to receive ERC1155
     *                                 tokens. Also note that the offer and
     *                                 consideration components for each order
     *                                 must have no remainder after multiplying
     *                                 the respective amount with the supplied
     *                                 fraction for the group of partial fills
     *                                 to be considered valid.
     * @custom:param criteriaResolvers An array where each element contains a
     *                                 reference to a specific offer or
     *                                 consideration, a token identifier, and a
     *                                 proof that the supplied token identifier
     *                                 is contained in the merkle root held by
     *                                 the item in question's criteria element.
     *                                 Note that an empty criteria indicates
     *                                 that any (transferable) token identifier
     *                                 on the token in question is valid and
     *                                 that no associated proof needs to be
     *                                 supplied.
     * @custom:param fulfillments      An array of elements allocating offer
     *                                 components to consideration components.
     *                                 Note that each consideration component
     *                                 must be fully met for the match operation
     *                                 to be valid, and that any unspent offer
     *                                 items will be sent unaggregated to the
     *                                 designated recipient.
     * @param recipient                The intended recipient for all unspent
     *                                 offer item amounts, or the caller if the
     *                                 null address is supplied.
     *
     * @return executions An array of elements indicating the sequence of
     *                     transfers performed as part of matching the given
     *                     orders. Note that unspent offer item amounts or
     *                     native tokens will not be reflected as part of this
     *                     array.
     */
    function matchAdvancedOrders(
        /**
         * @custom:name advancedOrders
         */
        AdvancedOrder[] calldata,
        /**
         * @custom:name criteriaResolvers
         */
        CriteriaResolver[] calldata,
        /**
         * @custom:name fulfillments
         */
        Fulfillment[] calldata,
        address recipient
    ) external payable override returns (Execution[] memory /* executions */ ) {
        // Validate and match the advanced orders using supplied fulfillments.
        return _matchAdvancedOrders(
            _toAdvancedOrdersReturnType(_decodeAdvancedOrders)(CalldataStart.pptr()),
            _toCriteriaResolversReturnType(_decodeCriteriaResolvers)(
                CalldataStart.pptr(Offset_matchAdvancedOrders_criteriaResolvers)
            ),
            _toFulfillmentsReturnType(_decodeFulfillments)(CalldataStart.pptr(Offset_matchAdvancedOrders_fulfillments)),
            _substituteCallerForEmptyRecipient(recipient)
        );
    }

    /**
     * @notice Cancel an arbitrary number of orders. Note that only the offerer
     *         or the zone of a given order may cancel it. Callers should ensure
     *         that the intended order was cancelled by calling `getOrderStatus`
     *         and confirming that `isCancelled` returns `true`.
     *
     * @param orders The orders to cancel.
     *
     * @return cancelled A boolean indicating whether the supplied orders have
     *                   been successfully cancelled.
     */
    function cancel(OrderComponents[] calldata orders) external override returns (bool cancelled) {
        // Cancel the orders.
        cancelled = _cancel(orders);
    }

    /**
     * @notice Validate an arbitrary number of orders, thereby registering their
     *         signatures as valid and allowing the fulfiller to skip signature
     *         verification on fulfillment. Note that validated orders may still
     *         be unfulfillable due to invalid item amounts or other factors;
     *         callers should determine whether validated orders are fulfillable
     *         by simulating the fulfillment call prior to execution. Also note
     *         that anyone can validate a signed order, but only the offerer can
     *         validate an order without supplying a signature.
     *
     * @custom:param orders The orders to validate.
     *
     * @return validated A boolean indicating whether the supplied orders have
     *                   been successfully validated.
     */
    function validate(
        /**
         * @custom:name orders
         */
        Order[] calldata
    ) external override returns (bool /* validated */ ) {
        return _validate(_toOrdersReturnType(_decodeOrders)(CalldataStart.pptr()));
    }

    /**
     * @notice Cancel all orders from a given offerer with a given zone in bulk
     *         by incrementing a counter. Note that only the offerer may
     *         increment the counter.
     *
     * @return newCounter The new counter.
     */
    function incrementCounter() external override returns (uint256 newCounter) {
        // Increment current counter for the supplied offerer.  Note that the
        // counter is incremented by a large, quasi-random interval.
        newCounter = _incrementCounter();
    }

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @custom:param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(
        /**
         * @custom:name order
         */
        OrderComponents calldata
    ) external view override returns (bytes32 orderHash) {
        CalldataPointer orderPointer = CalldataStart.pptr();

        // Derive order hash by supplying order parameters along with counter.
        orderHash = _deriveOrderHash(
            _toOrderParametersReturnType(_decodeOrderComponentsAsOrderParameters)(orderPointer),
            // Read order counter
            orderPointer.offset(OrderParameters_counter_offset).readUint256()
        );
    }

    /**
     * @notice Retrieve the status of a given order by hash, including whether
     *         the order has been cancelled or validated and the fraction of the
     *         order that has been filled. Since the _orderStatus[orderHash]
     *         does not get set for contract orders, getOrderStatus will always
     *         return (false, false, 0, 0) for those hashes. Note that this
     *         function is susceptible to view reentrancy and so should be used
     *         with care when calling from other contracts.
     *
     * @param orderHash The order hash in question.
     *
     * @return isValidated A boolean indicating whether the order in question
     *                     has been validated (i.e. previously approved or
     *                     partially filled).
     * @return isCancelled A boolean indicating whether the order in question
     *                     has been cancelled.
     * @return totalFilled The total portion of the order that has been filled
     *                     (i.e. the "numerator").
     * @return totalSize   The total size of the order that is either filled or
     *                     unfilled (i.e. the "denominator").
     */
    function getOrderStatus(bytes32 orderHash)
        external
        view
        override
        returns (bool isValidated, bool isCancelled, uint256 totalFilled, uint256 totalSize)
    {
        // Retrieve the order status using the order hash.
        return _getOrderStatus(orderHash);
    }

    /**
     * @notice Retrieve the current counter for a given offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return counter The current counter.
     */
    function getCounter(address offerer) external view override returns (uint256 counter) {
        // Return the counter for the supplied offerer.
        counter = _getCounter(offerer);
    }

    /**
     * @notice Retrieve configuration information for this contract.
     *
     * @return version           The contract version.
     * @return domainSeparator   The domain separator for this contract.
     * @return conduitController The conduit Controller set for this contract.
     */
    function information()
        external
        view
        override
        returns (string memory version, bytes32 domainSeparator, address conduitController)
    {
        // Return the information for this contract.
        return _information();
    }

    /**
     * @dev Gets the contract offerer nonce for the specified contract offerer.
     *      Note that this function is susceptible to view reentrancy and so
     *      should be used with care when calling from other contracts.
     *
     * @param contractOfferer The contract offerer for which to get the nonce.
     *
     * @return nonce The contract offerer nonce.
     */
    function getContractOffererNonce(address contractOfferer) external view override returns (uint256 nonce) {
        nonce = _contractNonces[contractOfferer];
    }

    /**
     * @notice Retrieve the name of this contract.
     *
     * @return contractName The name of this contract.
     */
    function name() external pure override returns (string memory /* contractName */ ) {
        // Return the name of the contract.
        return _name();
    }
}

/**
 * @title Seaport
 * @custom:version 1.5
 * @author 0age (0age.eth)
 * @custom:coauthor d1ll0n (d1ll0n.eth)
 * @custom:coauthor transmissions11 (t11s.eth)
 * @custom:coauthor James Wenzel (emo.eth)
 * @custom:contributor Kartik (slokh.eth)
 * @custom:contributor LeFevre (lefevre.eth)
 * @custom:contributor Joseph Schiarizzi (CupOJoseph.eth)
 * @custom:contributor Aspyn Palatnick (stuckinaboot.eth)
 * @custom:contributor Stephan Min (stephanm.eth)
 * @custom:contributor Ryan Ghods (ralxz.eth)
 * @custom:contributor Daniel Viau (snotrocket.eth)
 * @custom:contributor hack3r-0m (hack3r-0m.eth)
 * @custom:contributor Diego Estevez (antidiego.eth)
 * @custom:contributor Chomtana (chomtana.eth)
 * @custom:contributor Saw-mon and Natalie (sawmonandnatalie.eth)
 * @custom:contributor 0xBeans (0xBeans.eth)
 * @custom:contributor 0x4non (punkdev.eth)
 * @custom:contributor Laurence E. Day (norsefire.eth)
 * @custom:contributor vectorized.eth (vectorized.eth)
 * @custom:contributor karmacoma (karmacoma.eth)
 * @custom:contributor horsefacts (horsefacts.eth)
 * @custom:contributor UncarvedBlock (uncarvedblock.eth)
 * @custom:contributor Zoraiz Mahmood (zorz.eth)
 * @custom:contributor William Poulin (wpoulin.eth)
 * @custom:contributor Rajiv Patel-O'Connor (rajivpoc.eth)
 * @custom:contributor tserg (tserg.eth)
 * @custom:contributor cygaar (cygaar.eth)
 * @custom:contributor Meta0xNull (meta0xnull.eth)
 * @custom:contributor gpersoon (gpersoon.eth)
 * @custom:contributor Matt Solomon (msolomon.eth)
 * @custom:contributor Weikang Song (weikangs.eth)
 * @custom:contributor zer0dot (zer0dot.eth)
 * @custom:contributor Mudit Gupta (mudit.eth)
 * @custom:contributor leonardoalt (leoalt.eth)
 * @custom:contributor cmichel (cmichel.eth)
 * @custom:contributor PraneshASP (pranesh.eth)
 * @custom:contributor JasperAlexander (jasperalexander.eth)
 * @custom:contributor Ellahi (ellahi.eth)
 * @custom:contributor zaz (1zaz1.eth)
 * @custom:contributor berndartmueller (berndartmueller.eth)
 * @custom:contributor dmfxyz (dmfxyz.eth)
 * @custom:contributor daltoncoder (dontkillrobots.eth)
 * @custom:contributor 0xf4ce (0xf4ce.eth)
 * @custom:contributor phaze (phaze.eth)
 * @custom:contributor hrkrshnn (hrkrshnn.eth)
 * @custom:contributor axic (axic.eth)
 * @custom:contributor leastwood (leastwood.eth)
 * @custom:contributor 0xsanson (sanson.eth)
 * @custom:contributor blockdev (blockd3v.eth)
 * @custom:contributor fiveoutofnine (fiveoutofnine.eth)
 * @custom:contributor shuklaayush (shuklaayush.eth)
 * @custom:contributor dravee (dravee.eth)
 * @custom:contributor 0xPatissier
 * @custom:contributor pcaversaccio
 * @custom:contributor David Eiber
 * @custom:contributor csanuragjain
 * @custom:contributor sach1r0
 * @custom:contributor twojoy0
 * @custom:contributor ori_dabush
 * @custom:contributor Daniel Gelfand
 * @custom:contributor okkothejawa
 * @custom:contributor FlameHorizon
 * @custom:contributor vdrg
 * @custom:contributor dmitriia
 * @custom:contributor bokeh-eth
 * @custom:contributor asutorufos
 * @custom:contributor rfart(rfa)
 * @custom:contributor Riley Holterhus
 * @custom:contributor big-tech-sux
 * @notice Seaport is a generalized native token/ERC20/ERC721/ERC1155
 *         marketplace with lightweight methods for common routes as well as
 *         more flexible methods for composing advanced orders or groups of
 *         orders. Each order contains an arbitrary number of items that may be
 *         spent (the "offer") along with an arbitrary number of items that must
 *         be received back by the indicated recipients (the "consideration").
 */
contract Seaport is Consideration {
    /**
     * @notice Derive and set hashes, reference chainId, and associated domain
     *         separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) Consideration(conduitController) {}

    /**
     * @dev Internal pure function to retrieve and return the name of this
     *      contract.
     *
     * @return The name of this contract.
     */
    function _name() internal pure override returns (string memory) {
        // Return the name of the contract.
        assembly {
            mstore(0x20, 0x20)
            mstore(0x47, 0x07536561706f7274)
            return(0x20, 0x60)
        }
    }

    /**
     * @dev Internal pure function to retrieve the name of this contract as a
     *      string that will be used to derive the name hash in the constructor.
     *
     * @return The name of this contract as a string.
     */
    function _nameString() internal pure override returns (string memory) {
        // Return the name of the contract.
        return "Seaport";
    }
}

