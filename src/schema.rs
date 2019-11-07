use std::fmt;

#[derive(Debug, Clone)]
pub enum Backend {
    PostgreSQL {
        pool: r2d2::Pool<r2d2_postgres::PostgresConnectionManager>,
    },
    SQLite {
        pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    },
}

impl PartialEq for Backend {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Backend::PostgreSQL { .. }, Backend::PostgreSQL { .. }) => true,
            (Backend::SQLite { .. }, Backend::SQLite { .. }) => true,
            _ => false,
        }
    }
}

pub trait Schema {
    fn schema(b: Backend, m: &mut impl fmt::Write) -> fmt::Result;
}
