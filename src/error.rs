pub struct Error {
    kind: ErrorKind,
    code: ErrorCode,
}

pub enum ErrorKind {
    Pool { err: r2d2::Error },
    SQLite { err: rusqlite::Error },
}

pub enum ErrorCode {
    InternalError,
    ThirdParty,
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
