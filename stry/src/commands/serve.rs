use {
    crate::Boxed,
    anyhow::Context,
    fenn::VecExt,
    std::sync::Arc,
    stry_backend::DataBackend,
    stry_common::{backend::Backend, config::Config},
    tokio::sync::broadcast,
};

pub fn handle(args: pico_args::Arguments, cfg: Arc<Config>) -> anyhow::Result<Boxed> {
    Ok(Box::pin(run(cfg)))
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
