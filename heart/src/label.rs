use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Label {
    pub key: String,
    pub values: Vec<String>,
}
