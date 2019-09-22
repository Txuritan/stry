use {
    crate::{
        models::{Author, Origin, Series, Tag},
        schema::Schema,
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Story (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Language    TEXT                                        NOT NULL,
        Rating      TEXT                                        NOT NULL,
        State       TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg(feature = "rusqlite")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result as RusqliteResult,
};

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: String,

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

impl Schema for Story {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Language {
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

#[cfg(feature = "rusqlite")]
impl FromSql for Language {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "english" => Language::English,
            _ => unreachable!(),
        })
    }
}

#[cfg(feature = "rusqlite")]
impl ToSql for Language {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Rating {
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "mature")]
    Mature,
    #[serde(rename = "teen")]
    Teen,
    #[serde(rename = "general")]
    General,
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rating::Explicit => "explicit",
                Rating::Mature => "mature",
                Rating::Teen => "teen",
                Rating::General => "general",
            }
        )
    }
}

#[cfg(feature = "rusqlite")]
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

#[cfg(feature = "rusqlite")]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum Warning {
    #[serde(rename = "using")]
    Using,
    #[serde(rename = "none")]
    None,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Warning::Using => "using",
                Warning::None => "none",
            }
        )
    }
}

#[cfg(feature = "rusqlite")]
impl FromSql for Warning {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "using" => Warning::Using,
            "none" => Warning::None,
            _ => unreachable!(),
        })
    }
}

#[cfg(feature = "rusqlite")]
impl ToSql for Warning {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Warning::Using => "using",
            Warning::None => "none",
        }
        .into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum State {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "hiatus")]
    Hiatus,
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Completed => "completed",
                State::InProgress => "in-progress",
                State::Hiatus => "hiatus",
                State::Abandoned => "abandoned",
            }
        )
    }
}

#[cfg(feature = "rusqlite")]
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

#[cfg(feature = "rusqlite")]
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
