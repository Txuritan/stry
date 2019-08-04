use {
    crate::{Error, Pool, Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    [Chapter] (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Raw         TEXT                                        NOT NULL,
        Rendered    TEXT                                        NOT NULL,
        Words       INTEGER                                     NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryChapter (
        StoryId     TEXT    REFERENCES Story(Id)                NOT NULL,
        ChapterId   TEXT    REFERENCES Chapter(Id)              NOT NULL,
        Place       INTEGER                                     NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub raw: String,
    pub rendered: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Chapter {
    pub fn story(pool: Pool, story: &str, place: u32) -> Result<Self, Error> {
        let conn = pool.get()?;

        let chapter = conn.query_row(
            "SELECT C.Id, C.Name, C.Raw, C.Rendered, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = ? AND SC.Place = ?;",
            rusqlite::params![&story, &place], |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    raw: row.get("Raw")?,
                    rendered: row.get("Rendered")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                })
            }
        )?;

        Ok(chapter)
    }
}

impl Schema for Chapter {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
