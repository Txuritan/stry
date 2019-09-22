use crate::models::{Author, Chapter, Origin, Story, Tag};

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Wrapper<D> {
    pub status: String,
    pub code: u32,
    pub messages: Vec<String>,
    pub data: D,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WrapperDummy {}

impl<D> Wrapper<D> {
    pub fn ok(data: D) -> Self {
        Self {
            status: String::from("ok"),
            code: 200,
            messages: Vec::with_capacity(0),
            data,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == "ok"
    }

    pub fn is_error(&self) -> bool {
        self.status == "error"
    }
}

impl Wrapper<WrapperDummy> {
    pub fn err(code: u32, messages: Vec<String>) -> Self {
        Self {
            status: String::from("error"),
            code,
            messages,
            data: WrapperDummy {},
        }
    }
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SearchRequest {
    pub page: u32,
    pub search: String,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthorResponse {
    pub count: u32,
    pub pages: u32,
    pub authors: Vec<Author>,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChapterResponse {
    pub chapter: Chapter,
    pub story: Story,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct OriginResponse {
    pub count: u32,
    pub pages: u32,
    pub origins: Vec<Origin>,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct StoryResponse {
    pub count: u32,
    pub pages: u32,
    pub stories: Vec<Story>,
}

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TagResponse {
    pub count: u32,
    pub pages: u32,
    pub tags: Vec<Tag>,
}
