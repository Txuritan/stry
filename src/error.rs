// TODO: Somehow add context to this (eg: failure and anyhow)

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: ErrorCode,
}

impl Error {
    pub fn new(err: impl Into<Error>) -> Self {
        err.into()
    }

    pub fn custom(err: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Custom { err: err.into() },
            code: ErrorCode::InternalError,
        }
    }

    pub fn moved(location: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Moved {
                location: location.into(),
            },
            code: ErrorCode::InternalError,
        }
    }

    pub fn state(context: &'static str) -> Self {
        Self {
            kind: ErrorKind::State { context },
            code: ErrorCode::InternalError,
        }
    }

    pub fn no_rows_returned() -> Self {
        Self {
            kind: ErrorKind::NoRowsReturned,
            code: ErrorCode::InternalError,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.kind)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            ErrorKind::State { .. } => None,
            ErrorKind::Moved { .. } => None,

            ErrorKind::IO { ref err } => Some(err),
            ErrorKind::ParseInt { ref err } => Some(err),
            ErrorKind::Utf8 { ref err } => Some(err),
            ErrorKind::FromUtf8 { ref err } => Some(err),

            ErrorKind::Http { ref err } => Some(err),
            ErrorKind::UrlEncodedDes { ref err } => Some(err),
            ErrorKind::UrlEncodedSer { ref err } => Some(err),

            ErrorKind::Askama { ref err } => Some(err),
            ErrorKind::Json { ref err } => Some(err),

            ErrorKind::Pool { ref err } => Some(err),
            ErrorKind::PostgreSQL { ref err } => Some(err),
            ErrorKind::SQLite { ref err } => Some(err),

            ErrorKind::NoRowsReturned { .. } => None,

            ErrorKind::BoxSS { .. } => None,
            ErrorKind::Custom { .. } => None,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
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

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::State { context } => write!(f, "(State) {}", context),
            ErrorKind::Moved { location } => write!(f, "(Moved) {}", location),

            ErrorKind::IO { ref err } => write!(f, "(IO) {}", err),
            ErrorKind::ParseInt { ref err } => write!(f, "(ParseInt) {}", err),
            ErrorKind::Utf8 { ref err } => write!(f, "(Utf8) {}", err),
            ErrorKind::FromUtf8 { ref err } => write!(f, "(FromUtf8) {}", err),

            ErrorKind::Http { ref err } => write!(f, "(Http) {}", err),
            ErrorKind::UrlEncodedDes { ref err } => write!(f, "(UrlEncodedDes) {}", err),
            ErrorKind::UrlEncodedSer { ref err } => write!(f, "(UrlEncodedSer) {}", err),

            ErrorKind::Askama { ref err } => write!(f, "(Askama) {}", err),
            ErrorKind::Json { ref err } => write!(f, "(Json) {}", err),

            ErrorKind::Pool { ref err } => write!(f, "(Pool) {}", err),
            ErrorKind::PostgreSQL { ref err } => write!(f, "(PostgreSQL) {}", err),
            ErrorKind::SQLite { ref err } => write!(f, "(SQLite) {}", err),

            ErrorKind::NoRowsReturned => write!(
                f,
                "(NoRowsReturned) No rows were returned from the database"
            ),

            ErrorKind::BoxSS { ref err } => write!(f, "(BoxSS) {}", err),
            ErrorKind::Custom { ref err } => write!(f, "(Custom) {}", err),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    UnknownError,
    InternalError,
    ThirdParty,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorCode::UnknownError => "Unknown Error",
                ErrorCode::InternalError => "Stry Error",
                ErrorCode::ThirdParty => "External Error",
            }
        )
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            kind: ErrorKind::IO { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error {
            kind: ErrorKind::ParseInt { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error {
            kind: ErrorKind::Utf8 { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error {
            kind: ErrorKind::FromUtf8 { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error {
            kind: ErrorKind::Http { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(err: serde_urlencoded::de::Error) -> Error {
        Error {
            kind: ErrorKind::UrlEncodedDes { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Error {
        Error {
            kind: ErrorKind::UrlEncodedSer { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Error {
        Error {
            kind: ErrorKind::Askama { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error {
            kind: ErrorKind::Json { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Error {
        Error {
            kind: ErrorKind::Pool { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<postgres::Error> for Error {
    fn from(err: postgres::Error) -> Error {
        Error {
            kind: ErrorKind::PostgreSQL { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error {
            kind: ErrorKind::SQLite { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Error {
        Error {
            kind: ErrorKind::BoxSS { err },
            code: ErrorCode::ThirdParty,
        }
    }
}
