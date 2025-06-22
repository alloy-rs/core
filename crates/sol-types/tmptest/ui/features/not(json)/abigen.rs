use alloy_sol_types::sol;

sol!(EmptyStr, "");

sol!(PathDoesNotExist, "???");
sol!("pragma solidity ^0.8.0");
sol!("pragma solidity ^0.8.0;");

sol!(NoJsonFeature1, "{}");
sol!(NoJsonFeature2, "{ \"abi\": [] }");
sol!(NoJsonFeature3, "[]");

fn main() {}
