use std::io::Split;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Serialize};

pub struct HistoricObservations{
    query_time: DateTime<Utc>,
    station_id: String,
    observations: Vec<HistoricObservation>

}
//Datum (svensk sommartid);Vattenföring (Dygn);Kvalitet;;
pub struct HistoricObservation{
    date_time: DateTime<Utc>,
    quality: String,
    value: String
}
impl HistoricObservations{
    async fn from_smih_historic(csv_str: &str, station_id: &str) -> Self {
        let rows: Vec<&str> = csv_str.split("\n").collect();
        let data = &rows[8..];
        let mut observations = vec![];
        for row in data{
            let row_values: &[&str] = &*row.split(";").collect::<Vec<_>>();
            let obs = HistoricObservation{
                date_time: row_values[0].parse().unwrap(),
                value: row_values[1].to_string(),
                quality: row_values[2].to_string(),
            };
            observations.push(obs);
        }
        let obs = HistoricObservations{
            query_time: Utc::now(),
            station_id: station_id.to_string(),
            observations
        };
        obs

    }

}


#[cfg(test)]
mod tests{
    use super::*;
    use tokio;
    use crate::dev;

    #[tokio::test]
    pub async fn test_smih_historic_data_deserialization(){
        let csv = dev::_read_file("src/dev/json/smih/historic_corrected_data.csv").await.unwrap();
        let content = HistoricObservations::from_smih_historic(&csv, "1111").await;
        let a = "";

    }
}