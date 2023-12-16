use chrono;
use chrono::{DateTime, Utc};
use envy;
use envy::Error;
use serde::{Deserialize, Serialize};
use crate::api::internal::station::Station;

pub enum ApiContent{
    StationContent(Station)
}
pub struct WwnMetadata{
    pub version: GitVersion,
    pub minimum_compatibility_version: String,
    pub service_content: ApiContent,
    pub request_datetime: DateTime<Utc>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitVersion {
    pub VERSION_UPDATE_DATETIME: DateTime<Utc>,
    pub VERSION_NAME: String,
}
impl GitVersion{
    pub fn read_from_env()->Result<Self, Error>{
       envy::from_env::<GitVersion>()
    }
}

