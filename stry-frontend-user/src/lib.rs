pub mod utils;

pub mod controllers;
pub mod models;
pub mod pages;

pub mod pagination;
pub mod readable;

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
    utils::init_fluent().expect("Unable to initialize Fluent");

    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'"),
    );
    headers.insert("Feature-Policy", HeaderValue::from_static("accelerometer 'none'; ambient-light-sensor 'self'; battery 'none'; camera 'none'; gyroscope 'none'; geolocation 'none'; magnetometer 'none'; microphone 'none'; payment 'none'; web-share 'none'"));
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

    let dashboard: BoxedFilter<(_,)> = warp::path("dashboard")
        .and(
            dashboard::about(backend.clone())
                .or(dashboard::downloads(backend.clone()))
                .boxed()
                .or(dashboard::queue(backend.clone()))
                .boxed()
                .or(dashboard::updates(backend.clone()))
                .boxed()
                .or(dashboard::index(backend.clone()))
                .boxed(),
        )
        .boxed();

    let edit: BoxedFilter<(_,)> = warp::path("edit")
        .and(
            edit::story(backend.clone())
                .or(edit::chapter_get(backend.clone()))
                .or(edit::chapter_post(backend.clone()))
                .boxed(),
        )
        .boxed();

    let story: BoxedFilter<(_,)> = warp::path("story")
        .and(
            story::chapter(backend.clone())
                .or(story::index(backend.clone()))
                .boxed(),
        )
        .boxed();

    dashboard
        .or(edit)
        .boxed()
        .or(story)
        .boxed()
        .or(explore::explore(backend.clone()))
        .boxed()
        .or(search::index(backend.clone()))
        .boxed()
        .or(item::item(backend.clone()))
        .boxed()
        .or(controllers::assets::assets())
        .boxed()
        .or(akibisuto_stylus::route())
        .boxed()
        .or(controllers::index(backend))
        .boxed()
        .with(with::headers(headers))
        .boxed()
}
