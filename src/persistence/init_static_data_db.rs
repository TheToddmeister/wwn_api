use std::collections::{HashMap, HashSet};
use envy::Error;
use itertools::Itertools;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::log::info;

use crate::data::{internal, nve, uk};
use crate::data::internal::river::River;
use crate::data::internal::station::Station;
use crate::persistence::error::APIPersistenceError;
use crate::persistence::tables;
use crate::persistence::tables::Tables::{StaticRiver, StaticStation};
use crate::util::geo::location::Location;

#[tracing::instrument]
pub async fn build_static_station_info_tables(db: &Surreal<Client>) -> Result<(), APIPersistenceError> {
    let drop_table_staticrivers: Vec<String> = db.delete(StaticRiver.to_string()).await?;
    let drop_table_staticstations: Vec<String> = db.delete(StaticStation.to_string()).await?;
    let future_nve = nve::nve_requests::get_all_stations(true);
    let future_ukgov = uk::requests::get_station_info();
    let mut rivers = HashSet::new();
    let nve_stations = &future_nve.await?.data;
    for daum in nve_stations {
        //Todo: Collect as create as result and log errors instead of panic
        let id = &daum.station_id;
        let loc = Location::location_from_nve(daum).await;
        let dbloc: Vec<Location> = db.create("Location").content(loc).await?;
        
        let internal_station = Station::from_nve(daum).await;
        let dbstation: Option<Station> = db
            .create((StaticStation.to_string(), id.to_string()))
            .content(internal_station).await?;

        let internal_river  = River::from_nve(daum).await;
        rivers.insert(internal_river);

    }
    info!("Successfully persisted NVE");
    for item in &future_ukgov.await?.items {
        let id = item.notation.to_string().replace('-', "_");
        let loc = Location::location_from_uk(item, &id).await;
        let dbloc: Vec<Location> = db.create("Location").content(loc).await?;

        let internal_station = Station::from_ukgov(item).await;

        if let Some(i) = internal_station.as_ref().map(|a| &a.location) {
            let dbstation: Option<Station> = db
                .create(("StaticStation".to_string(), &i.location_id))
                .content(&internal_station).await?;
        }
        let internal_river  = River::from_ukgov(item).await;
        if let Some(ir) = internal_river {
            rivers.insert(ir);
        }
    }
    
    info!("Successfully persisted UKGOV");
    Ok(())
}

