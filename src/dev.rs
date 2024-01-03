use mockall::*;
use mockall::predicate::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{engine, Surreal};
use tokio;
use tracing::log::info;


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
#[cfg(test)]
pub async fn _read_file(path: &str) -> Result<String, tokio::io::Error> {
    let json: String = tokio::fs::read_to_string(path).await?;
    Ok(json)
}
#[cfg(test)]
pub async fn _build_static_database(db: Surreal<engine::any::Any>){
    db.import("dev/sql/db_copies/newest.surql").await.unwrap();
}
#[cfg(test)]
mod tests{
    use serde::Deserialize;

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