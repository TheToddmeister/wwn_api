use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

pub struct HistoricObservations{
    query_time: DateTime<Utc>,
    station_id: String,
    station_name: String,
    latitude: String,
    longitude: String,
    recent: String,
    data: Vec<HistoricObservation>

}
//Datum (svensk sommartid);Vattenföring (Dygn);Kvalitet;;
pub struct HistoricObservation{
    date_time: DateTime<Utc>,
    quality: String,
    value: String
}
impl HistoricObservations{
    async fn from_smih_historic(csv_str: &str) -> Self {
        let rows: Vec<&str> = csv_str.split("\n").collect();
        let data = &rows[8..];
        for row in data{
            todo!()
        }
        let obs = HistoricObservations{
            query_time: Utc::now(),
            station_id: "".to_string(),
            station_name:"".to_string(),
            latitude: "".to_string(),
            longitude: "".to_string(),
            recent: "".to_string(),
            data: vec![]

        };
        obs

    }

}


#[cfg(test)]
mod tests{
    use super::*;
    use tokio;
    use crate::devutils;

    #[tokio::test]
    pub async fn test_smih_historic_data_deserialization(){
        let csv = devutils::_read_file("src/dev/json/smih/historic_corrected_data.csv").await;
        let content = HistoricObservations::from_smih_historic(&csv).await;
        let a = "";
    }
}