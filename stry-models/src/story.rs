use {
    crate::{
        pairing::PairingBuilder, Author, Character, List, Origin, Pairing, Rating, Series, State,
        Tag, Warning,
    },
    anyhow::Context,
    chrono::{DateTime, TimeZone as _, Utc},
    std::fmt,
};

#[cfg(feature = "sqlite")]
use rewryte::sqlite::FromRow;

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

// #[juniper::graphql_object(Context = DataBackend)]
// impl Story {
//     pub fn id(&self) -> &str {
//         &self.id
//     }

//     pub fn name(&self) -> &str {
//         &self.name
//     }

//     pub fn summary(&self) -> &str {
//         &self.summary
//     }

//     // pub fn rating(&self) -> Rating {
//     //     self.square.rating
//     // }

//     // pub fn state(&self) -> State {
//     //     self.square.state
//     // }

//     pub fn chapters(&self) -> i32 {
//         self.chapters
//     }

//     pub fn words(&self) -> i32 {
//         self.words
//     }

//     pub fn warnings(&self) -> &[Warning] {
//         &self.warnings
//     }

//     pub fn pairings(&self) -> &[Pairing] {
//         &self.pairings
//     }

//     pub fn characters(&self) -> &[Character] {
//         &self.characters
//     }

//     pub fn tags(&self) -> &[Tag] {
//         &self.tags
//     }

//     pub fn created(&self) -> &DateTime<Utc> {
//         &self.created
//     }

//     pub fn updated(&self) -> &DateTime<Utc> {
//         &self.updated
//     }
// }

pub struct StoryRow {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,

    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub chapters: i32,
    pub words: i32,
}

#[cfg(feature = "sqlite")]
impl FromRow for StoryRow {
    fn from_row(row: &rewryte::sqlite::Row<'_>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for story (row)")?,

            name: row
                .get(1)
                .context("Attempting to get row index 1 for story (row)")?,

            created: row
                .get(2)
                .context("Attempting to get row index 2 for story (row)")?,
            updated: row
                .get(3)
                .context("Attempting to get row index 3 for story (row)")?,

            summary: row
                .get(4)
                .context("Attempting to get row index 4 for story (row)")?,

            rating: row
                .get(5)
                .context("Attempting to get row index 5 for story (row)")?,
            state: row
                .get(6)
                .context("Attempting to get row index 6 for story (row)")?,

            chapters: row
                .get(7)
                .context("Attempting to get row index 7 for story (row)")?,
            words: row
                .get(8)
                .context("Attempting to get row index 8 for story (row)")?,
        })
    }
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

// #[juniper::graphql_object(Context = DataBackend)]
// impl StoryList {
//     pub fn total(&self) -> i32 {
//         self.total
//     }

//     pub fn items(&self) -> &[Story] {
//         &self.items
//     }
// }

impl From<List<Story>> for StoryList {
    fn from(list: List<Story>) -> Self {
        StoryList {
            total: list.total,
            items: list.items,
        }
    }
}

// #[rustfmt::skip]
// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[derive(juniper::GraphQLEnum)]
// #[derive(postgres_types::ToSql, postgres_types::FromSql)]
// #[derive(serde::Deserialize, serde::Serialize)]
// #[postgres(name = "rating")]
// pub enum Rating {
//     #[serde(rename = "explicit")]
//     #[postgres(name = "explicit")]
//     Explicit,

//     #[serde(rename = "mature")]
//     #[postgres(name = "mature")]
//     Mature,

//     #[serde(rename = "teen")]
//     #[postgres(name = "teen")]
//     Teen,

//     #[serde(rename = "general")]
//     #[postgres(name = "general")]
//     General,
// }

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

// #[rustfmt::skip]
// #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[derive(juniper::GraphQLEnum)]
// #[derive(postgres_types::ToSql, postgres_types::FromSql)]
// #[derive(serde::Deserialize, serde::Serialize)]
// #[postgres(name = "state")]
// pub enum State {
//     #[serde(rename = "completed")]
//     #[postgres(name = "completed")]
//     Completed,

//     #[serde(rename = "in-progress")]
//     #[postgres(name = "in-progress")]
//     InProgress,

//     #[serde(rename = "hiatus")]
//     #[postgres(name = "hiatus")]
//     Hiatus,

//     #[serde(rename = "abandoned")]
//     #[postgres(name = "abandoned")]
//     Abandoned,
// }

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
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Square {
    pub rating: Rating,
    pub warnings: bool,
    pub state: State,
}

pub struct StoryBuilder {
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
}

impl StoryBuilder {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        summary: impl Into<String>,
        rating: Rating,
        state: State,
        chapters: i32,
        words: i32,
    ) -> Self {
        Self {
            id: id.into(),

            name: name.into(),
            summary: summary.into(),

            rating,
            state,

            chapters,
            words,

            authors: vec![],
            origins: vec![],

            warnings: vec![],
            pairings: vec![],
            characters: vec![],
            tags: vec![],
        }
    }

    pub fn finish(self) -> Story {
        Story {
            id: self.id,

            name: self.name,
            summary: self.summary,

            square: Square {
                rating: self.rating,
                state: self.state,
                warnings: !self.warnings.is_empty(),
            },

            chapters: self.chapters,
            words: self.words,

            authors: self.authors,
            origins: self.origins,

            warnings: self.warnings,
            pairings: self.pairings,
            characters: self.characters,
            tags: self.tags,

            series: None,

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }

    pub fn with_author(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.authors.push(Author::new_test(id, name));

        self
    }

    pub fn with_origin(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.origins.push(Origin::new_test(id, name));

        self
    }

    pub fn with_warning(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.warnings.push(Warning::new_test(id, name));

        self
    }

    pub fn with_pairing(
        mut self,
        id: impl Into<String>,
        platonic: bool,
        build: impl FnOnce(PairingBuilder) -> PairingBuilder,
    ) -> Self {
        let PairingBuilder {
            id,
            platonic,
            characters,
        } = build(PairingBuilder {
            id: id.into(),

            platonic,

            characters: vec![],
        });

        self.pairings.push(Pairing {
            id,

            platonic,

            characters,

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        });

        self
    }

    pub fn with_character(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.characters.push(Character::new_test(id, name));

        self
    }

    pub fn with_tag(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.tags.push(Tag::new_test(id, name));

        self
    }
}
