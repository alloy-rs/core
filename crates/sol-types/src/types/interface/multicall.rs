//! [`Multicall3`](https://github.com/mds1/multicall) interface bindings and call builder.

#![allow(missing_debug_implementations, missing_copy_implementations, clippy::missing_const_for_fn)]

use crate::{sol, Result, SolCall, SolStruct};
use alloc::vec::Vec;
use alloy_primitives::{Address, U256};
use core::{fmt, marker::PhantomData};

sol! {
/// [`Multicall3`](https://github.com/mds1/multicall) bindings.
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
interface IMulticall3 {
    struct Call {
        address target;
        bytes callData;
    }

    struct Call3 {
        address target;
        bool allowFailure;
        bytes callData;
    }

    struct Call3Value {
        address target;
        bool allowFailure;
        uint256 value;
        bytes callData;
    }

    struct Result {
        bool success;
        bytes returnData;
    }

    function aggregate(Call[] calldata calls) external payable returns (uint256 blockNumber, bytes[] memory returnData);

    function aggregate3(Call3[] calldata calls) external payable returns (Result[] memory returnData);

    function aggregate3Value(Call3Value[] calldata calls) external payable returns (Result[] memory returnData);

    function blockAndAggregate(
        Call[] calldata calls
    ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);

    function getBasefee() external view returns (uint256 basefee);

    function getBlockHash(uint256 blockNumber) external view returns (bytes32 blockHash);

    function getBlockNumber() external view returns (uint256 blockNumber);

    function getChainId() external view returns (uint256 chainid);

    function getCurrentBlockCoinbase() external view returns (address coinbase);

    function getCurrentBlockDifficulty() external view returns (uint256 difficulty);

    function getCurrentBlockGasLimit() external view returns (uint256 gaslimit);

    function getCurrentBlockTimestamp() external view returns (uint256 timestamp);

    function getEthBalance(address addr) external view returns (uint256 balance);

    function getLastBlockHash() external view returns (bytes32 blockHash);

    function tryAggregate(
        bool requireSuccess,
        Call[] calldata calls
    ) external payable returns (Result[] memory returnData);

    function tryBlockAndAggregate(
        bool requireSuccess,
        Call[] calldata calls
    ) external payable returns (uint256 blockNumber, bytes32 blockHash, Result[] memory returnData);
}
}

mod private {
    pub trait TupleSealed {}
    pub trait MarkerSealed {}
}
use private::{MarkerSealed, TupleSealed};

/// Marker trait for building specific calls to [`IMulticall3`] using [`MulticallBuilder`].
pub trait MulticallMarker: MarkerSealed {
    // Not public API.

    /// The corresponding [`IMulticall3`] function call type.
    #[doc(hidden)]
    type CallType: SolCall;

    /// The corresponding [`IMulticall3`] function call parameter struct.
    #[doc(hidden)]
    type CallStruct: SolStruct;

    /// [`CallStruct`](Self::CallStruct) additional parameters.
    #[doc(hidden)]
    type CallStructParameters: Default;

    /// Wrap the return value of the call.
    #[doc(hidden)]
    type ReturnWrap<T>;
    /// Wrap a single struct in the return value of the call.
    #[doc(hidden)]
    type ReturnWrapOne<T>;

    /// Create a new [`CallStruct`](Self::CallStruct) from the given encoded data and additional
    /// parameters.
    #[doc(hidden)]
    fn __multicall_new_call_struct(
        data: Vec<u8>,
        rest: Self::CallStructParameters,
    ) -> Self::CallStruct;

    /// ABI-encode the given calls.
    #[doc(hidden)]
    #[allow(clippy::ptr_arg)]
    fn __multicall_abi_encode(calls: &Vec<Self::CallStruct>) -> Vec<u8>;
}

/// Marker type for building a call to [`aggregate`](IMulticall3::aggregateCall).
pub struct Aggregate(());
impl MarkerSealed for Aggregate {}
impl MulticallMarker for Aggregate {
    type CallType = IMulticall3::aggregateCall;
    type CallStruct = IMulticall3::Call;
    type CallStructParameters = Address;
    type ReturnWrap<T> = (U256, T);
    type ReturnWrapOne<T> = T;

    fn __multicall_new_call_struct(
        data: Vec<u8>,
        rest: Self::CallStructParameters,
    ) -> Self::CallStruct {
        IMulticall3::Call { target: rest, callData: data.into() }
    }

    fn __multicall_abi_encode(calls: &Vec<Self::CallStruct>) -> Vec<u8> {
        IMulticall3::aggregateCall { calls: calls.clone() }.abi_encode()
    }
}

/// Marker type for building a call to [`aggregate3`](IMulticall3::aggregate3Call).
pub struct Aggregate3(());
impl MarkerSealed for Aggregate3 {}
impl MulticallMarker for Aggregate3 {
    type CallType = IMulticall3::aggregate3Call;
    type CallStruct = IMulticall3::Call3;
    type CallStructParameters = (Address, bool);
    type ReturnWrap<T> = T;
    type ReturnWrapOne<T> = Result<T, Vec<u8>>;

    fn __multicall_new_call_struct(
        data: Vec<u8>,
        rest: Self::CallStructParameters,
    ) -> Self::CallStruct {
        IMulticall3::Call3 { target: rest.0, allowFailure: rest.1, callData: data.into() }
    }

    fn __multicall_abi_encode(calls: &Vec<Self::CallStruct>) -> Vec<u8> {
        IMulticall3::aggregate3Call { calls: calls.clone() }.abi_encode()
    }
}

/// Marker type for building a call to [`aggregate3Value`](IMulticall3::aggregate3ValueCall).
pub struct Aggregate3Value(());
impl MarkerSealed for Aggregate3Value {}
impl MulticallMarker for Aggregate3Value {
    type CallType = IMulticall3::aggregate3ValueCall;
    type CallStruct = IMulticall3::Call3Value;
    type CallStructParameters = (Address, bool, U256);
    type ReturnWrap<T> = T;
    type ReturnWrapOne<T> = Result<T, Vec<u8>>;

    fn __multicall_new_call_struct(
        data: Vec<u8>,
        rest: Self::CallStructParameters,
    ) -> Self::CallStruct {
        IMulticall3::Call3Value {
            target: rest.0,
            allowFailure: rest.1,
            value: rest.2,
            callData: data.into(),
        }
    }

    fn __multicall_abi_encode(calls: &Vec<Self::CallStruct>) -> Vec<u8> {
        IMulticall3::aggregate3ValueCall { calls: calls.clone() }.abi_encode()
    }
}

/// Append a type to a tuple.
pub trait TuplePush<T>: TupleSealed {
    // Not public API.

    /// This tuple type with the given type appended to it.
    #[doc(hidden)]
    type Pushed;
}

/// A tuple of [`SolCall`]s.
pub trait SolCallTuple<C: MulticallMarker>: TupleSealed {
    // Not public API.

    /// The return type of the call.
    #[doc(hidden)]
    type Returns;

    /// ABI-decode the return values.
    #[doc(hidden)]
    fn __multicall_abi_decode(data: &[u8], validate: bool) -> Result<Self::Returns>;
}

macro_rules! impl_tuples {
    ($count:literal $($ty:ident),+) => {
        impl <$($ty: SolCall,)+> TupleSealed for ($($ty,)+) {}

        impl<Value: SolCall, $($ty: SolCall,)+> TuplePush<Value> for ($($ty,)+) {
            type Pushed = ($($ty,)+ Value,);
        }

        impl<C: MulticallMarker, $($ty: SolCall,)+> SolCallTuple<C> for ($($ty,)+) {
            type Returns = C::ReturnWrap<($(C::ReturnWrapOne<$ty::Return>,)+)>;

            fn __multicall_abi_decode(data: &[u8], validate: bool) -> Result<Self::Returns> {
                let _ret = C::CallType::abi_decode_returns(data, validate)?;
                todo!()
            }
        }
    };
}

impl TupleSealed for () {}

impl<Value> TuplePush<Value> for () {
    type Pushed = (Value,);
}

impl<C: MulticallMarker> SolCallTuple<C> for () {
    type Returns = <C::CallType as SolCall>::Return;

    fn __multicall_abi_decode(data: &[u8], validate: bool) -> Result<Self::Returns> {
        // TODO: Copy from macro above
        C::CallType::abi_decode_returns(data, validate)
    }
}

all_the_tuples!(impl_tuples);

/// [`IMulticall3`] call builder.
///
/// Generic over the type of[`MulticallMarker`] and the tuple of calls.
///
/// # Examples
///
/// ```
/// use alloy_primitives::{Address, U256};
/// use alloy_sol_types::{sol, MulticallBuilder};
///
/// sol! {
///     interface ERC20 {
///         function totalSupply() external view returns (uint256 totalSupply);
///         function balanceOf(address owner) external view returns (uint256 balance);
///     }
/// }
///
/// // Create a builder for an `aggregate` call.
/// let token = Address::with_last_byte(1);
/// let builder = MulticallBuilder::new_aggregate()
///     .push(&ERC20::totalSupplyCall {}, token)
///     .push(&ERC20::balanceOfCall { owner: Address::with_last_byte(2) }, token);
/// let call_data = builder.abi_encode();
///
/// // Call the contract...
///
/// // Decode the return data.
/// # stringify!(
/// let return_data = b"...";
/// # );
/// # let return_data = &alloy_primitives::hex!("000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000003");
/// let (block_number, (total_supply, balance_of)): (
///     U256,
///     (ERC20::totalSupplyReturn, ERC20::balanceOfReturn),
/// ) = builder.abi_decode(return_data, true)?;
/// # Ok::<_, alloy_sol_types::Error>(())
/// ```
#[must_use]
pub struct MulticallBuilder<C: MulticallMarker, T = ()> {
    calls: Vec<C::CallStruct>,
    _phantom: PhantomData<T>,
}

impl<C: MulticallMarker, T> fmt::Debug for MulticallBuilder<C, T>
where
    C::CallStruct: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MulticallBuilder")
            .field("calls", &self.calls)
            .field("tuple", &core::any::type_name::<T>())
            .finish()
    }
}

impl MulticallBuilder<Aggregate, ()> {
    /// Create a new empty [`MulticallBuilder`] using [`aggregate`](IMulticall3::aggregateCall).
    #[inline]
    pub const fn new_aggregate() -> Self {
        Self::new()
    }
}
impl MulticallBuilder<Aggregate3, ()> {
    /// Create a new empty [`MulticallBuilder`] using [`aggregate3`](IMulticall3::aggregate3Call).
    #[inline]
    pub const fn new_aggregate3() -> Self {
        Self::new()
    }
}
impl MulticallBuilder<Aggregate3Value, ()> {
    /// Create a new empty [`MulticallBuilder`] using
    /// [`aggregate3Value`](IMulticall3::aggregate3ValueCall).
    #[inline]
    pub const fn new_aggregate3_value() -> Self {
        Self::new()
    }
}

impl<C: MulticallMarker> MulticallBuilder<C, ()> {
    /// Create a new empty [`MulticallBuilder`].
    #[inline]
    pub const fn new() -> Self {
        Self { calls: Vec::new(), _phantom: PhantomData }
    }
}

impl<C: MulticallMarker, Tuple> MulticallBuilder<C, Tuple> {
    /// Returns the call structs.
    pub fn calls(&self) -> &Vec<C::CallStruct> {
        &self.calls
    }

    /// Consumes the builder and returns the call structs.
    pub fn into_calls(self) -> Vec<C::CallStruct> {
        self.calls
    }

    /// Appends the call's encoded data to the builder's data, and return a new builder with the
    /// call's return type appended to the tuple.
    pub fn push<Call>(
        mut self,
        call: &Call,
        rest: C::CallStructParameters,
    ) -> MulticallBuilder<C, Tuple::Pushed>
    where
        Call: SolCall,
        Tuple: TuplePush<Call>,
    {
        self.calls.push(C::__multicall_new_call_struct(call.abi_encode(), rest));
        MulticallBuilder { calls: self.calls, _phantom: PhantomData }
    }

    /// ABI-encodes the calls.
    pub fn abi_encode(&self) -> Vec<u8> {
        C::__multicall_abi_encode(&self.calls)
    }
}

impl<C: MulticallMarker, Tuple: SolCallTuple<C>> MulticallBuilder<C, Tuple> {
    /// ABI-decodes the return values.
    #[inline]
    pub fn abi_decode(&self, data: &[u8], validate: bool) -> Result<Tuple::Returns> {
        self.abi_decoder().abi_decode(data, validate)
    }

    /// Returns a [`MulticallDecoder`] for the given [`MulticallBuilder`].
    #[inline]
    pub fn abi_decoder(&self) -> MulticallDecoder<C, Tuple> {
        MulticallDecoder::new()
    }
}

/// Marker type for decoding the return values of a [`MulticallBuilder`].
pub struct MulticallDecoder<C, Tuple> {
    _phantom: PhantomData<(C, Tuple)>,
}

impl<C: MulticallMarker, Tuple: SolCallTuple<C>> Default for MulticallDecoder<C, Tuple> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: MulticallMarker, Tuple: SolCallTuple<C>> MulticallDecoder<C, Tuple> {
    /// Creates a new [`MulticallDecoder`].
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }

    /// ABI-decodes the return values.
    pub fn abi_decode(&self, data: &[u8], validate: bool) -> Result<Tuple::Returns> {
        Tuple::__multicall_abi_decode(data, validate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SolValue;

    sol! {
        #[derive(Debug, PartialEq)]
        interface ERC20 {
            function totalSupply() external view returns (uint256 totalSupply);
            function balanceOf(address owner) external view returns (uint256 balance);
        }
    }

    #[test]
    fn basic_builder() {
        let token = Address::with_last_byte(1);

        let builder: MulticallBuilder<Aggregate, ()> = MulticallBuilder::new_aggregate();
        assert_eq!(builder.calls(), &[]);

        let call1 = ERC20::totalSupplyCall {};
        let builder: MulticallBuilder<Aggregate, (ERC20::totalSupplyCall,)> =
            builder.push(&call1, token);
        assert_eq!(
            builder.calls(),
            &[IMulticall3::Call { target: token, callData: call1.abi_encode().into() }]
        );

        let call2 = ERC20::balanceOfCall { owner: Address::with_last_byte(2) };
        let builder: MulticallBuilder<Aggregate, (ERC20::totalSupplyCall, ERC20::balanceOfCall)> =
            builder.push(&call2, token);
        assert_eq!(
            builder.calls(),
            &[
                IMulticall3::Call { target: token, callData: call1.abi_encode().into() },
                IMulticall3::Call { target: token, callData: call2.abi_encode().into() },
            ]
        );

        let encoded_data = builder.abi_encode();
        let expected = IMulticall3::aggregateCall {
            calls: vec![
                IMulticall3::Call {
                    target: token,
                    callData: ERC20::totalSupplyCall {}.abi_encode().into(),
                },
                IMulticall3::Call {
                    target: token,
                    callData: ERC20::balanceOfCall { owner: Address::with_last_byte(2) }
                        .abi_encode()
                        .into(),
                },
            ],
        };
        assert_eq!(encoded_data, expected.abi_encode());

        let return_data = IMulticall3::aggregateCall::abi_encode_returns(&(
            U256::from(1),
            &[U256::from(2).abi_encode(), U256::from(3).abi_encode()][..],
        ));
        eprintln!("return_data: {:?}", hex::encode(&return_data));
        let decoded = builder.abi_decode(&return_data, true).unwrap();
        assert_eq!(
            decoded,
            (
                U256::from(1),
                (
                    ERC20::totalSupplyReturn { totalSupply: U256::from(2) },
                    ERC20::balanceOfReturn { balance: U256::from(3) }
                )
            )
        );
    }
}
