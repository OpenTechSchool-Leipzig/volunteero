use std::convert::TryFrom;
use std::convert::TryInto;

use serde::Serialize;

use crate::contact::Contact;
use crate::contact::ContactOption;
use crate::dto::DTO;
use crate::geocoder::geocode;
use crate::label::Label;
use crate::location::{Address, Location};
use crate::organisation::Organisation;
use crate::repository::Repository;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Opportunity {
    pub job_description: String,
    pub organisation: Organisation,
    pub locations: Vec<Location>,
    pub contact: Contact,
    pub labels: Vec<Label>,
}

impl Opportunity {
    fn get_labels_for_key(&self, key: &str) -> Vec<String> {
        self.labels
            .clone()
            .into_iter()
            .find(|label| label.key == key)
            .map(|label| label.values)
            .unwrap_or_else(Vec::new)
    }
}

// TODO: DTO mit Stadtteil

// TODO: Beispieldaten ins Repository als HashMap und bei fetch_all ausliefern

pub enum OpportunityFilter {
    LabelFilter(Label),
}

#[derive(Debug, PartialEq)]
pub struct OpportunityRepository {
    pub data: Vec<Opportunity>,
}

impl Repository<Opportunity> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<Opportunity> {
        self.data.clone()
    }
}

impl OpportunityRepository {
    pub fn find(&self, filters: &[OpportunityFilter]) -> Vec<Opportunity> {
        self.data
            .clone()
            .into_iter()
            .filter(|opportunity| {
                filters.iter().all(|filter| match filter {
                    OpportunityFilter::LabelFilter(filter_label) => {
                        self.filter_by_labels(opportunity, filter_label)
                    }
                })
            })
            .collect()
    }

    fn filter_by_labels(&self, opportunity: &Opportunity, filter_label: &Label) -> bool {
        let values = &filter_label.values;
        values.iter().any(|l| {
            opportunity
                .get_labels_for_key(&filter_label.key)
                .contains(&l)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample_data::OPPORTUNITIES;

    #[test]
    fn filter_opportunities_by_single_label() {
        let repository = OpportunityRepository {
            data: OPPORTUNITIES.clone(),
        };
        let filter = vec![OpportunityFilter::LabelFilter(Label {
            key: "Sportart".to_string(),
            values: vec!["Fußball".to_string()],
        })];

        let actual = repository.find(&filter);

        assert_eq!(actual, OPPORTUNITIES.clone())
    }

    #[test]
    fn filter_opportunities_by_multiple_labels() {
        let repository = OpportunityRepository {
            data: OPPORTUNITIES.clone(),
        };
        let filter = vec![OpportunityFilter::LabelFilter(Label {
            key: "Aufgabenfeld".to_string(),
            values: vec!["Vereinsleben".to_string(), "Trainigsbetrieb".to_string()],
        })];

        let actual = repository.find(&filter);

        assert_eq!(actual, OPPORTUNITIES.clone())
    }

    #[test]
    fn filter_opportunities_by_none_partially_existing_label() {
        let repository = OpportunityRepository {
            data: OPPORTUNITIES.clone(),
        };
        let filter = vec![OpportunityFilter::LabelFilter(Label {
            key: "Sportart".to_string(),
            values: vec!["Eishockey".to_string(), "Fußball".to_string()],
        })];

        let actual = repository.find(&filter);

        assert_eq!(actual, OPPORTUNITIES.clone())
    }

    #[test]
    fn filter_opportunities_by_none_existing_label() {
        let repository = OpportunityRepository {
            data: OPPORTUNITIES.clone(),
        };
        let filter = vec![OpportunityFilter::LabelFilter(Label {
            key: "Sportart".to_string(),
            values: vec!["Eishockey".to_string()],
        })];

        let actual = repository.find(&filter);

        assert_eq!(actual, vec![])
    }
}

impl TryFrom<DTO> for Opportunity {
    type Error = String;

    fn try_from(raw_data: DTO) -> Result<Self, Self::Error> {
        let address = Address {
            name: raw_data.address_1_name.trim().to_string(),
            street: raw_data.address_1_street.trim().to_string(),
            house_number: raw_data.address_1_housenr.trim().to_string(),
            postcode: raw_data.address_1_postcode.trim().to_string(),
            city: raw_data.address_1_city.trim().to_string(),
        };

        // TODO: validate
        Ok(Self {
            job_description: raw_data.job_description.trim().to_string(),
            organisation: Organisation {
                id: raw_data.organisation_id,
                name: raw_data.organisation_name.trim().to_string(),
            },
            locations: vec![Location {
                coordinates: geocode(&address).ok(),
                address,
            }],
            contact: Contact {
                name: raw_data.contact_name,
                options: extract_options(
                    raw_data.contact_email.trim().to_string(),
                    raw_data.contact_phone.trim().to_string(),
                    raw_data.contact_mobile.trim().to_string(),
                ),
            },
            labels: extract_labels(
                raw_data.label_1,
                raw_data.label_2,
                raw_data.label_3,
                raw_data.label_4,
            ),
        })
    }
}

// TODO should return all addresses ||  now only returns one adress
fn extract_options(email: String, phone: String, _mobile: String) -> Vec<ContactOption> {
    use ContactOption::*;
    let results = vec![
        (email, "".into()).try_into().map(EMail),
        (phone, "".into()).try_into().map(Phone),
    ];
    results.iter().filter_map(|r| r.clone().ok()).collect()
}

fn extract_labels(
    label_1: String,
    label_2: String,
    label_3: String,
    label_4: String,
) -> Vec<Label> {
    let mut my_vector: Vec<Label> = vec![];

    if label_1.split('=').next().unwrap() != "" {
        my_vector.push(Label {
            key: label_1.split('=').next().unwrap().trim().to_string(),
            values: vec![label_1.split('=').nth(1).unwrap().to_string()],
        })
    }

    if label_2.split('=').next().unwrap() != "" {
        my_vector.push(Label {
            key: label_2.split('=').next().unwrap().trim().to_string(),
            values: vec![label_2.split('=').nth(1).unwrap().to_string()],
        })
    }

    if label_3.split('=').next().unwrap() != "" {
        my_vector.push(Label {
            key: label_3.split('=').next().unwrap().trim().to_string(),
            values: vec![label_3.split('=').nth(1).unwrap().to_string()],
        })
    }

    if label_4.split('=').next().unwrap() != "" {
        my_vector.push(Label {
            key: label_4.split('=').next().unwrap().trim().to_string(),
            values: vec![label_4.split('=').nth(1).unwrap().to_string()],
        })
    }

    my_vector
}
