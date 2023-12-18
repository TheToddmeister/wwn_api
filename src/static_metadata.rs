use phf::{phf_map, PhfHash};
use serde::{Deserialize, Serialize};

pub const static_stations_nve: [&str; 11] =  ["2.13.0", "2.39.0", "2.39.0", "2.595.0", "2.661.0", "109.20.0", "109.20.0", "12.209.0", "122.14.0", "7.29.0", "7.30.0"];
pub const static_stations_ukgov: [&str; 5] = ["fcb795f4-07bc-4ae0-8372-041644e7f275", "b9933a62-f326-4d77-9206-ebb335161831", "9ad5d28c-7cfe-46db-b39d-58701689cd59", "17f5b1f6-9779-4cbe-a0a3-978d3326bc33", "24066dfc-316c-4093-b236-57b9d675150f"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Parameter {
    FLOW,
    WATERLEVEL,
    TEMPERATURE,
    RAINFALL,
}
pub struct ParameterMapper {
    pub flow: Option<&'static str>,
    pub waterlevel: Option<&'static str>,
    pub temperature:  Option<&'static str>,
    pub rainfall:  Option<&'static str>,
}

pub const SMIH: ParameterMapper = &ParameterMapper {
    flow: Some("Vattenföring (15 min)"),
    waterlevel: Some("Vattenstånd"),
    temperature: Some("Vattendragstemperatur"),
    rainfall: None,
};

pub const NVE: &'static ParameterMapper = &ParameterMapper {
    flow: Some("1001"),
    waterlevel: Some("1000"),
    temperature: Some("1003"),
    rainfall: Some("0"),
};

pub const UKGOV: &'static ParameterMapper = &ParameterMapper {
    flow: Some("waterFlow"),
    waterlevel: Some("rainWater"),
    temperature: Some("temperature"),
    rainfall: Some("waterLevel"),
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nation{
    Norway,
    Sweden,
    Uk,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Datatype {
    River,
    Station,
    Location,
    User
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Origin {
    NVE,
    SMIH,
    UKGOV,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType{
    STATION,
    PARKING,
    PUT_IN,
    TAKE_OUT,
    CAMP,
    SPOT,
    WARNING
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Regulation{
    REGULATED,
    UNKNOWN,
    UNREGULATED,

}

pub static EXTERNAL_TO_INTERNAL_REGULATION: phf::Map<&'static str, Regulation> = phf_map! {
    "regulert m/magasinregulering" => Regulation::REGULATED,
    "Regulert m/magasinregulering og overføringer" => Regulation::REGULATED,
    "regulert m/magasin og uregulert restfelt" => Regulation::REGULATED,
    "" => Regulation::UNKNOWN,
    "Uregulert" => Regulation::UNREGULATED,

};