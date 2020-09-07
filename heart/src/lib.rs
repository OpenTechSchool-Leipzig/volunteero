#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod contact;
mod csv_parser;
mod dto;
mod geocoder;
mod label;
mod location;
mod opportunity;
mod organisation;
mod repository;
mod sample_data;

use std::fmt;
use std::path::Path;

use rocket::response::NamedFile;
use rocket::{Rocket, State};
use rocket_contrib::json::Json;

use crate::csv_parser::csv_parser;
use crate::label::LabelList;
use crate::opportunity::{Opportunity, OpportunityFilter, OpportunityRepository};
use crate::repository::Repository;

#[get("/opportunities")] // TODO: pagination
fn index(opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    Json(opportunities.fetch_all())
}

/// GET /opportunities?labels=Sportart:Fußball,Kategorie:Vereinsarbeit,Kategorie:Vorstand
#[get("/opportunities?<labels>")]
fn find_by_labels(
    labels: LabelList,
    opportunities: State<OpportunityRepository>,
) -> Json<Vec<Opportunity>> {
    let label_filters: Vec<_> = labels
        .0
        .into_iter()
        .map(OpportunityFilter::LabelFilter)
        .collect();

    Json(opportunities.find(&label_filters))
}

#[get("/swagger.yaml")]
fn swagger_yaml() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/swagger.yaml")).ok()
}

// TODO: presumably our answers are missing proper encoding/chartes headers

pub fn rocket(csv_path: &Path) -> Result<Rocket, UserFacingError> {
    let opportunities = OpportunityRepository {
        data: csv_parser(csv_path)?,
    };

    Ok(rocket::ignite()
        .manage(opportunities)
        .mount("/", routes![index, find_by_labels, swagger_yaml]))
}

#[derive(Debug, PartialEq)]
pub struct UserFacingError {
    pub message: String,
}

impl fmt::Display for UserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for UserFacingError {
    fn from(e: std::io::Error) -> Self {
        UserFacingError {
            message: format!("{}", e),
        }
    }
}
