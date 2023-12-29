use chrono::{Datelike, DateTime, Duration, Utc};
use futures::future::join_all;
use futures::StreamExt;
use itertools::Itertools;
use serde::Deserialize;
use surrealdb::engine::any::Any;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tokio::join;
use tracing::log::log;
use warp;
use warp::Filter;

use crate::{persistence, static_metadata, util};
use crate::data::internal::parameter::ObservationResolution::{Nve, UkGov};
use crate::data::internal::parameter::{NveObservationResolution, StationParameter};
use crate::data::{internal, nve, uk};
use crate::data::api_error::APIError;
use crate::data::internal::timeseries::TimeSeries;
use crate::data::nve::requests::PostToNve;
use crate::persistence::init_static_data_db;
use crate::static_metadata::get_minimum_historic_data_date;
use crate::static_metadata::Origin;
use crate::static_metadata::Datatype::{Location, HistoricObservationMetadata};
use crate::static_controller::MinimalStation;

pub async fn init_historic_observation_data(db: &Surreal<Any>, min_historic_date: &DateTime<Utc>, max_historic_data: &DateTime<Utc>, all_minimal_stations: &Vec<MinimalStation>) -> Result<(), persistence::error::APIPersistenceError> {
    let mut internal_timeseries = vec![];
    for (key, group) in &all_minimal_stations.iter().group_by(|o| &o.origin) {
        let minimal_stations_from_origin: Vec<&MinimalStation> = group.collect_vec();
        match key {
            Origin::NVE => { internal_timeseries.append(&mut get_nve_observation_data_as_internal_observation(minimal_stations_from_origin, min_historic_date, max_historic_data).await?); },
            Origin::UKGOV => { internal_timeseries.append(&mut get_ukgov_observation_data_as_internal_observations(minimal_stations_from_origin, min_historic_date, max_historic_data).await?); }
            Origin::SMIH => {}
        }
    }
    let observations: Vec<TimeSeries> = db.create(HistoricObservationMetadata.to_string())
        .content(internal_timeseries).await?;
    Ok(())
}

pub async fn get_nve_observation_data_as_internal_observation(stations: Vec<&MinimalStation>,
                                                              min_historic_date: &DateTime<Utc>,
                                                              max_historic_data: &DateTime<Utc>) -> Result<Vec<internal::timeseries::TimeSeries>, APIError> {
    let mut nve_requests = vec![];
    for n in &stations {
        for p in &n.station_parameters {
            match &p.historic_stable_resolution_in_minutes {
                Nve(Some(nor)) => {
                    let nve_query_body = PostToNve::build_nve_observation_postquery_body(min_historic_date,
                                                                                         &max_historic_data,
                                                                                         &n.location_id,
                                                                                         &p.internal_parameter_id, &nor);
                    nve_requests.push(nve_query_body);
                }
                Nve(None) => {}
                _ => {
                    let err = &p.historic_stable_resolution_in_minutes;
                    let err_string = format!("{err:#?}");
                    tracing::error!("Found a station in get_nve_observation_data_as_internal_observation() where the ResolutionTypeSignature is wrong: {err_string} ")
                }
            }
        }
    }
    let nve_post_body = join_all(nve_requests).await;
    let nve_root = nve::requests::get_observations_using_post_to_nve_body(&nve_post_body).await?;
    let internal_observations = nve_root.data.iter()
        .map(|d| internal::timeseries::TimeSeries::from_nve(d, min_historic_date, max_historic_data))
        .flatten().collect();
    Ok(internal_observations)
}

pub async fn get_ukgov_observation_data_as_internal_observations(stations: Vec<&MinimalStation>,
                                                                 min_historic_date: &DateTime<Utc>,
                                                                 max_historic_date: &DateTime<Utc>) -> Result<Vec<TimeSeries>, APIError> {
    let mut uk_internal_timeseries = vec![];
    for n in &stations {
        for p in &n.station_parameters {
            match &p.historic_stable_resolution_in_minutes {
                UkGov(pars) => {
                    let uk_root = uk::requests::get_station_observations(&n.location_id,
                                                                         &p.internal_parameter_id,
                                                                         &min_historic_date.date_naive(),
                                                                         &max_historic_date.date_naive()).await?;
                    let internal_uk_timeseries = TimeSeries::from_ukgov(&uk_root,
                                                                        &n.location_id,
                                                                        &p.internal_parameter_id,
                                                                        min_historic_date,
                                                                        max_historic_date);
                    uk_internal_timeseries.push(internal_uk_timeseries);
                }
                _ => {
                    let err = &p.historic_stable_resolution_in_minutes;
                    let err_string = format!("{err:#?}");
                    tracing::error!("Found a station in get_nve_observation_data_as_internal_observation() where the ResolutionTypeSignature is wrong: {err_string} ")
                }
            }
        }
    }
    Ok(uk_internal_timeseries)
}