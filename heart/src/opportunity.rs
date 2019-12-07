use crate::contact::Contact;
use crate::organisation::Organisation;
use crate::label::Label;

use crate::repository::Repository;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Opportunity<'a> {
    title: String,
    organisation: &'a Organisation,
    contact: &'a Contact,
    labels: Vec<Label>,
}

// TODO: DTO mit Stadtteil

// TODO: Beispieldaten ins Repository als HashMap und bei fetch_all ausliefern

#[derive(Debug, PartialEq)]
pub struct OpportunityRepository {}

impl<'a> Repository<Opportunity<'a>> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<&Opportunity<'a>> {
        vec![]
    }
}
