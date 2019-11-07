use {
    crate::{
        error::{Error, ErrorKind},
        schema::{Backend, Schema},
        Pool,
    },
    postgres::to_sql_checked,
    rand::Rng,
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        OptionalExtension, Result as RusqliteResult,
    },
    std::{fmt, sync::Arc, thread, time},
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Scraper (
        Id          TEXT    PRIMARY KEY     NOT NULL    UNIQUE,
        Site        TEXT                    NOT NULL,
        Url         TEXT                    NOT NULL,
        Name        TEXT                    NOT NULL,
        Chapters    TEXT                    NOT NULL,
        Current     TEXT                    NOT NULL
    );";

pub struct Queue {}

impl Schema for Queue {
    fn schema(b: Backend, m: &mut impl fmt::Write) -> fmt::Result {
        match b {
            Backend::PostgreSQL { .. } => {}
            Backend::SQLite { .. } => {
                writeln!(m, "{}", SQLITE_TABLE)?;
            }
        }

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "site")]
pub enum Site {
    #[postgres(name = "archive-of-our-own")]
    ArchiveOfOurOwn,
    #[postgres(name = "fanfiction")]
    FanFiction,
}

impl Site {
    fn db_str(self) -> &'static str {
        match self {
            Site::ArchiveOfOurOwn => "archive-of-our-own",
            Site::FanFiction => "fanfiction",
        }
    }
}

impl FromSql for Site {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "archive-of-our-own" => Site::ArchiveOfOurOwn,
            "fanfiction" => Site::FanFiction,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Site {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Site::ArchiveOfOurOwn => "archive-of-our-own",
            Site::FanFiction => "fanfiction",
        }
        .into())
    }
}
