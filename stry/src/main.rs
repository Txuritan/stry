use {
    anyhow::Context,
    fenn::VecExt,
    std::{
        fs,
        io::{self, prelude::*},
        path::Path,
        sync::Arc,
    },
    stry_backend::DataBackend,
    stry_common::{
        backend::Backend,
        config::{Config, LogLevel},
    },
    tokio::{runtime::Builder, sync::broadcast},
    tracing::Level,
    tracing_subscriber::{
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

fn main() -> anyhow::Result<()> {
    let cfg = load_config().context("Failure to create config instance")?;

    let file_appender =
        tracing_appender::rolling::daily(&cfg.logging.directory, &cfg.logging.prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

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
        stry_backend::version()
            .appended(&mut stry_downloader::version())
            .sorted(),
    );

    let backend = DataBackend::init(cfg.database.typ, cfg.database.storage.clone(), version_info)
        .await
        .context("Unable to create backend instance")?;

    let download_handle = tokio::spawn(stry_downloader::start(
        cfg.clone(),
        download_rx,
        backend.clone(),
    ));
    let frontend_handle = tokio::spawn(stry_frontend::start(cfg.clone(), frontend_rx, backend));

    download_handle
        .await
        .context("Unable to join download process with main")?;
    frontend_handle
        .await
        .context("Unable to join frontend process with main")?;

    Ok(())
}

pub fn load_config() -> anyhow::Result<Arc<Config>> {
    let cfg_path = Path::new("stry.ron");

    let cfg = if cfg_path.exists() {
        let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
        let mut reader = io::BufReader::new(file);

        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        ron::de::from_str(&contents)?
    } else {
        Config::default()
    };

    Ok(Arc::new(cfg))
}
