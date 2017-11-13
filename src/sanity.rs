use ast::*;
use mongo_functions::FUNCTIONS;

#[test]
fn sanity_test() {
    assert!(FUNCTIONS.contains_key("not"))
}

pub trait Clarity {
    fn clarify(&self) -> Result<&Self, &'static str>;
}

