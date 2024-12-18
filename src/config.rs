use config::{Config, Map};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub ctp: CtpConfig,

    #[serde(default = "default_postgres_url")]
    pub postgres_url: String,

    pub libs: LibsConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CtpConfig {
    pub broker_id: String,
    pub investor_id: String,
    pub password: String,
    pub app_id: String,
    pub auth_code: String,

    pub trader_api_endpoint: String,
    pub md_api_endpoint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibsConfig {
    pub ctp_thosttraderapi_path: String,
    pub ctp_thostmduserapi_path: String,
}

pub fn get_config() -> AppConfig {
    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("config.toml"))
        .add_source(config::Environment::with_prefix("TRTFF"))
        .build();

    if let Err(e) = settings {
        panic!("config error: {}", e);
    }

    let settings = settings.unwrap();

    let settings = settings.try_deserialize();
    if let Err(e) = settings {
        panic!("config error: {}", e);
    }

    settings.unwrap()
}

fn default_postgres_url() -> String {
    "postgresql://postgres:postgres@localhost:5432".to_string()
}
