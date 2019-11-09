use {
    crate::{
        error::Error,
        row, execute,
        schema::{Backend, Schema},
    },
    http_req::uri::Uri,
    rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    std::fmt,
};

const POSTGRES_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Scraper (
        Id          VARCHAR(6)      NOT NULL    PRIMARY KEY     UNIQUE,
        Site        VARCHAR(256)    NOT NULL,
        State       VARCHAR(10)     NOT NULL,
        Url         VARCHAR(256)    NOT NULL,
        Name        VARCHAR(256)    NOT NULL,
        Chapters    INTEGER         NOT NULL
    );";

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Scraper (
        Id          TEXT        NOT NULL    PRIMARY KEY     UNIQUE,
        Site        TEXT        NOT NULL,
        State       TEXT        NOT NULL,
        Url         TEXT        NOT NULL,
        Name        TEXT        NOT NULL,
        Chapters    INTEGER     NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Queue {
    pub id: String,

    pub site: Site,
    pub state: State,

    pub url: String,
    pub name: String,

    pub chapters: u32,
}

impl Queue {
    pub fn create(backend: Backend, url: &str, name: &str, chapters: u32) -> Result<String, Error> {
        let id = crate::nanoid!();

        let site = Site::from_url(url).ok_or_else(|| Error::UnknownSite)?;

        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                execute!(p[conn] => (
                    "INSERT INTO Queue (Id, Site, State, Url, Name, Chapters) VALUES ($1, $2, $3, $4, $5, $6);",
                    [id, site, State::Queued, url, name, chapters]
                ));
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                execute!(s[conn] => (
                    "INSERT INTO Queue (Id, Site, State, Url, Name, Chapters) VALUES (?, ?, ?, ?, ?, ?);",
                    [id, site, State::Queued, url, name, chapters]
                ));
            } //#endregion
        }

        Ok(id)
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let queue = row!(p[conn] => (
                    "SELECT Url, Site, State, Name, Chapters FROM Queue WHERE Id = $1;",
                    [id],
                    |row| Ok(Self {
                        id: id.to_owned(),
                        url: row.get("Url"),
                        site: row.get("Site"),
                        state: row.get("State"),
                        name: row.get("Name"),
                        chapters: row.get("Chapters"),
                    })
                ));

                Ok(queue)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let queue = row!(s[conn] => (
                    "SELECT Url, Site, State, Name, Chapters FROM Queue WHERE Id = ?;",
                    [id],
                    |row| Ok(Self {
                        id: id.to_owned(),
                        url: row.get("Url")?,
                        site: row.get("Site")?,
                        state: row.get("State")?,
                        name: row.get("Name")?,
                        chapters: row.get("Chapters")?,
                    })
                ));

                Ok(queue)
            } //#endregion
        }
    }

    pub fn finish(backend: Backend, id: &str) -> Result<u64, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let mut conn = pool.get()?;

                let rows = execute!(p[conn] => (
                    "UPDATE Queue SET State = $1 FROM Queue WHERE Id = $2;",
                    [State::Finished, id]
                ));

                Ok(rows)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let rows = execute!(s[conn] => (
                    "UPDATE Queue SET State = ? FROM Queue WHERE Id = ?;",
                    [State::Finished, id]
                ));

                Ok(rows as u64)
            } //#endregion
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
    fn from_url(url: &str) -> Option<Self> {
        url.parse::<Uri>()
            .ok()
            .and_then(|uri| uri.host().map(String::from))
            .and_then(|host| match host.as_str() {
                "archiveofourown.org" | "www.archiveofourown.org" => Some(Site::ArchiveOfOurOwn),
                "fanfiction.net" | "www.fanfiction.net" | "m.fanfiction.net" => {
                    Some(Site::FanFiction)
                }
                _ => None,
            })
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
    #[postgres(name = "queued")]
    // #[sql(name = "queued")]
    Queued,

    #[postgres(name = "finished")]
    // #[sql(name = "finished")]
    Finished,
}

impl State {
    fn db_str(self) -> &'static str {
        match self {
            State::Queued => "queued",
            State::Finished => "finished",
        }
    }
}

impl FromSql for State {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "queued" => State::Queued,
            "finished" => State::Finished,
            _ => unreachable!(),
        })
    }
}

impl ToSql for State {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput> {
        Ok(match self {
            State::Queued => "queued",
            State::Finished => "finished",
        }
        .into())
    }
}
