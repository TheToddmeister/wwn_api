use anyhow;
use thiserror::Error;
use reqwest;
use http;
use crate::static_metadata::Origin;


#[derive(Error, Debug)]
pub enum APIError {
    #[error("Error while parsing data response from:")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Error from request when contacting:")]
    RequestError(#[from] reqwest::Error),
    #[error("Undefined: ")]
    Undefined(#[from] anyhow::Error)
}

pub async fn handle_http_response_not_200(origin: Origin, response: &reqwest::Response)->Result<(), anyhow::Error>{
    let status = &response.status();
    if !status.is_success(){
        let query = response.url().to_string();
        let error = &response.status();
        return Err(anyhow::anyhow!("Failed to fetch data from {origin:#?}: {error}. \n Query: {query}"))
    }
    Ok(())
}