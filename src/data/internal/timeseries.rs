use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::data::internal::observation::{Observation, ValidatedObservation};
use crate::data::internal::timeseries_metadata::Change;
use crate::data::nve;
use crate::data::uk;
use crate::static_metadata;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeries {
    pub station_id: String,
    pub parameter_id: static_metadata::ParameterDefinitions,
    pub latest_observations: Vec<Observation>,
    pub last_update_request: DateTime<Utc>,
}

impl TimeSeries {
    
    pub async fn get_newest_observation(&self) -> Option<ValidatedObservation> {
        self.latest_observations.iter().find(|o| o.value.is_some())
            .map(|q| ValidatedObservation::from_observation(q))
            .flatten()
    }
    pub async fn find_min_x_minutes_older_than_newest_max_2x(&self, minutes_diff: i64) -> Option<ValidatedObservation> {
        let newest = self.get_newest_observation().await?.datetime;
        let diff = Duration::minutes(minutes_diff);
        let min_date = newest - diff;
        let max_date = newest - diff*2;
        self.latest_observations.iter()
            .skip_while(|q| q.datetime>=min_date)
            .take_while(|w| w.datetime<=max_date)
            .find(|p| p.datetime <= max_date && p.value.is_some())
            .map(|o| ValidatedObservation::from_observation(o))
            .flatten()
    }
    pub async fn get_current_change(&self) -> Option<Change> {
        let newest = self.get_newest_observation().await;
        let min24h = self.find_min_x_minutes_older_than_newest_max_2x(24 * 60).await;
        let min1h = self.find_min_x_minutes_older_than_newest_max_2x(60).await;
        match (newest, min1h, min24h) {
            (Some(n), Some(m1h), Some(m24h)) => {
                return Some(Change {
                    in24h: Some((n.value - m24h.value) / m24h.value),
                    in1h: Some((n.value - m1h.value) / m1h.value),
                });
            }
            (Some(n), Some(m1h), None) => {
                return Some(Change {
                    in24h: None,
                    in1h: Some((n.value - m1h.value) / m1h.value),
                });
            }
            _ => None
        }
    }

    pub fn from_smih() -> Self {
        todo!()
    }

    pub fn from_nve48h(parameter: nve::observation::Daum,
                       station_id: &str,
                       parameter_id: static_metadata::ParameterDefinitions) -> Self {
        let end_date = Utc::now();
        let start_date = end_date - chrono::Duration::days(2);
        Self::from_nve(parameter, station_id, parameter_id, start_date, end_date)
    }

    pub fn from_nve(parameter: nve::observation::Daum,
                    station_id: &str,
                    parameter_id: static_metadata::ParameterDefinitions,
                    start_date: DateTime<Utc>,
                    end_date: DateTime<Utc>,
    ) -> Self {
        let o = parameter.observations;
        let obs = o.iter().rev()
            .skip_while(|d| d.time >= end_date)
            .take_while(|p| p.time >= start_date)
            .map(|v| Observation {
                datetime: v.time,
                value: v.value,
                quality: v.quality.to_string(),
            }).collect::<Vec<Observation>>();

        Self {
            station_id: station_id.to_string(),
            last_update_request: Utc::now(),
            parameter_id,
            latest_observations: obs,
        }
    }

    pub fn from_ukgov48h(station: &uk::observation::Root,
                         station_id: &str,
                         parameter_id: static_metadata::ParameterDefinitions) -> Self {
        let end_date = Utc::now();
        let start_date = end_date - chrono::Duration::days(2);
        Self::from_ukgov(station, station_id, parameter_id, &start_date, &end_date)
    }

    pub fn from_ukgov(station: &uk::observation::Root,
                      station_id: &str,
                      parameter_id: static_metadata::ParameterDefinitions,
                      start_date: &DateTime<Utc>,
                      end_date: &DateTime<Utc>,
    ) -> Self {
        let o = &station.items;
        let obs = o.iter().rev()
            .skip_while(|d| d.date_time >= end_date.naive_utc())
            .take_while(|p| p.date_time >= start_date.naive_utc())
            .map(|v| Observation {
                datetime: v.date_time.and_utc(),
                value: v.value,
                quality: v.quality.to_string(),
            }).collect::<Vec<Observation>>();
        let parameter = Self {
            station_id: station_id.to_string(),
            last_update_request: Utc::now(),
            parameter_id,
            latest_observations: obs,
        };
        parameter
    }
}