use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq,  Serialize, Deserialize)]
pub struct DailyParameterMetaData {
    pub ordinal: u32,
    pub count: u32,
    pub mean: f64,
    pub max: Option<f64>,
    pub min: Option<f64>
}
