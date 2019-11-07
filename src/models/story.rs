use {
    crate::{
        models::{Author, Origin, Series, Tag, TagType},
        schema::{Backend, Schema},
        Error,
    },
    chrono::{DateTime, Utc},
    postgres::to_sql_checked,
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        Result as RusqliteResult,
    },
    std::fmt,
};

const POSTGRES_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Story (
        Id          VARCHAR(6)                      NOT NULL    PRIMARY KEY     UNIQUE,
        Url         VARCHAR(256)                    NOT NULL,
        Name        VARCHAR(256)                    NOT NULL,
        Summary     VARCHAR(256)                    NOT NULL,
        Language    VARCHAR(24)                     NOT NULL,
        Rating      VARCHAR(24)                     NOT NULL,
        State       VARCHAR(24)                     NOT NULL,
        Created     TIMESTAMP WITHOUT TIME ZONE     NOT NULL    DEFAULT (DATETIME('now', 'utc')),
        Updated     TIMESTAMP WITHOUT TIME ZONE     NOT NULL    DEFAULT (DATETIME('now', 'utc'))
    );";

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Story (
        Id          TEXT    NOT NULL    PRIMARY KEY     UNIQUE,
        Url         TEXT    NOT NULL,
        Name        TEXT    NOT NULL,
        Summary     TEXT    NOT NULL,
        Language    TEXT    NOT NULL,
        Rating      TEXT    NOT NULL,
        State       TEXT    NOT NULL,
        Created     TEXT    NOT NULL    DEFAULT (DATETIME('now', 'utc')),
        Updated     TEXT    NOT NULL    DEFAULT (DATETIME('now', 'utc'))
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: String,

    pub url: String,
    pub name: String,
    pub summary: String,

    pub language: Language,
    pub square: Square,

    pub chapters: u32,
    pub words: u32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,
    pub tags: Vec<Tag>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Story {
    pub fn all(backend: Backend, page: u32, limit: u32) -> Result<(u32, Vec<Self>), Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let rows = conn.query(
                    "SELECT Id FROM Story ORDER BY Updated DESC LIMIT $1 OFFSET $2;",
                    &[&limit, &(10 * page)],
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let mut stories = Vec::with_capacity(rows.len());

                for row in rows.iter() {
                    stories.push(Story::get(backend.clone(), &row.get::<_, String>("Id"))?);
                }

                let count_rows = conn.query("SELECT COUNT(Id) as Count FROM Story;", &[])?;

                if count_rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let count = count_rows.get(0).get("Count");

                Ok((count, stories))
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let mut stmt =
                    conn.prepare("SELECT Id FROM Story ORDER BY Updated DESC LIMIT ? OFFSET ?;")?;

                let rows = stmt.query_map(rusqlite::params![limit, 10 * page], |row| {
                    row.get::<_, String>("Id")
                })?;

                let mut stories = Vec::with_capacity(limit as usize);

                for row in rows {
                    stories.push(Story::get(backend.clone(), &row?)?);
                }

                let count = conn.query_row(
                    "SELECT COUNT(Id) as Count FROM Story;",
                    rusqlite::NO_PARAMS,
                    |row| row.get("Count"),
                )?;

                Ok((count, stories))
            } //#endregion
        }
    }

    pub fn get(backend: Backend, id: &str) -> Result<Self, Error> {
        match &backend {
            //#region[rgba(241,153,31,0.1)] PostgreSQL
            Backend::PostgreSQL { pool } => {
                let conn = pool.get()?;

                let authors = Author::of_story(backend.clone(), id)?;
                let origins = Origin::of_story(backend.clone(), id)?;
                let tags = Tag::of_story(backend.clone(), id)?;

                let warn = tags.iter().any(|t| t.typ == TagType::Warning);

                let rows = conn.query(
                    "SELECT Id, Url, Name, Summary, Language, Rating, State, Created, Updated FROM Story WHERE Id = $1;",
                    &[&id]
                )?;

                if rows.is_empty() {
                    return Err(Error::no_rows_returned());
                }

                let row = rows.get(0);

                let story = Self {
                    id: row.get("Id"),
                    url: row.get("Url"),
                    name: row.get("Name"),
                    summary: row.get("Summary"),
                    language: row.get("Language"),
                    chapters: {
                        let count_rows = conn.query(
                                "SELECT COUNT(StoryId) as Chapters FROM StoryChapter WHERE StoryId = $1;",
                                &[&id],
                            )?;

                        if count_rows.is_empty() {
                            return Err(Error::no_rows_returned());
                        }

                        count_rows.get(0).get("Chapters")
                    },
                    words: {
                        let count_rows = conn.query(
                                "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = $1;",
                                &[&id],
                            )?;

                        if count_rows.is_empty() {
                            return Err(Error::no_rows_returned());
                        }

                        count_rows.get(0).get("Words")
                    },
                    created: row.get("Created"),
                    updated: row.get("Updated"),
                    square: Square {
                        rating: row.get("Rating"),
                        warnings: if warn { Warning::Using } else { Warning::None },
                        state: row.get("State"),
                    },
                    series: None,
                    authors,
                    origins,
                    tags,
                };

                Ok(story)
            }
            //#endregion

            //#region[rgba(51,103,145,0.1)] SQLite
            Backend::SQLite { pool } => {
                let conn = pool.get()?;

                let authors = Author::of_story(backend.clone(), id)?;
                let origins = Origin::of_story(backend.clone(), id)?;
                let tags = Tag::of_story(backend.clone(), id)?;

                let warn = tags.iter().any(|t| t.typ == TagType::Warning);

                let story = conn.query_row(
                    "SELECT Id, Name, Summary, Language, Rating, State, Created, Updated FROM Story WHERE Id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(Self {
                            id: row.get("Id")?,
                            url: row.get("Url")?,
                            name: row.get("Name")?,
                            summary: row.get("Summary")?,
                            language: row.get("Language")?,
                            chapters: conn.query_row(
                                "SELECT COUNT(StoryId) as Chapters FROM StoryChapter WHERE StoryId = ?;",
                                rusqlite::params![id],
                                |row| row.get("Chapters")
                            )?,
                            words: conn.query_row(
                                "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = ?;",
                                rusqlite::params![id],
                                |row| row.get("Words")
                            )?,
                            created: row.get("Created")?,
                            updated: row.get("Updated")?,
                            square: Square {
                                rating: row.get("Rating")?,
                                warnings: if warn { Warning::Using } else { Warning::None },
                                state: row.get("State")?,
                            },
                            series: None,
                            authors,
                            origins,
                            tags,
                        })
                    }
                )?;

                Ok(story)
            } //#endregion
        }
    }
}

impl Schema for Story {
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
#[postgres(name = "language")]
pub enum Language {
    #[postgres(name = "english")]
    #[serde(rename = "english")]
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::English => "english",
            }
        )
    }
}

impl FromSql for Language {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "english" => Language::English,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Language {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "rating")]
pub enum Rating {
    #[postgres(name = "explicit")]
    #[serde(rename = "explicit")]
    Explicit,

    #[postgres(name = "mature")]
    #[serde(rename = "mature")]
    Mature,

    #[postgres(name = "teen")]
    #[serde(rename = "teen")]
    Teen,

    #[postgres(name = "general")]
    #[serde(rename = "general")]
    General,
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rating::Explicit => "black-500",
                Rating::Mature => "red-500",
                Rating::Teen => "green-600",
                Rating::General => "gray-700",
            }
        )
    }
}

impl FromSql for Rating {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "explicit" => Rating::Explicit,
            "mature" => Rating::Mature,
            "teen" => Rating::Teen,
            "general" => Rating::General,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Rating {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Rating::Explicit => "explicit",
            Rating::Mature => "mature",
            Rating::Teen => "teen",
            Rating::General => "general",
        }
        .into())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "warning")]
pub enum Warning {
    #[postgres(name = "using")]
    #[serde(rename = "using")]
    Using,

    #[postgres(name = "none")]
    #[serde(rename = "none")]
    None,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Warning::Using => "orange-500",
                Warning::None => "gray-700",
            }
        )
    }
}

impl FromSql for Warning {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "using" => Warning::Using,
            "none" => Warning::None,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Warning {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Warning::Using => "using",
            Warning::None => "none",
        }
        .into())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(postgres_derive::FromSql, postgres_derive::ToSql)]
#[postgres(name = "state")]
pub enum State {
    #[postgres(name = "completed")]
    #[serde(rename = "completed")]
    Completed,

    #[postgres(name = "in-progress")]
    #[serde(rename = "in-progress")]
    InProgress,

    #[postgres(name = "hiatus")]
    #[serde(rename = "hiatus")]
    Hiatus,

    #[postgres(name = "abandoned")]
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Completed => "green-600",
                State::InProgress => "blue-500",
                State::Hiatus => "purple-500",
                State::Abandoned => "red-500",
            }
        )
    }
}

impl FromSql for State {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "completed" => State::Completed,
            "in-progress" => State::InProgress,
            "hiatus" => State::Hiatus,
            "abandoned" => State::Abandoned,
            _ => unreachable!(),
        })
    }
}

impl ToSql for State {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            State::Completed => "completed",
            State::InProgress => "in-progress",
            State::Hiatus => "hiatus",
            State::Abandoned => "abandoned",
        }
        .into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub struct Square {
    pub rating: Rating,
    pub warnings: Warning,
    pub state: State,
}
