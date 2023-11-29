use anyhow;
use thiserror::Error;
use reqwest;
use http;

#[derive(Error, Debug)]
pub enum APIResponseError {
    #[error("Error while parsing data response from:")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Error from request when contacting:")]
    RequestError(#[from] reqwest::Error)
}