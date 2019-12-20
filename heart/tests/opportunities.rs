mod common;

use serde_json::Value;

pub use crate::common::*;

#[test]
fn get_all_opportunities() {
    assert_json_response("/opportunities", read_reference_json("opportunities.json"));
}

#[test]
fn filter_one_label() {
    assert_filtered_opportunities("/opportunities?labels=sport:Schwimmen", vec![3]);
}

#[test]
fn filter_multiple_label_values() {
    assert_filtered_opportunities(
        "/opportunities?labels=sport:Schwimmen,sport:Handball",
        vec![2, 3],
    );
}

#[test]
fn filter_multiple_labels() {
    assert_filtered_opportunities(
        "/opportunities?labels=sport:Schwimmen,category:Sportbetrieb",
        vec![3],
    );
    assert_filtered_opportunities(
        "/opportunities?labels=sport:Schwimmen,category:Vorstandsarbeit",
        vec![],
    );
}

fn assert_filtered_opportunities(uri: &str, indices: Vec<usize>) {
    if let Value::Array(all) = read_reference_json("opportunities.json") {
        let filtered = Value::Array(
            indices
                .iter()
                .cloned()
                .map(|i| all.get(i).unwrap().clone())
                .collect(),
        );
        assert_json_response(uri, filtered);
    } else {
        panic!("should return a list of opportunities");
    }
}
