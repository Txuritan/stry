use {
    std::sync::Arc, stry_backend::DataBackend, stry_common::LibraryDetails, stry_config::Config,
    stry_worker_scraper::task, tokio::sync::broadcast::Receiver,
};

pub async fn start(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    // worker::WorkerPool::new(
    //     async move { rx.recv().await.expect("Failed to listen for event") },
    //     backend,
    //     cfg.workers,
    //     task::task,
    // )
    // .await;

    stry_evermore::worker(
        async move { rx.recv().await.expect("Failed to listen for event") },
        cfg.workers,
        task::task,
        backend,
    )
    .await;
}

pub fn library_details() -> Vec<LibraryDetails> {
    stry_worker_scraper::library_details()
}
