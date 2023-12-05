use std::error::Error;
use anyhow::anyhow;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use csv_async::{AsyncReaderBuilder, StringRecord};
use futures::StreamExt;
use reqwest;
use crate::api;
use crate::api::{api_error, uk};
use crate::api::api_error::{APIError, handle_http_response_not_200};
use crate::api::shared_api_metadata_struct::Parameter;
use crate::static_metadata::Origin;
use crate::api::uk::station;
use crate::static_metadata::Origin::UKGOV;


pub async fn request_station_info() ->Result<reqwest::Response, reqwest::Error>{
    let from_string = "1993-10-19";
    let end_string = Utc::now().naive_utc().date().to_string();
    let url_string = "http://environment.data.gov.uk/hydrology/id/stations?status.label=Active&_limit=200000&from=".to_string()+from_string+"&to="+&*end_string;
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}
pub async fn get_station_info() ->Result<station::Root, APIError>{
    let response = request_station_info().await?;
    handle_http_response_not_200(UKGOV, &response).await?;
    let root = response.json::<uk::station::Root>().await?;
    Ok(root)
}
pub async fn request_station_observations(station_id: &str, parameter: Parameter)->Result<reqwest::Response, reqwest::Error>{
    let url_string = "http://environment.data.gov.uk/hydrology/data/readings.json?station=".to_string();
    http://environment.data.gov.uk/hydrology/id/stations/28101/measures?observedProperty=waterLevel
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    Ok(response)
}


#[cfg(test)]
mod tests {
    use chrono::serde::ts_microseconds::deserialize;
    use super::*;
    use crate::dev::_read_file;
    use serde::{self, Serialize, Deserialize};
    use tokio;

    #[tokio::test]
    async fn test_deserialize_station_information(){
        let csv = _read_file("src/dev/json/ukgov/stationInformation.csv").await.unwrap();
    }
}
