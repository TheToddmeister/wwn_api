use futures::future::Lazy;
use surrealdb;
use surrealdb::dbs::Session;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::sync::OnceCell;
use warp::hyper::client::connect::Connect;


pub struct Db{
    session: Session,
    datastore: Datastore,
}


pub async fn connect_db() -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "toot",
    }).await?;

    // Select a specific namespace / database
    db.use_ns("namespace").use_db("database").await?;
    Ok(db)
}
