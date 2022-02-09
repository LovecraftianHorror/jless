#[test]
fn correct_json_parsing() {
    // Shouldn't parse
    // 5"5"
    assert!(jless::jsonparser::parse("5\"5\"".to_owned()).is_err());
}
