use crate::config::{Format, LogLevel, LoggerSettings};
use tracing_subscriber::{
    fmt::{self, MakeWriter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer, Registry,
};

const MODULE_WHITELIST: &[&str] = &["tower_http", "sqlx::query", "blog_rs"];

pub fn init(logger_settings: &LoggerSettings) {
    let mut layers: Vec<Box<dyn Layer<Registry> + Sync + Send>> = Vec::new();
    let stdout_layer = init_layer(std::io::stdout, &logger_settings.format, true);
    layers.push(stdout_layer);
    let env_filter = init_env_filter(&logger_settings.level);
    tracing_subscriber::registry()
        .with(layers)
        .with(env_filter)
        .init();
}

fn init_env_filter(level: &LogLevel) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .or_else(|_| {
            EnvFilter::try_new(
                MODULE_WHITELIST
                    .iter()
                    .map(|m| format!("{m}={level}"))
                    .chain(std::iter::once(format!("{}={}", "blog_rs", level)))
                    .collect::<Vec<_>>()
                    .join(","),
            )
        })
        .expect("logger initialization failed")
}

fn init_layer<W2>(
    make_writer: W2,
    format: &Format,
    ansi: bool,
) -> Box<dyn Layer<Registry> + Sync + Send>
where
    W2: for<'writer> MakeWriter<'writer> + Sync + Send + 'static,
{
    match format {
        Format::Compact => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .compact()
            .boxed(),
        Format::Pretty => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .pretty()
            .boxed(),
        Format::Json => fmt::Layer::default()
            .with_ansi(ansi)
            .with_writer(make_writer)
            .json()
            .boxed(),
    }
}
