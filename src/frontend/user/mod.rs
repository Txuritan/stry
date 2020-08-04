pub mod filters;
pub mod handlers;
pub mod pages;

pub mod pagination;
pub mod readable;
pub mod utils;

use {
    crate::backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

// const BOM: &str = include_str!("../bom.txt");

pub fn route(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    filters::dashboard(state.clone())
        .or(filters::story(state.clone()))
        .or(filters::explore(state.clone()))
        .or(filters::search(state.clone()))
        .or(filters::item(state.clone()))
        .or(filters::assets())
        .or(filters::index(state))
        .boxed()
}
