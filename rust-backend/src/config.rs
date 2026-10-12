//! Configuration management for Palet Vendéen Backend

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

/// Application configuration
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection URL
    pub database_url: String,
    /// Maximum database connections
    pub db_max_connections: u32,
    /// Database connection timeout in seconds
    pub db_timeout_secs: u64,
    /// CORS max age in seconds
    pub cors_max_age: u64,
    /// Environment (development, staging, production)
    pub environment: String,
    /// Log level
    pub log_level: String,
    /// JWT secret for authentication
    pub jwt_secret: String,
    /// JWT expiration in hours
    pub jwt_expiration_hours: u64,
}

impl AppConfig {
    /// Create a new AppConfig from config::Settings
    pub fn from_settings(settings: Config) -> Result<Self, ConfigError> {
        Ok(Self {
            host: settings.get_string("host")?.unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: settings.get_int("port")?.unwrap_or(3000) as u16,
            database_url: settings.get_string("database_url")?
                .unwrap_or_else(|_| "postgres://palet:palet@localhost:5432/palet".to_string()),
            db_max_connections: settings.get_int("db_max_connections")?.unwrap_or(20) as u32,
            db_timeout_secs: settings.get_int("db_timeout_secs")?.unwrap_or(30) as u64,
            cors_max_age: settings.get_int("cors_max_age")?.unwrap_or(86400) as u64,
            environment: settings.get_string("environment")?.unwrap_or_else(|_| "development".to_string()),
            log_level: settings.get_string("log_level")?.unwrap_or_else(|_| "info".to_string()),
            jwt_secret: settings.get_string("jwt_secret")?.unwrap_or_else(|_| "change-me-in-production".to_string()),
            jwt_expiration_hours: settings.get_int("jwt_expiration_hours")?.unwrap_or(24) as u64,
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            database_url: "postgres://palet:palet@localhost:5432/palet".to_string(),
            db_max_connections: 20,
            db_timeout_secs: 30,
            cors_max_age: 86400,
            environment: "development".to_string(),
            log_level: "info".to_string(),
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiration_hours: 24,
        }
    }
}

/// Load configuration from environment and files
pub fn load_config() -> Result<AppConfig, ConfigError> {
    dotenv::dotenv().ok();

    let mut builder = Config::builder()
        .add_source(File::with_name("config/default").required(false));

    // Add local config if it exists
    if std::path::Path::new("config/local.toml").exists() {
        builder = builder.add_source(File::with_name("config/local").required(false));
    }

    // Add environment variables with PALET__ prefix
    builder = builder.add_source(
        Environment::with_prefix("PALET")
            .prefix_separator("__")
            .separator("__"),
    );

    let settings = builder.build()?;
    AppConfig::from_settings(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "0.0.0.0");
        assert!(config.database_url.contains("postgres://"));
    }

    #[test]
    fn test_config_from_settings() {
        let mut settings = Config::new();
        settings.insert("host", "127.0.0.1").unwrap();
        settings.insert("port", 8080).unwrap();
        
        let config = AppConfig::from_settings(settings).unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }
}
