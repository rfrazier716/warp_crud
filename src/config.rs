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
        let mut settings = Config::new(); // Create a new config

        collect_configuration_files(&mut settings, &env)?;
        collect_environment_variables(&mut settings)?;

        // put the environment into the settings file
        settings
            .set("environment", env)
            .map_err(|source| Error::ConfigurationError { source })?;

        // Convert it into a settings Struct and raise an error if we could not
        settings
            .try_into()
            .map_err(|source| Error::ConfigurationError { source })
    }
}

fn collect_configuration_files<'a>(config: &'a mut Config, env: &str) -> Result<&'a mut Config> {
    // Merge Default Settings
    config
        .merge(File::with_name(DEFAULT_CONFIG_PATH))
        .map_err(|source| Error::ConfigurationError { source })?;

    //Merge the specific environment settings
    config
        .merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
        .map_err(|source| Error::ConfigurationError { source })
}

fn collect_environment_variables(config: &mut Config) -> Result<&mut Config> {
    // Get database login information from the Environment
    // These Env Variables should be EA_DATABASE__URI
    config
        .merge(Environment::with_prefix("ea").separator("__"))
        .map_err(|source| Error::ConfigurationError { source })
}
