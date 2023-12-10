use crate::persistence::{db, db_structure};
use crate::api::internal::{station, observation};
use crate::util::geo::{location, position};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{Connection, Surreal};

struct Content{
    station: station::Station,
    river: station::River,
    location: location::Location,
    position: position::Position,
}

