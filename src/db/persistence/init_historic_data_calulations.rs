use chrono::{DateTime, Utc};
use futures::future::join_all;
use futures::StreamExt;
use itertools::Itertools;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use warp;

use crate::data::{nve, uk};
use crate::data::api_error::APIError;
use crate::data::internal::parameter::ObservationResolution::{Nve, UkGov};
use crate::data::internal::timeseries::TimeSeries;
use crate::data::internal::timeseries_metadata;
use crate::data::nve::requests::PostToNve;
use crate::db::error::APIPersistenceError;
use crate::db::persistence;
use crate::db::tables::{DBObservationTableRow, Tables};
use crate::static_controller::StationInfo;
use crate::static_metadata::Origin;

pub async fn init_historic_observation_data(db: &Surreal<Any>, min_historic_date: &DateTime<Utc>, max_historic_data: &DateTime<Utc>, all_minimal_stations: &Vec<StationInfo>) -> Result<(), APIPersistenceError> {
    for (key, group) in &all_minimal_stations.iter().group_by(|o| &o.origin) {
        let minimal_stations_from_origin: Vec<&StationInfo> = group.collect_vec();
        match key {
            Origin::NVE => { get_nve_observation_data_as_internal_observation(db,minimal_stations_from_origin, min_historic_date, max_historic_data).await?; }
//            Origin::UKGOV => { internal_timeseries.append(&mut get_ukgov_observation_data_as_internal_observations(minimal_stations_from_origin, min_historic_date, max_historic_data).await?); }
            Origin::UKGOV => {}
            Origin::SMIH => {}
        }
    }
    Ok(())
}

pub async fn get_nve_observation_data_as_internal_observation(db: &Surreal<Any>,
                                                              stations: Vec<&StationInfo>,
                                                              min_historic_date: &DateTime<Utc>,
                                                              max_historic_data: &DateTime<Utc>) -> Result<(), APIPersistenceError> {
    let mut nve_requests = vec![];
    for n in &stations {
        for p in &n.station_parameters {
            match &p.historic_stable_resolution_in_minutes {
                Nve(Some(nor)) => {
                    let nve_query_body = PostToNve::build_nve_observation_postquery_body(min_historic_date,
                                                                                         &max_historic_data,
                                                                                         &n.loc,
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
    for chunk in nve_post_body.chunks(10) {
        let nve_root = nve::requests::get_observations_using_post_to_nve_body(chunk).await?;
        let internal_observations_chunk = nve_root.data.iter()
            .filter_map(|d| TimeSeries::from_nve(d, min_historic_date, max_historic_data))
            .collect_vec();
        for obs in internal_observations_chunk{
            let db_historic_metadata = timeseries_metadata::TimeSeriesMetaData::from_internal_timeseries(&obs).await;
            let db_observation_inset = DBObservationTableRow {
                station_id: obs.station_id.to_owned(),
                parameter_id: obs.parameter_id.to_owned(),
                observations: obs.to_owned(),
                metadata: db_historic_metadata,
                updated_when: Utc::now(),
            };
            let observation_response:Vec<DBObservationTableRow> = db.create(Tables::HistoricObservationData.to_string())
                .content(db_observation_inset).await?;
        }
    }
    Ok(())
}

pub async fn get_ukgov_observation_data_as_internal_observations(stations: Vec<&StationInfo>,
                                                                 min_historic_date: &DateTime<Utc>,
                                                                 max_historic_date: &DateTime<Utc>) -> Result<Vec<TimeSeries>, APIError> {
    let mut uk_internal_timeseries = vec![];
    for n in &stations {
        for p in &n.station_parameters {
            match &p.historic_stable_resolution_in_minutes {
                UkGov(pars) => {
                    let uk_root = uk::requests::get_observations_from_station(&n.loc,
                                                                              &p.internal_parameter_id,
                                                                              &min_historic_date.date_naive(),
                                                                              &max_historic_date.date_naive()).await?;
                    let internal_uk_timeseries = TimeSeries::from_ukgov(&uk_root,
                                                                        &n.loc,
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