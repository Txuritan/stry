#[macro_use]
pub mod sites;

#[cfg(test)]
pub mod tests;

pub mod models;
pub mod task;
pub mod utils;

pub use {isahc::http::uri::Uri, sites::*};

use stry_common::LibraryDetails;

pub fn library_details() -> Vec<LibraryDetails> {
    vec![
        LibraryDetails::Curl {
            number: curl::Version::num(),
            version: curl::Version::get().version().to_string(),
        },
        LibraryDetails::OpenSSL {
            version: openssl::version::version(),
        },
    ]
}
