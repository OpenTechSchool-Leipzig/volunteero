#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

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

#[get("/opportunities")] // TODO: pagination
fn index(opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    Json(opportunities.fetch_all())
}

fn main() {
    rocket::ignite()
        .manage(OpportunityRepository {})
        .mount("/", routes![index]).launch();
    // let or = OpportunitiesRepository::new();
//    rocket::ignite().mount("/", routes![index]).launch();

    if let Err(err) = csv_parser() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
