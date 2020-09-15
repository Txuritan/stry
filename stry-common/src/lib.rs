use std::{future::Future, pin::Pin};

pub mod backend;
pub mod models;

pub mod config;
pub mod search;
pub mod test_helpers;
pub mod version;
pub mod worker;

pub type BoxedFuture<'l, T> = Pin<Box<dyn Future<Output = T> + Send + 'l>>;
