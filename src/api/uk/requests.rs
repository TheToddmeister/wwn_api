use std::error::Error;
use anyhow::anyhow;
use csv_async::{AsyncReaderBuilder, StringRecord};
use futures::StreamExt;
use reqwest;
use crate::api;
use crate::api::api_error;
use crate::api::api_error::handle_http_response_not_200;
use crate::static_metadata::Origin;
use crate::api::uk::station;
use crate::api::uk::station::Station;
use crate::util::geo::location::Location;
use crate::util::geo::position::Position;

pub async fn request_station_info()->Result<reqwest::Response, reqwest::Error>{
    let url_string = "https://nrfaapps.ceh.ac.uk/nrfa/ws/station-info?station=*&format=csv&fields=all";
    let url = url::Url::parse(&url_string).expect("Failed to parse url to string");
    let response = reqwest::get(url).await?;
    return Ok(response);
}
async fn deserialize_ukgov_station_info_text(csv_text: &str) ->Result<Vec<Station>, Box<dyn Error>>{
    let mut stations: Vec<Station> = vec![];
    let mut rdr = AsyncReaderBuilder::new().create_deserializer(csv_text.as_bytes());
    let mut records = rdr.into_deserialize::<Station>();
    while let Some(record) = records.next().await {
        stations.push(record?)
    }
    Ok(stations)
}

pub async fn get_station_info()->Result<station::Station, api_error::APIError>{
 //   let response = request_station_info().await?;
//    handle_http_response_not_200(Origin::UKGOV, response).await?;
//    let text = response.text().await?
    todo!()

}
#[cfg(test)]
mod tests {
    use chrono::serde::ts_microseconds::deserialize;
    use super::*;
    use crate::dev::_read_file;
    use serde::{self, Serialize, Deserialize};
    use tokio;

    #[tokio::test]
    async fn test_deserialize_station_information(){
        let csv = _read_file("src/dev/json/ukgov/stationInformation.csv").await.unwrap();
        let content = deserialize_ukgov_station_info_text(&csv).await.unwrap();
        let station: &Station = content.get(0).unwrap();
        let station1 = content.get(1).unwrap();
        let a = &station.river;
    }
}
