mod task;
mod worker;

use {
    std::sync::Arc,
    stry_backend::DataBackend,
    stry_common::{config::Config, LibVersion},
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
        task::task,
        backend,
    )
    .await;
}

pub fn version() -> Vec<LibVersion> {
    let curl_version = curl::Version::get();

    vec![
        LibVersion::Curl {
            host: curl_version.host().to_string(),
            number: curl::Version::num(),
            version: curl_version.version().to_string(),
        },
        LibVersion::OpenSSL {
            built_on: openssl::version::built_on(),
            c_flags: openssl::version::c_flags(),
            platform: openssl::version::platform(),
            version: openssl::version::version(),
        },
    ]
}
