use chrono::{Datelike, DateTime, Utc};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use crate::data::internal;

use crate::data::internal::observation::Observation;
use crate::util;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Change {
    pub in24h: Option<f64>,
    pub in1h: Option<f64>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalculatedOrdinalValues {
    pub mean: f64,
    pub max: Option<f64>,
    pub min: Option<f64>,
    pub var: Option<f64>,
    pub last_year: Vec<Observation>,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesMetaData {
    pub min_date: Option<DateTime<Utc>>,
    pub max_date: Option<DateTime<Utc>>,
    pub calculations: Vec<CalculatedOrdinalValues>,
    pub calculation_date: DateTime<Utc>,
}

impl TimeSeriesMetaData {
    pub async fn from_internal_timeseries(internal_parameter: &internal::timeseries::TimeSeries) ->Self{
        let p = &internal_parameter;
        let calculation_date = Utc::now();
        let max_date = p.observations.iter().map(|q| q.datetime).into_iter().max();
        let min_date =  p.observations.iter().map(|q| q.datetime).into_iter().min();

        let calculations = util::analytics::historic_statistics::calculate_daily_metadata(&p.observations).await;
        TimeSeriesMetaData {
            min_date,
            max_date,
            calculation_date,
            calculations
        }
    }

}
