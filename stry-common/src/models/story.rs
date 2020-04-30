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

    pub language: Language,
    pub square: Square,

    pub chapters: u32,
    pub read: u32,
    pub words: u32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,
    pub tags: Vec<Tag>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
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

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
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
