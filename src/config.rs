use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub postgres: Postgres,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postgres {
    pub url: String,
    pub ssl: Option<SslConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    pub cert_path: String,
    pub ca_path: String,
    pub key_path: String,
}

pub fn cfg() -> Config {
    let cfg_path = if cfg!(features = "production") { "config/prod.toml" } else { "config/dev.toml" };
    let file_content = std::fs::read_to_string(std::path::PathBuf::from(cfg_path))
        .expect(format!("could not access {}", cfg_path).as_str());
    return toml::from_str(file_content.as_str())
        .expect("Could not deserialize from file content");
}