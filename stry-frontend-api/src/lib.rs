mod controllers;
mod filters;
mod handlers;

mod support;

use {
    crate::filters::{graphql, playground},
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn route(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    graphql(state).or(playground()).boxed()
}
