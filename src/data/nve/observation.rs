use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

pub fn _read_and_deserialize_test_observations() -> Root {
    let path: &str = "src/test/json/observations.json";
    let json: String = std::fs::read_to_string(path).unwrap();
    let root = deserialize_observations(&json).unwrap();
    return root;
}

pub fn deserialize_observations(json: &str) -> Result<Root, Box<dyn std::error::Error>>{
    let nve = serde_json::from_str::<Root>(&json)?;
    Ok(nve)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current_link: String,
    pub api_version: String,
    pub license: String,
    pub created_at: DateTime<Utc>,
    pub query_time: String,
    pub item_count: i64,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, )]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub station_id: String,
    pub station_name: String,
    pub parameter: i64,
    pub parameter_name: String,
    pub parameter_name_eng: String,
    pub serie_version_no: i64,
    pub method: String,
    pub unit: String,
    pub observation_count: i64,
    pub observations: Vec<Observation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, )]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub time: DateTime<Utc>,
    pub value: Option<f64>,
    pub correction: i64,
    pub quality: i64,
}

// TESTS ################################################################

#[cfg(test)]
mod tests {
    use serde::{self};
    use tokio;

    use crate::data::nve::observation::{Daum, deserialize_observations, Observation, Root};

    use super::*;

    fn read_test_observation()->String{
        let path: &str = "src/test/json/observations.json";
        let json: String = std::fs::read_to_string(path).unwrap();
        return json;
    }

    #[tokio::test]
    async fn test_observation_deserializsation(){
        let a=false;
        let path: &str = "dev/json/nve/singleObservation.json";
        let json: String = std::fs::read_to_string(path).unwrap();
        let root = deserialize_observations(&json).unwrap();
        let daum = &root.data[0];
        
    }

}
