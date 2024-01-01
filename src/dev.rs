use serde::{Deserialize, Serialize};
use tokio;
use once_cell::sync::{Lazy, OnceCell};
use tracing::log::info;
use mockall::*;
use mockall::predicate::*;
use surrealdb::{engine, Surreal};

enum InitStaticDbError{
    t
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DevProfiles{
    AutoTest,
    DevTest,
   // StaticTest,
   // Dev,
   // Prod,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DevConfig{
    pub dev_profile: DevProfiles,
}
pub static DEV_PROFILE:Lazy<DevConfig> = Lazy::new(|| {
    let profile = envy::from_env::<DevConfig>().expect("Failed to read devConfig from env");
    info!("¤ DEV_PROFILE initated with profile {profile:#?}");
    profile
});

pub async fn _read_file(path: &str) -> Result<String, tokio::io::Error> {
    let json: String = tokio::fs::read_to_string(path).await?;
    Ok(json)
}

pub async fn _build_static_database(db: Surreal<engine::any::Any>){
    db.import("dev/sql/db_copies/second.surql").await.unwrap();
}
async fn _export_static_database(db: Surreal<engine::any::Any>){
    tokio::fs::remove_file("src/dev/sql/db_copies/previous.surql").await.unwrap();
    tokio::fs::rename("dev/sql/db_copies/second.surql", "src/dev/sql/db_copies/previous.surql").await.unwrap();
    db.export("src/dev/sql/db_copies/newest.surql").await.unwrap();
}

#[cfg(test)]
mod tests{
    use serde::Deserialize;
    use crate::dev::_export_static_database;
    use crate::persistence::connection::connect_to_local_dev_db;
    use crate::static_controller;

    #[tokio::test]
    #[ignore]
    async fn export_static_test_database_to_file(){
        let db = connect_to_local_dev_db().await.unwrap();
        static_controller::static_controller(&db).await.unwrap();
        _export_static_database(db).await;
    }
    #[tokio::test]
    async fn test_read_env_variables(){
        #[derive(Debug, Deserialize)]
        struct TestVariable {
            env_test_variable: String
        }
        let env_test_var = envy::from_env::<TestVariable>().unwrap();
        assert_eq!(env_test_var.env_test_variable, "Test");
    }
}