use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, OneOrMany, formats::PreferMany};
use crate::api::serde::deserializor::{convert_active_to_bool};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub meta: Meta,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "@id")]
    pub id: String,
    pub publisher: String,
    pub license: String,
    pub license_name: String,
    pub comment: String,
    pub version: String,
    pub has_format: Vec<String>,
    pub limit: i64,
}
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "@id")]
    pub id: String,
    pub label: Value,
    pub notation: String,
    pub easting: Value,
    pub northing: Value,
    pub lat: f64,
    pub long: f64,
    #[serde(rename = "type")]
    pub type_field: Vec<Type>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub river_name: Vec<String>,
    pub station_guid: Option<Value>,
    #[serde(rename = "wiskiID")]
    pub wiski_id: Option<Value>,
    pub date_opened: Option<String>,
    pub observed_property: Vec<ObservedProperty>,
    pub status: Option<Status>,
    pub measures: Vec<Measure>,
    pub station_reference: Option<Value>,
    #[serde(rename = "RLOIid")]
    pub rloiid: Option<Value>,
    pub rloi_station_link: Option<Value>,
    pub catchment_area: Option<f64>,
    #[serde(rename = "nrfaStationID")]
    pub nrfa_station_id: Option<String>,
    #[serde(rename = "nrfaStationURL")]
    pub nrfa_station_url: Option<String>,
    #[serde(default)]
    pub colocated_station: Vec<ColocatedStation>,
    pub datum: Option<f64>,
    #[serde(skip_serializing)]
    pub borehole_depth: Option<f64>,
    pub aquifer: Option<String>,
    pub status_reason: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(deserialize_with="convert_active_to_bool")]
    pub label: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservedProperty {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
    #[serde(rename = "@id")]
    pub id: String,
    pub parameter: String,
    pub period: Option<i64>,
    pub value_statistic: ValueStatistic,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueStatistic {
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColocatedStation {
    #[serde(rename = "@id")]
    pub id: String,
}
