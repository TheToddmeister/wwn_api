use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub updated: i64,
    pub parameter: Parameter,
    pub period: Period,
    pub link: Vec<Link>,
    pub station: Vec<Station>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub key: String,
    pub name: String,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub key: String,
    pub from: i64,
    pub to: i64,
    pub summary: String,
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
pub struct Station {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub owner: String,
    pub measuring_stations: String,
    pub region: Option<i64>,
    pub from: i64,
    pub to: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub value: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub date: i64,
    pub value: f64,
    pub quality: String,
}
