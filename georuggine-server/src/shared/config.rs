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

    pub fn mock() -> Self {
        Config {
            host: "127.0.0.1".to_string(),
            port: 0,
            jwt_secret: "my-very1-secret-secret!".to_string(),
            db_url: ":memory:".to_string(),
            log_file: "test_georuggine.log".to_string(),
        }
    }
}
