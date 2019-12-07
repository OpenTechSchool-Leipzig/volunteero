use std::convert::TryInto;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::env;

use crate::dto::DTO;
use crate::opportunity::Opportunity;

pub fn csv_parser() -> Result<Vec<Opportunity>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut opportunities: Vec<Opportunity> = Vec::new();

    for result in rdr.deserialize() {
        let record: DTO = result?;
        opportunities.push(record.try_into().unwrap());
    }

    println!("Successfully imported {} records.", opportunities.len());

    Ok(opportunities)
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
