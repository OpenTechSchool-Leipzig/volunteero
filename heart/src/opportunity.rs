use crate::address::Address;

use crate::contact::Contact;
use crate::contact::ContactOption;

use crate::organisation::Organisation;
use crate::dto::DTO;
use std::convert::TryFrom;
use std::convert::TryInto ;

use crate::label::Label;


use crate::repository::Repository;

use serde::Serialize;
use crate::sample_data::OPPORTUNITIES;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Opportunity {
    pub job_description: String,
    pub organisation: Organisation,
    pub locations: Vec<Address>,
    pub contact: Contact,
    pub labels: Vec<Label>,
}

impl Opportunity {
    fn get_labels_for_key(&self, key: &str) -> Vec<String> {
        self.labels.clone()
            .into_iter()
            .find(|label| label.key == key)
            .map(|label| label.values)
            .unwrap_or(vec![])
    }
}

// TODO: DTO mit Stadtteil

// TODO: Beispieldaten ins Repository als HashMap und bei fetch_all ausliefern


pub enum OpportunityFilter {
    LabelFilter(Label)
}

#[derive(Debug, PartialEq)]
pub struct OpportunityRepository {
    pub data: Vec<Opportunity>
}

impl Repository<Opportunity> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<Opportunity> {
        self.data.clone()
    }
}

impl OpportunityRepository {
    pub fn find(&self, filters: &Vec<OpportunityFilter>) -> Vec<Opportunity> {
        OPPORTUNITIES.clone()
            .into_iter()
            .filter(|opportunity|
                filters
                    .into_iter()
                    .all(|filter|
                        match filter {
                            OpportunityFilter::LabelFilter(filter_label) =>
                                self.filter_by_labels(opportunity, filter_label)
                        }
                    )
            ).collect()
    }

    fn filter_by_labels(&self, opportunity: &Opportunity, filter_label: &Label) -> bool {
        let values = &filter_label.values;
        values
            .into_iter()
            .any(|l|
                opportunity.get_labels_for_key(&filter_label.key).contains(&l)
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_opportunities_by_single_label() {
        let repository = OpportunityRepository {};
        let filter = vec![
            OpportunityFilter::LabelFilter(Label {
                key: "Sportart".to_string(),
                values: vec!["Fußball".to_string()],
            })
        ];

        let actual = repository.find(
            &filter
        );

        assert_eq!(
            actual,
            OPPORTUNITIES.clone()
        )
    }

    #[test]
    fn filter_opportunities_by_multiple_labels() {
        let repository = OpportunityRepository {};
        let filter = vec![
            OpportunityFilter::LabelFilter(Label {
                key: "Aufgabenfeld".to_string(),
                values: vec!["Vereinsleben".to_string(), "Trainigsbetrieb".to_string()],
            })
        ];

        let actual = repository.find(
            &filter
        );

        assert_eq!(
            actual,
            OPPOTrainerRTUNITIES.clone()
        )
    }

    #[test]
    fn filter_opportunities_by_none_partially_existing_label() {
        let repository = OpportunityRepository {};
        let filter = vec![
            OpportunityFilter::LabelFilter(Label {
                key: "Sportart".to_string(),
                values: vec!["Eishockey".to_string(), "Fußball".to_string()],
            })
        ];

        let actual = repository.find(
            &filter
        );

        assert_eq!(
            actual,
            OPPORTUNITIES.clone()
        )
    }

    #[test]
    fn filter_opportunities_by_none_existing_label() {
        let repository = OpportunityRepository {};
        let filter = vec![
            OpportunityFilter::LabelFilter(Label {
                key: "Sportart".to_string(),
                values: vec!["Eishockey".to_string()],
            })
        ];

        let actual = repository.find(
            &filter
        );

        assert_eq!(
            actual,
            vec![]
        )
    }
}

impl TryFrom<DTO> for Opportunity {
    type Error = String;
  
    fn try_from(raw_data: DTO) -> Result<Self, Self::Error> {
        // TODO: validate
        Ok(Self { 
                job_description: raw_data.job_description, 
                organisation: Organisation { 
                    id: raw_data.organisation_id,
                    name: raw_data.organisation_name,
                },
                locations: vec![Address {
                    name: raw_data.address_1_name,
                    street: raw_data.address_1_street,
                    house_number: raw_data.address_1_housenr,
                    postcode: raw_data.address_1_postcode,
                    city: raw_data.address_1_city,
                }]
                ,
                contact: Contact {
                    name: raw_data.contact_name,
                    options: extract_options(
                        raw_data.contact_email, 
                        raw_data.contact_phone, 
                        raw_data.contact_mobile, 
                    )},
                labels:  extract_labels(
                        raw_data.label_1, 
                        raw_data.label_2, 
                        raw_data.label_3, 
                        raw_data.label_4
                    )
                }
        )}
    }
    
  

// TODO should return all addresses ||  now only returns one adress
  fn extract_options(
      email: String, 
      phone: String, 
      mobile: String) -> Vec<ContactOption> {
        
        use ContactOption::*;
        vec![
            EMail((email, "".to_string()).try_into().unwrap()),
            Phone((phone, "".to_string()).try_into().unwrap()),
            ]
  }

  fn extract_labels(
        label_1: String,
        label_2: String,
        label_3: String,
        label_4: String) -> Vec<Label> {

            let mut myVector: Vec<Label> = vec![];

            if label_1.to_string().split("=").nth(0).unwrap().to_string() != "".to_string() {
                myVector.push(Label {
                    key: label_1.to_string().split("=").nth(0).unwrap().to_string(),
                    values: vec![label_1.to_string().split("=").nth(1).unwrap().to_string()],
                })
            }

            if label_2.to_string().split("=").nth(0).unwrap().to_string() != "".to_string()  {
                myVector.push(Label {
                    key: label_2.to_string().split("=").nth(0).unwrap().to_string(),
                    values: vec![label_2.to_string().split("=").nth(1).unwrap().to_string()],
                })
            }

            if label_3.to_string().split("=").nth(0).unwrap().to_string() != "".to_string() {
                myVector.push(Label {
                    key: label_3.to_string().split("=").nth(0).unwrap().to_string(),
                    values: vec![label_3.to_string().split("=").nth(1).unwrap().to_string()],
                })
            }

            if label_4.to_string().split("=").nth(0).unwrap().to_string() != "".to_string()  {
                myVector.push(Label {
                    key: label_4.to_string().split("=").nth(0).unwrap().to_string(),
                    values: vec![label_4.to_string().split("=").nth(1).unwrap().to_string()],
                })
            }

            return myVector
        }
    
    
