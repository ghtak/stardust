#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingFileConfig {
    pub directory: String,
    pub filename: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    pub filter: String,
    pub file: Option<LoggingFileConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
}

impl Config {
    pub fn from_file(path: &str) -> crate::Result<Self> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(
                config::Environment::with_prefix("APPCONFIG").separator("_"),
            )
            .build()
            .map_err(|e| crate::Error::LoadError(e.into()))?
            .try_deserialize::<Config>()
            .map_err(|e| crate::Error::ParseError(e.into()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file(
            format!(
                "{}/assets/config.example.toml",
                crate::utils::manifest_dir().unwrap()
            )
            .as_str(),
        )
        .unwrap();
        println!("{:?}", config);
    }
}
