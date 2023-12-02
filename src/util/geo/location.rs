use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::api::nve::station;
use crate::util::geo::position::{Coordinates, Position};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub label: String,
    pub id: String,
    pub name: String,
    pub position: Position,
}

impl Location {
    fn location_from_nvestation(daum: station::Daum) -> Self {
        Location {
            id: daum.station_id,
            label: "station".to_string(),
            name: daum.station_name,
            position: Position {
                name: daum.station_name.to_string(),
                description: "".to_string(),
                coordinate: Coordinates {
                    latitude: daum.latitude,
                    longitude: daum.longitude,
                },
            },
        }
    }
}
