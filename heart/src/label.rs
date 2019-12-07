use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Label {
    key: String,
    values: Vec<String>,
}
