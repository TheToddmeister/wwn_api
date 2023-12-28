use serde::{self, Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub key: String,
    pub title: String,
    pub summary: String,
    pub unit: String,
    pub link: Vec<Link>,
    pub station_set: Vec<StationSet>,
    pub station: Vec<Station>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StationSet {
    pub key: String,
    pub updated: i64,
    pub title: String,
    pub summary: String,
    pub link: Vec<Link2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link2 {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub key: String,
    pub updated: Option<i64>,
    pub title: String,
    pub summary: String,
    pub link: Vec<Link3>,
    pub name: String,
    pub id: i64,
    pub owner: String,
    pub measuring_stations: String,
    pub active: bool,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub region: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link3 {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
