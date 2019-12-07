#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod address;
mod contact;
mod csv_parser;
mod dto;
mod label;
mod opportunity;
mod organisation;
mod repository;
mod sample_data;

use std::process;

use rocket::State;
use rocket_contrib::json::Json;

use crate::csv_parser::csv_parser;
use crate::label::LabelList;
use crate::opportunity::{Opportunity, OpportunityRepository, OpportunityFilter};
use crate::repository::Repository;
use crate::opportunity::OpportunityFilter::LabelFilter;

#[get("/opportunities")] // TODO: pagination
fn index(opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    Json(opportunities.fetch_all())
}

/// GET /opportunities?labels=Sportart:Fußball,Kategorie:Vereinsarbeit,Kategorie:Vorstand
#[get("/opportunities?<labels>")]
fn find_by_labels(labels: LabelList, opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    let label_filters = labels.0
        .into_iter()
        .map(|label| OpportunityFilter::LabelFilter(label))
        .collect();

    Json(opportunities.find(&label_filters))
}

// TODO: presumably our answers are missing proper encoding/chartes headers

fn main() {
    let result = match csv_parser() {
        Err() => SystemExit::1,
        Ok() => 
    }

    // convert Vec<Opportunity> into Opportunity

    rocket::ignite()
        .manage(OpportunityRepository {})
        .mount("/", routes![index, find_by_labels])
        .launch();
}
