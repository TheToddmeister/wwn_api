use std::collections::HashMap;
use crate::api::nve::station;

pub struct Location {
    label: String,
    id: String,
    name: String,
    longtitude: f64,
    latitude: f64,
}

pub struct EucledianMatrix{
    stations: Vec<String>,
    matrix: Vec<Vec<f64>>,
}
impl EucledianMatrix{
    pub fn calculate_euclidean_matrix_from_locations(locations: &Vec<Location>) -> EucledianMatrix {
        let len = locations.len().clone();
        let mut distance_matrix = vec![vec![0.0; len]; len];
        for x in 0..len {
            for y in x..len {
                distance_matrix[x][y] = calculate_euclidian_distance(&locations[x], &locations[y])
            }
        }
        EucledianMatrix{
            stations: locations.iter().map(|s| s.id.clone()).collect(),
            matrix: distance_matrix
        }
    }
    pub fn locate_associated_stations_for_location(location: &Location, locations: Vec<Location>, stations: Vec<station::Daum>){
        let station_locations: Vec<&Location> = locations.iter().filter(|l| l.label=="station").collect();
        let shared_parental_hierarchy = "";
    }
}

impl Location {
    fn location_from_NVEstation(daum: station::Daum) -> Location {
        Location {
            id: daum.station_id,
            label: "station".to_string(),
            name: daum.station_name,
            longtitude: daum.longitude,
            latitude: daum.latitude,
        }
    }
}


fn calculate_euclidian_distance(location1: &Location, location2: &Location) -> f64 {
    let distance = ((&location1.latitude - location2.latitude).exp2() + (&location1.longtitude - location2.longtitude)).sqrt();
    distance
}