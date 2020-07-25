use {
    crate::{
        backend::DataBackend,
        frontend::api::{schema, support},
    },
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn graphql(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::path("graphql")
        .and(support::make_graphql_filter(
            schema(),
            warp::any().and(state).boxed(),
        ))
        .boxed()
}

pub fn playground() -> BoxedFilter<(impl Reply,)> {
    warp::path("playground")
        .and(support::playground_filter("/graphql", None))
        .boxed()
}
