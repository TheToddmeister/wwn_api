use serde::{Deserialize, Serialize};
use surrealdb;
use surrealdb::dbs::Session;
use surrealdb::engine::any::Any;
use surrealdb::engine::any::connect;
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use warp::hyper::client::connect::Connect;

use crate::dev;
use crate::dev::DevProfiles::{AutoTest, DevTest};

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
pub async fn connect_to_db()->surrealdb::Result<Surreal<Any>>{
    let dev_profile = &dev::DEV_PROFILE.dev_profile;
    let db = match dev_profile {
        AutoTest=> {return  Ok(connect_to_automatic_testing_in_memory_embedded_db().await?)},
        DevTest=> {connect_to_local_dev_db().await?},
    };
    Ok(db)
}
pub async fn connect_to_static_local_db_built_from_file() -> surrealdb::Result<Surreal<Any>> {
    let db = connect("ws://127.0.0.1:8080").await?;
    db.signin(Root {
        username: "test",
        password: "test",
    }).await?;
    db.use_ns("test").use_db("test").await?;
    db.import("src/dev/sql/db_copies/newest.surql").await?;
    Ok(db)
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
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;

    use super::*;

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