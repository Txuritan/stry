mod downloader;
mod task;

use {
    std::sync::Arc, stry_backend::DataBackend, stry_common::config::Config,
    tokio::sync::broadcast::Receiver,
};

pub async fn start(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    downloader::Downloader::new(
        async move { rx.recv().await.expect("Failed to listen for event") },
        backend,
        cfg.workers,
        task::task,
    )
    .await;
}
