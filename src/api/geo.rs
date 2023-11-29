use std;
use std::str::FromStr;
use haversine;
use haversine::Units;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};
use crate::api::static_metadata::{Nation, Source};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocationId {
    src: Source,
    id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geolocation{
    pub id: LocationId,
    pub name: String,
    pub region: String,
    pub country: Nation,
    pub altitude: Option<i64>,
    pub longitude: f64,
    pub latitude: f64,
}
impl Geolocation{
    pub fn distance(&self, longitude:f64, latitude: f64)->f64{
        let l1 = haversine::Location{
            longitude: self.longitude,
            latitude: self.latitude

        };
        let l2 = haversine::Location{
            latitude,
            longitude
        };
        haversine::distance(l1, l2, Units::Kilometers)
    }
    pub fn gmaps(&self) -> Result<Url, ParseError> {
         let latitude = &self.latitude;
        let longitude = &self.longitude;
         Url::from_str(&*format!("https://maps.google.com/?q={latitude},{longitude}"))
    }
}