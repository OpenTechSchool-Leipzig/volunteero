use crate::contact::Contact;
use crate::organisation::Organisation;
use crate::label::Label;

use crate::repository::Repository;

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Opportunity {
    title: String,
    organisation: Organisation,
    contact: Contact,
    labels: Vec<Label>,
}

// TODO: DTO mit Stadtteil

// TODO: Beispieldaten ins Repository als HashMap und bei fetch_all ausliefern

#[derive(Debug, PartialEq)]
pub struct OpportunityRepository {}

impl Repository<Opportunity> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<Opportunity> {
        vec![]
    }
}
