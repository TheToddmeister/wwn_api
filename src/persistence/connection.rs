use futures::future::Lazy;
use serde::{Deserialize, Serialize};
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

pub struct DbConnectionConfig {
   pub address: &'static str,
   pub port: &'static str,
   pub default_namespace: &'static str,
   pub default_db: &'static str,
}
#[derive(Serialize, Deserialize)]
pub struct Login{
    DBPASSWORD: String,
    DBUSER: String,
}


pub async fn connect_db(db_parameters: DbConnectionConfig) -> surrealdb::Result<Surreal<Client>> {
    let address = db_parameters.address;
    let port = db_parameters.port;
    let namespace = db_parameters.default_namespace;
    let db_name = db_parameters.default_db;
    let db = Surreal::new::<Ws>(format!("{address}:{port}")).await?;
    // Signin as a namespace, database, or root user
    let root = envy::from_env::<Login>();
    // todo() Read from env
    db.signin(Root {
        username: "test",
        password: "test",
    }).await?;

    // Select a specific namespace / database
    db.use_ns(namespace).use_db(db_name).await?;
    Ok(db)
}
