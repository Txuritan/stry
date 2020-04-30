use {crate::filters, db_derive::Pool, std::future::Future, warp::Filter};

pub async fn start(pool: Pool) -> impl Future<Output = ()> + 'static {
    let with_state = warp::any().map(move || pool.clone()).boxed();

    let routes = akibisuto_stylus::route()
        .or(filters::dashboard(with_state.clone()))
        .or(filters::story(with_state.clone()))
        .or(filters::assets())
        .or(filters::explore(with_state.clone()))
        .or(filters::item(with_state.clone()))
        .or(filters::search(with_state.clone()))
        .or(filters::index(with_state))
        .with(warp::log("stry"));

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 8901), async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for event")
        });

    tracing::info!("warp drive engaged: listening on http://{}", addr);

    server
}
