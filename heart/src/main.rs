#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod contact;
mod label;
mod opportunity;
mod organisation;
mod repository;

use rocket::State;
use rocket_contrib::json::Json;

use crate::opportunity::{Opportunity, OpportunityRepository};
use crate::repository::Repository;

#[get("/opportunities")] // TODO: pagination
fn index<'a>(opportunities: State<&'a OpportunityRepository>) -> Json<Vec<&'a Opportunity<'a>>> {
    Json(opportunities.fetch_all())
}

fn main() {
    // let or = OpportunitiesRepository::new();
    rocket::ignite().mount("/", routes![index]).launch();
}
