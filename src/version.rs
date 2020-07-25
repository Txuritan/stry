use std::fmt;

pub const BOM: &str = include_str!("../bom.txt");
pub const GIT_VERSION: &str = env!("GIT_VERSION");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum LibVersion {
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

impl fmt::Display for LibVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibVersion::Curl { number, version } => writeln!(f, "cURL {} ({})", version, number),
            LibVersion::OpenSSL { version } => writeln!(f, "{}", version),
            LibVersion::SQLite { version } => writeln!(f, "SQLite {}", version),
        }
    }
}
