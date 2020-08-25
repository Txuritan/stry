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
    fenn::VecExt,
    std::{future::Future, pin::Pin, sync::Arc},
    tokio::{runtime::Builder, sync::broadcast},
    tracing::{Level, Subscriber},
    tracing_subscriber::{
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

pub type Boxed = Pin<Box<dyn Future<Output = anyhow::Result<()>>>>;

pub fn setup(cfg: Config) -> anyhow::Result<()> {
    let cfg = Arc::new(cfg);

    let (normal, json) = if cfg.logging.json {
        let layer = fmt::Layer::default()
            .with_ansi(cfg.logging.ansi)
            .with_thread_ids(cfg.logging.thread_ids)
            .with_thread_names(cfg.logging.thread_names)
            .with_span_events(FmtSpan::CLOSE)
            .json();

        (None, Some(layer))
    } else {
        let layer = fmt::Layer::default()
            .with_ansi(cfg.logging.ansi)
            .with_thread_ids(cfg.logging.thread_ids)
            .with_thread_names(cfg.logging.thread_names)
            .with_span_events(FmtSpan::CLOSE);

        (Some(layer), None)
    };

    let (file_normal, file_json, _appender_guard) =
        if let Some(directory) = cfg.logging.directory.as_ref() {
            let file_appender = tracing_appender::rolling::daily(&directory, &cfg.logging.prefix);
            let (non_blocking, appender_guard) = tracing_appender::non_blocking(file_appender);

            if cfg.logging.json {
                let layer = fmt::Layer::default()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_thread_ids(cfg.logging.thread_ids)
                    .with_thread_names(cfg.logging.thread_names)
                    .with_span_events(FmtSpan::CLOSE);

                (Some(layer), None, None)
            } else {
                let layer = fmt::Layer::default()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_thread_ids(cfg.logging.thread_ids)
                    .with_thread_names(cfg.logging.thread_names)
                    .with_span_events(FmtSpan::CLOSE);
                (None, Some(layer), None)
            }
        } else {
            (None, None, None)
        };

    // TODO: Get JSON output working
    let reg = Registry::default()
        .with(normal)
        .with(json)
        .with(file_normal)
        .with(file_json)
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
