use chrono::{Datelike, DateTime, Duration, Utc};
use futures::StreamExt;
use itertools::Itertools;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use warp;
use warp::Filter;

use crate::{persistence, static_metadata, util};
use crate::data::internal::parameter::Resolution::{Nve, UkGov};
use crate::data::internal::parameter::{NveParameterResolution, StationParameter};
use crate::data::nve::requests::PostToNve;
use crate::persistence::init_static_data_db;
use crate::static_metadata::get_minimum_historic_data_date;
use crate::static_metadata::Origin;

#[derive(Deserialize)]
struct MinimalStation {
    id: String,
    origin: Origin,
    status: bool,
    station_parameter: Vec<StationParameter>,
}

async fn nve_build_observation_query_body(min_historic_date: &DateTime<Utc>, max_historic_data: &DateTime<Utc>, n: &MinimalStation, p: &StationParameter, pars: &Option<NveParameterResolution>) -> Option<PostToNve> {
    let nve_query_body = pars.as_ref().map(|f| {
        let parameter_min_date = &f.start_date.max(*min_historic_date).to_rfc3339();
        let parameter_max_date = &max_historic_data.to_rfc3339();
        let reference_time = format!("{parameter_min_date}/{parameter_max_date}");
        PostToNve {
            stationId: n.id.to_string(),
            parameter: static_metadata::ParameterDefinitions::to_nve(&p.internal_parameter_id).to_string(),
            resolutionTime: f.resolution.to_string(),
            referenceTime: reference_time.to_string(),
        }
    });
    nve_query_body
}

pub async fn static_controller(db: &Surreal<Client>) -> Result<(), persistence::error::APIPersistenceError> {
    let min_historic_date = get_minimum_historic_data_date().await;
    let max_historic_data = util::time::get_first_moment_of_year(Utc::now().year() - 2).await.expect("Failed to calulate max historic data.");
    init_static_data_db::build_static_station_info_tables(db).await?;
    let stations: Vec<MinimalStation> = db.query("select id, origin, status, station_parameters from StaticStation").await?.take(0)?;
    init_historic_observation_data(min_historic_date, &max_historic_data, &stations).await;
    Ok(())
}

async fn init_historic_observation_data(min_historic_date: &DateTime<Utc>, max_historic_data: &DateTime<Utc>, stations: &Vec<MinimalStation>) {
    for (_, group) in &stations.iter().group_by(|o| &o.origin) {
        let stations: Vec<&MinimalStation> = group.collect_vec();
        let mut nve_requests = vec![];
        for n in &stations {
            for p in &n.station_parameter {
                match &p.historic_stable_resolution_in_minutes {
                    Nve(pars) => {
                        let nve_query_body = nve_build_observation_query_body(min_historic_date, &max_historic_data, n, p, pars).await;
                        nve_requests.push(nve_query_body);
                    }
                    UkGov(pars) => {}
                }
            }
        }
    }
}
