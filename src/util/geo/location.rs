use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::api::nve::station;
use crate::api::uk;
use crate::static_metadata::LocationType;
use crate::util::geo::position::{Coordinates, Position};
use crate::static_metadata::LocationType::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub label: LocationType,
    pub id: String,
    pub name: String,
    pub position: Vec<Position>,
}

impl Location {
    pub async fn location_from_uk(station: &uk::station::Item) -> Self {
        let s = station;
        Location {
            id: s.notation.to_string(),
            label: STATION,
            name: s.label.to_string(),
            position: Vec::from(
                [
                    Position {
                        name: s.label.to_string(),
                        description: "".to_string(),
                        coordinate: Coordinates {
                            latitude: s.lat,
                            longitude: s.long,
                        },
                    }]),
        }
    }
    pub async fn location_from_nve(daum: &station::Daum) -> Self {
        Location {
            id: daum.station_id.to_string(),
            label: STATION,
            name: daum.station_name.to_string(),
            position: Vec::from(
                [
                    Position {
                        name: daum.station_name.to_string(),
                        description: "".to_string(),
                        coordinate: Coordinates {
                            latitude: daum.latitude,
                            longitude: daum.longitude,
                        },
                    }]),
        }
    }
}
