use {
    crate::models::{Author, Origin, Series, Tag},
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: String,

    pub name: String,
    pub summary: String,

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

pub struct StoryPart {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "types-postgres", derive(postgres_types::ToSql, postgres_types::FromSql))]
#[cfg_attr(feature = "types-postgres", postgres(name = "rating"))]
pub enum Rating {
    #[serde(rename = "explicit")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "explicit"))]
    Explicit,
    #[serde(rename = "mature")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "mature"))]
    Mature,
    #[serde(rename = "teen")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "teen"))]
    Teen,
    #[serde(rename = "general")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "general"))]
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

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::FromSql for Rating {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| match s.to_lowercase().as_str() {
                "explicit" => Ok(Rating::Explicit),
                "mature" => Ok(Rating::Mature),
                "teen" => Ok(Rating::Teen),
                "general" => Ok(Rating::General),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            })
    }
}

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::ToSql for Rating {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        match self {
            Rating::Explicit => Ok("explicit".into()),
            Rating::Mature => Ok("mature".into()),
            Rating::Teen => Ok("teen".into()),
            Rating::General => Ok("general".into()),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Warning {
    #[serde(rename = "using")]
    Using,
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
#[cfg_attr(feature = "types-postgres", derive(postgres_types::ToSql, postgres_types::FromSql))]
#[cfg_attr(feature = "types-postgres", postgres(name = "state"))]
pub enum State {
    #[serde(rename = "completed")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "completed"))]
    Completed,
    #[serde(rename = "in-progress")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "in-progress"))]
    InProgress,
    #[serde(rename = "hiatus")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "hiatus"))]
    Hiatus,
    #[serde(rename = "abandoned")]
    #[cfg_attr(feature = "types-postgres", postgres(name = "abandoned"))]
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

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::FromSql for State {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| match s.to_lowercase().as_str() {
                "completed" => Ok(State::Completed),
                "in-progress" => Ok(State::InProgress),
                "hiatus" => Ok(State::Hiatus),
                "abandoned" => Ok(State::Abandoned),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            })
    }
}

#[cfg(feature = "types-sqlite")]
impl rusqlite::types::ToSql for State {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput> {
        match self {
            State::Completed => Ok("completed".into()),
            State::InProgress => Ok("in-progress".into()),
            State::Hiatus => Ok("hiatus".into()),
            State::Abandoned => Ok("abandoned".into()),
        }
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
