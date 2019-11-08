// TODO: Somehow add context to this (eg: failure and anyhow)

#[derive(Debug)]
pub enum Error {
    State {
        context: &'static str,
    },
    Moved {
        location: String,
    },

    IO {
        err: std::io::Error,
    },
    ParseInt {
        err: std::num::ParseIntError,
    },
    Utf8 {
        err: std::str::Utf8Error,
    },
    FromUtf8 {
        err: std::string::FromUtf8Error,
    },

    Http {
        err: http::Error,
    },
    UrlEncodedDes {
        err: serde_urlencoded::de::Error,
    },
    UrlEncodedSer {
        err: serde_urlencoded::ser::Error,
    },

    Askama {
        err: askama::Error,
    },
    Json {
        err: serde_json::Error,
    },

    Pool {
        err: r2d2::Error,
    },
    PostgreSQL {
        err: postgres::Error,
    },
    SQLite {
        err: rusqlite::Error,
    },

    NoRowsReturned,

    BoxSS {
        err: Box<dyn std::error::Error + Send + Sync>,
    },
    Custom {
        err: String,
    },
}

impl Error {
    pub fn new(err: impl Into<Error>) -> Self {
        err.into()
    }

    pub fn custom(err: impl Into<String>) -> Self {
        Error::Custom { err: err.into() }
    }

    pub fn moved(location: impl Into<String>) -> Self {
        Error::Moved {
            location: location.into(),
        }
    }

    pub fn state(context: &'static str) -> Self {
        Error::State { context }
    }

    pub fn no_rows_returned() -> Self {
        Error::NoRowsReturned
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::State { context } => write!(f, "(State) {}", context),
            Error::Moved { location } => write!(f, "(Moved) {}", location),

            Error::IO { ref err } => write!(f, "(IO) {}", err),
            Error::ParseInt { ref err } => write!(f, "(ParseInt) {}", err),
            Error::Utf8 { ref err } => write!(f, "(Utf8) {}", err),
            Error::FromUtf8 { ref err } => write!(f, "(FromUtf8) {}", err),

            Error::Http { ref err } => write!(f, "(Http) {}", err),
            Error::UrlEncodedDes { ref err } => write!(f, "(UrlEncodedDes) {}", err),
            Error::UrlEncodedSer { ref err } => write!(f, "(UrlEncodedSer) {}", err),

            Error::Askama { ref err } => write!(f, "(Askama) {}", err),
            Error::Json { ref err } => write!(f, "(Json) {}", err),

            Error::Pool { ref err } => write!(f, "(Pool) {}", err),
            Error::PostgreSQL { ref err } => write!(f, "(PostgreSQL) {}", err),
            Error::SQLite { ref err } => write!(f, "(SQLite) {}", err),

            Error::NoRowsReturned => write!(
                f,
                "(NoRowsReturned) No rows were returned from the database"
            ),

            Error::BoxSS { ref err } => write!(f, "(BoxSS) {}", err),
            Error::Custom { ref err } => write!(f, "(Custom) {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::State { .. } => None,
            Error::Moved { .. } => None,

            Error::IO { ref err } => Some(err),
            Error::ParseInt { ref err } => Some(err),
            Error::Utf8 { ref err } => Some(err),
            Error::FromUtf8 { ref err } => Some(err),

            Error::Http { ref err } => Some(err),
            Error::UrlEncodedDes { ref err } => Some(err),
            Error::UrlEncodedSer { ref err } => Some(err),

            Error::Askama { ref err } => Some(err),
            Error::Json { ref err } => Some(err),

            Error::Pool { ref err } => Some(err),
            Error::PostgreSQL { ref err } => Some(err),
            Error::SQLite { ref err } => Some(err),

            Error::NoRowsReturned { .. } => None,

            Error::BoxSS { .. } => None,
            Error::Custom { .. } => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO { err }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt { err }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8 { err }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::FromUtf8 { err }
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error::Http { err }
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(err: serde_urlencoded::de::Error) -> Error {
        Error::UrlEncodedDes { err }
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Error {
        Error::UrlEncodedSer { err }
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Error {
        Error::Askama { err }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json { err }
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error::Pool { err }
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Error {
        Error::PostgreSQL { err }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::SQLite { err }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Error {
        Error::BoxSS { err }
    }
}
