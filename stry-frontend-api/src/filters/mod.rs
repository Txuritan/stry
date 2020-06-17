use {
    crate::{controllers::schema, support},
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub fn graphql(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("graphql"))
        .and(support::make_graphql_filter(
            schema(),
            warp::any().and(state).boxed(),
        ))
        .boxed()
}

pub fn playground() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(warp::path("playground"))
        .and(support::playground_filter("/graphql", None))
        .boxed()
}
