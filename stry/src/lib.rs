mod layer;
mod setup;

use {
    anyhow::Context,
    fenn::VecExt,
    std::sync::Arc,
    stry_backend::DataBackend,
    stry_common::{backend::Backend, config::Config},
    tokio::{runtime::Builder, sync::broadcast},
};

pub fn start(cfg: Config) -> anyhow::Result<()> {
    let cfg = Arc::new(cfg);

    let (_file_guard, _flame_guard, reg) = setup::logger(cfg.clone())?;

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
        stry_backend::version()
            .appended(&mut stry_worker::version())
            .sorted(),
    );

    let backend = DataBackend::init(cfg.database.typ, cfg.database.storage.clone(), version_info)
        .await
        .context("Unable to create backend instance")?;

    let download_handle = tokio::spawn(stry_worker::start(
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
