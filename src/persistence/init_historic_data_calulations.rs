use chrono::{Datelike, DateTime, Duration, Utc};
use futures::StreamExt;
use itertools::Itertools;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use warp;
use warp::Filter;

use crate::{persistence, static_metadata, util};
use crate::data::internal::parameter::ObservationResolution::{Nve, UkGov};
use crate::data::internal::parameter::{NveObservationResolution, StationParameter};
use crate::data::{nve, uk};
use crate::data::nve::requests::PostToNve;
use crate::persistence::init_static_data_db;
use crate::static_metadata::get_minimum_historic_data_date;
use crate::static_metadata::Origin;
use crate::static_controller::MinimalStation;

pub async fn init_historic_observation_data(min_historic_date: &DateTime<Utc>, max_historic_data: &DateTime<Utc>, stations: &Vec<MinimalStation>) -> Result<(), persistence::error::APIPersistenceError> {
    for (_, group) in &stations.iter().group_by(|o| &o.origin) {
        let stations: Vec<&MinimalStation> = group.collect_vec();
        let mut nve_requests = vec![];
        let mut uk_stations = vec![];
        for n in &stations {
            for p in &n.station_parameter {
                match &p.historic_stable_resolution_in_minutes {
                    Nve(pars) => {
                        if let Some(p) == pars {
                            let nve_query_body =
                                PostToNve::build_nve_observation_postquery_body(min_historic_date, &max_historic_data, &n.station_id, &p.internal_parameter_id,p);

                            nve_requests.push(nve_query_body);
                        }
                    }
                    UkGov(pars) => {
                        let uk_root = uk::requests::get_station_observations(&n.station_id, &p.internal_parameter_id, &min_historic_date.date_naive(), &max_historic_data.date_naive()).await?;
                        uk_stations.push(uk_root);
                    }
                }
            }
        }
        let nve_root = nve::requests::get_observations_using_post_to_nve_body(&nve_requests).await?
    }
    Ok(())
}