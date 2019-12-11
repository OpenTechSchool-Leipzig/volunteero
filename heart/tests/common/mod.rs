use std::path::PathBuf;

use rocket::Rocket;

use heart::{rocket, UserFacingError};

pub(crate) fn default_rocket() -> Result<Rocket, UserFacingError> {
    rocket(&PathBuf::from("tests/sample_csvs/sample.csv"))
}
