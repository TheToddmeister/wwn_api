use std::collections::{HashMap, HashSet};
use chrono::{Datelike, Utc};
use crate::api::internal::observation::{Observation};
use crate::api::internal::parameter_metadata::{CalculatedValues, ParameterMetaData};
use itertools;
use itertools::Itertools;
use ordered_float::OrderedFloat;

async fn ordinal_span(day: &i32, delta: &i32) -> Vec<i32> {
    (-delta..*delta).map(|a| (day + a) % 365).collect()
}

pub async fn calculate_daily_metadata(observations: &[Observation]) ->Vec<CalculatedValues> {
    let min = 1990;
    let metadata = observations.into_iter()
        .filter(|a| a.datetime.year()<=min)
        .group_by(|f| f.datetime.ordinal())
        .into_iter()
        .map(|(key, group)|{
            let values: Vec<&Observation> = group.collect();
            let count = values.iter().count() as u32;
            CalculatedValues {
                var: None, //todo
                count,
                max: values.iter()
                    .map(|m| m.value.map(|f| OrderedFloat(f)))
                    .flatten()
                    .max().map(|k| k.0),
                min: values.iter().map(|m|  m.value.map(|f| OrderedFloat(f)))
                    .flatten()
                    .min().map(|k| k.0),
                mean: values.iter().map(|m| m.value).flatten().sum::<f64>()/count as f64,
                last_year: observations.into_iter().rev()
                    .skip_while(|t| t.datetime.year()>=Utc::now().year())
                    .take_while(|q| q.datetime.year()>=Utc::now().year()-1)
                    .map(|a| a.to_owned())
                    .collect()
            }
        }
        ).collect::<Vec<CalculatedValues>>();
    return metadata;
}