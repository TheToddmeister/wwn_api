use serde::{Deserialize, Serialize};

use serde_json::Value;
use serde_with:: {serde_as, OneOrMany, formats::PreferMany,formats::PreferOne};
use crate::data::serde::deserializor::{convert_active_to_bool};

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
    #[serde(skip)]
    pub publisher: String,
    #[serde(skip)]
    pub license: String,
    #[serde(skip)]
    pub license_name: String,
    #[serde(skip)]
    pub comment: String,
    pub version: String,
    pub has_format: Vec<String>,
    #[serde(skip)]
    pub limit: Option<i64>,
}
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub label: Vec<String>,
    pub notation: String,
    #[serde(skip)]
    pub easting: Value,
    #[serde(skip)]
    pub northing: Value,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub lat: Vec<f64>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    pub long: Vec<f64>,
    #[serde(rename = "type")]
    pub type_field: Vec<Type>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    pub river_name: Vec<String>,
    pub station_guid: Option<Value>,
    #[serde(rename = "wiskiID")]
    #[serde(skip)]
    pub wiski_id: Option<Value>,
    pub date_opened: Option<String>,
    #[serde(skip)]
    pub observed_property: Vec<ObservedProperty>,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferMany>")]
    #[serde(default)]
    pub status: Vec<Status>,
    pub measures: Vec<Measure>,
    #[serde(skip)]
    pub station_reference: Option<Value>,
    #[serde(rename = "RLOIid")]
    #[serde(skip)]
    pub rloiid: Option<Value>,
    #[serde(skip)]
    pub rloi_station_link: Option<Value>,
    #[serde(skip)]
    pub catchment_area: Option<f64>,
    #[serde(rename = "nrfaStationID")]
    #[serde(skip)]
    pub nrfa_station_id: Option<String>,
    #[serde(rename = "nrfaStationURL")]
    #[serde(skip)]
    pub nrfa_station_url: Option<String>,
    #[serde(skip)]
    pub colocated_station: Vec<ColocatedStation>,
    #[serde(skip)]
    pub datum: Option<f64>,
    #[serde(skip)]
    pub borehole_depth: Option<f64>,
    pub aquifer: Option<String>,
    pub status_reason: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "@id")]
    pub id: String,
    pub label: String,
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
