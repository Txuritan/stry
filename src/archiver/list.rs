use crate::{tag::TagType, Error};

#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Archiver {
    pub imports: Vec<Import>,
}

impl Archiver {
    pub fn read() -> Result<Self, Error> {
        log::info!("Reading import archive file");
        Ok(serde_json::from_slice(
            &std::fs::read("./import.json")?[..],
        )?)
    }
}

#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Import {
    pub id: String,
    pub site: Site,
    pub origins: Vec<String>,
    pub tags: Vec<Tag>,
}

#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Tag {
    pub name: String,
    #[serde(rename = "type")]
    pub tag_type: TagType,
}

#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Site {
    #[serde(rename = "archive-of-our-own")]
    ArchiveOfOurOwn,
    #[serde(rename = "fanfiction")]
    FanFiction,
}
