#[derive(Debug, derive_more::Display)]
#[display(fmt = "{}: {}", code, kind)]
pub struct Error {
    pub kind: ErrorKind,
    pub code: ErrorCode,
}

impl Error {
    pub fn custom(err: &'static str) -> Self {
        Self {
            kind: ErrorKind::Custom { err },
            code: ErrorCode::InternalError,
        }
    }
}

#[derive(Debug, derive_more::Display)]
pub enum ErrorKind {
    #[display(fmt = "(Askama) {}", err)]
    Askama { err: askama::Error },
    #[display(fmt = "(IO) {}", err)]
    IO { err: std::io::Error },
    #[display(fmt = "(Json) {}", err)]
    Json { err: serde_json::Error },
    #[display(fmt = "(Pool) {}", err)]
    Pool { err: r2d2::Error },
    #[display(fmt = "(Reqwest) {}", err)]
    Reqwest { err: reqwest::Error },
    #[display(fmt = "(SQLite) {}", err)]
    SQLite { err: rusqlite::Error },
    #[display(fmt = "(UTF8) {}", err)]
    UTF8 { err: std::string::FromUtf8Error },

    #[display(fmt = "(Custom) {}", err)]
    Custom { err: &'static str },
}

#[derive(Debug, derive_more::Display)]
pub enum ErrorCode {
    #[display(fmt = "Stry Error")]
    InternalError,
    #[display(fmt = "External Error")]
    ThirdParty,
}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Error {
        Error {
            kind: ErrorKind::Askama { err },
            code: ErrorCode::ThirdParty,
        }
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error {
            kind: ErrorKind::Reqwest { err },
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

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().finish()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            ErrorKind::Askama { ref err } => Some(err),
            ErrorKind::IO { ref err } => Some(err),
            ErrorKind::Json { ref err } => Some(err),
            ErrorKind::Pool { ref err } => Some(err),
            ErrorKind::Reqwest { ref err } => Some(err),
            ErrorKind::SQLite { ref err } => Some(err),
            ErrorKind::UTF8 { ref err } => Some(err),
            ErrorKind::Custom { .. } => None,
        }
    }
}
