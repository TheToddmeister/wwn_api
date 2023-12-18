use crate::data::{internal, nve};
use crate::static_metadata::Origin;

pub async  fn persist_historic_observation_meta_data(station_ids: &[&str], origin: Origin, static_station_data:  internal::station::Station){
    for station_id in station_ids{
        
    }
}

pub async fn find_parameters_for_nve_station(){
    todo!()
}