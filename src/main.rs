use futures::StreamExt;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use tracing_subscriber::filter::FilterExt;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use warp;
use warp::Filter;

use wwn_api::persistence;
use wwn_api::persistence::init_static_data_db;
use wwn_api::util::geo::location::Location;

#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // Connect to the server
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let db = persistence::connection::connect_db().await?;
    static_controller(&db).await?;
    warp_controller().await;
    Ok(())
}
async fn static_controller(db: &Surreal<Client>)->surrealdb::Result<()>{
    init_static_data_db::build_static_station_info_table(db).await.unwrap();
    let mut stations = db.query("select * from Locations where").await?;
    
    Ok(())
}

async fn station_parameter_historical_data(db: &Surreal<Client>)->surrealdb::Result<()>{
    
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