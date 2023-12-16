use serde::Deserialize;
use surrealdb::sql::Thing;

use wwn_api::persistence::init_static_data_db;
use wwn_api::persistence;

#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // Connect to the server
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let db = persistence::connection::init_db().await?;
    init_static_data_db::build_static_station_info_table(db).await.unwrap();

    Ok(())
}