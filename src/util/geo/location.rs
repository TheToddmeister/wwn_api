use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::data::nve::station;
use crate::data::uk;
use crate::static_metadata::LocationType;
use crate::static_metadata::LocationType::*;
use crate::util::geo::position::{Coordinates, Position};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub label: LocationType,
    pub location_id: String,
    pub name: String,
    pub position: Vec<Position>,
}

impl Location {
    pub async fn location_from_uk(station: &uk::station::Item, id: &str) -> Option<Self> {
        let s = station;
        let latlong =  [s.lat.first(), s.long.first()];
        match latlong {
            [Some(x), Some(y)] =>{
                let loc = Location {
                    location_id: id.to_string(),
                    label: STATION,
                    name: s.label.to_string(),
                    position: Vec::from(
                        [
                            Position {
                                name: s.label.to_string(),
                                description: "".to_string(),
                                coordinate: Coordinates {
                                    latitude: *x,
                                    longitude: *y,
                                },
                            }]),
                };
                Some(loc)
            },
            _ => None
        }
        
    }
    pub async fn location_from_nve(daum: &station::Daum) -> Self {
        Location {
            location_id: daum.station_id.to_string(),
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
