use crate::models::tag::TagType;

#[derive(serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Archiver {
    pub imports: Vec<Import>,
}

impl Archiver {
    pub fn read() -> Self {
        serde_json::from_slice(&std::fs::read("./import.json").expect("Import file is either missing or unreadable")[..]).expect("Import file is not valid JSON")
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
