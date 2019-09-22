use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    [Chapter] (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Raw         TEXT                                        NOT NULL,
        Words       INTEGER                                     NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryChapter (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        ChapterId   TEXT    REFERENCES Chapter(Id)              ON UPDATE CASCADE   NOT NULL,
        Place       INTEGER                                                         NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub raw: String,

    pub words: u32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Schema for Chapter {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
