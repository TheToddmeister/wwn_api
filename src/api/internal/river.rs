use serde::{Deserialize, Serialize};
use crate::api::nve;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct River{
    pub name:String,
    pub alias: Vec<String>,
    pub drainage_basin: Option<String>,
    pub tributary_hierarchy: Vec<String>,
    pub catchment_area: Option<f64>,
}

impl River{
    pub fn from_nve(daum: nve::station::Daum) -> River{
        River{
            name: daum.river_name.clone(),
            alias: vec![daum.river_name.clone()],
            drainage_basin: Some(daum.drainage_basin_key.to_string()),
            tributary_hierarchy: daum.hierarchy,
            catchment_area: daum.drainage_basin_area,
        }
    }
}