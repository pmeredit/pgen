#[test]
fn normalize_test() {
    assert!(::mongo_config::FUNCTIONS.contains_key("not"));
    assert!(::mongo_config::FUNCTIONS.contains_key(String::from("not").as_str()));
}
