use {
    crate::Error,
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        Result as RusqliteResult,
    },
    std::fmt,
};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TagType {
    #[serde(rename = "warning")]
    Warning,

    #[serde(rename = "pairing")]
    Pairing,

    #[serde(rename = "character")]
    Character,

    #[serde(rename = "general")]
    General,
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TagType::Warning => "red",
                TagType::Pairing => "orange",
                TagType::Character => "purple",
                TagType::General => "black-light",
            }
        )
    }
}

impl FromSql for TagType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "warning" => TagType::Warning,
            "paring" => TagType::Pairing,
            "character" => TagType::Character,
            "general" => TagType::General,
            _ => unreachable!(),
        })
    }
}

impl ToSql for TagType {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            TagType::Warning => "warning",
            TagType::Pairing => "paring",
            TagType::Character => "character",
            TagType::General => "general",
        }
        .into())
    }
}
