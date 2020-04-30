pub mod backend;
pub mod models;

pub use crate::backend::{
    Backend, BackendAuthor, BackendChapter, BackendConnection, BackendOrigin, BackendStory,
    BackendTag,
};

pub const GIT_VERSION: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
