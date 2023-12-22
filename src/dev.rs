use serde::{Deserialize, Serialize};
use tokio;
use tokio::sync::OnceCell;
use crate::dev;

pub async fn _read_file(path: &str) -> Result<String, tokio::io::Error> {
    let json: String = tokio::fs::read_to_string(path).await?;
    Ok(json)
}
///
/// Mocks the most resource intensive endpoints using local files for testing and development.
/// Calling these endpoints will be intercepted and a local file string is returned instead
///  - allStations
///
#[cfg(test)]
pub async fn create_mocked_allStation_endpoints(){
    let nve_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();
    let uk_all_stations = _read_file("src/dev/json/ukgov/stationInformation.json").await.unwrap();
    //let smih_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();

    let mut smih = mockito::Server::new_async().await;
    let mut nve = mockito::Server::new_async().await;
    let mut uk = mockito::Server::new_async().await;

    let nve_mock = nve.mock("/GET", "https://hydapi.nve.no/api/v1/Stations?")
        .with_status(200)
        .with_body(nve_all_stations)
        .create_async().await;

    let uk_mock = uk.mock("/GET", "http://environment.data.gov.uk/hydrology/id/stations?status.label=Active&_limit=200000")
        .with_status(200)
        .with_body(uk_all_stations)
        .create_async().await;
}
#[cfg(test)]
pub async fn create_mockito_server_with_single_station(){
    let nve_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();
    let uk_all_stations = _read_file("src/dev/json/ukgov/stationInformation.json").await.unwrap();
    //let smih_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();

    let mut smih = mockito::Server::new_async().await;
    let mut nve = mockito::Server::new_async().await;
    let mut uk = mockito::Server::new_async().await;

    let nve_mock = nve.mock("/GET", "https://hydapi.nve.no/api/v1/Stations?")
        .with_status(200)
        .with_body(nve_all_stations)
        .create_async().await;

    let uk_mock = uk.mock("/GET", "http://environment.data.gov.uk/hydrology/id/stations?status.label=Active&_limit=200000")
        .with_status(200)
        .with_body(uk_all_stations)
        .create_async().await;
}
