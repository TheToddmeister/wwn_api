pub mod request;
pub mod period;
pub mod station_and_recent_observations;
pub mod observation_data;

//flow24*60 - flow15 - cm - C
pub const SWEDEN_PARAMETER_IDS:[i64; 4] = [1, 2, 3, 4];
