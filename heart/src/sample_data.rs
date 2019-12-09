use std::convert::TryFrom;

use crate::address::Address;
use crate::contact::ContactOption;
use crate::contact::{Contact, EMailAddress, PhoneNumber};
use crate::label::Label;
use crate::opportunity::Opportunity;
use crate::organisation::Organisation;

lazy_static! {
    pub static ref OPPORTUNITIES: Vec<Opportunity> = vec![Opportunity {
        job_description: "Trainer".to_string(),
        organisation: ORGANISATIONS.first().unwrap().clone(),
        locations: LOCATIONS.clone(),
        contact: CONTACTS.first().unwrap().clone(),
        labels: LABELS.clone()
    }];
    static ref ORGANISATIONS: Vec<Organisation> = vec![Organisation {
        id: "42".to_string(),
        name: "Fußball Verein Leipzig e.V.".to_string(),
    }];
    static ref LOCATIONS: Vec<Address> = vec![
        Address {
            name: "Kaninchenrennbahn".to_string(),
            street: "Musterallee".to_string(),
            house_number: "37 F".to_string(),
            postcode: "04229".to_string(),
            city: "Leipzig".to_string(),
        }
    ];
    static ref CONTACTS: Vec<Contact> = vec![Contact {
        name: "Herr Müller".to_string(),
        options: vec![
            ContactOption::EMail(
                EMailAddress::try_from(("mueller@fussball-leipzig.de".to_string(), "".to_string()))
                    .unwrap()
            ),
            ContactOption::Phone(
                PhoneNumber::try_from((
                    "+4927182818284".to_string(),
                    "Nur bis 18:00Uhr".to_string()
                ))
                .unwrap()
            ),
        ]
    }];
    static ref LABELS: Vec<Label> = vec![
        Label {
            key: "Sportart".to_string(),
            values: vec!["Fußball".to_string()]
        },
        Label {
            key: "Aufgabenfeld".to_string(),
            values: vec!["Vereinsleben".to_string(), "Trainigsbetrieb".to_string()]
        },
    ];
}
