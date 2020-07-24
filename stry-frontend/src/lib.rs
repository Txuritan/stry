use {
    std::sync::Arc,
    stry_backend::DataBackend,
    stry_common::config::Config,
    tokio::sync::broadcast::Receiver,
    warp::{Filter, Rejection},
};

// TODO: figure out how to have a conditional building for routes
pub async fn start(
    cfg: Arc<Config>,
    mut rx: Receiver<()>,
    backend: DataBackend,
    enable_frontend: bool,
) {
    let state = warp::any().map(move || backend.clone()).boxed();

    let routes = akibisuto_stylus::route()
        .or(stry_frontend_api::route(state.clone()))
        .or(enable(enable_frontend).and(stry_frontend_user::route(state.clone())))
        .with(warp::trace::request());

    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((cfg.host, cfg.port), async move {
            rx.recv().await.expect("Failed to listen for event")
        });

    tracing::info!("warp drive engaged: listening on http://{}", addr);

    server.await;
}

// https://github.com/seanmonstar/warp/issues/131
fn enable(is_enabled: bool) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::any()
        .and_then(move || async move {
            if is_enabled {
                Ok(())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .untuple_one()
        .boxed()
}
