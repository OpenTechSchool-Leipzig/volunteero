use std::convert::TryFrom;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Contact {
    name: String,
    options: Vec<ContactOption>,
}

#[derive(Debug, PartialEq, Serialize)]
enum ContactOption {
    EMail(EMailAddress),
    Phone(PhoneNumber),
}

#[derive(Debug, PartialEq, Serialize)]
struct EMailAddress {
    address: String
}

impl TryFrom<String> for EMailAddress {
    type Error = String;

    fn try_from(s: String) -> Result<Self, String> {
        // TODO: validate
        Ok(Self { address: s })
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct PhoneNumber {
    number: String
}

impl TryFrom<String> for PhoneNumber {
    type Error = String;

    fn try_from(s: String) -> Result<Self, String> {
        // TODO: validate
        Ok(Self { number: s })
    }
}
