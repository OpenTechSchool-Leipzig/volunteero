use std::error::Error;
use std::io;

use crate::dto::DTO;

pub fn csv_parser() -> Result<(), Box<dyn Error>> {
  let mut rdr = csv::Reader::from_reader(io::stdin());
  for result in rdr.deserialize() {
      // Notice that we need to provide a type hint for automatic
      // deserialization.
      println!("i am here!");
      let record: DTO = result?;
      println!("{:?}", record);
  }
  Ok(())
}