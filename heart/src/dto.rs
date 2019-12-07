use serde::Deserialize;
use std::convert::TryInto;

use crate::label::Label;

#[derive(Debug, Deserialize)]
pub struct DTO {
    pub organisation_id: String,
    pub organisation_name: String,
    pub job_description: String,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub contact_mobile: String,
    pub label_1: String,
    pub label_2: String,
    pub label_3: String,
    pub label_4: String,
    pub label_5: String,
    pub address_1_name: String,
    pub address_1_street: String,
    pub address_1_housenr: String,
    pub address_1_postcode: String,
    pub address_1_city: String,
    pub address_2_name: Option<String>,
    pub address_2_street: Option<String>,
    pub address_2_housenr: Option<String>,
    pub address_2_postcode: Option<String>,
    pub address_2_city: Option<String>,
    pub address_3_name: Option<String>,
    pub address_3_street: Option<String>,
    pub address_3_housenr: Option<String>,
    pub address_3_postcode: Option<String>,
    pub address_3_city: Option<String>,
    pub address_4_name: Option<String>,
    pub address_4_street: Option<String>,
    pub address_4_housenr: Option<String>,
    pub address_4_postcode: Option<String>,
    pub address_4_city: Option<String>,
}