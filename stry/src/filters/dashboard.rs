use {
    db_derive::Pool,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn downloads(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("downloads")
        .and(state)
        .and_then(crate::handlers::dashboard::downloads)
        .boxed()
}

pub fn index(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    state.and_then(crate::handlers::dashboard::index).boxed()
}

pub fn queue(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("queue")
        .and(state)
        .and_then(crate::handlers::dashboard::queue)
        .boxed()
}

pub fn updates(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("updates")
        .and(state)
        .and_then(crate::handlers::dashboard::updates)
        .boxed()
}
