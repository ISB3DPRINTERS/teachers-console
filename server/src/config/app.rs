use std::{env, path::Path};

use serde::{Deserialize, Serialize};
use tokio::fs;

use super::env::make_env_key;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn parse_from_env(prefix: &str) -> AppConfig {
        let mut config = AppConfig::default();

        let env_host = env::var(make_env_key(prefix, "HOST"));
        let env_port = env::var(make_env_key(prefix, "PORT"));
        let database_url = env::var(make_env_key(prefix, "DATABASE_URL"));

        if let Ok(host) = env_host {
            config.host = host;
        }

        if let Ok(port) = env_port {
            config.port = port.parse::<u16>().unwrap_or(config.port);
        }

        if let Ok(database_url) = database_url {
            config.database_url = database_url;
        }

        return config;
    }

    pub fn parse_from_toml(toml_str: String) -> AppConfig {
        let config = toml::from_str(toml_str.as_str()).unwrap();

        return config;
    }

    pub fn layer(first: AppConfig, second: AppConfig) -> AppConfig {
        let mut config = first.clone();
        let default = AppConfig::default();

        if first.host == default.host {
            config.host = second.host;
        }

        if first.port == default.port {
            config.port = second.port;
        }

        if first.database_url == default.database_url {
            config.database_url = second.database_url;
        }

        return config;
    }

    pub async fn parse() -> AppConfig {
        let prefix = "TC";
        let file_name = Path::new("Console.toml");
        let toml_str = fs::read_to_string(file_name).await;

        let mut toml_parsed = None;

        if let Ok(toml_val) = toml_str {
            toml_parsed = Some(AppConfig::parse_from_toml(toml_val));
        }

        let env_parsed = AppConfig::parse_from_env(prefix);

        if toml_parsed.is_some() {
            return AppConfig::layer(toml_parsed.unwrap(), env_parsed);
        }

        return env_parsed;
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        return Self {
            host: "127.0.0.1".to_string(),
            port: 4001,
            database_url: String::from("postgres://postgres:postgres@localhost:5432/tc-dev"),
        };
    }
}

pub async fn get_config() -> AppConfig {
    return AppConfig::parse().await;
}
