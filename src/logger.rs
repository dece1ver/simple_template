use crate::config::{LogType, Settings};
use eyre::Result;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init(config: &Settings) -> Result<Option<WorkerGuard>> {
    let timer = ChronoLocal::new("%d.%m.%Y %H:%M:%S%.3f".to_string());

    let format = fmt::format()
        .pretty()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(timer.clone());

    let mut layers = Vec::new();
    let mut guard = None;

    if let LogType::FILE | LogType::BOTH = config.general.log_type {
        let file_appender = RollingFileAppender::new(Rotation::NEVER, "logs", "log");
        let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::Layer::new()
            .event_format(format.clone())
            .with_writer(non_blocking)
            .with_ansi(false)
            .boxed();
        layers.push(file_layer);
        guard = Some(file_guard);
    }

    if matches!(config.general.log_type, LogType::STDERR | LogType::BOTH) {
        let stderr_layer = fmt::Layer::new()
            .event_format(format)
            .with_writer(std::io::stderr)
            .with_span_events(fmt::format::FmtSpan::CLOSE)
            .boxed();
        layers.push(stderr_layer);
    }

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new(&config.general.log_level))
        .with(layers);

    subscriber.try_init()?;

    Ok(guard)
}
