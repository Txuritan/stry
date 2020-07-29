pub mod scraper;
pub mod worker;

use {
    crate::{backend::DataBackend, config::Config, version::LibVersion},
    std::sync::Arc,
    tokio::sync::broadcast::Receiver,
};

pub async fn start(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    // worker::WorkerPool::new(
    //     async move { rx.recv().await.expect("Failed to listen for event") },
    //     backend,
    //     cfg.workers,
    //     task::task,
    // )
    // .await;

    worker::worker(
        async move { rx.recv().await.expect("Failed to listen for event") },
        cfg.workers,
        scraper::task::task,
        backend,
    )
    .await;
}

pub fn version() -> Vec<LibVersion> {
    vec![
        LibVersion::Curl {
            number: curl::Version::num(),
            version: curl::Version::get().version().to_string(),
        },
        LibVersion::OpenSSL {
            version: openssl::version::version(),
        },
    ]
}
