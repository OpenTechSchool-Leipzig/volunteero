use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::dto::DTO;
use crate::opportunity::Opportunity;

pub fn csv_parser(path: &Path) -> Result<Vec<Opportunity>, io::Error> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut opportunities: Vec<Opportunity> = Vec::new();

    for result in rdr.deserialize() {
        let record: DTO = result?;
        opportunities.push(record.try_into().unwrap());
    }

    println!("Successfully imported {} records.", opportunities.len());

    Ok(opportunities)
}
