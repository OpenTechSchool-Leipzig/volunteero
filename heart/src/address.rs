use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Address {
    pub name: String,
    pub street: String,
    pub house_number: String,
    pub postcode: String,
    pub city: String,
}

impl Address {
    pub(crate) fn as_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.name, self.street, self.house_number, self.postcode, self.city
        )
    }
}
