use std::any::Any;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::util::geo::location::Location;
use url::Url;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position{
    pub name: String,
    pub description: String,
    pub coordinate: Coordinates
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates{
    pub longitude: f64,
    pub latitude: f64
}
impl Coordinates {
    pub fn distance(&self, end: Coordinates) -> f64 {
        let kilometers: f64 = 6371.0;
        let mut r: f64 = 0.0 + kilometers;

        let d_lat: f64 = (end.latitude - self.latitude).to_radians();
        let d_lon: f64 = (end.longitude - self.longitude).to_radians();
        let lat1: f64 = (end.latitude).to_radians();
        let lat2: f64 = (end.latitude).to_radians();

        let a: f64 = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin()) + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
        let c: f64 = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

        r * c
    }
    pub fn gmaps(&self) -> Result<Url, url::ParseError> {
        let latitude = &self.latitude;
        let longitude = &self.longitude;
        url::Url::from_str(&*format!("https://maps.google.com/?q={latitude},{longitude}"))
    }
}