use std::path::Path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HttpConfig {
    pub static_dir: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub http: Option<HttpConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LoggingFormat {
    Json,
    Pretty,
    Full,
    Compact,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingFileConfig {
    pub format: LoggingFormat,
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    pub format: LoggingFormat,
    pub filter: String,
    pub file: Option<LoggingFileConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_file(path: &str) -> crate::Result<Self> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::with_prefix("APPCONFIG").separator("_"))
            .build()
            .map_err(|e| crate::Error::LoadError(e.into()))?
            .try_deserialize::<Config>()
            .map_err(|e| crate::Error::ParseError(e.into()))
    }

    pub fn test_config() -> Self {
        let path = Path::new(&crate::utils::manifest_dir().unwrap())
            .join("..")
            .join("testenv")
            .join("config.test.toml");
        let mut config = Config::from_file(path.to_str().unwrap()).unwrap();
        config.logging.file = None;
        config
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config() {
        let config = Config::test_config();
        println!("{:?}", config);
    }
}
