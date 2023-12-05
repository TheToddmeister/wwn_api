use serde::{Deserialize, Serialize};

pub struct ParameterMapper {
    flow: Option<&'static str>,
    waterlevel: Option<&'static str>,
    temperature:  Option<&'static str>,
    rainfall:  Option<&'static str>,
}
const UKGOV: &'static ParameterMapper = &ParameterMapper {
    flow: Some("waterFlow"),
    waterlevel: Some("waterLevel"),
    temperature: Some("temperature"),
    rainfall: Some("rainfall"),
};

const SMIH: &'static ParameterMapper = &ParameterMapper {
    flow: Some("Vattenföring (15 min)"),
    waterlevel: Some("Vattenstånd"),
    temperature: Some("Vattendragstemperatur"),
    rainfall: None,
};

const NVE: &'static ParameterMapper = &ParameterMapper {
    flow: Some("1001"),
    waterlevel: Some("1000"),
    temperature: Some("1003"),
    rainfall: Some("0"),
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
    PARKING,
    PUT_IN,
    TAKE_OUT,
    CAMP,
    SPOT,
}