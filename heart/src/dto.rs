use serde::Deserialize;

use crate::label::Label;

#[derive(Debug, Deserialize)]
pub struct DTO {
    organisation_id: String,
    organisation_name: String,
    jobdescription: String,
    contact_name: String,
    contact_email: String,
    contact_phone: String,
    contact_mobile: String,
    label_1: String,
    label_2: String,
    label_3: String,
    label_4: String,
    label_5: String,
    address_1_name: String,
    address_1_street: String,
    address_1_housenr: String,
    address_1_postcode: String,
    address_1_city: Option<String>,
    address_2_name: Option<String>,
    address_2_street: Option<String>,
    address_2_housenr: Option<String>,
    address_2_postcode: Option<String>,
    address_2_city: Option<String>,
    address_3_name: Option<String>,
    address_3_street: Option<String>,
    address_3_housenr: Option<String>,
    address_3_postcode: Option<String>,
    address_3_city: Option<String>,
    address_4_name: Option<String>,
    address_4_street: Option<String>,
    address_4_housenr: Option<String>,
    address_4_postcode: Option<String>,
    address_4_city: Option<String>,
}