use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with;

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
    pub measure: Measure,
    #[serde(skip)]
    pub date: NaiveDate,
    pub date_time: NaiveDateTime,
    pub value: Option<f64>,
    pub quality: String,
    pub valid: Option<String>,
    pub invalid: Option<String>,
    pub missing: Option<String>,
    pub completeness: Option<String>,
    pub qcode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
    #[serde(rename = "@id")]
    pub id: String,
}
