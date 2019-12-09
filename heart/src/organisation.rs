use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Organisation {
    pub id: String,
    pub name: String,
}
