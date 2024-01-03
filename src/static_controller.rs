use chrono::{Datelike, Utc};
use futures::StreamExt;
use serde::Deserialize;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use warp;

use crate::{db, util};
use crate::data::internal::parameter::StationParameter;
use crate::db::persistence::{init_historic_data_calulations, init_static_data_db};
use crate::static_metadata::get_minimum_historic_data_date;
use crate::static_metadata::Origin;

#[derive(Deserialize, Debug)]
pub struct StationInfo {
    pub loc: String,
    pub origin: Origin,
    pub status: bool,
    pub station_parameters: Vec<StationParameter>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    origin: String,
}

pub async fn static_controller(db: &Surreal<Any>) -> Result<(), db::error::APIPersistenceError> {
    let min_historic_date = get_minimum_historic_data_date().await;
    let max_historic_data = util::time::get_first_moment_of_year(Utc::now().year() - 2).await.expect("Failed to calulate max historic data.");
    init_static_data_db::build_static_station_info_tables(db).await?;
    let stations: Vec<StationInfo>  = db.query("select location.location_id as loc, origin, status, station_parameters from StaticStation;").await?.take(0)?;
    init_historic_data_calulations::init_historic_observation_data(&db, min_historic_date, &max_historic_data, &stations).await?;
    Ok(())
}


