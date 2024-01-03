use builder_pattern::Builder;
use chrono::{DateTime, Utc};
use itertools::{concat, Itertools};
use mockall::predicate::ge;
use serde::{Deserialize, Serialize};
use strum::Display;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use surrealdb_extra::query::statement::StatementBuilder;

use crate::data::internal::parameter::StationParameter;
use crate::data::nve;
use crate::data::uk;
use crate::static_metadata::{Nation, Origin, ParameterDefinitions, Regulation};
use crate::util::geo::location::Location;
use crate::util::geo::position::Position;

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
    pub station_parameters: Vec<StationParameter>,
}

impl Station {
    pub async fn from_nve(daum: &nve::station::Daum) -> Self {
        let d = daum;
        let location = Location::location_from_nve(d).await;
        let parameters = StationParameter::from_nve_station(d).await;
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
            station_parameters: parameters,
        }
    }
    pub async fn from_ukgov(item: &uk::station::Item) -> Option<Self> {
        let s = item;
        let id = s.notation.to_string();
        let loc = Location::location_from_uk(item, &id).await;
        let station_parameters = StationParameter::from_ukgov_station(item).await;
        match loc {
            Some(location) => {
                let status = [s.status.is_empty(), s.status.iter().any(|v| v.label == "Active")].iter()
                    .any(|a| a == &true);

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
                    station_parameters,
                };
                Some(station)
            }
            None => None,
        }
    }
}


#[cfg(test)]
mod Tests {
    use tokio;

    use crate::dev;

    use super::*;

    #[tokio::test]
    async fn nve_build_internal_station() {
        let test = dev::_read_file("src/dev/json/nve/gryta/6.10.0station.json").await.unwrap();
        let root = serde_json::from_str::<nve::station::Root>(&test).unwrap();
        let daum = root.data.get(0).unwrap();
        let internal_station = Station::from_nve(daum).await;
        let parameters = internal_station.station_parameters;
    }

    #[tokio::test]
    async fn ukgov_build_internal_station() {
        let test = dev::_read_file("src/dev/json/ukgov/ulting/ultingStation.json").await.unwrap();
        let root = serde_json::from_str::<uk::station::Root>(&test).unwrap();
        let item = root.items.get(0).unwrap();
        let internal_station = Station::from_ukgov(item).await.unwrap();
        let parameters = internal_station.station_parameters;
    }
}

