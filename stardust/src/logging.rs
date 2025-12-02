use std::sync::OnceLock;
use std::vec;

use crate::config::{LoggingConfig, LoggingFormat};
use tracing::Subscriber;
use tracing_appender::{non_blocking::WorkerGuard, rolling::daily};
use tracing_subscriber::fmt;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt};

static LOGGING_INIT: OnceLock<Vec<WorkerGuard>> = OnceLock::new();

fn init_layer<S>(
    layred: S,
    format: &LoggingFormat,
    writer: tracing_appender::non_blocking::NonBlocking,
) where
    S: Subscriber + for<'a> LookupSpan<'a> + Sync + Send + 'static,
{
    match format {
        LoggingFormat::Full => {
            layred.with(fmt::layer().with_writer(writer)).init()
        }
        LoggingFormat::Compact => {
            layred.with(fmt::layer().with_writer(writer).compact()).init()
        }
        LoggingFormat::Pretty => {
            layred.with(fmt::layer().with_writer(writer).pretty()).init()
        }
        LoggingFormat::Json => {
            layred.with(fmt::layer().with_writer(writer).json()).init()
        }
    }
}

fn init_layers<S>(
    layered: S,
    format: &LoggingFormat,
    writer: tracing_appender::non_blocking::NonBlocking,
    format1: &LoggingFormat,
    writer1: tracing_appender::non_blocking::NonBlocking,
) where
    S: Subscriber + for<'a> LookupSpan<'a> + Sync + Send + 'static,
{
    match format {
        LoggingFormat::Full => init_layer(
            layered.with(fmt::layer().with_writer(writer)),
            format1,
            writer1,
        ),
        LoggingFormat::Compact => init_layer(
            layered.with(fmt::layer().with_writer(writer).compact()),
            format1,
            writer1,
        ),
        LoggingFormat::Pretty => init_layer(
            layered.with(fmt::layer().with_writer(writer).pretty()),
            format1,
            writer1,
        ),
        LoggingFormat::Json => init_layer(
            layered.with(fmt::layer().with_writer(writer).json()),
            format1,
            writer1,
        ),
    }
}

pub fn init(logging_config: &LoggingConfig) {
    LOGGING_INIT.get_or_init(|| {
        let layered = tracing_subscriber::registry().with(
            // RUST_LOG 환경 변수 확인
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(logging_config.filter.as_str())),
        );
        let (console, console_guard) =
            tracing_appender::non_blocking::NonBlockingBuilder::default()
                .buffered_lines_limit(256_000)
                .lossy(true) // drop if buffer full
                .finish(std::io::stdout());

        match logging_config.file {
            None => {
                init_layer(layered, &logging_config.format, console);
                vec![console_guard]
            }
            Some(ref file_config) => {
                let (file, file_guard) =
                    tracing_appender::non_blocking::NonBlockingBuilder::default()
                        .buffered_lines_limit(256_000)
                        .lossy(true) // drop if buffer full
                        .finish(daily(
                            file_config.directory.as_str(),
                            file_config.filename.as_str(),
                        ));

                init_layers(
                    layered,
                    &logging_config.format,
                    console,
                    &file_config.format,
                    file,
                );
                vec![console_guard, file_guard]
            }
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
            format: crate::config::LoggingFormat::Json,
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
