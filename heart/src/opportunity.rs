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
pub struct OpportunityRepository {}

impl Repository<Opportunity> for OpportunityRepository {
    fn fetch_all(&self) -> Vec<Opportunity> {
        OPPORTUNITIES.clone()
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
            .all(|l|
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

