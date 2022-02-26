use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: Server,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: String,
    pub port: i32,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub username: String,
    pub password: String,
}
