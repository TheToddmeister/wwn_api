use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Display)]
pub enum Tables{
    StaticRiver,
    StaticStation,
}

