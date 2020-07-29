pub mod backend;
pub mod frontend;
pub mod models;
pub mod workers;

pub mod config;
pub mod search;
pub mod version;

use {
    crate::{
        backend::{Backend, DataBackend},
        config::{Config, LogLevel},
    },
    anyhow::Context,
    clap::{App, Arg},
    fenn::VecExt,
    std::{future::Future, pin::Pin, sync::Arc},
    tokio::{runtime::Builder, sync::broadcast},
    tracing::Level,
    tracing_subscriber::{
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

pub type Boxed = Pin<Box<dyn Future<Output = anyhow::Result<()>>>>;

fn main() -> anyhow::Result<()> {
    let matches = app().get_matches();

    let cfg = config::load_config(&matches).context("Failure to create config instance")?;

    let file_appender =
        tracing_appender::rolling::daily(&cfg.logging.directory, &cfg.logging.prefix);
    let (non_blocking, _appender_guard) = tracing_appender::non_blocking(file_appender);

    // TODO: Get JSON output working
    let reg = Registry::default()
        .with(
            fmt::Layer::default()
                .with_ansi(cfg.logging.ansi)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(LevelFilter::from(match cfg.logging.level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }));

    tracing::subscriber::set_global_default(reg)
        .context("Failed to set Tracing global subscriber")?;

    tracing_log::LogTracer::init().context("Failed to set Tracing as global Log drain")?;

    let mut builder = Builder::new();

    builder.threaded_scheduler();

    if let Some(count) = &cfg.executor.core_threads {
        builder.core_threads(*count);
    }

    if let Some(count) = &cfg.executor.max_threads {
        builder.max_threads(*count);
    }

    let mut rt = builder
        .enable_all()
        .build()
        .context("Unable to build Tokio runtime")?;

    rt.block_on(run(cfg))?;

    tracing::info!("Thank you. I'll say goodbye soon.. Though its the end of the would. Don't blame yourself now.. I'll be okay");

    Ok(())
}

#[rustfmt::skip]
#[inline]
fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(concat!(env!("CARGO_PKG_VERSION"), "-", env!("GIT_VERSION")))
        .arg(value("config", "c", "Use a specified config instead of the default", "CONFIG"))
        .arg(value("workers", "w", "Sets the amount of task workers", "COUNT"))
        .arg(value("backend-database", "d", "Database name for remote backend database", "DATABASE"))
        .arg(value("backend-host", "H", "Host address for remote backend database", "HOST"))
        .arg(value("backend-port", "P", "Port number for remote backend database", "PORT"))
        .arg(value("backend-file", "f", "The file of the SQLite backend database", "FILE"))
        .arg(value("backend-password", "s", "Password for remote backend database user", "PASSWORD"))
        .arg(value("backend-type", "t", "The type of the backend database", "TYPE")
            .possible_values(&["postgres", "sqlite"]))
        .arg(value("backend-username", "u", "Username for remote backend database user", "USERNAME"))
        .arg(value("tracing-directory", "o", "Directory to write tracing files to", "DIRECTORY"))
        .arg(value("tracing-flame", "F", "Enables and writes tracing flame graph to the given file", "FILE"))
        .arg(value("tracing-level", "l", "Lowest level for tracing output", "LEVEL")
            .possible_values(&["error", "warn", "info", "debug", "trace"]))
        .arg(value("tracing-prefix", "x", "Logging file name prefix", "PREFIX"))
        .arg(value("server-ip", "i", "IP that the server will listen for requests on", "IP"))
        .arg(value("server-port", "p", "Port used by the server", "PORT"))
        .arg(flag("tracing-ansi", "a", "Enables ANSI coloring of tracing output"))
        .arg(flag("tracing-json", "j", "Output tracing in JSON format"))
        .arg(flag("tracing-thread-ids", "D", "Logging output contains the ID of its source thread"))
        .arg(flag("tracing-thread-names", "n", "Logging output contains the name of its source thread"))
        .version_short("v")
}

fn flag<'b>(name: &'b str, short: &'b str, help: &'b str) -> Arg<'b, 'b> {
    Arg::with_name(name).long(name).short(short).help(help)
}

fn value<'b>(name: &'b str, short: &'b str, help: &'b str, value: &'b str) -> Arg<'b, 'b> {
    flag(name, short, help).takes_value(true).value_name(value)
}

async fn run(cfg: Arc<Config>) -> anyhow::Result<()> {
    let (tx, frontend_rx) = broadcast::channel::<()>(2);
    let download_rx = tx.subscribe();

    ctrlc::set_handler(move || {
        if tx.send(()).is_err() {
            tracing::error!("Unable to send shutdown signal");
        }
    })?;

    let version_info = Arc::new(
        backend::version()
            .appended(&mut workers::version())
            .sorted(),
    );

    let backend = DataBackend::init(cfg.database.typ, cfg.database.storage.clone(), version_info)
        .await
        .context("Unable to create backend instance")?;

    let download_handle = tokio::spawn(workers::start(cfg.clone(), download_rx, backend.clone()));
    let frontend_handle = tokio::spawn(frontend::start(cfg.clone(), frontend_rx, backend, true));

    download_handle
        .await
        .context("Unable to join download process with main")?;
    frontend_handle
        .await
        .context("Unable to join frontend process with main")?;

    Ok(())
}
