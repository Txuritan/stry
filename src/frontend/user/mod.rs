pub mod controllers;
pub mod pages;

pub mod pagination;
pub mod readable;
pub mod utils;

use {
    crate::{backend::DataBackend, frontend::user::controllers::{dashboard, explore, item, search, story}},
    warp::{filters::BoxedFilter, Filter, Reply},
};

// const BOM: &str = include_str!("../bom.txt");

pub fn route(state: DataBackend) -> BoxedFilter<(impl Reply,)> {
    let dashboard = warp::path("dashboard")
        .and(
            dashboard::about(state.clone())
                .or(dashboard::downloads(state.clone()))
                .or(dashboard::queue(state.clone()))
                .or(dashboard::updates(state.clone()))
                .or(dashboard::index(state.clone()))
        );

    let story = warp::path("story")
        .and(
            story::chapter(state.clone())
                .or(story::index(state.clone()))
        );

    dashboard.or(story)
        .or(explore::explore(state.clone()))
        .or(search::index(state.clone()))
        .or(item::item(state.clone()))
        .or(controllers::assets::assets())
        .or(controllers::index(state))
        .boxed()
}
