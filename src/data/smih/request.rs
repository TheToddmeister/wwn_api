use futures;
use reqwest;
use serde::Deserialize;

use crate::data::smih::station_and_recent_observations::Root;
use crate::static_metadata::ParameterDefinitions;

#[cfg(test)]
mod SMIHtest {
}
