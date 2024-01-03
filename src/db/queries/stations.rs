use builder_pattern::Builder;
use surrealdb::{engine, Surreal};
use crate::data::internal;
use crate::data::internal::station::Station;
use crate::static_metadata::{Nation, ParameterDefinitions, Regulation};

pub async fn internal_stations_query_builder(filter: StationFilter) -> String {
    " select * from StaticStation ".to_string() + &StationFilter::build_station_filter_query_from_filter(filter).await
}
pub async fn get_internal_stations(db: &Surreal<engine::any::Any>, filter: StationFilter)->Vec<Station>{
    let query = internal_stations_query_builder(filter).await;
    let stations: Vec<Station> = db.query(query).await?.take(0)?;
    stations
}

#[derive(Builder)]
pub struct StationFilter<'f> {
    countries: Vec<Nation>,
    exclude_activity_status_inactive: bool,
    station_ids: Vec<&'f str>,
    regualtion_status: Vec<Regulation>,
    river: Option<&'f str>,
    //geographic_position_and_radius: Option<(Position, i64)>,
    parameters: Vec<ParameterDefinitions>,
}

impl StationFilter {
    pub async fn build_station_filter_query_from_filter(self) -> String{
        //Todo! Sql injection attack?
        let mut conditions_queries = vec![];
        if let Some(river) = &self.river {
            conditions_queries.push(format!(" {river} contains river_name "));
        }
        if !&self.station_ids.is_empty() {
            conditions_queries.push(format!(" {:?} contains location.location_id ", &self.station_ids));
        }
        if !&self.countries.is_empty() {
            conditions_queries.push(format!(" {:?} contains location.location_id ", &self.river));
        }
        if !&self.exclude_activity_status_inactive {
            conditions_queries.push(format!(" activity_status= {:?} ", &self.exclude_activity_status_inactive));
        }
        if !&self.regualtion_status.is_empty() {
            conditions_queries.push(format!(" {:?} contains regulation_status ", &self.regualtion_status));
        }
        if !&self.parameters.is_empty() {
            conditions_queries.push(format!(" {:?} contains regulation_status ", &self.parameters));
        }
        " where ".to_string() + &conditions_queries.join(" and ")
    }
}
#[cfg!(test)]
mod tests{
    use crate::db::config;
    use crate::dev;
    use super::*;
    #[tokio::test]
    pub async fn test_get_internal_stations(){
        let db = config::connection::connect_to_automatic_testing_in_memory_embedded_db().await.unwrap();
        dev::_build_static_database(db).await;
        let filter = StationFilter {
            countries: vec![],
            exclude_activity_status_inactive: false,
            station_ids: vec!["6.10.0"],
            regualtion_status: vec![],
            river: None,
            parameters: vec![],
        };

    }
}