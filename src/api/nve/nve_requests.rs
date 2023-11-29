use reqwest::{self};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Utc, Duration};
use std;
use std::cmp::max;
use std::collections::{HashMap};
use std::io::{Error, ErrorKind};

use crate::api::nve::station;
use crate::api::nve::observation;
use crate::api::nve::nve_connect;
use crate::api::nve::nve_requests::RequestToServiceError::HTTPError;

enum RequestToServiceError {
    HTTPError,
    Error,
}

pub struct NveHTTPError {
    pub is_the_problem_nve: bool,
    pub code: u16,
    pub code_message: Option<String>,
    pub path: String,
}

pub const GAUGES: [&str; 7] = ["12.209.0", "2.13.0", "2.39.0", "2.595.0", "2.661.0", "7.29.0", "7.30.0"];
//pub const PARAMETER: [i64; 1] = [1000];
pub const PARAMETER: [i64; 8] = [0, 1000, 1001, 1003, 1066, 17, 2002, 2003];

#[derive(Serialize)]
pub struct PostToNve {
    pub stationId: String,
    pub parameter: String,
    pub resolutionTime: String,
    pub referenceTime: String,
}


pub async fn get_all_nve_stations(exclude_inactive: bool) -> Result<reqwest::Response, reqwest::Error> {
    let endpoint = "Stations?";

    let build = nve_connect::build_nve_httpclient();
    let mut url = build.url.join(endpoint).unwrap();
    if exclude_inactive == true {
        url.join("Active=OnlyActive").unwrap();
    }

    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

pub async fn reqwest_all_stations(exclude_inactive: bool) -> Result<reqwest::Response, reqwest::Error> {
    let response = get_all_nve_stations(exclude_inactive).await?;
    return Ok(response);
}

pub async fn get_all_stations(exclude_inactive: bool) -> Result<station::Root, reqwest::Error> {
    let response = reqwest_all_stations(exclude_inactive).await?;
    let data = response.json::<station::Root>().await?;
    return Ok(data);
}


pub async fn request_latest_nve_observations() -> Result<reqwest::Response, reqwest::Error> {
    let endpoint = "/observations?";
    let build = nve_connect::build_nve_httpclient();
    let url = build.url.join(endpoint).unwrap();
    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

pub async fn get_latest_nve_observations() -> Result<observation::Root, reqwest::Error> {
    let response = request_latest_nve_observations().await?;
    let data = response.json::<observation::Root>().await?;
    Ok(data)
}


pub async fn get_specific_nve_observations_and_parameters(parameters: &str, stations: &str) -> Result<reqwest::Response, reqwest::Error> {
    let query = "Observations?Parameters=".to_owned() + parameters + "&StationId=" + stations;
    let build = nve_connect::build_nve_httpclient();
    let url = build.url.join(&query).unwrap();
    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

async fn request_nve_last_24h_observations(station_id_list: Vec<&str>, parameters: Vec<i64>) -> Result<reqwest::Response, reqwest::Error> {
    let end_date = Utc::now();
    let start_date = end_date - Duration::hours(24);
    let response = get_specific_NVE_observations(station_id_list,
                                                 parameters,
                                                 start_date,
                                                 end_date).await?;
    return Ok(response);
}

pub async fn get_last_24h_observations(station_id_list: Vec<&str>, parameters: Vec<i64>) -> Result<observation::Root, reqwest::Error> {
    let response = request_nve_last_24h_observations(station_id_list, parameters).await?;
    let root = response.json::<observation::Root>().await?;
    return Ok(root);
}

pub async fn get_specific_NVE_observations(station_id_list: Vec<&str>,
                                           parameters: Vec<i64>,
                                           startDate: DateTime<Utc>,
                                           endDate: DateTime<Utc>, ) -> Result<reqwest::Response, reqwest::Error> {
    let stations_query_parameter = station_id_list.join(",");
    let parameters: Vec<String> = parameters.iter().map(|a| a.to_string()).collect();
    let parameters_query_parameter = parameters.join(",");
    let iso_8601_start = &startDate.to_rfc3339()[0..16];
    let iso_8601_end = &endDate.to_rfc3339()[0..16];
    let reference_time = format!("{iso_8601_start}/{iso_8601_end}");

    let query = format!("Observations?Parameter={parameters_query_parameter}&StationId={stations_query_parameter}&ReferenceTime={reference_time}&ResolutionTime=0");
    let build = nve_connect::build_nve_httpclient();
    let url = build.url.join(&query).unwrap();
    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

async fn find_highest_resolution_or_reject(station_id: &str,
                                           parameter: i64,
                                           station_metadata: &station::Root, ) -> Option<i64> {
    let available = station_metadata.data.iter()
        .find(|daum| daum.station_id == station_id).map(|a| a.series_list.iter()
        .find(|s| s.parameter == parameter)
        .map(|r| r.resolution_list.iter()
            .map(|v| v.res_time).max()))
        .flatten()
        .flatten();
    return available;
}

pub async fn reqwest_observations_using_PostToNVE_body(body: Vec<PostToNve>
) -> Result<reqwest::Response, reqwest::Error> {
    let build = nve_connect::build_nve_httpclient();
    let endpoint = "Observations?";
    let query_url = build.url
        .join(endpoint)
        .unwrap();
    let response = build.client
        .post(query_url)
        .json(&body)
        .headers(build.header)
        .send()
        .await?;
    return Ok(response);
}

#[cfg(test)]
mod Tests {
    use std::error::Error;
    use super::*;
    use tokio;
    use crate::api::nve::nve_requests;
    use crate::api::nve::observation::deserialize_observations;

    static YEAR: i32 = 2010;
    static MONTH: u32 = 01;
    static DAY: u32 = 01;

    static STATION_ID_LIST: [&str; 2] = ["1.200.0", "1.15.0"];
    static PARAMETERS: [i32; 2] = [1000, 1001];
    static RESOLUTION_TIME: i32 = 0;


    fn read_test_json(filename: &str) -> String {
        let path: String = "dev/json/nve/".to_string() + filename;
        let json: String = std::fs::read_to_string(path).unwrap();
        return json;
    }

    #[tokio::test]
    async fn test_deserialize_single_observations() {
        let test = read_test_json("singleObservation.json");
        let root = deserialize_observations(&test).unwrap();
        assert_eq!(root.data.len(), 1);
        let observation_date = root.data.get(0).unwrap().observations.get(0).unwrap().time;
        let rootDate = root.query_time;
    }

    #[tokio::test]
    async fn ut_deserialize_observations() {
        let test = read_test_json("allObservations.json");
        let root = deserialize_observations(&test).unwrap();
        let data = assert_eq!(root.data.len(), 3);
    }

    #[tokio::test]
    async fn xtt_request_24H_from_observations() {
        let response = request_nve_last_24h_observations(nve_requests::GAUGES.to_vec(), nve_requests::PARAMETER.to_vec());
        let result = response.await.unwrap();
        let status = &result.status();
        let root = &result.json::<observation::Root>().await.unwrap();
        assert_eq!(status.is_success(), true);
        let data0 = &root.data.get(0);
    }
}
