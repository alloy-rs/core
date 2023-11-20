macro_rules! assert_ser_de {
    ($type:ty, $value:expr) => {{
        let ser = serde_json::to_string(&$value).unwrap();
        let de: $type = serde_json::from_str(&ser).unwrap();
        assert_eq!(&$value, &de, "Original value and deserialized value do not match.");
    }};
}

macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {{
        let left_val: serde_json::Value = serde_json::from_str($left).unwrap();
        let right_val: serde_json::Value = serde_json::from_str($right).unwrap();
        assert_eq!(left_val, right_val, "JSON values are not equal: {} != {}", $left, $right);
    }};
}
