use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize Clone)]
pub struct Label {
    pub key: String,
    pub values: Vec<String>,
}
