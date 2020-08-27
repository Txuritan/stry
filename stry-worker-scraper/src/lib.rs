#[macro_use]
pub mod sites;

#[cfg(test)]
pub mod tests;

pub mod converter;
pub mod models;
pub mod query;
pub mod task;
pub mod utils;

pub use {isahc::http::uri::Uri, sites::*};

use stry_common::version::LibVersion;

pub fn version() -> Vec<LibVersion> {
    vec![
        LibVersion::Curl {
            number: curl::Version::num(),
            version: curl::Version::get().version().to_string(),
        },
        LibVersion::OpenSSL {
            version: openssl::version::version(),
        },
    ]
}
