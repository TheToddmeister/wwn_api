use std::collections::{HashMap, HashSet};
use chrono::Datelike;
use crate::api::shared_api_metadata_struct::{Observation};
use itertools;
use itertools::Itertools;
use ordered_float::OrderedFloat;

async fn ordinal_span(day: &i32, delta: &i32) -> Vec<i32> {
    (-delta..*delta).map(|a| (day + a) % 365).collect()
}

pub struct DailyParameterMetaData {
    pub ordinal: u32,
    pub count: u32,
    pub mean: f64,
    pub max: Option<f64>,
    pub min: Option<f64>
}

pub async fn calculate_daily_metadata(observations: &[Observation]) ->Vec<DailyParameterMetaData> {
    let min = 1990;
    let metadata = observations.into_iter()
        .filter(|a| a.date.year()<=min)
        .group_by(|f| f.date.ordinal())
        .into_iter()
        .map(|(key, group)|{
            let values: Vec<&Observation> = group.collect();
            let count = values.iter().count() as u32;
            DailyParameterMetaData {
                ordinal: key,
                count,
                max: values.iter().map(|m| OrderedFloat(m.value)).max().map(|k| k.0),
                min: values.iter().map(|m| OrderedFloat(m.value)).min().map(|k| k.0),
                mean: values.iter().map(|m| m.value).sum::<f64>()/count as f64,
            }
        }
        ).collect::<Vec<DailyParameterMetaData>>();
    return metadata;
}