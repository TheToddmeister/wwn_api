#[cfg(test)]
mod data_storage_with_db {
    use crate::data::internal;
    use crate::db::config::connection::{connect_to_db, connect_to_local_dev_db};
    use crate::db::persistence::init_static_data_db::build_static_station_info_tables;
    use crate::db::tables::Tables::StaticStation;
    use crate::static_controller;
    use crate::static_controller::StationInfo;
    use crate::static_metadata::Datatype::HistoricObservationMetadata;
    use crate::static_metadata::Origin::{NVE, UKGOV};

    #[tokio::test]
    #[ignore]
    pub async fn static_controller_init_static_does_not_throw(){
        let db = connect_to_db().await.unwrap();
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
    #[ignore]
    /// Require running db with content
    pub async fn test_read_station_identity(){
        let db = connect_to_local_dev_db().await.unwrap();
        let stations: Vec<StationInfo>  = db.query("select location.location_id as loc, origin, status, station_parameters from StaticStation;;").await.unwrap().take(0).unwrap();
        dbg!(stations);
    }

    #[tokio::test]
    #[ignore]
    pub async fn test_itt_build_static_stations_and_store_does_not_throw(){
        let db = connect_to_db().await.unwrap();
        static_controller::static_controller(&db).await.unwrap();
        let w: Vec<internal::timeseries::TimeSeries> = db.select(HistoricObservationMetadata.to_string()).await.unwrap();
    }
}