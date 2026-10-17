use alloy_sol_types::sol_data;

pub(crate) type ZonesTempoShape = (
    sol_data::Bytes,
    sol_data::String,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Array<sol_data::Array<sol_data::Bytes>>,
    sol_data::FixedArray<sol_data::Bytes, 2>,
    (sol_data::Address, sol_data::Uint<128>, sol_data::Bytes),
);
pub(crate) type EncoderShape = (
    sol_data::Bytes,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Array<sol_data::Array<sol_data::Bytes>>,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);
pub(crate) type PrimitiveShape = (
    sol_data::Bool,
    sol_data::Int<16>,
    sol_data::Int<256>,
    sol_data::Uint<8>,
    sol_data::Uint<256>,
    sol_data::FixedBytes<4>,
    sol_data::FixedBytes<32>,
    sol_data::Address,
);
pub(crate) type StaticArrayShape = (
    sol_data::FixedArray<sol_data::Uint<256>, 2>,
    sol_data::FixedArray<sol_data::FixedArray<sol_data::Address, 2>, 2>,
);
pub(crate) type MixedShape = (
    sol_data::Uint<256>,
    sol_data::Bytes,
    sol_data::Address,
    sol_data::Array<sol_data::Bytes>,
    sol_data::Bool,
    sol_data::FixedArray<sol_data::Bytes, 2>,
);
pub(crate) type NestedArrayShape = (
    sol_data::Array<sol_data::FixedArray<sol_data::Bytes, 2>>,
    sol_data::Array<(sol_data::Bytes, sol_data::String)>,
);
pub(crate) type EmptyFixedThenBytes =
    (sol_data::FixedArray<sol_data::Bytes, 0>, sol_data::Bytes);
