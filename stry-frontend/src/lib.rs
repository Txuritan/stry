use {
    std::sync::Arc, stry_backend::DataBackend, stry_common::config::Config,
    tokio::sync::broadcast::Receiver, warp::Filter,
};

// TODO: figure out how to have a conditional building for routes
pub async fn start(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    let state = warp::any().map(move || backend.clone()).boxed();

    let routes = akibisuto_stylus::route()
        .or(stry_frontend_api::route(state.clone()))
        .or(stry_frontend_user::route(state.clone()))
        .with(warp::trace::request());

    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((cfg.host, cfg.port), async move {
            rx.recv().await.expect("Failed to listen for event")
        });

    tracing::info!("warp drive engaged: listening on http://{}", addr);

    server.await;
}

pub async fn start_api(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    let state = warp::any().map(move || backend.clone()).boxed();

    let routes = akibisuto_stylus::route()
        .or(stry_frontend_api::route(state.clone()))
        .with(warp::trace::request());

    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((cfg.host, cfg.port), async move {
            rx.recv().await.expect("Failed to listen for event")
        });

    tracing::info!("warp drive engaged: listening on http://{}", addr);

    server.await;
}
