extern crate serde_json;

use ::codegen::JsonType::*;
use linked_hash_map::LinkedHashMap;

#[test]
fn serialize_test() {
        let x = O(
            linked_hash_map![
               "foo".to_string() =>
                  Some(Box::new(A(
                          vec![
                            Some(Box::new(I(42))), 
                            Some(Box::new(F(42.0))), 
                            Some(Box::new(S("Hello".to_string())))
                          ]))), 
               "bar".to_string() =>
                  Some(Box::new(S("bar".to_string()))),
               "car".to_string() => 
                  None
            ]); 
    assert_eq!(&format!("{}", serde_json::to_string(&x).unwrap()),
		r#"{"foo":[42,42.0,"Hello"],"bar":"bar","car":null}"#)
}
