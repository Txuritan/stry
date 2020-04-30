pub mod assets;
pub mod dashboard;
pub mod story;

use {
    crate::models::{Paging, Search},
    db_derive::Pool,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub use crate::filters::assets::assets;

pub fn dashboard(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    use crate::filters::dashboard::{downloads, index, queue, updates};

    warp::get()
        .and(warp::path("dashboard"))
        .and(
            downloads(state.clone())
                .or(queue(state.clone()))
                .or(updates(state.clone()))
                .or(index(state)),
        )
        .boxed()
}

pub fn explore(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("explore"))
        .and(warp::path::param::<String>())
        .and(warp::query::<Paging>())
        .and(state)
        .and_then(crate::handlers::explore)
        .boxed()
}

pub fn index(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::query::<Paging>())
        .and(state)
        .and_then(crate::handlers::index)
        .boxed()
}

pub fn item(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::query::<Paging>())
        .and(state)
        .and_then(crate::handlers::item)
        .boxed()
}

pub fn search(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("search"))
        .and(warp::query::<Paging>())
        .and(warp::query::<Search>())
        .and(state)
        .and_then(crate::handlers::search::index)
        .boxed()
}

pub fn story(state: BoxedFilter<(Pool,)>) -> BoxedFilter<(impl Reply,)> {
    use crate::filters::story::{chapter, story};

    warp::get()
        .and(warp::path("story"))
        .and(chapter(state.clone()).or(story(state)))
        .boxed()
}
