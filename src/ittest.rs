#[cfg(test)]
mod data_storage_with_db {
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;
    use crate::dev::_read_file;
    use crate::persistence;
    use crate::persistence::init_static_data_db::build_static_station_info_tables;

    ///
    /// Mocks the most resource intensive endpoints using local files for testing and development.
    /// Calling these endpoints will be intercepted and a local file string is returned instead
    ///  - allStations
    ///
    pub async fn create_mockito_server_with_allStations_and_allObservations(){
        let nve_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();
        let uk_all_stations = _read_file("src/dev/json/nve/stationInformation.json").await.unwrap();
        //let smih_all_stations = _read_file("src/dev/json/nve/allStations.json").await.unwrap();

        let mut smih = mockito::Server::new_async().await;
        let mut nve = mockito::Server::new_async().await;
        let mut uk = mockito::Server::new_async().await;

        let nve_mock = nve.mock("/GET", "https://hydapi.nve.no/api/v1/Stations?")
            .with_status(200)
            .with_body(nve_all_stations)
            .create_async().await;

        let uk_mock = nve.mock("/GET", "http://environment.data.gov.uk/hydrology/id/stations?status.label=Active&_limit=200000")
            .with_status(200)
            .with_body(uk_all_stations)
            .create_async().await;
    }
    ///
    /// Requires an existing db instance at:
    ///             address: "127.0.0.1",
    ///             port: "8000",
    ///             default_namespace: "test",
    ///             default_db: "test_db",
    ///
    pub async fn connect_with_test_db_client()->Surreal<Client>{
        let db_connect_info = persistence::connection::DbConnect{
            address: "127.0.0.1",
            port: "8000",
            default_namespace: "test",
            default_db: "test_db",
        };
        persistence::connection::connect_db(db_connect_info).await.unwrap()
    }

    pub async fn test_itt_static_stations(){
        let db = connect_with_test_db_client().await;
        create_mockito_server_with_allStations_and_allObservations().await;
        build_static_station_info_tables()
    }
}