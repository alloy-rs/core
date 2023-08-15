#[cfg(all(test))]
mod test {
    use serde_json::Value;

    use crate::StateMutability;

    fn assert_json_eq(left: &str, right: &str) {
        let left: Value = serde_json::from_str(left).unwrap();
        let right: Value = serde_json::from_str(right).unwrap();
        assert_eq!(left, right);
    }

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

        assert_json_eq(json, &serde_json::to_string(&deserialized).unwrap());
    }
}
