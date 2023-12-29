use futures::future::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb;
use surrealdb::dbs::Session;
use surrealdb::engine::any::Any;
use surrealdb::engine::any::connect;
use surrealdb::engine::remote::ws::{Client, Ws, Wss};
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::sync::OnceCell;
use warp::hyper::client::connect::Connect;
use crate::dev;
use crate::dev::DevProfiles::{AutoTest, DevTest, Prod, Dev, StaticTest};

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
/*
Connect to db routes to different surreal db implementations based on the current environment.
AutoTest is connects to an embedded closed/inaccessible(harder to access) environment intended for github actions and other testrunners.
The db environment does not persist after the test has been run.
DevTest connects to an external in memory local db session and is intended for active development.
The db enironment is persisted beyond the test and must be manually reset between tests.
StaticTest is a static in memory db intended for read testing
Dev is a local persisted database intended to simulate a production setting.
Prod is prod.
 */
pub async fn connect_to_db()->Surreal<Any>{
    let db_profile = dev::DevConfig::read_dev_profile().await;
    let db = match db_profile {
        AutoTest=> {connect_to_automatic_testing_in_memory_embedded_db().await?},
        DevTest=> {connect_to_local_dev_db().await},
        StaticTest=>{todo!()},
        Prod=> {todo!()},
        Dev=> {todo!()},
    };
}

pub async fn connect_to_local_dev_db() -> surrealdb::Result<Surreal<Any>> {
    let db = connect("ws://127.0.0.1:8000").await?;
    db.signin(Root {
        username: "test",
        password: "test",
    }).await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}

pub async fn connect_to_automatic_testing_in_memory_embedded_db() -> surrealdb::Result<Surreal<Any>>{
    let db = connect("mem://").await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}
#[cfg(test)]
mod integration_testing {
    use super::*;
    use serde::{Deserialize, Serialize};
    use surrealdb::engine::local::{Db, Mem};
    use surrealdb::sql::Thing;
    use surrealdb::Surreal;

    #[derive(Debug, Deserialize)]
    struct Record {
        #[allow(dead_code)]
        id: Thing,
    }
    #[derive(Debug, Serialize, Deserialize)]
    struct TestRecord{
        something: String,
    }

    #[tokio::test]
    async fn test_embedded_in_memory_db(){
        let db = connect_to_automatic_testing_in_memory_embedded_db().await.unwrap();
        let a:Option<TestRecord> = db.create(("Nothing", "0")).content(TestRecord{something:"nothing".to_string()}).await.unwrap();
        let b = a.unwrap();
        assert_eq!(b.something, "nothing");
        let db2 = connect_to_automatic_testing_in_memory_embedded_db().await.unwrap();
        let empty: Vec<TestRecord> = db2.select("Nothing").await.unwrap();
        assert!(empty.is_empty())
    }
}