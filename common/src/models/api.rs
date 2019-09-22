use crate::models::{Author, Chapter, Origin, Story, Tag};

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SearchRequest {
    pub page: u32,
    pub search: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Wrapper<D> {
    pub data: D,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct AuthorResponse {
    pub count: u32,
    pub pages: u32,
    pub authors: Vec<Author>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ChapterResponse {
    pub chapter: Chapter,
    pub story: Story,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct OriginResponse {
    pub count: u32,
    pub pages: u32,
    pub origins: Vec<Origin>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StoryResponse {
    pub count: u32,
    pub pages: u32,
    pub stories: Vec<Story>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TagResponse {
    pub count: u32,
    pub pages: u32,
    pub tags: Vec<Tag>,
}
