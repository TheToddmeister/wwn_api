#[cfg(test)]
mod data_storage_with_db {
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;
    use crate::static_controller;
    use crate::dev::_read_file;
    use crate::persistence::connection::connect_to_in_memory_embedded_db;
    use crate::{dev, persistence};
    use crate::data::internal;
    use crate::persistence::init_static_data_db::build_static_station_info_tables;
    use crate::persistence::tables::Tables::{StaticRiver, StaticStation};
    use crate::static_metadata::Origin::{NVE, UKGOV};

    #[tokio::test]
    pub async fn static_controller_init_static_does_not_throw(){
        dev::create_mocked_allStation_endpoints().await;
        let db = connect_to_in_memory_embedded_db().await.unwrap();
        build_static_station_info_tables(&db).await.unwrap();
        let nve_station_result:Option<internal::station::Station> = db.select((StaticStation.to_string(), "6.10.0")).await.unwrap();
        let nve_station = nve_station_result.unwrap();
        assert_eq!(&nve_station.origin, &NVE);
        assert_eq!(&nve_station.location.name, "Gryta");
        let uk_stations: Vec<internal::station::Station> = db.select(StaticStation.to_string()).await.unwrap();
        let uk_station_result:Option<internal::station::Station> = db.select((StaticStation.to_string(), "000cee27-182a-40f3-b0e9-9728b572b012")).await.unwrap();
        let uk_station = uk_station_result.unwrap();
        assert_eq!(&uk_station.origin, &UKGOV);
        assert_eq!(&uk_station.location.name, "North Street");

    }
    #[tokio::test]
    pub async fn test_itt_build_static_stations_and_store_does_not_throw(){
        dev::create_mocked_allStation_endpoints().await;
        let db = connect_to_in_memory_embedded_db().await.unwrap();
        static_controller::static_controller(&db).await.unwrap();
        let w = db.query("select")
    }
}