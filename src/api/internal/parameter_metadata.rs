use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::api::internal::observation::Observation;

use crate::static_metadata::{Origin, Nation};
use crate::util::geo::location::Location;
use crate::api::nve;
use crate::api::smih;
use crate::api::uk;
use crate::static_metadata;
use derive_builder::Builder;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Change {
    pub in24h: Option<f64>,
    pub in1h: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatedValues{
    pub mean: f64,
    pub max: Option<f64>,
    pub min: Option<f64>,
    pub var: Option<f64>,
    pub last_year: Vec<Observation>,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterMetaData{
    pub min_date: DateTime<Utc>,
    pub max_date: DateTime<Utc>,
    pub calculations: Option<CalculatedValues>,
}


impl ParameterMetaData{
    pub fn from_parameter(){

    }
}
