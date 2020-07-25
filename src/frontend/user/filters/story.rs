use {
    crate::{backend::DataBackend, frontend::user::handlers},
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn chapter(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path::param::<String>()
        .and(warp::path::param::<u32>())
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::story::chapter)
        .boxed()
}

pub fn story(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path::param::<String>()
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::story::index)
        .boxed()
}
