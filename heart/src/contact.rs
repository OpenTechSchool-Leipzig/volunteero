use std::convert::TryFrom;

use serde::Serialize;

use regex::Regex;

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
    note: Option<String>,
}

impl TryFrom<(String, String)> for EMailAddress {
    type Error = String;

    fn try_from(raw_data: (String, String)) -> Result<Self, String> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        if email_regex.is_match(&raw_data.0) {
            Ok(Self {
                value: raw_data.0,
                note: extract_note(raw_data.1),
            })
        } else {
            Err(String::from("The e-mail has an incorrect format"))
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct PhoneNumber {
    value: String,
    note: Option<String>,
}

impl TryFrom<(String, String)> for PhoneNumber {
    type Error = String;

    fn try_from(raw_data: (String, String)) -> Result<Self, String> {
        // allow digits and white spaces, min. 5 max. 15 characters
        let phone_regex = Regex::new(r"^[(0-9\s)]{5,15}$").unwrap();
        if phone_regex.is_match(&raw_data.0) {
            Ok(Self {
                value: raw_data.0,
                note: extract_note(raw_data.1),
            })
        } else {
            Err(String::from("The phone number has an incorrect format"))
        }
    }
}

fn extract_note(maybe_note: String) -> Option<String> {
    match maybe_note {
        empty_string if empty_string.is_empty() => None,
        note => Some(note),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_correct_email() {
        let correct_email = String::from("someone@example.com");
        let note = String::from("Primary");
        let mail_tuple: (String, String) = (correct_email, note);

        let correct_email_expected = EMailAddress {
            value: String::from("someone@example.com"),
            note: Option::Some(String::from("Primary")),
        };
        assert_eq!(
            EMailAddress::try_from(mail_tuple),
            Ok(correct_email_expected)
        );
    }

    #[test]
    fn throw_err_for_email_without_at_sign() {
        let incorrect_email: (String, String) = (String::from("abc.de"), String::from(""));
        assert_eq!(
            EMailAddress::try_from(incorrect_email),
            Err(String::from("The e-mail has an incorrect format"))
        );
    }

    #[test]
    fn throw_err_for_email_without_characters_after_at_sign() {
        let incorrect_email: (String, String) = (String::from("3837@"), String::from(""));
        assert_eq!(
            EMailAddress::try_from(incorrect_email),
            Err(String::from("The e-mail has an incorrect format"))
        );
    }

    #[test]
    fn throw_err_for_email_with_dot_before_at_sign() {
        let incorrect_email: (String, String) = (String::from("opi.@d.de"), String::from(""));
        assert_eq!(
            EMailAddress::try_from(incorrect_email),
            Err(String::from("The e-mail has an incorrect format"))
        );
    }

    #[test]
    fn validate_correct_phone_number() {
        let correct_phone = String::from("123 456");
        let note = String::from("Primary phone number");
        let phone_tuple: (String, String) = (correct_phone, note);

        let correct_phone_expected = PhoneNumber {
            value: String::from("123 456"),
            note: Option::Some(String::from("Primary phone number")),
        };
        assert_eq!(
            PhoneNumber::try_from(phone_tuple),
            Ok(correct_phone_expected)
        );
    }

    #[test]
    fn throw_err_for_phone_number_with_incorrect_char() {
        let phone_incorrect_char: (String, String) =
            (String::from("0341 # 14384"), String::from(""));
        assert_eq!(
            PhoneNumber::try_from(phone_incorrect_char),
            Err(String::from("The phone number has an incorrect format"))
        );
    }

    #[test]
    fn throw_err_for_too_short_phone_number() {
        let phone_too_short: (String, String) = (String::from("0341"), String::from(""));
        assert_eq!(
            PhoneNumber::try_from(phone_too_short),
            Err(String::from("The phone number has an incorrect format"))
        );
    }

    fn throw_err_for_too_long() {
        let phone_too_long: (String, String) = (String::from("0345 3784 473472"), String::from(""));
        assert_eq!(
            PhoneNumber::try_from(phone_too_long),
            Err(String::from("The phone number has an incorrect format"))
        );
    }
}
