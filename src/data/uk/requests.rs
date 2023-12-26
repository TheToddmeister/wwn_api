use std::error::Error;

use chrono::NaiveDate;
use futures::StreamExt;
use reqwest;

use crate::data::api_error::{APIError, handle_http_response_not_200};
use crate::data::uk::{observation, station};
use crate::static_metadata::{Origin, ParameterDefinitions};

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

pub async fn request_station_observations(station_id: &str, parameter: &ParameterDefinitions, min_date: &NaiveDate, max_date: &NaiveDate) -> Result<reqwest::Response, reqwest::Error> {
    let min_date_string = min_date.format("%Y-%m-%d").to_string();
    let max_date_string = max_date.format("%Y-%m-%d").to_string();
    let uk_parameter = ParameterDefinitions::to_uk(parameter);
    let url_string = format!("https://environment.data.gov.uk/hydrology/data/readings.json?measure={uk_parameter}&min-date={min_date_string}&max-date={max_date_string}&station={station_id}");
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}

pub async fn get_station_observations(station_id: &str, parameter: &ParameterDefinitions, min_date: &NaiveDate, max_date:&NaiveDate) -> Result<observation::Root, APIError> {
    let response = request_station_observations(station_id, parameter, min_date, max_date).await?;
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
    use crate::data::internal::timeseries::TimeSeries;
    use crate::data::nve::requests::PARAMETER;
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
        let parameter_id = static_metadata::ParameterDefinitions::FLOW;
        let start_date = DateTime::parse_from_str("2022-10-11T00:00:00", "%Y %m %dT%H:%M:%S").unwrap().with_timezone(&Utc);
        let end_date = DateTime::parse_from_str("2023-11-11T00:00:00", "%Y %m %dT%H:%M:%S").unwrap().with_timezone(&Utc);

        let inter = TimeSeries::from_ukgov(&data, station_id, &parameter_id, &start_date, &end_date);
    }

    #[tokio::test]
    async fn test_get_station_observation() {
        let min_date = NaiveDate::from_ymd_opt(1993, 10, 19).unwrap();
        let max_date = NaiveDate::from_yo_opt(2022, 1).unwrap();
        let root = get_station_observations("052d0819-2a32-47df-9b99-c243c9c8235b",&ParameterDefinitions::FLOW, &min_date, &max_date).await.unwrap();
        let obs_to_high = root.items.iter().find(|f| f.date_time>=max_date.and_hms_opt(00,00, 01).unwrap().and_utc());
        let obs_to_low = root.items.iter().find(|f| f.date_time>=min_date.and_hms_opt(00,00, 01).unwrap().and_utc());
    }
}
