use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub encryption: EncryptionConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionConfig {
    pub key_size: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgresql://nyx_user:nyx_password@localhost:5432/nyx_db".to_string(),
                max_connections: 10,
            },
            encryption: EncryptionConfig { key_size: 2048 },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let mut builder = Config::builder()
            .add_source(Config::try_from(&AppConfig::default())?)
            .add_source(
                File::with_name("config")
                    .required(false)
                    .format(config::FileFormat::Toml),
            )
            .add_source(
                Environment::with_prefix("NYX")
                    .prefix_separator("_")
                    .separator("__"),
            );

        if let Ok(config_path) = env::var("NYX_CONFIG_PATH") {
            builder = builder.add_source(
                File::with_name(&config_path)
                    .required(true)
                    .format(config::FileFormat::Toml),
            );
        }

        let config = builder.build().context("Failed to build configuration")?;

        config
            .try_deserialize()
            .context("Failed to deserialize configuration")
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub fn max_connections(&self) -> u32 {
        self.database.max_connections
    }

    pub fn key_size(&self) -> usize {
        self.encryption.key_size
    }
}
