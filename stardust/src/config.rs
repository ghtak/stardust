use std::path::Path;

macro_rules! config_model {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            $item
        )*
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LoggingFormat {
    Json,
    Pretty,
    Full,
    Compact,
}

config_model! {
    pub struct HttpConfig {
        pub static_root: String,
        pub static_dir: String,
    }

    pub struct ServerConfig {
        pub host: String,
        pub port: u16,
        pub http: Option<HttpConfig>,
    }

    pub struct LoggingFileConfig {
        pub format: LoggingFormat,
        pub directory: String,
        pub filename: String,
    }

    pub struct LoggingConfig {
        pub format: LoggingFormat,
        pub filter: String,
        pub file: Option<LoggingFileConfig>,
    }

    pub struct DatabaseConfig {
        pub url: String,
        pub pool_size: u32,
    }

    pub struct Argon2Config {
        pub memory_kib: u32,
        pub iterations: u32,
        pub parallelism: u32,
        pub algorithm: String, // argon2id | argon2i | argon2d
        pub version: u32,
        pub output_len: Option<u32>,
    }

    pub struct Config {
        pub server: ServerConfig,
        pub logging: LoggingConfig,
        pub database: DatabaseConfig,
    }
}

impl Config {
    pub fn from_file(path: &str) -> crate::Result<Self> {
        config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(
                config::Environment::with_prefix("APPCONFIG").separator("_"),
            )
            .build()
            .map_err(|e| {
                crate::Error::Unhandled(
                    anyhow::Error::new(e).context("build config error"),
                )
            })?
            .try_deserialize::<Config>()
            .map_err(|e| {
                crate::Error::Unhandled(
                    anyhow::Error::new(e).context("serialize config error"),
                )
            })
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
