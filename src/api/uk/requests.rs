use std::error::Error;
use anyhow::anyhow;
use reqwest;
use crate::api::api_error;
use crate::static_metadata::Origin;

pub async fn request_station_info()->Result<reqwest::Response, reqwest::Error>{
    let url_string = "https://nrfaapps.ceh.ac.uk/nrfa/ws/station-info?station=*&format=csv&fields=all";
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    return Ok(response);
}

pub async fn handle_http_response_not_200(origin: Origin)->Result<(), anyhow::Error>{
    let response = request_station_info().await?;
    let status = &response.status();
    let origin_name = origin
    if !status.is_success(){
        let query = response.url().to_string();
        let error = &response.status();
        return Err(anyhow::anyhow!("Failed to fetch data from {origin}: {error}. \n Query: {query}"))
    }
    Ok(())
}