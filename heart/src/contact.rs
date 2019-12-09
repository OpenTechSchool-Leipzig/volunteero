use std::convert::TryFrom;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Contact {
    pub name: String,
    pub options: Vec<ContactOption>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ContactOption {
    #[serde(rename = "email")]
    EMail(EMailAddress),
    #[serde(rename = "phone")]
    Phone(PhoneNumber),
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct EMailAddress {
    value: String,
    note: Option<String>
}

impl TryFrom<(String, String)> for EMailAddress {
    type Error = String;

    fn try_from(raw_data: (String, String)) -> Result<Self, String> {
        // TODO: validate
        Ok(Self { value: raw_data.0, note: extract_note(raw_data.1) })
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct PhoneNumber {
    value: String,
    note: Option<String>
}

impl TryFrom<(String, String)> for PhoneNumber {
    type Error = String;


    fn try_from(raw_data: (String, String)) -> Result<Self, String> {
        // TODO: validate

        Ok(Self { value: raw_data.0, note: extract_note(raw_data.1) })
    }
}

fn extract_note(maybe_note: String) -> Option<String> {
    match maybe_note {
        empty_string if empty_string.is_empty() => None,
        note => Some(note)
    }
}
