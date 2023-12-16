use anyhow;
use thiserror::Error;
use reqwest;
use http;
use crate::static_metadata::Origin;
use surrealdb;
use crate::api::api_error::APIError;


#[derive(Error, Debug)]
pub enum APIPersistenceError {
    #[error("Error while requesting data:")]
    ApiError(#[from] APIError),
    #[error("Error in the persistence layer between API to DB: ")]
    PersistenceError(#[from] surrealdb::Error),
    #[error("Error while parsing data response from:")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Error from request when contacting:")]
    RequestError(#[from] reqwest::Error),
    #[error("Undefined: ")]
    Undefined(#[from] anyhow::Error)
}
