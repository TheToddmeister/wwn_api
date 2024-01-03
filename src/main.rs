use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use warp;
use warp::Filter;

use wwn_api::data::internal;
use wwn_api::db::config;
use wwn_api::db::queries::{generics::MinimalStation, stations::StationFilter};
use wwn_api::static_controller::static_controller;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //todo() Evaluate if use Surrealdb strict mode
    //surreal.exe start memory --log debug -A --auth --user test --pass test --bind 0.0.0.0:8484
    controller().await?;
    Ok(())
}

pub async fn controller() -> surrealdb::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let db = config::connection::connect_to_db().await?;
    static_controller(&db).await.expect("Failed to initiate static database");
    warp_controller(&db).await;
    Ok(())
}
/// todo! Handle SQL injection attacks using $ -> treat it as a string
/// https://discord.com/channels/902568124350599239/1191781040579162223/1192035844010291260
/// todo! Export internal struct definitions and internal struct methods to independent crate???
async fn warp_controller(db: &Surreal<Any>) {
    let get_credential_route = warp::path("login")
        .and(warp::header("user"))
        .and(warp::header("password"))
        .map(|u: String, p: String| "you have been verified with ".to_string() + &*u + &*p);

    warp::serve(get_credential_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

    let get_minimal_stations_route = warp::path("minStations")
        .map(|| MinimalStation::fetch_minimal_stations_from_db(db));
    warp::serve(get_minimal_stations_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

    let get_stations_route = warp::path("stations")
        .and(warp::body::json())
        .map(|station_filter: StationFilter| internal::station::Station::(&db, ));
    warp::serve(get_stations_route)
        .run(([127, 0, 0, 1], 3030))
        .await;

}