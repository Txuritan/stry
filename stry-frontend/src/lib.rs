use {
    std::sync::Arc,
    stry_backend::DataBackend,
    stry_common::config::Config,
    tokio::sync::broadcast::Receiver,
    warp::{Filter, Rejection, Reply},
};

pub async fn start(cfg: Arc<Config>, mut rx: Receiver<()>, backend: DataBackend) {
    let (enable_api, enable_user) = cfg.frontend.as_bool();

    let routes = api_routes(enable_api, backend.clone())
        .or(user_routes(enable_user, backend.clone()))
        // I want to use brotli, but Firefpx isn't adding new features to HTTP (non HTTPS),
        // as such it only sends `Accept-Encoding: gzip, deflate`.
        // That means I'll either wrap the server in NGINX, or allow for TLS through the config.
        // See: https://bugzilla.mozilla.org/show_bug.cgi?id=1218924
        .with(warp::compression::gzip())
        .with(warp::trace::request());

    let (addr, server) = warp::serve(routes)
        .bind_with_graceful_shutdown((cfg.ip, cfg.port), async move {
            rx.recv().await.expect("Failed to listen for event")
        });

    tracing::info!("warp drive engaged: listening on http://{}", addr);

    server.await;
}

#[cfg(feature = "api")]
fn api_routes(
    enable_api: bool,
    backend: DataBackend,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    enable(enable_api)
        .and(stry_frontend_api::route(backend))
        .boxed()
}

#[cfg(not(feature = "api"))]
fn api_routes(
    _enable_api: bool,
    _backend: DataBackend,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::any()
        .and_then(move || async move { Result::<String, _>::Err(warp::reject::not_found()) })
        .boxed()
}

#[cfg(feature = "user")]
fn user_routes(
    enable_user: bool,
    backend: DataBackend,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    enable(enable_user)
        .and(stry_frontend_user::route(backend))
        .boxed()
}

#[cfg(not(feature = "user"))]
fn user_routes(
    _enable_user: bool,
    _backend: DataBackend,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::any()
        .and_then(move || async move { Result::<String, _>::Err(warp::reject::not_found()) })
        .boxed()
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
