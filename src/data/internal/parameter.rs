use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::data::{nve, uk};
use crate::static_metadata::{ParameterDefinitions,get_minimum_current_date, get_minimum_historic_data_date};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StationParameter {
    unit: Option<String>,
    internal_id: ParameterDefinitions,
    current_resolution_in_minutes: Resolution,
    historic_stable_resolution_in_minutes: Resolution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    Nve(Option<NveParameterResolution>),
    UkGov(UkGovParameterResolution),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct  UkGovParameterResolution{
    internal_resolution: i64,
    external_period: i64
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NveParameterResolution {
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    resolution: i64,
}

async fn measures_per_24_hours_to_every_nth_minutes(measure_count_per_24_hours: i64) -> i64 {
    measure_count_per_24_hours / (24 * 60)
}

impl StationParameter {
    pub async fn from_nve_station(station: &nve::station::Daum) -> Vec<Self> {
        let min_date_for_current = get_minimum_current_date().await;
        let min_date_for_historic = *get_minimum_historic_data_date().await;
        let mut internal_params: Vec<StationParameter> = vec![];
        for par in &station.series_list {
            let internal_parameter_id = ParameterDefinitions::from_nve(par.parameter);
            if let Some(id) = internal_parameter_id {
                let min_delta_for_current = Utc::now() - chrono::Duration::hours(48);
                let max_current_resolution = par.resolution_list.iter()
                    .filter(|q| &q.data_from_time <= min_date_for_current && q.data_to_time <= min_delta_for_current)
                    .max_by(|x, y| x.res_time.cmp(&y.res_time)).map(|r|
                    NveParameterResolution {
                        start_date: r.data_from_time,
                        end_date: r.data_to_time,
                        resolution: r.res_time,
                    }
                );
                let max_historic_resolution = par.resolution_list.iter()
                    .filter(|s| s.res_time >= 60)
                    .filter(|q| &q.data_from_time <=&min_date_for_historic)
                    .max_by(|x, y| x.res_time.cmp(&y.res_time))
                    .map(|r| NveParameterResolution {
                        start_date: r.data_from_time,
                        end_date: r.data_to_time,
                        resolution: r.res_time,
                    });
                let sp = StationParameter {
                    unit: Some(par.unit.to_string()),
                    internal_id: id,
                    current_resolution_in_minutes: Resolution::Nve(max_current_resolution),
                    historic_stable_resolution_in_minutes: Resolution::Nve(max_historic_resolution),
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
                    let uk_resolution = UkGovParameterResolution{
                        internal_resolution: measurements_delta_in_minutes,
                        external_period: period,
                    };
                    let sp = StationParameter {
                        unit: None,
                        internal_id: id,
                        current_resolution_in_minutes: Resolution::UkGov(uk_resolution.clone()),
                        historic_stable_resolution_in_minutes: Resolution::UkGov(uk_resolution),
                    };
                    internal_params.push(sp);
                }
            }
        }
        internal_params
    }
}