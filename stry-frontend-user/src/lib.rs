pub mod controllers;
pub mod pages;

pub mod pagination;
pub mod readable;
pub mod utils;

use {
    crate::controllers::{dashboard, edit, explore, item, search, story},
    stry_backend::DataBackend,
    warp::{
        filters::BoxedFilter,
        http::header::{HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, X_FRAME_OPTIONS},
        reply::with,
        Filter, Reply,
    },
};

// const BOM: &str = include_str!("../bom.txt");

pub fn route(backend: DataBackend) -> BoxedFilter<(impl Reply,)> {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'"),
    );
    headers.insert("Feature-Policy", HeaderValue::from_static("accelerometer 'none'; ambient-light-sensor 'self'; battery 'none'; camera 'none'; gyroscope 'none'; geolocation 'none'; magnetometer 'none'; microphone 'none'; payment 'none'; web-share 'none'"));
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

    let dashboard = warp::path("dashboard").and(
        dashboard::about(backend.clone())
            .or(dashboard::downloads(backend.clone()))
            .or(dashboard::queue(backend.clone()))
            .or(dashboard::updates(backend.clone()))
            .or(dashboard::index(backend.clone())),
    );

    let edit =
        warp::path("edit").and(edit::story(backend.clone()).or(edit::chapter(backend.clone())));

    let story =
        warp::path("story").and(story::chapter(backend.clone()).or(story::index(backend.clone())));

    dashboard
        .or(edit)
        .or(story)
        .or(explore::explore(backend.clone()))
        .or(search::index(backend.clone()))
        .or(item::item(backend.clone()))
        .or(controllers::assets::assets())
        .or(akibisuto_stylus::route())
        .or(controllers::index(backend))
        .with(with::headers(headers))
        .boxed()
}
