use std;

use chrono::{DateTime, Duration, Utc};
use reqwest::{self};
use serde::{Deserialize, Serialize};

use crate::data::nve::connect;
use crate::data::nve::observation;
use crate::data::nve::station;

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


pub async fn reqwest_all_stations() -> Result<reqwest::Response, reqwest::Error> {
    let url = "https://hydapi.nve.no/api/v1/Stations?";
    let build = connect::build_nve_httpclient();
    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

pub async fn get_all_stations() -> Result<station::Root, reqwest::Error> {
    let response = reqwest_all_stations().await?;
    let data = response.json::<station::Root>().await?;
    return Ok(data);
}



pub async fn request_latest_nve_observations() -> Result<reqwest::Response, reqwest::Error> {
    let endpoint = "/observations?";
    let build = connect::build_nve_httpclient();
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
    let build = connect::build_nve_httpclient();
    let url = build.url.join(&query).unwrap();
    let response = build.client.get(url)
        .headers(build.header)
        .send().await?;
    Ok(response)
}

async fn request_nve_last_24h_observations(station_id_list: Vec<&str>, parameters: Vec<i64>) -> Result<reqwest::Response, reqwest::Error> {
    let end_date = Utc::now();
    let start_date = end_date - Duration::hours(24);
    let response = get_specific_nve_observations(station_id_list,
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

pub async fn get_specific_nve_observations(station_id_list: Vec<&str>,
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
    let build = connect::build_nve_httpclient();
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

pub async fn reqwest_observations_using_post_to_nve_body(body: Vec<PostToNve>
) -> Result<reqwest::Response, reqwest::Error> {
    let build = connect::build_nve_httpclient();
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
    use tokio;

    use crate::data::{internal, nve};
    use crate::data::nve::observation::deserialize_observations;
    use crate::dev::_read_file;

    static YEAR: i32 = 2010;
    static MONTH: u32 = 01;
    static DAY: u32 = 01;

    static STATION_ID_LIST: [&str; 2] = ["1.200.0", "1.15.0"];
    static PARAMETERS: [i32; 2] = [1000, 1001];
    static RESOLUTION_TIME: i32 = 0;


    #[tokio::test]
    async fn test_deserialize_single_stations() {
        let test = _read_file("src/dev/json/nve/singleStation.json").await.unwrap();
        let root = serde_json::from_str::<nve::station::Root>(&test).unwrap();
        assert_eq!(root.data.len(), 1);
        let river_name = &root.data.get(0).unwrap().river_name;
        let rootDate = root.query_time;
    }
    #[tokio::test]
    async fn test_deserialize_all_stations() {
        let test = _read_file("src/dev/json/nve/allStations.json").await.unwrap();
        let root = serde_json::from_str::<nve::station::Root>(&test).unwrap();
        let river_name = &root.data.get(0).unwrap().river_name;
        let rootDate = root.query_time;
    }

    #[tokio::test]
    async fn test_deserialize_all_stations_to_internal() {
        let test = _read_file("src/dev/json/nve/allStations.json").await.unwrap();
        let nve_root = serde_json::from_str::<nve::station::Root>(&test).unwrap();
        let mut internal_stations: Vec<internal::station::Station> = vec![];
        for daum in &nve_root.data{
            let internal = internal::station::Station::from_nve(&daum).await;
            internal_stations.push(internal);
        }

    }

    #[tokio::test]
    async fn ut_deserialize_observations() {
        let test = _read_file("src/dev/json/nve/allObservations.json").await.unwrap();
        let root = deserialize_observations(&test).unwrap();
        let data = assert_eq!(root.data.len(), 3);
    }
}
