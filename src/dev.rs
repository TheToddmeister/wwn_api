use serde::{Deserialize, Serialize};
use tokio;
use tokio::sync::OnceCell;
use crate::dev;
use crate::dev::DevProfiles::{AutoTest, DevTest};

pub enum DevProfiles{
    AutoTest,
    DevTest,
    StaticTest,
    Dev,
    Prod,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DevConfig{
    dev_profile: DevProfiles,
}
static DEV_PROFILE: OnceCell<DevConfig> = OnceCell::new();
impl DevConfig{
    pub async fn read_dev_profile()->&'static DevProfiles{
        let profile = &DEV_PROFILE.get_or_init(envy::from_env::<DevConfig>().expect("Failed to read devConfig from env")).await.dev_profile;
        profile
    }
    /*
    Auto test is intended to be a profile where tests are run in a memory db and partially/fully mocked enpoints
     */
    pub async fn set_dev_profile_to_auto_test(){
       &DEV_PROFILE.set(DevConfig{ dev_profile: AutoTest} .expect("Failed to set devConfig to TEST")).await;
    }
    /*
    Dev test is intended to be a profile where tests are run using a local external in memory db and partially moceked enpoints
    This allows for manual insight into test results, but requires manual configurations and reset of db between tests
     */
    pub async fn set_dev_profile_to_dev_test(){
        &DEV_PROFILE.set(DevConfig{ dev_profile: DevTest} .expect("Failed to set devConfig to TEST")).await;
    }
}

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
pub async fn create_mocked_all_station_endpoints(){
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
