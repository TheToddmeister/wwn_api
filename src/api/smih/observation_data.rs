use std::io::Split;
use chrono::{DateTime, Utc};
use csv_async::AsyncReaderBuilder;
use futures::StreamExt;
use serde::{self, Deserialize, Serialize};
use crate::api::shared_api_metadata_struct::{};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricObservation{
    date_time: DateTime<Utc>,
    quality: String,
    value: String
}

async fn deserialize_smih_station_info_text(csv_text: &str) ->Result<Vec<HistoricObservation>, Box<dyn std::error::Error>>{
    let mut stations: Vec<HistoricObservation> = vec![];
    let mut rdr = AsyncReaderBuilder::new().create_deserializer(csv_text.as_bytes());
    let mut records = rdr.into_deserialize::<HistoricObservation>();
    while let Some(record) = records.next().await {
        stations.push(record?)
    }
    Ok(stations)
}


#[cfg(test)]
mod tests{
    use super::*;
    use tokio;
    use crate::dev;

    #[tokio::test]
    pub async fn test_smih_historic_data_deserialization(){
        let csv = dev::_read_file("src/dev/json/smih/historic_corrected_data.csv").await.unwrap();
        let content = deserialize_smih_station_info_text(&csv).await.unwrap();

    }
}