use crate::address::Address;
use crate::contact::Contact;
use crate::organisation::Organisation;
use crate::label::Label;

use crate::repository::Repository;

use serde::Serialize;
use crate::sample_data::OPPORTUNITIES;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Opportunity {
    pub title: String,
    pub organisation: Organisation,
    pub locations: Vec<Address>,
    pub contact: Contact,
    pub labels: Vec<Label>,
}

// TODO: DTO mit Stadtteil

// TODO: Beispieldaten ins Repository als HashMap und bei fetch_all ausliefern

#[derive(Debug, PartialEq)]
pub struct OpportunityRepository {}

impl Repository<Opportunity> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<Opportunity> {
       OPPORTUNITIES.clone()
    }
}
