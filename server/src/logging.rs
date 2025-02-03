use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn init() -> WorkerGuard {
    let appender = tracing_appender::rolling::daily(Path::new("./.logs/"), "debug.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::new(
                    std::env::var("RUST_LOG").unwrap_or_else(|_| "trace".into()),
                ))
                .with_filter(LevelFilter::TRACE),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_appender)
                .with_ansi(false)
                .with_filter(EnvFilter::new("trace")),
        )
        .init();
    guard
}
