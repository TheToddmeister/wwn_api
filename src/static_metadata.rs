use phf::{phf_map, PhfHash};
use serde::{Deserialize, Serialize};

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

pub const SMIH: &'static ParameterMapper = &ParameterMapper {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}