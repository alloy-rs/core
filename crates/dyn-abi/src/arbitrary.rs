// TODO: remove this after updating `ruint2`.
#![allow(clippy::arc_with_non_send_sync)]

use crate::{DynSolType, DynSolValue};
use alloc::{boxed::Box, string::String, vec::Vec};
use alloy_primitives::{Address, B256, I256, U256};
use arbitrary::size_hint;
use core::ops::RangeInclusive;
use proptest::{
    collection::{vec as vec_strategy, VecStrategy},
    prelude::*,
    strategy::{Flatten, Map, Recursive, TupleUnion, WA},
};

const DEPTH: u32 = 16;
const DESIZED_SIZE: u32 = 64;
const EXPECTED_BRANCH_SIZE: u32 = 32;

#[derive(Debug, derive_arbitrary::Arbitrary)]
enum Choice {
    Bool,
    Int,
    Uint,
    Address,
    FixedBytes,
    // CustomValue,
    Bytes,
    String,

    Array,
    FixedArray,
    Tuple,
    // CustomStruct,
}

#[inline]
const fn bytes_size(n: usize) -> usize {
    (n % 31) + 1
}

#[inline]
const fn int_size(n: usize) -> usize {
    let n = (n % 255) + 1;
    n + (8 - (n % 8))
}

impl<'a> arbitrary::Arbitrary<'a> for DynSolType {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        match u.arbitrary::<Choice>()? {
            Choice::Bool => Ok(Self::Bool),
            Choice::Int => u.arbitrary().map(int_size).map(Self::Int),
            Choice::Uint => u.arbitrary().map(int_size).map(Self::Uint),
            Choice::Address => Ok(Self::Address),
            Choice::FixedBytes => Ok(Self::FixedBytes(bytes_size(u.arbitrary()?))),
            Choice::Bytes => Ok(Self::Bytes),
            Choice::String => Ok(Self::String),
            Choice::Array => u.arbitrary().map(Self::Array),
            Choice::FixedArray => Ok(Self::FixedArray(u.arbitrary()?, u.int_in_range(1..=16)?)),
            Choice::Tuple => u.arbitrary().map(Self::Tuple),
        }
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        if depth == DEPTH as usize {
            (0, Some(0))
        } else {
            size_hint::and(
                u32::size_hint(depth),
                size_hint::or_all(&[usize::size_hint(depth), Self::size_hint(depth + 1)]),
            )
        }
    }
}

impl<'a> arbitrary::Arbitrary<'a> for DynSolValue {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        match u.arbitrary::<Choice>()? {
            Choice::Bool => u.arbitrary().map(Self::Bool),
            Choice::Int => Ok(Self::Int(u.arbitrary()?, int_size(u.arbitrary()?))),
            Choice::Uint => Ok(Self::Uint(u.arbitrary()?, int_size(u.arbitrary()?))),
            Choice::Address => u.arbitrary().map(Self::Address),
            Choice::FixedBytes => Ok(Self::FixedBytes(u.arbitrary()?, u.int_in_range(1..=32)?)),
            Choice::Bytes => u.arbitrary().map(Self::Bytes),
            Choice::String => u.arbitrary().map(Self::String),
            Choice::Array => u.arbitrary().map(Self::Array),
            Choice::FixedArray => u.arbitrary().map(Self::FixedArray),
            Choice::Tuple => u.arbitrary().map(Self::Tuple),
        }
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        if depth == DEPTH as usize {
            (0, Some(0))
        } else {
            size_hint::and(
                u32::size_hint(depth),
                size_hint::or_all(&[
                    B256::size_hint(depth),
                    usize::size_hint(depth),
                    Self::size_hint(depth + 1),
                ]),
            )
        }
    }
}

#[inline]
#[allow(dead_code)]
fn arbitrary_string(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<String> {
    let len = u.arbitrary::<u8>()?;
    let mut s = String::with_capacity(len as usize);
    for _ in 0..len {
        s.push(u.int_in_range(b'a'..=b'z')? as char);
    }
    Ok(s)
}

// rustscript
type ValueOfStrategy<S> = <S as Strategy>::Value;

type StratMap<S, T> = Map<S, fn(ValueOfStrategy<S>) -> T>;

type MappedWA<S, T> = WA<StratMap<S, T>>;

type Rec<T, S> = Recursive<T, fn(BoxedStrategy<T>) -> S>;

// we must explicitly the final types of the strategies
type TypeRecurseStrategy = TupleUnion<(
    WA<BoxedStrategy<DynSolType>>,                   // Basic
    MappedWA<BoxedStrategy<DynSolType>, DynSolType>, // Array
    MappedWA<(BoxedStrategy<DynSolType>, RangeInclusive<usize>), DynSolType>, // FixedArray
    MappedWA<VecStrategy<BoxedStrategy<DynSolType>>, DynSolType>, // Tuple
)>;
type TypeStrategy = Rec<DynSolType, TypeRecurseStrategy>;

type Flat<S, T> = Flatten<StratMap<S, T>>;
type ValueArrayStrategy = Flat<BoxedStrategy<DynSolValue>, VecStrategy<BoxedStrategy<DynSolValue>>>;

type ValueRecurseStrategy = TupleUnion<(
    WA<BoxedStrategy<DynSolValue>>,            // Basic
    MappedWA<ValueArrayStrategy, DynSolValue>, // Array
    MappedWA<ValueArrayStrategy, DynSolValue>, // FixedArray
    MappedWA<VecStrategy<BoxedStrategy<DynSolValue>>, DynSolValue>, // Tuple
)>;
type ValueStrategy = Rec<DynSolValue, ValueRecurseStrategy>;

impl proptest::arbitrary::Arbitrary for DynSolType {
    type Parameters = (u32, u32, u32);
    type Strategy = TypeStrategy;

    #[inline]
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with((DEPTH, DESIZED_SIZE, EXPECTED_BRANCH_SIZE))
    }

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let (depth, desired_size, expected_branch_size) = args;
        Self::leaf().prop_recursive(depth, desired_size, expected_branch_size, Self::recurse)
    }
}

impl DynSolType {
    #[inline]
    fn leaf() -> impl Strategy<Value = Self> {
        prop_oneof![
            Just(Self::Bool),
            Just(Self::Address),
            any::<usize>().prop_map(|x| Self::Int(int_size(x))),
            any::<usize>().prop_map(|x| Self::Uint(int_size(x))),
            (1..=32usize).prop_map(Self::FixedBytes),
            Just(Self::Bytes),
            Just(Self::String),
        ]
    }

    #[inline]
    fn recurse(element: BoxedStrategy<Self>) -> TypeRecurseStrategy {
        prop_oneof![
            1 => element.clone(),
            2 => element.clone().prop_map(|ty| Self::Array(Box::new(ty))),
            2 => (element.clone(), 1..=16).prop_map(|(ty, sz)| Self::FixedArray(Box::new(ty), sz)),
            2 => vec_strategy(element, 1..=16).prop_map(Self::Tuple),
        ]
    }
}

impl proptest::arbitrary::Arbitrary for DynSolValue {
    type Parameters = (u32, u32, u32);
    type Strategy = ValueStrategy;

    #[inline]
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with((DEPTH, DESIZED_SIZE, EXPECTED_BRANCH_SIZE))
    }

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let (depth, desired_size, expected_branch_size) = args;
        Self::leaf().prop_recursive(depth, desired_size, expected_branch_size, Self::recurse)
    }
}

impl DynSolValue {
    // TODO: make this `SBoxedStrategy` after updating `ruint2`.
    /// Create a [proptest strategy][Strategy] to generate [`DynSolValue`]s from
    /// the given type.
    pub fn type_strategy(ty: &DynSolType) -> BoxedStrategy<Self> {
        match ty {
            DynSolType::Bool => any::<bool>().prop_map(Self::Bool).boxed(),
            DynSolType::Address => any::<Address>().prop_map(Self::Address).boxed(),
            &DynSolType::Int(sz) => any::<I256>().prop_map(move |x| Self::Int(x, sz)).boxed(),
            &DynSolType::Uint(sz) => any::<U256>().prop_map(move |x| Self::Uint(x, sz)).boxed(),
            &DynSolType::FixedBytes(sz) => any::<B256>()
                .prop_map(move |x| Self::FixedBytes(x, sz))
                .boxed(),
            DynSolType::Bytes => any::<Vec<u8>>().prop_map(Self::Bytes).boxed(),
            DynSolType::String => any::<String>().prop_map(Self::String).boxed(),
            DynSolType::Array(ty) => {
                let element = Self::type_strategy(ty);
                vec_strategy(element, 1..=16).prop_map(Self::Array).boxed()
            }
            DynSolType::FixedArray(ty, sz) => {
                let element = Self::type_strategy(ty);
                vec_strategy(element, *sz)
                    .prop_map(Self::FixedArray)
                    .boxed()
            }
            DynSolType::Tuple(tys) => tys
                .iter()
                .map(Self::type_strategy)
                .collect::<Vec<_>>()
                .prop_map(Self::Tuple)
                .boxed(),
            t @ (DynSolType::CustomStruct { .. } | DynSolType::CustomValue { .. }) => {
                unimplemented!("DynSolValue::type_strategy({t:?})")
            }
        }
    }

    /// Create a [proptest strategy][Strategy] to generate [`DynSolValue`]s from
    /// the given value's type.
    #[inline]
    pub fn value_strategy(&self) -> BoxedStrategy<Self> {
        Self::type_strategy(&self.as_type().unwrap())
    }

    #[inline]
    fn leaf() -> impl Strategy<Value = Self> {
        prop_oneof![
            any::<bool>().prop_map(Self::Bool),
            any::<Address>().prop_map(Self::Address),
            int_strategy::<I256>().prop_map(|(x, sz)| Self::Int(x, sz)),
            int_strategy::<U256>().prop_map(|(x, sz)| Self::Uint(x, sz)),
            (any::<B256>(), 1..=32usize).prop_map(|(x, sz)| DynSolValue::FixedBytes(x, sz)),
            any::<Vec<u8>>().prop_map(Self::Bytes),
            any::<String>().prop_map(Self::String),
        ]
    }

    #[inline]
    fn recurse(element: BoxedStrategy<Self>) -> ValueRecurseStrategy {
        prop_oneof![
            1 => element.clone(),
            2 => Self::array_strategy(element.clone()).prop_map(Self::Array),
            2 => Self::array_strategy(element.clone()).prop_map(Self::FixedArray),
            2 => vec_strategy(element, 1..=16).prop_map(Self::Tuple),
        ]
    }

    /// Recursive array strategy that generates same-type arrays of up to 16
    /// elements.
    ///
    /// NOTE: this has to be a separate function so Rust can turn the closure
    /// type (`impl Fn`) into an `fn` type.
    ///
    /// If you manually inline this into the function above, the compiler will
    /// fail with "expected fn pointer, found closure":
    ///
    /// ```ignore (error)
    ///    error[E0308]: mismatched types
    ///    --> crates/dyn-abi/src/arbitrary.rs:264:18
    ///     |
    /// 261 | /         prop_oneof![
    /// 262 | |             1 => element.clone(),
    /// 263 | |             2 => Self::array_strategy(element.clone()).prop_map(Self::Array),
    /// 264 | |             2 => element.prop_flat_map(|x| vec_strategy(x.value_strategy(), 1..=16)).prop_map(Self::FixedArray),
    ///     | |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected fn pointer, found closure
    /// 265 | |             2 => vec_strategy(element, 1..=16).prop_map(Self::Tuple),
    /// 266 | |         ]
    ///     | |_________- arguments to this function are incorrect
    ///     |
    ///     = note: expected struct `Map<Flatten<Map<BoxedStrategy<DynSolValue>, fn(DynSolValue) -> VecStrategy<BoxedStrategy<DynSolValue>>>>, ...>`
    ///                found struct `Map<Flatten<Map<BoxedStrategy<DynSolValue>, [closure@arbitrary.rs:264:40]>>, ...>`
    /// ```
    #[inline]
    fn array_strategy(element: BoxedStrategy<Self>) -> ValueArrayStrategy {
        element.prop_flat_map(|x| vec_strategy(x.value_strategy(), 1..=16))
    }
}

#[inline]
fn int_strategy<T: Arbitrary>() -> impl Strategy<Value = (<T::Strategy as Strategy>::Value, usize)>
{
    (any::<T>(), any::<usize>().prop_map(int_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest!(
        #![proptest_config(ProptestConfig {
            cases: 1024,
            ..Default::default()
        })]

        #[test]
        fn fuzz_dyn_sol_type(ty: DynSolType) {
            let s = ty.sol_type_name();
            let parsed = DynSolType::parse(&s);
            prop_assert_eq!(parsed, Ok(ty), "type strings don't match");
        }

        #[test]
        fn fuzz_dyn_sol_value(value: DynSolValue) {
            let ty = value.as_type().expect("generated invalid type");
            // this shouldn't fail after the previous assertion
            let s = value.sol_type_name().unwrap();
            let data = value.encode_params();

            prop_assert_eq!(&s, &ty.sol_type_name(), "type strings don't match");

            prop_assert_eq!(s.parse::<DynSolType>(), Ok(ty.clone()), "types don't match {:?}", s);

            match ty.decode_params(&data) {
                Ok(decoded) => prop_assert_eq!(
                    &decoded,
                    &value,
                    "decoded value doesn't match {:?}\ndata: {:?}",
                    s,
                    hex::encode_prefixed(&data),
                ),
                Err(e) => prop_assert!(
                    false,
                    "failed to decode {s:?}: {e}\nvalue: {value:?}\ndata: {:?}",
                    hex::encode_prefixed(&data),
                ),
            }

            match &value {
                DynSolValue::Array(values) | DynSolValue::FixedArray(values) => {
                    prop_assert!(!values.is_empty());
                    let mut values = values.iter();
                    let ty = values.next().unwrap().as_type().unwrap();
                    prop_assert!(
                        values.all(|v| v.as_type().as_ref() == Some(&ty)),
                        "array elements have different types: {value:#?}",
                    );
                }
                _ => {}
            }
        }
    );
}
