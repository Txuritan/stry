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
    tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber},
};

fn main() -> anyhow::Result<()> {
    let cfg = load_config().context("Failure to create config instance")?;

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

    let builder = FmtSubscriber::builder()
        .with_max_level(match cfg.logging.level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        })
        .with_span_events(FmtSpan::CLOSE);

    // TODO: figure out a way so i don't have to have to different `set_global_default` calls
    if cfg.logging.json {
        tracing::subscriber::set_global_default(builder.json().finish())
            .context("Failed to set Tracing global subscriber")?;
    } else {
        tracing::subscriber::set_global_default(builder.finish())
            .context("Failed to set Tracing global subscriber")?;
    }

    tracing_log::LogTracer::init().context("Failed to set Tracing as global Log drain")?;

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
