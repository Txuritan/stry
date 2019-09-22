use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

#[cfg(feature = "rusqlite")]
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result as RusqliteResult,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Tag (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Type        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryTag (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        TagId       TEXT    REFERENCES Tag(Id)                  ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: String,

    pub name: String,
    #[serde(rename = "type")]
    pub typ: TagType,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Schema for Tag {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
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
                TagType::Warning => "warning",
                TagType::Pairing => "pairing",
                TagType::Character => "character",
                TagType::General => "general",
            }
        )
    }
}

#[cfg(feature = "rusqlite")]
impl FromSql for TagType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "warning" => TagType::Warning,
            "pairing" => TagType::Pairing,
            "character" => TagType::Character,
            "general" => TagType::General,
            _ => unreachable!(),
        })
    }
}

#[cfg(feature = "rusqlite")]
impl ToSql for TagType {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            TagType::Warning => "warning",
            TagType::Pairing => "pairing",
            TagType::Character => "character",
            TagType::General => "general",
        }
        .into())
    }
}