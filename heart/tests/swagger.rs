mod common;

use std::fs;

pub use crate::common::*;

#[test]
fn get_swagger_yaml() {
    let expected_body =
        fs::read_to_string("static/swagger.yaml").expect("could not read swagger.yaml");
    assert_response("/swagger.yaml", None, &expected_body);
}
