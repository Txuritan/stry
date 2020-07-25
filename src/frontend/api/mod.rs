pub mod filters;
pub mod handlers;

pub mod support;

use {
    crate::{
        backend::DataBackend,
        frontend::api::handlers::{mutation::Mutation, query::Query},
    },
    juniper::{EmptySubscription, RootNode},
    warp::{filters::BoxedFilter, Filter, Reply},
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<DataBackend>>;

pub fn schema() -> Schema {
    RootNode::new(Query, Mutation, EmptySubscription::<DataBackend>::new())
}

pub fn route(state: BoxedFilter<(DataBackend,)>) -> BoxedFilter<(impl Reply,)> {
    filters::graphql(state).or(filters::playground()).boxed()
}
