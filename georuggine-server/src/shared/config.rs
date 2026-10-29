use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub db_url: String,
    pub log_file: String,
}

impl Config {
    pub fn init() -> Result<Self, envy::Error> {
        dotenvy::dotenv().ok();
        envy::from_env::<Config>()
    }
}
