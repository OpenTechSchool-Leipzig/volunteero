#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
#[macro_use]
extern crate lazy_static;

mod address;
mod contact;
mod csv_parser;
mod dto;
mod geocoder;
mod label;
mod opportunity;
mod organisation;
mod repository;
mod sample_data;

use std::path::Path;
use std::process;

use rocket::response::NamedFile;
use rocket::State;
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
    let label_filters = labels
        .0
        .into_iter()
        .map(|label| OpportunityFilter::LabelFilter(label))
        .collect();

    Json(opportunities.find(&label_filters))
}

#[get("/swagger.yaml")]
fn swagger_yaml() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/swagger.yaml")).ok()
}

// TODO: presumably our answers are missing proper encoding/chartes headers

fn main() {
    let csv_data = match csv_parser() {
        Err(_) => {
            println!("shit, exiting!");
            process::exit(1);
        }
        Ok(result) => OpportunityRepository { data: result },
    };

    rocket::ignite()
        .manage(csv_data)
        .mount("/", routes![index, find_by_labels, swagger_yaml])
        .launch();
}
