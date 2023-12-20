use futures::StreamExt;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use warp;
use warp::Filter;

use wwn_api::persistence;
use wwn_api::persistence::init_static_data_db;

#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // Connect to the server
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db = persistence::connection::connect_db().await?;
    static_controller(&db).await.expect("Failed to initiate db");
    warp_controller().await;
    Ok(())
}

async fn static_controller(db: &Surreal<Client>)->Result<(), persistence::error::APIPersistenceError>{
    init_static_data_db::build_static_station_info_tables(db).await?;
    let mut stations = db.query("select * from Locations where").await?;
    Ok(())
}

async fn station_parameter_historical_data(db: &Surreal<Client>)->surrealdb::Result<()>{
    todo!()
}

async fn warp_controller(){
    let get_credential_route = warp::path("login")
        .and(warp::header("user"))
        .and(warp::header("password"))
        .map(|u: String, p: String| "you have been verified with ".to_string() + &*u + &*p);

    warp::serve(get_credential_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}