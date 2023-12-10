use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::api::{uk, nve};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Observation {
    pub datetime: DateTime<Utc>,
    pub value: Option<f64>,
    pub quality: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatedObservation {
    pub datetime: DateTime<Utc>,
    pub value: f64,
    pub quality: String,
}
impl ValidatedObservation{
    pub fn from_observation(obs: &Observation) -> Option<Self>{
        match obs.value {
            Some(value)=> return Some(Self{
                datetime: obs.datetime,
                value: value,
                quality: obs.quality.to_string(),
            }),
            None => None,
        }
    }
}

impl Observation {
    pub fn from_ukgov(observation_item: &uk::observation::Item)->Self{
        let o = observation_item;
        Self{
            value: o.value,
            quality: o.quality.to_string(),
            datetime: o.time
        }
    }
    pub fn from_nve(observation: nve::observation::Observation)->Self{
        let o = observation;
        Self{
            value: o.value,
            quality: o.quality.to_string(),
            datetime: o.time
        }
    }
}


