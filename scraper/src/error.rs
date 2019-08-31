#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: ErrorCode,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.kind)
    }
}

impl Error {
    pub fn custom(err: &'static str) -> Self {
        Self {
            kind: ErrorKind::Custom { err },
            code: ErrorCode::InternalError,
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IO {
        err: std::io::Error,
    },
    NumParseInt {
        err: std::num::ParseIntError,
    },
    Pool {
        err: r2d2::Error,
    },
    SQLite {
        err: rusqlite::Error,
    },
    UTF8 {
        err: std::string::FromUtf8Error,
    },
    Json {
        err: serde_json::Error,
    },
    Http {
        err: http::Error,
    },
    Isahc {
        err: isahc::Error,
    },
    BoxSS {
        err: Box<dyn std::error::Error + Send + Sync>,
    },
    Custom {
        err: &'static str,
    },
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::IO { ref err } => write!(f, "(IO) {}", err),
            ErrorKind::NumParseInt { ref err } => write!(f, "(NumParseInt) {}", err),
            ErrorKind::Pool { ref err } => write!(f, "(Pool) {}", err),
            ErrorKind::SQLite { ref err } => write!(f, "(SQLite) {}", err),
            ErrorKind::UTF8 { ref err } => write!(f, "(UTF8) {}", err),
            ErrorKind::Json { ref err } => write!(f, "(Json) {}", err),
            ErrorKind::Http { ref err } => write!(f, "(Http) {}", err),
            ErrorKind::Isahc { ref err } => write!(f, "(Isahc) {}", err),
            ErrorKind::BoxSS { ref err } => write!(f, "(BoxSS) {}", err),
            ErrorKind::Custom { ref err } => write!(f, "(Custom) {}", err),
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    InternalError,
    ThirdParty,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            kind: ErrorKind::NumParseInt { err },
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

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error {
            kind: ErrorKind::SQLite { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error {
            kind: ErrorKind::UTF8 { err },
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

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error {
            kind: ErrorKind::Http { err },
            code: ErrorCode::ThirdParty,
        }
    }
}

impl From<isahc::Error> for Error {
    fn from(err: isahc::Error) -> Error {
        Error {
            kind: ErrorKind::Isahc { err },
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            ErrorKind::IO { ref err } => Some(err),
            ErrorKind::NumParseInt { ref err } => Some(err),
            ErrorKind::Pool { ref err } => Some(err),
            ErrorKind::SQLite { ref err } => Some(err),
            ErrorKind::UTF8 { ref err } => Some(err),
            ErrorKind::Json { ref err } => Some(err),
            ErrorKind::Http { ref err } => Some(err),
            ErrorKind::Isahc { ref err } => Some(err),
            ErrorKind::BoxSS { .. } => None,
            ErrorKind::Custom { .. } => None,
        }
    }
}
