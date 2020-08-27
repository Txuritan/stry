pub mod filters;
pub mod handlers;

pub mod scalar;
pub mod support;

use {
    crate::handlers::{Mutation, Query, Subscription},
    juniper::{EmptySubscription, RootNode},
    stry_backend::DataBackend,
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<DataBackend>>;

pub fn schema() -> Schema {
    RootNode::new(Query, Mutation, EmptySubscription::<DataBackend>::new())
}

pub fn route(backend: DataBackend) -> BoxedFilter<(impl Reply,)> {
    let boxed = warp::any().map(move || backend.clone()).boxed();

    filters::graphql(boxed).or(filters::playground()).boxed()
}
