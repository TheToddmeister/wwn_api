use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use phf::{phf_map, PhfHash};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tokio::sync::OnceCell;

//These are not intended to stay permanently todo!
pub const static_stations_nve: [&str; 11] = ["2.13.0", "2.39.0", "1.15.0", "2.595.0", "2.661.0", "109.20.0", "109.20.0", "12.209.0", "122.14.0", "7.29.0", "7.30.0"];
pub const static_stations_ukgov: [&str; 5] = ["fcb795f4-07bc-4ae0-8372-041644e7f275", "b9933a62-f326-4d77-9206-ebb335161831", "9ad5d28c-7cfe-46db-b39d-58701689cd59", "17f5b1f6-9779-4cbe-a0a3-978d3326bc33", "24066dfc-316c-4093-b236-57b9d675150f"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterDefinitions {
    FLOW,
    WATERLEVEL,
    TEMPERATURE,
    RAINFALL,
}

impl ParameterDefinitions {
    pub fn from_nve(s: i64) -> Option<Self> {
        match s {
            1001 => Some(Self::FLOW),
            1000 => Some(Self::WATERLEVEL),
            1003 => Some(Self::TEMPERATURE),
            0 => Some(Self::RAINFALL),
            _ => None,
        }
    }
    pub fn to_nve(&self) -> &'static str {
        match self {
            Self::FLOW => "1001",
            Self::WATERLEVEL => "1000",
            Self::TEMPERATURE => "1003",
            Self::RAINFALL => "0",
        }
    }
    pub fn from_uk(s: &str) -> Option<Self> {
        match s {
            "waterFlow" => Some(Self::FLOW),
            "waterLevel" => Some(Self::WATERLEVEL),
            "temperature" => Some(Self::TEMPERATURE),
            "rainFall" => Some(Self::RAINFALL),
            _ => None,
        }
    }
    pub fn to_uk(&self) -> &'static str {
        match self {
            Self::FLOW => "waterLevel",
            Self::WATERLEVEL => "waterLevel",
            Self::TEMPERATURE => "temperature",
            Self::RAINFALL => "rainfall",
        }
    }
    pub fn from_smih(s: &str) -> Option<Self> {
        match s {
            "Vattenföring (15 min)" => Some(Self::FLOW),
            "Vattenstånd" => Some(Self::WATERLEVEL),
            "Vattendragstemperatur" => Some(Self::TEMPERATURE),
            _ => None,
        }
    }
    pub fn to_smih(self) -> Option<&'static str> {
       let s= match self {
            Self::FLOW => "Vattenföring (15 min)",
            Self::WATERLEVEL => "Vattenstånd",
            Self::TEMPERATURE => "Vattendragstemperatur",
            Self::RAINFALL => return None,
        };
        Some(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Nation {
    Norway,
    Sweden,
    Uk,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Datatype {
    River,
    Station,
    Location,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Origin {
    NVE,
    SMIH,
    UKGOV,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    STATION,
    PARKING,
    PUT_IN,
    TAKE_OUT,
    CAMP,
    SPOT,
    WARNING,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Regulation {
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

static MINIMUM_HISTORIC_DATA_DATE: OnceCell<DateTime<Utc>> = OnceCell::const_new();
pub async fn get_minimum_historic_data_date()-> &'static DateTime<Utc>{
    MINIMUM_HISTORIC_DATA_DATE.get_or_init(|| async {
        NaiveDate::from_yo_opt(2020, 1).expect("Failed to create minimum datetime").and_time(NaiveTime::MIN).and_utc()
    }).await
}