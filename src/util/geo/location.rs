use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::data::nve::station;
use crate::data::uk;
use crate::static_metadata::{LocationType, Nation};
use crate::static_metadata::LocationType::*;
use crate::static_metadata::Nation::Uk;
use crate::util::geo::position::{Coordinates, Position};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub label: LocationType,
    pub location_id: String,
    pub name: String,
    pub country: Nation,
    pub position: Position,
}

impl Location {
    pub async fn location_from_uk(station: &uk::station::Item, id: &str) -> Option<Self> {
        let s = station;
        let latlong = [s.lat.first(), s.long.first()];
        let name = s.label.first().unwrap_or(&"Unnknown".to_string()).to_owned(); //Fixme!
        match latlong {
            [Some(x), Some(y)] => {
                let loc = Location {
                    country: Uk,
                    location_id: id.to_owned(),
                    label: STATION,
                    name: name.to_owned(),
                    position: Position {
                        name,
                        description: "".to_owned(),
                        coordinate: Coordinates {
                            latitude: *x,
                            longitude: *y,
                        },
                    },
                };
                Some(loc)
            }
            _ => None //Todo
        }
    }
    pub async fn location_from_nve(daum: &station::Daum) -> Self {
        Location {
            country: Nation::Norway,
            location_id: daum.station_id.to_string(),
            label: STATION,
            name: daum.station_name.to_string(),
            position:
            Position {
                name: daum.station_name.to_string(),
                description: "".to_string(),
                coordinate: Coordinates {
                    latitude: daum.latitude,
                    longitude: daum.longitude,
                }
            }
        }
    }
}
