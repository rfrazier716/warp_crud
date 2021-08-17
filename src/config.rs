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

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    const TEST_CONFIG: &str = r#"
server:
    application_port: 3030
    address: 127.0.0.1
database:
    uri: mongodb://root:example@localhost:27017
log:
    - info
"#;

    #[test]
    fn test_loading_config_string() {
        let mut s = Config::new();
        s.merge(File::from_str(TEST_CONFIG, config::FileFormat::Yaml))
            .unwrap();
        s.set("configuration", "development").unwrap(); // have to do this since it will be populated by default for the test
        let config = s.try_into::<Settings>().unwrap(); //panic if we cannot convert it
        assert_eq!(
            config.database.uri,
            "mongodb://root:example@localhost:27017"
        );
        assert_eq!(config.server.address, "127.0.0.1");
        assert_eq!(config.configuration, "development");
    }

    #[test]
    fn test_overwriting_nested_values() {
        // set the environment variable for the database username
        env::set_var("EA_DATABASE__URI", "changed");
        let mut s = Config::new();
        s.merge(File::from_str(TEST_CONFIG, config::FileFormat::Yaml))
            .unwrap();
        s.merge(Env::with_prefix("ea").separator("__")).unwrap();
        let config: Settings = s.try_into().unwrap();
        assert_eq!(config.database.uri, "changed");
    }
}
