use std::sync::OnceLock;

use crate::config::LoggingConfig;
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::fmt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

static LOGGING_INIT: OnceLock<Vec<WorkerGuard>> = OnceLock::new();

pub fn init(logging_config: &LoggingConfig) {
    LOGGING_INIT.get_or_init(|| {
        let filter =
            EnvFilter::try_from_default_env() // RUST_LOG 환경 변수 확인
                .unwrap_or_else(|_| {
                    EnvFilter::new(logging_config.filter.as_str())
                }); // 없으면 설정 파일 값 사용
        if let Some(file_config) = &logging_config.file {
            let (nb, wg) = tracing_appender::non_blocking(daily(
                file_config.directory.as_str(),
                file_config.filename.as_str(),
            ));
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer())
                .with(fmt::layer().with_writer(nb).json())
                .init();
            vec![wg]
        } else {
            tracing_subscriber::registry()
                .with(filter)
                .with(fmt::layer().json())
                .init();
            vec![]
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn inner_span() {
        tracing::info!("inside inner span");
        tracing::error!("inside inner span");
    }

    fn default_config() -> LoggingConfig {
        LoggingConfig {
            filter: "debug".into(),
            file: None,
        }
    }

    #[tokio::test]
    async fn test_logging() {
        init(&default_config());
        tracing::debug!("debug");
        tracing::info!("info");
        tracing::warn!("warn");
        tracing::error!("error");

        let span = tracing::info_span!("my_span", foo = 3);
        let _enter = span.enter();
        inner_span().await;
        tracing::info!("after inner span");
    }

    #[tokio::test]
    async fn test_nested_span() {
        init(&default_config());

        let span = tracing::info_span!("my_span", foo = 3);
        let _enter = span.enter();
        tracing::debug!("my_span");

        let child_span = tracing::info_span!("my_span", foo = 4);
        let _enter2 = child_span.enter();
        tracing::debug!("child_span");
        drop(_enter2);
        tracing::debug!("my_span");
    }
}
