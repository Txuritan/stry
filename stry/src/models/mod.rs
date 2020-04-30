pub mod api;
pub mod author;
pub mod chapter;
pub mod notification;
pub mod origin;
pub mod series;
pub mod site;
pub mod story;
pub mod tag;

use {
    chrono::{DateTime, Utc},
    std::fmt,
    warp::{reject::not_found, Rejection},
};

pub use self::{
    author::Author,
    chapter::Chapter,
    notification::{Level, Notification},
    origin::Origin,
    series::Series,
    site::{MutexSite, Site},
    story::{Language, Rating, Square, State, Story, StoryRow, Warning},
    tag::{Tag, TagType},
};

pub trait Resource {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn created(&self) -> &DateTime<Utc>;
    fn updated(&self) -> &DateTime<Utc>;

    fn color(&self) -> (&str, &str);
}

pub trait Schema {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result;
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Paging {
    pub page: u32,
    pub page_size: u32,
}

impl Paging {
    pub fn normalize(self) -> Self {
        let mut norm = self;

        if norm.page == 0 {
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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

impl RouteType {
    pub fn parse(typ: &str) -> Result<Self, Rejection> {
        match typ {
            "authors" => Ok(RouteType::Authors),
            "characters" => Ok(RouteType::Characters),
            "origins" => Ok(RouteType::Origins),
            "pairings" => Ok(RouteType::Pairings),
            "tags" => Ok(RouteType::Tags),
            "warnings" => Ok(RouteType::Warnings),
            _ => Err(not_found()),
        }
    }
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
