pub mod assets;
pub mod dashboard;
pub mod story;

use {
    stry_backend::DataBackend,
    stry_common::models::{Paging, Search},
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub use crate::filters::assets::assets;

pub fn dashboard(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    use crate::filters::dashboard::{about, downloads, index, queue, updates};

    warp::path("dashboard")
        .and(
            about(state.clone())
                .or(downloads(state.clone()))
                .or(queue(state.clone()))
                .or(updates(state.clone()))
                .or(index(state)),
        )
        .boxed()
}

pub fn explore(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("explore")
        .and(warp::path::param::<String>())
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::explore)
        .boxed()
}

pub fn index(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::index)
        .boxed()
}

pub fn item(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::item)
        .boxed()
}

pub fn search(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("search")
        .and(warp::query::<Paging>())
        .and(warp::query::<Search>())
        .and(state)
        .and(warp::path::end())
        .and_then(crate::handlers::search::index)
        .boxed()
}

pub fn story(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    use crate::filters::story::{chapter, story};

    warp::get()
        .and(warp::path("story"))
        .and(chapter(state.clone()).or(story(state)))
        .boxed()
}
