use std::error::Error;

use chrono::NaiveDate;
use futures::StreamExt;
use reqwest;

use crate::data::api_error::{APIError, handle_http_response_not_200};
use crate::data::uk::{observation, station};
use crate::static_metadata::Origin;

pub async fn request_station_info() -> Result<reqwest::Response, reqwest::Error> {
    let url_string = "http://environment.data.gov.uk/hydrology/id/stations?status.label=Active&_limit=200000";
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

pub async fn get_station_info() -> Result<station::Root, APIError> {
    let response = request_station_info().await?;
    handle_http_response_not_200(Origin::UKGOV, &response).await?;
    let root = response.json::<station::Root>().await?;
    Ok(root)
}

pub async fn request_station_observations(station_id: &str, parameter: &'static str, min_date: NaiveDate) -> Result<reqwest::Response, reqwest::Error> {
    let min_date_string = min_date.format("%Y-%m-%d");
    let url_string = format!("https://environment.data.gov.uk/hydrology/data/readings.json?measure={parameter}&min-date={min_date_string}&station={station_id}");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

pub async fn get_station_observations(station_id: &str, parameter: &'static str, min_date: NaiveDate) -> Result<observation::Root, APIError> {
    let response = request_station_observations(station_id, parameter, min_date).await?;
    handle_http_response_not_200(Origin::UKGOV, &response).await?;
    let root = response.json::<observation::Root>().await?;
    Ok(root)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize};
    use tokio;

    use crate::data::{internal, uk};
    use crate::data::internal::parameter::Parameter;
    use crate::data::nve::nve_requests::PARAMETER;
    use crate::dev::_read_file;
    use crate::static_metadata;

    use super::*;

    #[tokio::test]
    async fn test_deserialize_all_station_information() {
        let csv = _read_file("src/dev/json/ukgov/stationInformation.json").await.unwrap();
        let data = serde_json::from_str::<station::Root>(&csv).unwrap();
        assert_eq!(data.items.get(0).clone().unwrap().notation, "052d0819-2a32-47df-9b99-c243c9c8235b");
    }

    #[tokio::test]
    async fn test_deserialize_all_stations_to_internal() {
        let test = _read_file("src/dev/json/ukgov/stationInformation.json").await.unwrap();
        let uk_root = serde_json::from_str::<uk::station::Root>(&test).unwrap();
        let mut internal_stations: Vec<Option<internal::station::Station>> = vec![];
        for daum in &uk_root.items {
            let internal = internal::station::Station::from_ukgov(&daum).await;
            internal_stations.push(internal);
        }
        assert_eq!(internal_stations.len(), uk_root.items.len());
    }

    #[tokio::test]
    async fn test_deserialize_station_observation() {
        let csv = _read_file("src/dev/json/ukgov/stationObservation.json").await.unwrap();
        let data = serde_json::from_str::<observation::Root>(&csv).unwrap();
        assert_eq!(data.items.get(0).unwrap().value, Some(0.433));
    }

    #[tokio::test]
    async fn test_deserialize_station_observation_to_internal() {
        let csv = _read_file("src/dev/json/ukgov/stationObservation.json").await.unwrap();
        let data = serde_json::from_str::<observation::Root>(&csv).unwrap();

        let station_id = "052d0819-2a32-47df-9b99-c243c9c8235b";
        let parameter_id = static_metadata::Parameter::FLOW;
        let start_date = DateTime::parse_from_str("2022-10-11T00:00:00", "%Y %m %dT%H:%M:%S").unwrap().with_timezone(&Utc);
        let end_date = DateTime::parse_from_str("2023-11-11T00:00:00", "%Y %m %dT%H:%M:%S").unwrap().with_timezone(&Utc);

        let inter = Parameter::from_ukgov(&data, station_id, parameter_id, &start_date, &end_date);
    }

    #[tokio::test]
    async fn test_get_station_observation() {
        let min_date = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap();
        let root = get_station_observations("052d0819-2a32-47df-9b99-c243c9c8235b","waterFlow", min_date).await.unwrap();
        assert_eq!(root.items.get(0).unwrap().value, Some(0.433));
    }
}
