use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::static_metadata::{Origin, Nation};
use crate::util::geo::location::Location;
use crate::api::nve;
use crate::api::smih;
use crate::api::uk;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub location: Location,
    pub status: bool,
    pub parental_hierarchy: Vec<String>,
    pub last_update: DateTime<Utc>,
    pub origin: Origin,
    pub country: Nation,
    pub measuring_authority_id: String,
    pub station_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct River{
    pub name:String,
    pub alias: Vec<String>,
    pub drainage_basin: Option<String>,
    pub tributary_hierarchy: Option<Vec<String>>,
    pub catchment_area: Option<String>,
}

impl Station{
    pub async fn from_nve(daum: &nve::station::Daum) -> Self{
        let s = daum;
        todo!()
    }
    pub async fn from_ukgov(item: &uk::station::Item) -> Self{
        let s = item;
        let location = Location::location_from_uk(&item).await;
        let status = &s.status.as_ref().map(|a| a.label).unwrap_or_default();
        Self {
            location,
            parental_hierarchy: s.river_name.to_vec(),
            last_update: Utc::now(),
            origin: Origin::NVE,
            country: Nation::Norway,
            status: *status,
            measuring_authority_id: "".to_string(),
            station_type: None,
        }
    }
}

