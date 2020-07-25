use {
    crate::{backend::DataBackend, frontend::user::handlers::dashboard},
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn about(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("about")
        .and(state)
        .and(warp::path::end())
        .and_then(dashboard::about)
        .boxed()
}

pub fn downloads(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("downloads")
        .and(state)
        .and(warp::path::end())
        .and_then(dashboard::downloads)
        .boxed()
}

pub fn index(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    state
        .and(warp::path::end())
        .and_then(dashboard::index)
        .boxed()
}

pub fn queue(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("queue")
        .and(state)
        .and(warp::path::end())
        .and_then(dashboard::queue)
        .boxed()
}

pub fn updates(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("updates")
        .and(state)
        .and(warp::path::end())
        .and_then(dashboard::updates)
        .boxed()
}
