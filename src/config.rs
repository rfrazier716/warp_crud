use crate::error::{Error, Result};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_PATH: &str = "./config/Default.yml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Clone, Debug, Deserialize)]
pub enum Env {
    Default,
    Test,
    Development,
    Production,
}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Env::Default => write!(f, "Default"),
            Env::Test => write!(f, "Test"),
            Env::Development => write!(f, "Development"),
            Env::Production => write!(f, "Production"),
        }
    }
}

impl std::str::FromStr for Env {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Default" => Ok(Env::Default),
            "Test" => Ok(Env::Test),
            "Development" => Ok(Env::Development),
            "Production" => Ok(Env::Production),
            _ => Err(Error::ServerConfigError(String::from(s))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerSettings {
    pub rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    pub application_port: u16,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub environment: Env,
    pub database: DatabaseSettings,
    pub log: Vec<String>,
    pub server: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self> {
        // Figure out what config to load based on environment Variables
        // Use Development by Default
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| String::from("Default"));
        Config::builder()
            .add_source(File::with_name(DEFAULT_CONFIG_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .add_source(Environment::with_prefix("ea").separator("__"))
            .set_override("environment", env)
            .map_err(|source| Error::ConfigurationError { source })?
            .build()
            .map_err(|source| Error::ConfigurationError { source })?
            .try_deserialize()
            .map_err(|source| Error::ConfigurationError { source })
    }
}
