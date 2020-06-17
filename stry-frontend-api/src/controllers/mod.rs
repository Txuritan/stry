use {
    crate::handlers::{mutation::Mutation, query::Query},
    juniper::{EmptySubscription, RootNode},
    stry_backend::DataBackend,
};

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<DataBackend>>;

pub fn schema() -> Schema {
    RootNode::new(Query, Mutation, EmptySubscription::<DataBackend>::new())
}
