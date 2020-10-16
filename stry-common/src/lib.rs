use std::{fmt, future::Future, pin::Pin};

pub mod backend;

pub mod config;
pub mod worker;

pub type BoxedFuture<'l, T> = Pin<Box<dyn Future<Output = T> + Send + 'l>>;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum LibraryDetails {
    Curl {
        // TODO: add feature members
        number: &'static str,
        version: String,
    },
    OpenSSL {
        version: &'static str,
    },
    SQLite {
        version: &'static str,
    },
}

impl fmt::Display for LibraryDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryDetails::Curl { number, version } => {
                writeln!(f, "cURL {} ({})", version, number)
            }
            LibraryDetails::OpenSSL { version } => writeln!(f, "{}", version),
            LibraryDetails::SQLite { version } => writeln!(f, "SQLite {}", version),
        }
    }
}
