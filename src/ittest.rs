#[cfg(test)]
mod data_storage_with_db {
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;
    use crate::dev::_read_file;
    use crate::{dev, persistence};
    use crate::persistence::init_static_data_db::build_static_station_info_tables;

    ///
    /// Requires an existing db instance at:
    ///             address: "127.0.0.1",
    ///             port: "8000",
    ///             default_namespace: "test",
    ///             default_db: "test",
    ///
    pub async fn connect_with_test_db_client()->Surreal<Client>{
        //surreal.exe start memory --log debug -A --auth --user test --pass test --bind 127.0.0.1:8000
        let db_connect_config = persistence::connection::DbConnectionConfig {
            address: "127.0.0.1",
            port: "8000",
            default_namespace: "test",
            default_db: "test",
        };
        persistence::connection::connect_db(db_connect_config).await.unwrap()
    }
    #[tokio::test]
    pub async fn endToEnd_controller_init_does_not_throw(){
        let db = connect_with_test_db_client().await;
        dev::create_mocked_allStation_endpoints().await;
    }


    #[tokio::test]
    pub async fn test_itt_build_static_stations_and_store_does_not_throw(){
        let db = connect_with_test_db_client().await;
        dev::create_mocked_allStation_endpoints().await;


    }
}