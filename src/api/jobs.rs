use itertools::Itertools;
use crate::api::{internal, request};
use crate::api::{nve, uk, smih};
use crate::api::api_error::APIError;

pub async fn download_stations_info_and_store_to_db()->Result<(), APIError>{
       let nve_station_info = nve::nve_requests::get_all_stations(false).await?;
       for daum in nve_station_info.data{
              let nve_info = internal::station::Station::from_nve(&daum).await;
       }

       let ukgov_station_info = uk::requests::get_station_info().await?;
       for item in ukgov_station_info.items{
              let ukgov_info =  internal::station::Station::from_ukgov(&item).await;
       }
       Ok(())
}

