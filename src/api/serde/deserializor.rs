use chrono::{DateTime, Utc};
use serde_json;
use serde::{de::Error, Deserialize, Deserializer}; // 1.0.94use serde::de::{

pub  fn convert_slash_string_to_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer).unwrap();
        return Ok(s.split("/").map(|a| a.to_lowercase().to_string()).collect::<Vec<String>>())
}
pub  fn convert_zulutime_string_to_UTC<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &Option<&str> = &Deserialize::deserialize(deserializer)?;
    if s.is_some_and(|a| a.len()>8){
        return Ok(None)
    }
    match s {
        Some(val) => {
            let datetime = chrono::DateTime::parse_from_rfc3339(val).unwrap();
            let utc = datetime.with_timezone(&Utc);
             Ok(Some(utc))}
        None=>Ok(None)
    }
}

pub   fn convert_aktiv_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer).unwrap();
    if s.contains("Aktiv") {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub  fn convert_active_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer).unwrap();
    if s.contains("Active") {
        Ok(true)
    } else {
        Ok(false)
    }
}
