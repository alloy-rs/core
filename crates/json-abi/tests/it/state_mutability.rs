use alloy_json_abi::StateMutability;

#[test]
fn state_mutability() {
    let json = r#"
            [
                "pure",
                "view",
                "nonpayable",
                "payable"
            ]
        "#;

    let deserialized: Vec<StateMutability> = serde_json::from_str(json).unwrap();

    assert_eq!(
        deserialized,
        vec![
            StateMutability::Pure,
            StateMutability::View,
            StateMutability::NonPayable,
            StateMutability::Payable,
        ]
    );

    assert_json_eq!(json, &serde_json::to_string(&deserialized).unwrap());
}
