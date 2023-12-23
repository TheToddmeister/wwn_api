use chrono::{DateTime, Utc};

pub async fn get_first_moment_of_year(year: i32) -> Option<DateTime<Utc>> {
    let new_years_second = chrono::NaiveDate::from_yo_opt(year, 1)?.and_hms_opt(00, 00, 00)?.and_utc();
    Some(new_years_second)
}