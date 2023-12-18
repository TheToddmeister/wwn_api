use serde::{Deserialize, Serialize};

use crate::data::{nve, uk};
use crate::static_metadata::Origin;
use crate::static_metadata::Origin::{NVE, UKGOV};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct River{
    pub name: String,
    pub alias: Vec<String>,
    pub drainage_basin: Option<String>,
    pub tributary_hierarchy: Vec<String>,
    pub catchment_area: Option<u64>,
    pub origin: Origin,
}

impl River{
    pub async fn from_nve(daum: &nve::station::Daum) -> River{
        River{
            name: daum.river_name.to_string(),
            alias: vec![daum.river_name.to_string()],
            drainage_basin: Some(daum.drainage_basin_key.to_string()),
            tributary_hierarchy: daum.hierarchy.to_vec(),
            catchment_area: daum.drainage_basin_area.map(|d| d as u64),
            origin: NVE,
        }
    }
    pub async fn from_ukgov(daum: &uk::station::Item) -> Option<River> {
        let river = daum.river_name.first();
        river.map(|r| River{
            name: r.to_string(),
            alias: daum.river_name.to_vec(),
            drainage_basin: daum.aquifer.to_owned(),
            tributary_hierarchy: daum.river_name.to_vec(),
            catchment_area: daum.catchment_area.map(|d| d as u64),
            origin: UKGOV,
        })
    }
}