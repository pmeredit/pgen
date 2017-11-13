use ast::*;
use mongo_functions::FUNCTIONS;

#[test]
fn sanity_test() {
    assert!(FUNCTIONS.contains_key("not"))
}

