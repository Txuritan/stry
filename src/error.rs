#[derive(Debug, derive_more::Display)]
#[display(fmt = "{}: {}", code, kind)]
pub struct Error {
    kind: ErrorKind,
    code: ErrorCode,
}

#[derive(Debug, derive_more::Display)]
pub enum ErrorKind {
    #[display(fmt = "(Askama) {}", err)]
    Askama { err: askama::Error },
    #[display(fmt = "(Pool) {}", err)]
    Pool { err: r2d2::Error },
    #[display(fmt = "(SQLite) {}", err)]
    SQLite { err: rusqlite::Error },
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

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().finish()
    }
}
