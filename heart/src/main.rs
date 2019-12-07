#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, type_ascription)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod contact;
mod label;
mod opportunity;
mod organisation;
mod repository;
mod sample_data;
mod csv_parser;
mod dto;

use std::process;

use rocket::State;
use rocket_contrib::json::Json;

use crate::opportunity::{Opportunity, OpportunityRepository};
use crate::repository::Repository;
use crate::csv_parser::csv_parser;
use crate::label::LabelList;

#[get("/opportunities")] // TODO: pagination
fn index(opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    Json(opportunities.fetch_all())
}

/// GET /opportunities?labels=Sportart:Fußball,Kategorie:Vereinsarbeit,Kategorie:Vorstand
#[get("/opportunities?<labels>")]
fn find_by_labels(labels: LabelList) -> Json<String> {
    Json(format!("{:?}", labels))
}

fn main() {
    if let Err(err) = csv_parser() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    rocket::ignite()
        .manage(OpportunityRepository {})
        .mount("/", routes![index, find_by_labels]).launch();
}
