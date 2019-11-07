pub mod api;
pub mod author;
pub mod chapter;
pub mod origin;
pub mod queue;
pub mod series;
pub mod story;
pub mod tag;

use {
    chrono::{DateTime, Utc},
    std::fmt,
};

pub use self::{
    author::Author,
    chapter::Chapter,
    origin::Origin,
    queue::Queue,
    series::Series,
    story::{Language, Rating, Square, State, Story, Warning},
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
