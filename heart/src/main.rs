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
fn index(opportunities: State<OpportunityRepository>) -> Json<Vec<Opportunity>> {
    Json(opportunities.fetch_all())
}

fn main() {
    rocket::ignite()
        .manage(OpportunityRepository {})
        .mount("/", routes![index]).launch();
}
