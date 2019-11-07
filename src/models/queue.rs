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
        OptionalExtension,
    },
    std::{fmt, sync::Arc, thread, time},
};

const POSTGRES_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Scraper (
        Site        VARCHAR(256)    NOT NULL    PRIMARY KEY     UNIQUE,
        State       VARCHAR(10)     NOT NULL,
        Url         VARCHAR(256)    NOT NULL,
        Name        VARCHAR(256)    NOT NULL,
        Chapters    INTEGER         NOT NULL,
        Current     INTEGER         NOT NULL
    );";

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Scraper (
        Site        TEXT        NOT NULL    PRIMARY KEY     UNIQUE,
        State       TEXT        NOT NULL,
        Url         TEXT        NOT NULL,
        Name        TEXT        NOT NULL,
        Chapters    INTEGER     NOT NULL,
        Current     INTEGER     NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Queue {
    pub site: Site,
    pub state: State,

    pub url: String,
    pub name: String,

    pub chapters: u32,
    pub current: u32,
}

impl Queue {
    pub fn is_finished(backend: Backend, site: Site) -> Result<bool, Error> {
        match &backend {
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Url FROM Queue WHERE Site = $1 AND State = $2;",
                    &[&site, &State::Finished],
                )?;

                Ok(!rows.is_empty())
            }
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let row = conn.query_row(
                    "SELECT Url FROM Queue WHERE Site = ? AND State = ?;",
                    rusqlite::params![site, State::Finished],
                    |row| row.get::<_, String>("Url"),
                ).optional()?;

                Ok(row.is_some())
            }
        }
    }

    pub fn get(backend: Backend, site: Site) -> Result<Self, Error> {
        match &backend {
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Url, State, Name, Chapters, Current FROM Queue WHERE Site = $1;",
                    &[&site],
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let row = rows.get(0);

                Ok(Self {
                    url: row.get("Url"),
                    state: row.get("State"),
                    name: row.get("Name"),
                    chapters: row.get("Chapters"),
                    current: row.get("Current"),
                    site,
                })
            }
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let queue = conn.query_row(
                    "SELECT Url, State, Name, Chapters, Current FROM Queue WHERE Site = ?;",
                    rusqlite::params![site],
                    |row| {
                        Ok(Self {
                            url: row.get("Url")?,
                            state: row.get("State")?,
                            name: row.get("Name")?,
                            chapters: row.get("Chapters")?,
                            current: row.get("Current")?,
                            site,
                        })
                    },
                )?;

                Ok(queue)
            }
        }
    }

    pub fn start(backend: Backend, url: &str, site: Site, name: &str, chapters: u32) -> Result<u64, Error> {
        match &backend {
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Url = $1, State = $2, Name = $3, Chapters = $4, Current = 1 FROM Queue WHERE Site = $5;",
                    &[&url, &State::Running, &name, &chapters, &site],
                )?;

                Ok(rows)
            }
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Url = ?, State = ?, Name = ?, Chapters = ?, Current = 1 FROM Queue WHERE Site = ?;",
                    rusqlite::params![url, State::Running, name, chapters, site],
                )?;

                Ok(rows as u64)
            }
        }
    }

    pub fn update(backend: Backend, site: Site, place: u32) -> Result<u64, Error> {
        match &backend {
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Current = $1 FROM Queue WHERE Site = $2;",
                    &[&place, &site],
                )?;

                Ok(rows)
            }
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Current = ? FROM Queue WHERE Site = ?;",
                    rusqlite::params![place, site],
                )?;

                Ok(rows as u64)
            }
        }
    }

    pub fn finish(backend: Backend, site: Site) -> Result<u64, Error> {
        match &backend {
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Url = $1, State = $2, Name = $3, Chapters = $4, Current = 1 FROM Queue WHERE Site = $5;",
                    &[&"", &State::Finished, &"", &0, &site],
                )?;

                Ok(rows)
            }
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let rows = conn.execute(
                    "UPDATE Queue SET Url = ?, State = ?, Name = ?, Chapters = ?, Current = 1 FROM Queue WHERE Site = ?;",
                    rusqlite::params!["", State::Finished, "", 0, site],
                )?;

                Ok(rows as u64)
            }
        }
    }
}

impl Schema for Queue {
    fn schema(b: Backend, m: &mut impl fmt::Write) -> fmt::Result {
        match b {
            Backend::PostgreSQL { .. } => {
                writeln!(m, "{}", POSTGRES_TABLE)?;
            }
            Backend::SQLite { .. } => {
                writeln!(m, "{}", SQLITE_TABLE)?;
            }
        }

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
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
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(match self {
            Site::ArchiveOfOurOwn => "archive-of-our-own",
            Site::FanFiction => "fanfiction",
        }
            .into())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "state")]
// #[derive(db_derive::Sql)]
pub enum State {
    #[postgres(name = "running")]
    // #[sql(name = "running")]
    Running,

    #[postgres(name = "finished")]
    // #[sql(name = "finished")]
    Finished,
}

impl State {
    fn db_str(self) -> &'static str {
        match self {
            State::Running => "running",
            State::Finished => "finished",
        }
    }
}

impl FromSql for State {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "running" => State::Running,
            "finished" => State::Finished,
            _ => unreachable!(),
        })
    }
}

impl ToSql for State {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(match self {
            State::Running => "running",
            State::Finished => "finished",
        }
            .into())
    }
}
