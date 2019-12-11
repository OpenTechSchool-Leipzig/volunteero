mod common;

use std::path::PathBuf;

use heart::rocket;

use crate::common::default_rocket;

#[test]
fn crash_on_missing_csv() {
    assert_err("non-existing.csv", "No such file or directory");
}

#[ignore]
#[test]
fn crash_on_empty_csv() {
    assert_err("empty.csv", "empty.csv contains no opportunities");
}

#[test]
fn crash_on_parse_error() {
    assert_err("broken.csv", "missing field `organisation_id`");
}

#[test]
fn success_on_correct_csv() {
    if let Err(e) = default_rocket() {
        panic!("failed with: {}", e);
    }
}

fn assert_err(file_name: &str, expected_error: &str) {
    let mut path = PathBuf::from("tests/sample_csvs");
    path.push(file_name);
    match rocket(&path) {
        Err(e) => {
            let err = format!("{}", e);
            assert!(
                err.contains(expected_error),
                format!("«{}» does not contain «{}»", err, expected_error)
            );
        }
        _ => panic!(),
    };
}
