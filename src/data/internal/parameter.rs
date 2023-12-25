use chrono::{Datelike, DateTime, Duration, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::data::{nve, uk};
use crate::data::nve::station::SeriesList;
use crate::static_metadata::{ParameterDefinitions, get_minimum_historic_data_date};
use crate::util;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationParameter {
    pub unit: Option<String>,
    pub internal_parameter_id: ParameterDefinitions,
    pub current_resolution_in_minutes: ObservationResolution,
    pub historic_stable_resolution_in_minutes: ObservationResolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObservationResolution {
    Nve(Option<NveObservationResolution>),
    UkGov(UkGovObservationResolution),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UkGovObservationResolution {
    pub internal_resolution: i64,
    pub external_period: i64
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NveObservationResolution {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub resolution: i64,
}

async fn measures_per_24_hours_to_every_nth_minutes(measure_count_per_24_hours: i64) -> i64 {
    measure_count_per_24_hours / (24 * 60)
}

async fn nve_select_highest_observationresolution_for_historic_data_that_is_greater_than_or_equal_to_1hr(min_date_for_historic: DateTime<Utc>, par: &SeriesList) -> Option<NveObservationResolution> {
    let max_historic_resolution = par.resolution_list.iter()
        .filter(|s| s.res_time >= 60)
        .filter(|q| q.data_from_time <= min_date_for_historic)
        .max_by(|x, y| x.res_time.cmp(&y.res_time))
        .map(|r| NveObservationResolution {
            start_date: r.data_from_time,
            end_date: r.data_to_time,
            resolution: r.res_time,
        });
    max_historic_resolution
}

async fn max_resolution_available_for_the_current_year(min_date_for_current: &DateTime<Utc>, par: &SeriesList, min_delta_for_current: DateTime<Utc>) -> Option<NveObservationResolution> {
    let max_current_resolution = par.resolution_list.iter()
        .filter(|q| &q.data_from_time <= &min_date_for_current && q.data_to_time <= min_delta_for_current)
        .max_by(|x, y| x.res_time.cmp(&y.res_time)).map(|r|
        NveObservationResolution {
            start_date: r.data_from_time,
            end_date: r.data_to_time,
            resolution: r.res_time,
        }
    );
    max_current_resolution
}

impl StationParameter {
    pub async fn from_nve_station(station: &nve::station::Daum) -> Vec<Self> {
        let min_date_for_current = util::time::get_first_moment_of_year(Utc::now().year()).await.expect("Failed to build a start of the year date from the current year");
        let min_date_for_historic = *get_minimum_historic_data_date().await;
        let mut internal_params: Vec<StationParameter> = vec![];
        for par in &station.series_list {
            let internal_parameter_id = ParameterDefinitions::from_nve(par.parameter);
            if let Some(id) = internal_parameter_id {
                let min_delta_for_current = Utc::now() -Duration::hours(48);
                let max_current_resolution = max_resolution_available_for_the_current_year(&min_date_for_current, par, min_delta_for_current).await;
                let max_historic_resolution = nve_select_highest_observationresolution_for_historic_data_that_is_greater_than_or_equal_to_1hr(min_date_for_historic, par).await;
                let sp = StationParameter {
                    unit: Some(par.unit.to_string()),
                    internal_parameter_id: id,
                    current_resolution_in_minutes: ObservationResolution::Nve(max_current_resolution),
                    historic_stable_resolution_in_minutes: ObservationResolution::Nve(max_historic_resolution),
                };
                internal_params.push(sp);
            }
        }
        internal_params
    }

    pub async fn from_ukgov_station(station: &uk::station::Item) -> Vec<Self> {
        let mut internal_params: Vec<StationParameter> = vec![];
        for (par, measures) in &station.measures.iter().group_by(|p| &p.parameter) {
            let internal_parameter_id = ParameterDefinitions::from_uk(&par);
            if let Some(id) = internal_parameter_id {
                let max_resolution = measures
                    .max_by(|x, y| x.period.cmp(&y.period));
                if let Some(period) = max_resolution.map(|m| m.period).flatten(){
                    let measurements_delta_in_minutes = measures_per_24_hours_to_every_nth_minutes(period).await;
                    let uk_resolution = UkGovObservationResolution {
                        internal_resolution: measurements_delta_in_minutes,
                        external_period: period,
                    };
                    let sp = StationParameter {
                        unit: None,
                        internal_parameter_id: id,
                        current_resolution_in_minutes: ObservationResolution::UkGov(uk_resolution.clone()),
                        historic_stable_resolution_in_minutes: ObservationResolution::UkGov(uk_resolution),
                    };
                    internal_params.push(sp);
                }
            }
        }
        internal_params
    }
}