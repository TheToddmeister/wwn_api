use std::collections::HashMap;
use chrono::{DateTime, Utc};
use envy::Error;
use reqwest;
use futures;
use futures::stream::iter;
use crate::api::smih::station_recent_observations::Root;
use serde::Deserialize;


async fn request_smih_historic_data(station_id: &str, parameter: u8) -> Result<reqwest::Response, reqwest::Error> {
    let url_string = format!("https://opendata-download-hydroobs.smhi.se/api/version/latest/parameter/{parameter}/station/{station_id}/period/corrected-archive/data.csv");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

async fn request_smih_latest_observation_data(parameter_id: u8) -> Result<reqwest::Response, reqwest::Error> {
    let url_string = format!("https://opendata-download-hydroobs.smhi.se/api/version/latest/parameter/{parameter_id}/station-set/all/period/latest-hour/data.csv");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

async fn deserialize_csv_from_smih(text: &str) -> Vec<Vec<String>>{
    let rows:Vec<Vec<String>> = text.to_string().lines().map(|q| q.split(';').map(|a|a.to_string()).collect()).collect();
    rows
}

async fn get_smih_24h_observations_data(parameter_id: u8) -> Result<Root, reqwest::Error> {
    let response = request_smih_latest_observation_data(parameter_id).await?;
    let status = response.status();

    if !status.is_success() {
        return Err(response.error_for_status().unwrap_err());
    }
    let root = response.json::<Root>().await?;
    Ok(root)
}

#[derive(Deserialize)]
struct CorrectedData<'csv> {
    date: &'csv str,
    value: f64,
    quality: &'csv str,
}

#[cfg(test)]
mod SMIHtest {
    use std::fmt::Debug;
    use super::*;
    use tokio;
    use warp::Stream;
    use std::error::Error;
    use futures::stream::StreamExt;

    #[tokio::test]
    pub async fn test_historic_json_deserialize() {}

    #[tokio::test]
    pub async fn test_historic_json_request() {
        let parameter_id = 2;
        let url_string = format!("https://opendata-download-hydroobs.smhi.se/api/version/latest/parameter/{parameter_id}/station-set/all/period/latest-hour/data.json");
        let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
        let response = reqwest::get(url).await.unwrap();
        let status = response.status();
        assert_eq!(status, 200);
    }

    #[tokio::test]
    pub async fn test_deserialize_csv_request() {
        let response = request_smih_historic_data("11", 1).await.unwrap();
        let text = response.text().await.unwrap();
        let values = deserialize_csv_from_smih(&text).await;
         
    }

    #[tokio::test]
    pub async fn test_historic_json_request2() {
        let parameter_id = 2;
        let response = request_smih_latest_observation_data(2).await.unwrap();
        let status = response.status();
        assert_eq!(status, 200)
    }

    #[tokio::test]
    pub async fn test_historic_json() {
        let root = get_smih_24h_observations_data(2).await.unwrap();
        let stations = root;
    }
}