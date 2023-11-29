use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use crate::api::geo::Geolocation;
use crate::api::static_metadata::{Source, Nation};
use crate::api::nve;
use crate::api::smih;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Station {
    location: Geolocation,
    status: bool,
    river: String,
    drainage_basin: Option<String>,
    parental_hierarchy: Vec<String>,
    parameter: Vec<Parameter>,
    last_update: DateTime<Utc>,
    source: Source,
    country: Nation
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Parameter {
    id: i64,
    name: String,
    latest_observations: Vec<Observation>,
    last_update: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq,  Serialize, Deserialize)]
struct Observation {
    date: DateTime<Utc>,
    value: f64,
}

#[derive(Default, Debug, Clone, PartialEq,  Serialize, Deserialize)]
struct Change{
    in24h: Option<f64>,
    in1h: Option<f64>,
}

impl Parameter {
    pub async fn get_newest_observation(&self) -> Option<&Observation> {
        self.latest_observations.first()    }
    async fn  get_x_minutes_older_than_newest(&self, min:i64) -> Option<Observation> {
        let newest = self.get_newest_observation().await?.date;
        let diff = Duration::minutes(min);
        let max_date = newest-diff;
        self.latest_observations.iter()
            .find(|p| p.date <=max_date)
            .map(|a| a.clone())
    }
    pub async fn get_current_change(&self) -> Option<Change>{
        let newest = &self.get_newest_observation().await?;
        let min24h = &self.get_x_minutes_older_than_newest(24*60).await?;
        let min1h = &self.get_x_minutes_older_than_newest(60).await?;
        let change = Change{
            in24h: Some((&newest.value - &min24h.value) / &min24h.value),
            in1h:Some((&newest.value - &min1h.value) / &min1h.value),
        };
        Some(change)
    }
    pub fn from_smih() -> Self {
        todo!()
    }
    pub fn from_nve(){
        todo!()
    }

}

