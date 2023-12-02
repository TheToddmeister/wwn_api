use serde::{Deserialize, Serialize};

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