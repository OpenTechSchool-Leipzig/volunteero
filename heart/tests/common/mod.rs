use std::fs;
use std::path::PathBuf;

use rocket::http::{ContentType, Status};
use rocket::local::Client;
use rocket::Rocket;

use heart::{rocket, UserFacingError};

pub fn default_rocket() -> Result<Rocket, UserFacingError> {
    rocket(&PathBuf::from("tests/sample_csvs/sample.csv"))
}

pub fn read_reference_json(filename: &str) -> serde_json::Value {
    let mut path = PathBuf::from("tests/reference_files");
    path.push(filename);

    let expected_json = fs::read_to_string(&path).expect("could not read reference file");
    serde_json::from_str(&expected_json).expect("reference file does not contain valid JSON")
}

pub fn assert_response(uri: &str, content_type: Option<ContentType>, expected_body: &str) {
    let rocket = default_rocket().expect("valid rocket instance");
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get(uri).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), content_type);
    assert_eq!(&response.body_string().unwrap(), &expected_body);
}

pub fn assert_json_response(uri: &str, expected_json: serde_json::Value) {
    let rocket = default_rocket().expect("valid rocket instance");
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get(uri).dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));

    let body: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).expect("response is no valid JSON");

    assert_json_diff::assert_json_eq!(body, expected_json);
}
