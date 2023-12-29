use std::io::BufRead;

use chrono::NaiveDate;
use csv_async::AsyncReaderBuilder;
use futures::StreamExt;
use serde::{self, Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricObservation{
    date_time: NaiveDate,
    value: String,
    quality: String,
}

async fn deserialize_smih_station_observation<>(csv_text: &str) ->Result<Vec<HistoricObservation>, Box<dyn std::error::Error>>{
    //let csv_bytes: String= csv_text.lines().skip(100).collect::<Vec<&str>>().join("\n").to_string();
    let mut stations: Vec<HistoricObservation> = vec![];
    let mut rdr = AsyncReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .create_deserializer(csv_text.as_bytes());
    let mut records = rdr.into_deserialize::<HistoricObservation>();
    while let Some(record) = records.next().await {
        stations.push(record?);
    }

    Ok(stations)
}


#[cfg(test)]
mod tests{
    use tokio;

    use crate::dev;

    use super::*;
    /*

    #[tokio::test]
    pub async fn test_smih_historic_data_deserialization(){
        let csv = dev::_read_file("src/dev/json/smih/historic_corrected_data.csv").await.unwrap();
        let content = deserialize_smih_station_observation(&csv).await.unwrap();
        let ho1 = &content[0];
        assert_eq!(ho1.value, "12".to_string());
    }
    */
}