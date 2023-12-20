use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::data::{internal, nve};
use crate::data::internal::station::Station;
use crate::static_metadata::Origin;

pub async  fn persist_historic_observation_meta_data(station_ids: &[&str], db: Surreal<Client>){
    //let ids: Vec<Station> = db.query("select * from StaticStation where location_ID=",station_ids)).await?
}

pub async fn find_parameters_for_nve_station(){
    todo!()
}