use {
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn chapter(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path::param::<String>()
        .and(warp::path::param::<u32>())
        .and(state)
        .and_then(crate::handlers::story::chapter)
        .boxed()
}

pub fn story(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path::param::<String>()
        .and(state)
        .and_then(crate::handlers::story::index)
        .boxed()
}
