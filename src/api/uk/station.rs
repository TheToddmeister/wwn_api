use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "@id")]
    pub id: String,
    pub label: Value,
    pub notation: String,
    pub easting: Value,
    pub northing: Value,
    pub lat: Value,
    pub long: Value,
    #[serde(rename = "type")]
    pub type_field: Vec<Type>,
    pub river_name: Value,
    pub station_guid: Value,
    #[serde(rename = "wiskiID")]
    pub wiski_id: Value,
    pub date_opened: Option<String>,
    pub observed_property: Vec<ObservedProperty>,
    pub status: Value,
    pub measures: Vec<Measure>,
    pub station_reference: Value,
    #[serde(rename = "RLOIid")]
    pub rloiid: Value,
    pub rloi_station_link: Value,
    pub catchment_area: Option<f64>,
    #[serde(rename = "nrfaStationID")]
    pub nrfa_station_id: Option<String>,
    #[serde(rename = "nrfaStationURL")]
    pub nrfa_station_url: Option<String>,
    #[serde(default)]
    pub colocated_station: Vec<ColocatedStation>,
    pub datum: Option<f64>,
    pub borehole_depth: Option<f64>,
    pub aquifer: Option<String>,
    pub status_reason: Option<String>,
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
