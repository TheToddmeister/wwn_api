use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use strum::Display;
use surrealdb::{opt::RecordId, sql::Id};
use crate::data::internal::observation::Observation;
use crate::data::internal::timeseries::TimeSeries;
use crate::data::internal::timeseries_metadata::TimeSeriesMetaData;
use crate::static_metadata::ParameterDefinitions;

#[derive(Display)]
pub enum Tables{
    StaticRiver,
    StaticStation,
    HistoricObservationData,
    RecentObservationData,
}

#[derive(Serialize, Deserialize)]
pub struct InternalStationId(surrealdb::opt::RecordId);

#[derive(Serialize, Deserialize)]
pub struct InternalObservationId(surrealdb::opt::RecordId);
#[derive(Serialize, Deserialize)]
pub struct DBObservationTableRow {
    pub station_id: String,
    pub parameter_id: ParameterDefinitions,
    pub observations: TimeSeries,
    pub metadata: TimeSeriesMetaData,
    pub updated_when: DateTime<Utc>,
}
impl DBObservationTableRow {

}