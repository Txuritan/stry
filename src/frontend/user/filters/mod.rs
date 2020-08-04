pub mod assets;
pub mod dashboard;
pub mod story;

use {
    crate::{
        backend::DataBackend,
        frontend::user::{handlers, utils::Items},
        models::{Paging, Search},
    },
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub use assets::assets;

pub fn dashboard(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    use dashboard::{about, downloads, index, queue, updates};

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
        .and(warp::path::param::<Items>())
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::explore)
        .boxed()
}

pub fn index(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::index)
        .boxed()
}

pub fn item(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::param::<Items>())
        .and(warp::path::param::<String>())
        .and(warp::query::<Paging>())
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::item)
        .boxed()
}

pub fn search(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("search")
        .and(warp::query::<Paging>())
        .and(warp::query::<Search>())
        .and(state)
        .and(warp::path::end())
        .and_then(handlers::search::index)
        .boxed()
}

pub fn story(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("story"))
        .and(story::chapter(state.clone()).or(story::story(state)))
        .boxed()
}
