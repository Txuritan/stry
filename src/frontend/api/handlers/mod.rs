pub mod mutation;
pub mod query;
pub mod subscription;

pub use crate::frontend::api::handlers::{
    mutation::Mutation, query::Query, subscription::Subscription,
};
