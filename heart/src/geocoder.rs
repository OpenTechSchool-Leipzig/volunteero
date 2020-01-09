use crate::location::Address;
use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Clone, Serialize)]
pub struct LatLon {
    lat: String,
    lon: String,
}

pub fn geocode(address: &Address) -> Result<LatLon, String> {
    let base_url = "https://nominatim.openstreetmap.org/";

    let client = reqwest::Client::new();
    let maybe_response = client
        .get(base_url)
        .header(USER_AGENT, "volunteero/0.1")
        .query(&[
            ("format", "json"),
            ("addressdetails", "0"),
            ("limit", "1"),
            ("q", &address.as_string()),
        ])
        .send();

    let result: Result<LatLon, String> = maybe_response
        .and_then(|mut response| response.json(): Result<Vec<LatLon>, Error>)
        .map_err(|error| error.to_string())
        .and_then(|lat_lons| match lat_lons {
            list if !list.is_empty() => Ok(list.first().unwrap().clone()),
            _ => Err("Could not find location".to_string()),
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_geo_coding() {
        let address = Address {
            name: "".to_string(),
            street: "Alte Str.".to_string(),
            house_number: "8".to_string(),
            postcode: "04229".to_string(),
            city: "Leipzig".to_string(),
        };

        assert_eq!(
            geocode(&address),
            Ok(LatLon {
                lat: "51.3310959".to_string(),
                lon: "12.3418933350183".to_string(),
            })
        );
    }

    #[test]
    fn reverse_geo_coding_fails_if_no_result_is_found() {
        let address = Address {
            name: "".to_string(),
            street: "".to_string(),
            house_number: "".to_string(),
            postcode: "".to_string(),
            city: "".to_string(),
        };

        assert_eq!(
            geocode(&address),
            Err("Could not find location".to_string())
        );
    }
}
