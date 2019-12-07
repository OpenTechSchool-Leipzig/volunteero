use serde::{Serialize, Deserialize};
use itertools::Itertools;
use std::convert::TryFrom;

use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Label {
    pub key: String,
    pub values: Vec<String>,
}

impl TryFrom<&str> for Label {
    type Error = String; // TODO: custom error type

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(":").collect();
        if parts.len() == 2 {
            Ok(Label {
                key: parts[0].into(),
                values: parts[1].split(",").map(|s| s.to_string()).collect(),
            })
        } else {
            Err(format!("invalid label format: {}", s))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LabelList(pub Vec<Label>);

impl<'v> FromFormValue<'v> for LabelList {
    type Error = String;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let labels = (form_value
            .to_string()
            .split(",")
            .map(|l| Label::try_from(l))
            .collect(): Result<Vec<Label>, String>)?
            .into_iter()
            .group_by(|l| l.key.clone())
            .into_iter()
            .map(|(key, labels)| Label {
                key,
                values: labels.flat_map(|l| l.values).collect(),
            })
            .collect();
        Ok(LabelList(labels))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_label() {
        assert_eq!(
            Label::try_from("foo:bar"),
            Ok(Label {
                key: "foo".into(),
                values: vec!["bar".into()]
            })
        );
        assert_eq!(
            Label::try_from("foo:bar"),
            Ok(Label {
                key: "foo".into(),
                values: vec!["bar".into()]
            })
        );
    }

    #[test]
    fn parse_label_list() {
        assert_eq!(
            FromFormValue::from_form_value("foo:bar".into()),
            Ok(LabelList(vec![Label {
                key: "foo".into(),
                values: vec!["bar".into()]
            }]))
        );
        assert_eq!(
            FromFormValue::from_form_value("foo:bar,baz:42,baz:43".into()),
            Ok(LabelList(vec![
                Label {
                    key: "foo".into(),
                    values: vec!["bar".into()]
                },
                Label {
                    key: "baz".into(),
                    values: vec!["42".into(), "43".into()]
                },
            ]))
        );
    }
}
