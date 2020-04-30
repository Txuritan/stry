use crate::models::{Author, Chapter, Origin, Story, Tag};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Wrapper<D> {
    pub status: String,
    pub code: u32,
    pub messages: Vec<String>,
    pub data: D,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct WrapperDummy {}

impl<D> Wrapper<D> {
    // Holding onto form the json api
    #[allow(dead_code)]
    pub fn ok(data: D) -> Self {
        Self {
            status: String::from("ok"),
            code: 200,
            messages: Vec::with_capacity(0),
            data,
        }
    }
}

impl Wrapper<WrapperDummy> {
    // Holding onto form the json api
    #[allow(dead_code)]
    pub fn err(code: u32, messages: Vec<String>) -> Self {
        Self {
            status: String::from("error"),
            code,
            messages,
            data: WrapperDummy {},
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SearchRequest {
    pub page: u32,
    pub search: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthorResponse {
    pub count: u32,
    pub pages: u32,
    pub authors: Vec<Author>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ChapterResponse {
    pub chapter: Chapter,
    pub story: Story,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OriginResponse {
    pub count: u32,
    pub pages: u32,
    pub origins: Vec<Origin>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct StoryResponse {
    pub count: u32,
    pub pages: u32,
    pub stories: Vec<Story>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TagResponse {
    pub count: u32,
    pub pages: u32,
    pub tags: Vec<Tag>,
}
