use std::collections::HashMap;
use std::fmt::format;
use crate::persistence::{db, db_structure};
use crate::util::geo::{location, position};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{Connection, Surreal};
use crate::api::api_error::APIError;
use crate::api::{internal, nve, uk};
use crate::static_metadata::Datatype::{Location, Station};


pub async fn build_static_station_info_table(db: Surreal<Client>) -> Result<(), APIError> {
    let mut stations: HashMap<String, &internal::station::Station> = HashMap::new();
    let nve_root = nve::nve_requests::get_all_stations(true).await?;
    for nve_daum in nve_root.data {
        let data = internal::station::Station::from_nve(&nve_daum).await;
        stations.insert(data.location.id, &data);
    };
    let ukgov_stations = uk::requests::get_station_info().await?;
    for uk_item in ukgov_stations.items {
        let data = internal::station::Station::from_ukgov(&uk_item).await;
        stations.insert(data.location.id, &data);
    };
    for station in stations {
        let insert: Option<internal::station::Station> = db.create(("River", station.0))
            .content(station.1);
    }
    Ok(())
}