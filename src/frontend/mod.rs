pub mod api;
pub mod user;

use {
    crate::{backend::DataBackend, config::Config},
    std::sync::Arc,
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
    let state = warp::any()
        .map({
            let backend = backend.clone();

            move || backend.clone()
        })
        .boxed();

    let routes = akibisuto_stylus::route()
        .or(api::route(state.clone()))
        .or(enable(enable_frontend).and(user::route(backend.clone())))
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
