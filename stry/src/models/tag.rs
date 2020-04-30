use {
    crate::{models::Resource, schema::Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Tag (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Type        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryTag (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        TagId       TEXT    REFERENCES Tag(Id)                  ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Table)]
#[table(schema)]
pub struct Tag {
    #[table(rename = "Id")]
    pub id: String,

    #[table(rename = "Name")]
    pub name: String,

    #[table(rename = "Type")]
    #[serde(rename = "type")]
    pub typ: TagType,

    #[table(rename = "Created")]
    pub created: DateTime<Utc>,

    #[table(rename = "Updated")]
    pub updated: DateTime<Utc>,
}

impl Resource for Tag {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    fn color(&self) -> (&str, &str) {
        (self.typ.str(), self.typ.str())
    }
}

impl Schema for Tag {
    fn postgres_schema(_buff: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;
        writeln!(buff, "{}", SQLITE_TABLE_BRIDGE)?;

        Ok(())
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<li><a class=\"label {}\" href=\"/tag/{}\">{}</a></li>",
            self.typ, self.id, self.name
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Kind)]
pub enum TagType {
    #[kind(rename = "warning")]
    #[serde(rename = "warning")]
    Warning,

    #[kind(rename = "pairing")]
    #[serde(rename = "pairing")]
    Pairing,

    #[kind(rename = "character")]
    #[serde(rename = "character")]
    Character,

    #[kind(rename = "general")]
    #[serde(rename = "general")]
    General,
}

impl TagType {
    fn str(self) -> &'static str {
        match self {
            TagType::Warning => "background--red",
            TagType::Pairing => "background--yellow",
            TagType::Character => "background--purple",
            TagType::General => "background--gray",
        }
    }
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
