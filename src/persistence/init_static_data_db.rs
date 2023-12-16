use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing::log::info;
use tables::Tables;

use crate::api::{nve, uk};
use crate::api::internal::river::River;
use crate::api::internal::station::Station;
use crate::persistence::error::APIPersistenceError;
use crate::persistence::tables;
use crate::persistence::tables::Tables::{StaticRiver, StaticStation};

#[tracing::instrument]
pub async fn build_static_station_info_table(db: Surreal<Client>) -> Result<(), APIPersistenceError> {
    let drop_table_staticrivers: Vec<String> = db.delete(StaticRiver.to_string()).await?;
    let drop_table_staticstations: Vec<String> = db.delete(StaticStation.to_string()).await?;
    let future_nve = nve::nve_requests::get_all_stations(true);
    let future_ukgov = uk::requests::get_station_info();

    let nve_stations = &future_nve.await?.data;
    for daum in nve_stations {
        let internal_station = Station::from_nve(daum).await;

        let dbstation: Option<Station> = db
            .create((StaticStation.to_string(), &internal_station.location.id))
            .content(internal_station).await?;

        let internal_river  = River::from_nve(daum).await;
        //let river: Option<River> = db.create((StaticRiver.to_string(), &internal_river.name)).content(internal_river).await?;

    }
    info!("Successfully persisted NVE");
    for item in &future_ukgov.await?.items {
        let internal_station = Station::from_ukgov(item).await;

        if let Some(i) = internal_station.as_ref().map(|a| &a.location) {
            let dbstation: Option<Station> = db
                .create(("StaticStation".to_string(), &i.id))
                .content(&internal_station).await?;
        }
    info!("Successfully persisted UKGOV")
    }
    Ok(())
}