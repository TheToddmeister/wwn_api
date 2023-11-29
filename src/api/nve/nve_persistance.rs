use std::error::Error;
use crate::api::nve::{station, observation};
use chrono::{DateTime, Utc};
use reqwest::Response;
use serde::Serialize;
use serde_json::json;
use serde_json::value::Serializer;
use crate::api::nve::nve_requests::{PostToNve, reqwest_observations_using_PostToNVE_body, PARAMETER};
use crate::api::nve::station::{Daum, ResolutionList, SeriesList};
use tokio;

pub struct StationResolution {
    station_id: String,
    parameter: i64,
    resolution: i64,
    resolution_from: DateTime<Utc>,
    resolution_to: DateTime<Utc>,
}


pub async fn observation_data_request_scheduler(station_metadata: &station::Root,
                                                parameters: &[i64; 8]) -> Result<(), Box<dyn Error>>  {
    for s in &station_metadata.data {
        for p in &*s.series_list {
            //if !parameters.contains(&p.parameter) { continue; };
            for r in &p.resolution_list {
                let reqwest_body = build_observation_request(s, p, r).await;
                let response = reqwest_observations_using_PostToNVE_body(vec![reqwest_body]).await?;
                let content = response.json::<observation::Root>().await?;
                save_observation_response(&s.station_id, &p.parameter, &r.res_time, content).await?;
            }
        }
    }
    Ok(())
}

async fn build_observation_request(s: &Daum, p: &SeriesList, r: &ResolutionList) -> PostToNve {
    let start_date = &r.data_from_time;
    let end_date = &r.data_to_time;

    let iso_8601_start: String = start_date.to_rfc3339()[0..16].to_string();
    let iso_8601_end: String = end_date.to_rfc3339()[0..16].to_string();

    let reference_time = format!("{iso_8601_start}/{iso_8601_end}");

    let reqwest_body = PostToNve {
        stationId: s.station_id.to_string(),
        parameter: p.parameter.to_string(),
        resolutionTime: r.res_time.to_string(),
        referenceTime: reference_time.to_string(),
    };
    reqwest_body
}

async fn save_observation_response(station_id: &str, parameter_id: &i64, resolution: &i64, content: observation::Root) -> Result<(), Box<dyn Error>> {
    let filename = format!("src/api/nve/historic_data/{station_id}_{parameter_id}_{resolution}.json");
    let json = serde_json::to_string(&content)?;
    tokio::fs::write(filename, &json).await?;
    Ok(())
}

pub async fn build_observation_jsonbody_from_metadata(start_date: DateTime<Utc>,
                                                      end_date: DateTime<Utc>,
                                                      station_metadata: Vec<StationResolution>,
) -> Vec<PostToNve> {
    let iso_8601_start = &start_date.to_rfc3339()[0..16];
    let iso_8601_end = &end_date.to_rfc3339()[0..16];
    let reference_time = format!("{iso_8601_start}/{iso_8601_end}");
    let posts = station_metadata.iter().map(|s|
        PostToNve {
            stationId: s.station_id.to_string(),
            parameter: s.parameter.to_string(),
            referenceTime: reference_time.to_string(),
            resolutionTime: s.resolution.to_string(),
        }
    ).collect();
    posts
}

#[cfg(test)]
mod tests {
    use super::*;

      async  fn read_test_json(path: &str) -> String {
        let json: String = std::fs::read_to_string(path).unwrap();
        return json;
    }

    #[tokio::test]
    async fn test_persist_observations() {
        let station_metadata = read_test_json("src/dev/json/nve/allStations.json").await;
        let root = serde_json::from_str::<station::Root>(&*station_metadata).unwrap();
        let failed = observation_data_request_scheduler(&root, &PARAMETER).await.unwrap();
    }
}
