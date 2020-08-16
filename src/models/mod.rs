pub mod generated;

pub mod api;
pub mod author;
pub mod chapter;
pub mod character;
pub mod notification;
pub mod origin;
pub mod pairing;
pub mod series;
pub mod site;
pub mod story;
pub mod tag;
pub mod warning;
pub mod worker;

use std::fmt;

pub use self::{
    generated::{
        Author, Chapter, Character, Origin, Pairing as PairingRow, Rating, State,
        Story as StoryRow, Tag, Warning, Worker as WorkerRow, WorkerSite, WorkerState, WorkerTask,
    },
    notification::{Level, Notification},
    pairing::Pairing,
    series::Series,
    site::Site,
    story::{Square, Story},
    worker::Worker,
};

pub trait Node {
    fn id(&self) -> &str;
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct List<T> {
    pub total: i32,
    pub items: Vec<T>,
}

impl<T> List<T> {
    pub fn into_parts(self) -> (i32, Vec<T>) {
        (self.total, self.items)
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entity {
    pub id: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Paging {
    pub page: i32,
    pub page_size: i32,
}

impl Paging {
    pub fn normalize(self) -> Self {
        let mut norm = self;

        if norm.page <= 0 {
            norm.page = 1;
        }

        norm.page -= 1;

        norm
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 15,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Search {
    pub search: String,
}

#[derive(Clone, Copy)]
pub enum RouteType {
    Authors,
    Characters,
    Origins,
    Pairings,
    Tags,
    Warnings,
}

impl fmt::Display for RouteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RouteType::Authors => "authors",
                RouteType::Characters => "characters",
                RouteType::Origins => "origins",
                RouteType::Pairings => "pairings",
                RouteType::Tags => "tags",
                RouteType::Warnings => "warnings",
            }
        )
    }
}
