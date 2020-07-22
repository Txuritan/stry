use {
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn about(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("about")
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::dashboard::about)
        .boxed()
}

pub fn downloads(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("downloads")
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::dashboard::downloads)
        .boxed()
}

pub fn index(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    state
        .and(warp::path::end())
        .and_then(crate::handlers::dashboard::index)
        .boxed()
}

pub fn queue(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("queue")
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::dashboard::queue)
        .boxed()
}

pub fn updates(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("updates")
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::dashboard::updates)
        .boxed()
}
