use serde::Deserialize;
use surrealdb::{engine, Surreal};
use surrealdb_extra::query::statement::StatementBuilder;
use crate::data::internal::parameter::StationParameter;
use crate::static_metadata::Datatype::StaticStation;
use crate::static_metadata::Origin;
use crate::util::geo::location::Location;

#[derive(Deserialize, Debug)]
pub struct MinimalStation {
    pub origin: Origin,
    pub location: Location,
    pub river: String,
    pub status: bool,
    pub station_parameters: Vec<StationParameter>,
}
impl MinimalStation{
    pub async fn fetch_minimal_stations_from_db(db: &Surreal<engine::any::Any>) ->surrealdb::Result<Vec<Self>>{
        let minimal_stations: Vec<MinimalStation> = db.query("select origin, location, river, status, station_parameters from StaticStation;").await.unwrap().take(0)?;
        Ok(minimal_stations)
    }
}



#[cfg(test)]
mod tests{
    use crate::db::config;
    use crate::db::queries::generics::MinimalStation;

    #[tokio::test]
    async fn fetch_test_database_to_file(){
        let db = config::connection::connect_to_automatic_testing_in_memory_embedded_db().await.unwrap();
        db.import("src/dev/sql/synthetic_sql/StaticStations.surql").await.unwrap();
        let minimal_stations = MinimalStation::fetch_minimal_stations_from_db(&db).await;
    }
}