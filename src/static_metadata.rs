use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nation{
    Norway,
    Sweden,
    Uk,
}

pub enum Datatype {
    River,
    Station,
    Location,
    User
}
pub enum Origin {
    NVE,
    SMIH,
    UKGOV,
}
pub enum LocationType{
    PARKING,
    PUT_IN,
    TAKE_OUT,
    CAMP,
    SPOT,
}