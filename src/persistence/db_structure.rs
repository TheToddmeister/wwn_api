pub enum ConnectionMode {
    Anonymus,
    Password
}
pub struct Config {

}

pub struct Db{
    authentication_mode: ConnectionMode,
    namespace: String,
    db_name: String,
    url: String,
}

