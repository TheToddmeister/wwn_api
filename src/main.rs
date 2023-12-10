use futures::future::Lazy;
use surrealdb::{Connect, sql};
use wwn_api::persistence::{};
use wwn_api::api::internal::river::River;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;


#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // Connect to the server
    let db: Surreal<Client> = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    let created: Vec<Record> = db
        .create("River")
        .content(River {
            name: "Sjoa".to_string(),
            alias: vec!["Sjoa".to_string()],
            drainage_basin: None,
            tributary_hierarchy: vec!["Sjoa".to_string()],
            catchment_area: None,
            }).await?;
    dbg!(created);
    Ok(())
}