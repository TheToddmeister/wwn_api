use futures::future::Lazy;
use surrealdb::{Surreal};
use surrealdb::sql;
use surrealdb::opt::auth::Root;
use surrealdb::engine::remote::ws::Ws;
#[tokio::main]
async fn main() -> surrealdb::Result<()>{
    // Connect to the server
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    db.use_ns("test").use_db("test").await?;
    let db_core =
    // Select a specific namespace / database
    Ok(())
}