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
use crate::data::nve::requests::PostToNve;
use crate::persistence::{init_static_data_db, init_historic_data_calulations};
use crate::static_metadata::get_minimum_historic_data_date;
use crate::static_metadata::Origin;

#[derive(Deserialize)]
pub struct MinimalStation {
    pub station_id: String,
    pub origin: Origin,
    pub status: bool,
    pub station_parameter: Vec<StationParameter>,
}


pub async fn static_controller(db: &Surreal<Client>) -> Result<(), persistence::error::APIPersistenceError> {
    let min_historic_date = get_minimum_historic_data_date().await;
    let max_historic_data = util::time::get_first_moment_of_year(Utc::now().year() - 2).await.expect("Failed to calulate max historic data.");
    init_static_data_db::build_static_station_info_tables(db).await?;
    let stations: Vec<MinimalStation> = db.query("select id, origin, status, station_parameters from StaticStation").await?.take(0)?;
    init_historic_data_calulations::get_historic_observation_data(min_historic_date, &max_historic_data, &stations).await?;
    Ok(())
}

