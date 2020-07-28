use {
    crate::{
        backend::DataBackend,
        models::{Author, Character, List, Origin, Pairing, Series, Tag, Warning},
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub square: Square,

    pub chapters: i32,
    pub words: i32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,

    pub warnings: Vec<Warning>,
    pub pairings: Vec<Pairing>,
    pub characters: Vec<Character>,
    pub tags: Vec<Tag>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl Story {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn rating(&self) -> Rating {
        self.square.rating
    }

    pub fn state(&self) -> State {
        self.square.state
    }

    pub fn chapters(&self) -> i32 {
        self.chapters
    }

    pub fn words(&self) -> i32 {
        self.words
    }

    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    pub fn pairings(&self) -> &[Pairing] {
        &self.pairings
    }

    pub fn characters(&self) -> &[Character] {
        &self.characters
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
}

pub struct StoryRow {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

pub struct StoryPart {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub chapters: i32,
    pub words: i32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,

    pub warnings: Vec<Warning>,
    pub pairings: Vec<Pairing>,
    pub characters: Vec<Character>,
    pub tags: Vec<Tag>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

pub struct StoryList {
    pub total: i32,
    pub items: Vec<Story>,
}

#[juniper::graphql_object(Context = DataBackend)]
impl StoryList {
    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn items(&self) -> &[Story] {
        &self.items
    }
}

impl From<List<Story>> for StoryList {
    fn from(list: List<Story>) -> Self {
        StoryList {
            total: list.total,
            items: list.items,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(juniper::GraphQLEnum)]
#[derive(postgres_types::ToSql, postgres_types::FromSql)]
#[derive(serde::Deserialize, serde::Serialize)]
#[postgres(name = "rating")]
pub enum Rating {
    #[serde(rename = "explicit")]
    #[postgres(name = "explicit")]
    Explicit,

    #[serde(rename = "mature")]
    #[postgres(name = "mature")]
    Mature,

    #[serde(rename = "teen")]
    #[postgres(name = "teen")]
    Teen,

    #[serde(rename = "general")]
    #[postgres(name = "general")]
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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(juniper::GraphQLEnum)]
#[derive(postgres_types::ToSql, postgres_types::FromSql)]
#[derive(serde::Deserialize, serde::Serialize)]
#[postgres(name = "state")]
pub enum State {
    #[serde(rename = "completed")]
    #[postgres(name = "completed")]
    Completed,

    #[serde(rename = "in-progress")]
    #[postgres(name = "in-progress")]
    InProgress,

    #[serde(rename = "hiatus")]
    #[postgres(name = "hiatus")]
    Hiatus,

    #[serde(rename = "abandoned")]
    #[postgres(name = "abandoned")]
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

impl rusqlite::types::FromSql for State {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| match s.to_lowercase().as_str() {
                "completed" => Ok(State::Completed),
                "in-progress" => Ok(State::InProgress),
                "hiatus" => Ok(State::Hiatus),
                "abandoned" => Ok(State::Abandoned),
                invalid => {
                    tracing::debug!("Invalid state value: {}", invalid);
                    Err(rusqlite::types::FromSqlError::InvalidType)
                }
            })
    }
}

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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Square {
    pub rating: Rating,
    pub warnings: bool,
    pub state: State,
}
