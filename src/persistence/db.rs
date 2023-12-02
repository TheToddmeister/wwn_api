use envy::Error;
use futures::future::Lazy;
use surrealdb;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

use crate::api;

pub struct Db {
    rivers: River,
}

pub struct Content {
}

pub struct River {
    nve: Content,
    smih: Content,
}
