use {
    crate::{
        models::{Author, Origin, Series, Tag},
        schema::Schema,
    },
    chrono::{DateTime, Utc},
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

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Table)]
#[table(schema)]
pub struct StoryRow {
    #[table(rename = "Id")]
    pub id: String,

    #[table(rename = "Url")]
    pub url: String,

    #[table(rename = "Name")]
    pub name: String,

    #[table(rename = "Summary")]
    pub summary: String,

    #[table(rename = "Language")]
    pub language: Language,

    #[table(rename = "Rating")]
    pub rating: Rating,

    #[table(rename = "State")]
    pub state: State,

    #[table(rename = "Created")]
    pub created: DateTime<Utc>,

    #[table(rename = "Updated")]
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
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

impl Schema for Story {
    fn postgres_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", POSTGRES_TABLE)?;

        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;

        Ok(())
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum Language {
    #[kind(rename = "english")]
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

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum Rating {
    #[kind(rename = "explicit")]
    #[serde(rename = "explicit")]
    Explicit,

    #[kind(rename = "mature")]
    #[serde(rename = "mature")]
    Mature,

    #[kind(rename = "teen")]
    #[serde(rename = "teen")]
    Teen,

    #[kind(rename = "general")]
    #[serde(rename = "general")]
    General,
}

impl Rating {
    pub fn title(self) -> &'static str {
        match self {
            Rating::Explicit => "Explicit",
            Rating::Mature => "Mature",
            Rating::Teen => "Teen",
            Rating::General => "General",
        }
    }
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rating::Explicit => "background--red",
                Rating::Mature => "background--yellow",
                Rating::Teen => "background--green",
                Rating::General => "background--blue",
            }
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum Warning {
    #[kind(rename = "using")]
    #[serde(rename = "using")]
    Using,

    #[kind(rename = "none")]
    #[serde(rename = "none")]
    None,
}

impl Warning {
    pub fn title(self) -> &'static str {
        match self {
            Warning::Using => "Using",
            Warning::None => "None",
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Warning::Using => "background--red",
                Warning::None => "background--gray",
            }
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum State {
    #[kind(rename = "completed")]
    #[serde(rename = "completed")]
    Completed,

    #[kind(rename = "in-progress")]
    #[serde(rename = "in-progress")]
    InProgress,

    #[kind(rename = "hiatus")]
    #[serde(rename = "hiatus")]
    Hiatus,

    #[kind(rename = "abandoned")]
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl State {
    pub fn title(self) -> &'static str {
        match self {
            State::Completed => "Completed",
            State::InProgress => "In Progress",
            State::Hiatus => "Hiatus",
            State::Abandoned => "Abandoned",
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Completed => "background--green",
                State::InProgress => "background--blue",
                State::Hiatus => "background--purple",
                State::Abandoned => "background--red",
            }
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Square {
    pub rating: Rating,
    pub warnings: Warning,
    pub state: State,
}
