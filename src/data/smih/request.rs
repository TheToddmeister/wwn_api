use futures;
use reqwest;
use serde::Deserialize;

use crate::data::smih::station_and_recent_observations::Root;

async fn request_smih_historic_data(station_id: &str, parameter: u8) -> Result<reqwest::Response, reqwest::Error> {
    let url_string = format!("https://opendata-download-hydroobs.smhi.se/api/version/latest/parameter/{parameter}/station/{station_id}/period/corrected-archive/data.csv");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

async fn request_smih_stations_with_recent_observation_data(parameter_id: u8) -> Result<reqwest::Response, reqwest::Error> {
    let url_string = format!("https://opendata-download-hydroobs.smhi.se/api/version/latest/parameter/{parameter_id}/station-set/all/period/latest-hour/data.csv");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

pub async fn get_smih_stations_with_recent_observation_data(parameter_id: u8) -> Result<Root, reqwest::Error>{
   let response = request_smih_stations_with_recent_observation_data(parameter_id).await?;
    let root = response.json::<Root>().await?;
    Ok(root)
}

#[cfg(test)]
mod SMIHtest {
    use std::fmt::Debug;
    use tokio;
    use super::*;

    #[tokio::test]
    pub async fn get_station_with_recent_observation_data(){

    }
}