use futures::StreamExt;
use itertools::Itertools;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use warp;
use warp::Filter;
use wwn_api::data::internal::parameter;
use wwn_api::data::internal::parameter::Resolution::{Nve, UkGov};
use wwn_api::data::internal::parameter::StationParameter;
use wwn_api::data::nve;
use wwn_api::data::nve::requests::PostToNve;
use wwn_api::static_metadata::{get_minimum_historic_data_date};

use wwn_api::{persistence, static_metadata};
use wwn_api::persistence::connection::DbConnectionConfig;
use wwn_api::persistence::init_static_data_db;
use wwn_api::static_metadata::Origin;
use wwn_api::static_controller::static_controller;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //todo() Evaluate if use Surrealdb strict mode
    //surreal.exe start memory --log debug -A --auth --user test --pass test --bind 0.0.0.0:8484
    let db_config = DbConnectionConfig {
        default_namespace: "dev",
        default_db: "dev",
        port: "8484",
        address: "127.0.0.1",
    };
    controller(db_config).await?;
    Ok(())
}

pub async fn controller(db_config: DbConnectionConfig) -> surrealdb::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let db = persistence::connection::connect_db(db_config).await?;
    static_controller(&db).await.expect("Failed to initiate static database");
    warp_controller().await;
    Ok(())
}


async fn warp_controller() {
    let get_credential_route = warp::path("login")
        .and(warp::header("user"))
        .and(warp::header("password"))
        .map(|u: String, p: String| "you have been verified with ".to_string() + &*u + &*p);

    warp::serve(get_credential_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}