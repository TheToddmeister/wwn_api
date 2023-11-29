use chrono::{DateTime, Utc};
use serde::{self, Serialize, Deserialize};
use serde_json::{Value};
use crate::api::serde::deserializor::{convert_slash_string_to_list,convert_active_to_bool, convert_zulutime_string_to_UTC};


pub static URL: &str = "https://hydapi.nve.no/api/v1/";
pub static PARAMETERS: &str = "0, 1000, 1001, 1002, 1003, 1004, 17, 2002";
pub static TIME_LIM: &str = "2010-09-15T15:00:00Z";

pub  fn _read_and_deserialize_test_station() -> Root {
    let path: &str = "src/test/json/stations.json";
    let json: String = std::fs::read_to_string(path).unwrap();
    let root = deserialize_stations(&json).unwrap();
    return root;
}

pub  fn deserialize_stations(json: &str)->Result<Root, serde_json::Error> {
    let root = serde_json::from_str::<Root>(json)?;
    Ok(root.clone())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaugeData {
    id: String,
    name: String,
    riverName: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, )]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current_link :Option<String>,
    pub api_version :String,
    pub license :String,
    pub created_at :String,
    pub query_time :String,
    pub item_count: i64,
    pub data :  Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub station_id :String,
    pub station_name :String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "utmEast_Z33")]
    pub utm_east_z33: i64,
    #[serde(rename = "utmNorth_Z33")]
    pub utm_north_z33: i64,
    pub masl: i64,
    pub river_name :String,
    pub council_number :String,
    pub council_name :String,
    pub county_name :String,
    pub drainage_basin_key: i64,
    #[serde(deserialize_with="convert_slash_string_to_list")]
    pub hierarchy :Vec<String>,
    pub lake_area: f64,
    pub lake_name :String,
    pub lake_no: i64,
    pub regine_no :String,
    pub reservoir_no :String,
    pub reservoir_name :String,
    pub station_type_name :String,
    #[serde(deserialize_with="convert_active_to_bool")]
    pub station_status_name: bool,
    pub drainage_basin_area: Option<f64>,
    pub drainage_basin_area_norway: Option<f64>,
    pub gradient1085: Option<f64>,
    pub gradient_basin: Option<f64>,
    pub gradient_river: Option<f64>,
    pub height_minimum: Option<i64>,
    pub height_hypso10: Option<i64>,
    pub height_hypso20: Option<i64>,
    pub height_hypso30: Option<i64>,
    pub height_hypso40: Option<i64>,
    pub height_hypso50: Option<i64>,
    pub height_hypso60: Option<i64>,
    pub height_hypso70: Option<i64>,
    pub height_hypso80: Option<i64>,
    pub height_hypso90: Option<i64>,
    pub height_maximum: Option<i64>,
    pub length_km_basin: Option<f64>,
    pub length_km_river: Option<f64>,
    pub percent_agricul: Option<f64>,
    pub percent_bog: Option<f64>,
    pub percent_eff_bog: Option<f64>,
    pub percent_eff_lake: Option<f64>,
    pub percent_forest: Option<f64>,
    pub percent_glacier: Option<f64>,
    pub percent_lake: Option<f64>,
    pub percent_mountain: Option<f64>,
    pub percent_urban: Option<f64>,
    pub utm_zone_gravi: Option<i64>,
    pub utm_east_gravi: Option<i64>,
    pub utm_north_gravi: Option<i64>,
    pub utm_zone_inlet: Option<i64>,
    pub utm_east_inlet: Option<i64>,
    pub utm_north_inlet: Option<i64>,
    pub utm_zone_outlet: Value,
    pub utm_east_outlet: Value,
    pub utm_north_outlet: Value,
    pub annual_runoff: Option<f64>,
    pub specific_discharge: Option<f64>,
    pub regulation_area: Option<f64>,
    pub area_reservoirs: Option<f64>,
    pub volume_reservoirs: Option<f64>,
    pub regulation_part_reservoirs: Option<f64>,
    pub transfer_area_in: Option<f64>,
    pub transfer_area_out: Option<f64>,
    pub reservoir_area_in: Option<f64>,
    pub reservoir_area_out: Option<f64>,
    pub reservoir_volume_in: Option<f64>,
    pub reservoir_volume_out: Option<f64>,
    pub remaining_area: Option<f64>,
    pub number_reservoirs: Option<i64>,
    pub first_year_regulation: Option<i64>,
    pub catchment_reg_type_name :String,
    pub owner :String,
    pub q_number_of_years: Option<i64>,
    pub q_start_year: Option<i64>,
    pub q_end_year: Option<i64>,
    pub qm: Option<f64>,
    pub q5: Option<f64>,
    pub q10: Option<f64>,
    pub q20: Option<f64>,
    pub q50: Option<f64>,
    pub hm: Option<f64>,
    pub h5: Option<f64>,
    pub h10: Option<f64>,
    pub h20: Option<f64>,
    pub h50: Option<f64>,
    pub cul_qm: Option<f64>,
    pub cul_q5: Option<f64>,
    pub cul_q10: Option<f64>,
    pub cul_q20: Option<f64>,
    pub cul_q50: Option<f64>,
    pub cul_hm: Option<f64>,
    pub cul_h5: Option<f64>,
    pub cul_h10: Option<f64>,
    pub cul_h20: Option<f64>,
    pub cul_h50: Option<f64>,
    pub series_list :  Vec<SeriesList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesList {
    pub parameter_name :String,
    pub parameter: i64,
    pub version_no: i64,
    pub unit :String,

    #[serde(deserialize_with="convert_zulutime_string_to_UTC")]
    pub serie_from : Option<DateTime<Utc>>,
    #[serde(deserialize_with="convert_zulutime_string_to_UTC")]
    pub serie_to :  Option<DateTime<Utc>>,
    pub resolution_list : Vec<ResolutionList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolutionList {
    pub res_time: i64,
    pub method: String,
    pub time_offset: i64,
    pub data_from_time: DateTime<Utc>,
    pub data_to_time: DateTime<Utc>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::{self, Serialize, Deserialize};
    use tokio;

    fn read_test_station(filename: &str)->String{
        let path: String = format!("dev/json/nve/{filename}.json");
        let json: String = std::fs::read_to_string(path).unwrap();
        return json;
    }

    #[tokio::test]
    async fn test_station_deserializsation(){
        let json = read_test_station("singleStation");
        let root = deserialize_stations(&json).unwrap();
        let daum = &root.data[0];
        let hierarchy = &daum.hierarchy;
        assert_eq!(hierarchy, &["Röjdan-Løvhaugsåa".to_string(), "Norsälven".to_string()].to_vec());
        assert_eq!(daum.station_status_name, true);
    }
}
