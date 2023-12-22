use std::collections::{HashMap, HashSet};
use chrono::{Datelike, Utc};
use crate::data::internal::observation::{Observation};
use crate::data::internal::timeseries_metadata::{CalculatedOrdinalValues, TimeSeriesMetaData};
use itertools;
use itertools::Itertools;
use ordered_float::OrderedFloat;

async fn ordinal_span(day: &i32, delta: &i32) -> Vec<i32> {
    (-delta..*delta).map(|a| (day + a) % 365).collect()
}

pub async fn calculate_daily_metadata(observations: &[Observation]) ->Vec<CalculatedOrdinalValues> {
    let min = 1990;
    let metadata = observations.iter()
        .filter(|a| a.datetime.year()<=min)
        .group_by(|f| f.datetime.ordinal())
        .into_iter()
        .map(|(key, group)|{
            let values: Vec<&Observation> = group.collect();
            let count = values.len() as u32;
            CalculatedOrdinalValues {
                var: None, //todo
                count,
                max: values.iter()
                    .filter_map(|m| m.value.map(OrderedFloat))
                    .max().map(|k| k.0),
                min: values.iter().filter_map(|m|  m.value.map(OrderedFloat))
                    .min().map(|k| k.0),
                mean: values.iter().filter_map(|m| m.value)
                    .sum::<f64>()/count as f64,
                last_year: observations.iter().rev()
                    .skip_while(|t| t.datetime.year()>=Utc::now().year())
                    .take_while(|q| q.datetime.year()>=Utc::now().year()-1)
                    .map(|a| a.to_owned())
                    .collect()
            }
        }
        ).collect::<Vec<CalculatedOrdinalValues>>();
    metadata
}