#[derive(Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    // pub name: String,
    pub username: String,
    pub password: String,
}