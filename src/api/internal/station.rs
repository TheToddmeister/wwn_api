use chrono::{DateTime, Duration, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::static_metadata::{Origin, Nation, Regulation};
use crate::util::geo::location::Location;
use crate::api::nve;
use crate::api::smih;
use crate::api::uk;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Station {
    pub location: Location,
    pub status: bool,
    pub status_description: Vec<String>,
    pub parental_hierarchy: Vec<String>,
    pub last_update: DateTime<Utc>,
    pub origin: Origin,
    pub country: Nation,
    pub measuring_authority_id: Option<String>,
    pub station_type: Option<String>,
    pub regulation_status: Regulation,
}

impl Station{
    pub async fn from_nve(daum: &nve::station::Daum) -> Self{
        let d = daum;
        let location = Location::location_from_nve(d).await;
        Self {
            location,
            parental_hierarchy: d.hierarchy.clone(),
            last_update: Utc::now(),
            origin: Origin::NVE,
            country: Nation::Norway,
            status: d.station_status_name.eq("Aktiv"),
            status_description: vec![d.station_status_name.to_string()],
            measuring_authority_id: Some(d.owner.to_string()),
            station_type: Some(d.station_type_name.to_string()),
            regulation_status: d.catchment_reg_type_name.clone(),
        }

    }
    pub async fn from_ukgov(item: &uk::station::Item) -> Option<Self>{
        let s = item;
        let loc = Location::location_from_uk(item).await;
        match loc {
            Some(location) => {
                let status = [s.status.is_empty(), s.status.iter().any(|v|v.label == "Active")].iter()
                    .any(|a| a==&true);

                let station = Self {
                    location,
                    parental_hierarchy: s.river_name.to_vec(),
                    last_update: Utc::now(),
                    origin: Origin::UKGOV,
                    country: Nation::Uk,
                    status,
                    status_description: s.status.iter().map(|q| q.label.to_string()).collect(),
                    measuring_authority_id: Some("".to_string()),
                    station_type: None,
                    regulation_status: Regulation::UNKNOWN,
                };
                Some(station)
            },
            None => None,
        }

    }
}
