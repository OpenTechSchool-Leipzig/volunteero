use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Organisation {
    id: String,
    name: String,
}
