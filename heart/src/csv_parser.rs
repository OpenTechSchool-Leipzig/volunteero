use std::convert::TryInto;
use std::error::Error;
use std::io;


use crate::dto::DTO;
use crate::opportunity::Opportunity;

pub fn csv_parser() -> Result<Vec<Opportunity>, Box<dyn Error>> {
  let mut rdr = csv::Reader::from_reader(io::stdin());
  let mut opportunities: Vec<Opportunity> = Vec::new();

  for result in rdr.deserialize() {

      let record: DTO = result?;
      opportunities.push(record.try_into().unwrap());
  }
  Ok(opportunities)
}