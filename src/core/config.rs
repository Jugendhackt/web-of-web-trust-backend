use config::{Config as MetaConfig, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct DomainQueryConfig {
    pub per_page: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
    pub user: String,
    pub domains: DomainQueryConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CachingConfig {
    pub redis_host: String,
    pub redis_port: u32,
    pub redis_user: Option<String>,
    pub redis_password: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracingConfig {
    pub meter: String,
    pub service_name: String,
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct APIConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub debug: bool,
    pub database: DatabaseConfig,
    pub api: APIConfig,
    pub tracing: TracingConfig,
    pub caching: CachingConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = MetaConfig::default();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/base"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        s.merge(
            File::with_name(&format!(
                "config/{}",
                env::var("RUN_MODE").unwrap_or_else(|_| "development".into())
            ))
            .required(true),
        )?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of BACKEND)
        s.merge(Environment::with_prefix("backend"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new().expect("Failed to load config");
    pub static ref PER_PAGE_ERROR: String = format!(
        "per_page is limited too {} for this instance",
        CONFIG.database.domains.per_page
    );
}
