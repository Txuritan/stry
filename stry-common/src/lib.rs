pub mod models;

pub mod backend;
pub mod utils;

pub use crate::backend::{
    Backend, BackendAuthor, BackendChapter, BackendOrigin, BackendStory, BackendTag,
};

pub const GIT_VERSION: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
