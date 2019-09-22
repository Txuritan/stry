use {
    crate::{Error, Pool, Schema},
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Chapter {
    pub id: String,

    pub name: String,

    pub raw: String,

    pub words: u32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Chapter {
    pub fn of_story(pool: Pool, story: &str, place: u32) -> Result<Self, Error> {
        let conn = pool.get()?;

        let chapter = conn.query_row(
            "SELECT C.Id, C.Name, C.Raw, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = ? AND SC.Place = ?;",
            rusqlite::params![&story, &place], |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    raw: row.get("Raw")?,
                    words: row.get("Words")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                })
            }
        )?;

        Ok(chapter)
    }
}

impl Into<json::JsonValue> for Chapter {
    fn into(self) -> json::JsonValue {
        json::object! {
            "id" => self.id,

            "name" => self.name,

            "raw" => self.raw,
            "words" => self.words,

            "created" => self.created.to_rfc3339(),
            "updated" => self.updated.to_rfc3339(),
        }
    }
}

impl Schema for Chapter {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
