use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Author (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryAuthor (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        AuthorId    TEXT    REFERENCES Author(Id)               ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Author {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Schema for Author {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
