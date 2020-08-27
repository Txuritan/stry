pub mod controllers;
pub mod pages;

pub mod pagination;
pub mod readable;
pub mod utils;

use {
    crate::controllers::{dashboard, explore, item, search, story},
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

// const BOM: &str = include_str!("../bom.txt");

pub fn route(backend: DataBackend) -> BoxedFilter<(impl Reply,)> {
    let dashboard = warp::path("dashboard").and(
        dashboard::about(backend.clone())
            .or(dashboard::downloads(backend.clone()))
            .or(dashboard::queue(backend.clone()))
            .or(dashboard::updates(backend.clone()))
            .or(dashboard::index(backend.clone())),
    );

    let story =
        warp::path("story").and(story::chapter(backend.clone()).or(story::index(backend.clone())));

    dashboard
        .or(story)
        .or(explore::explore(backend.clone()))
        .or(search::index(backend.clone()))
        .or(item::item(backend.clone()))
        .or(controllers::assets::assets())
        .or(akibisuto_stylus::route())
        .or(controllers::index(backend))
        .boxed()
}
