pub mod models;

pub mod backend;
pub mod config;
pub mod search;
pub mod utils;

#[cfg(feature = "nanoid")]
pub mod nanoid;

pub const GIT_VERSION: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub enum LibVersion {
    Curl {
        // TODO: add feature members
        host: String,
        number: &'static str,
        version: String,
    },
    OpenSSL {
        built_on: &'static str,
        c_flags: &'static str,
        platform: &'static str,
        version: &'static str,
    },
    SQLite {
        version: &'static str,
    },
}
