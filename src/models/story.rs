use {
    crate::{
        models::{Author, Tag},
        Error, Pool, Schema,
    },
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Story (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Story {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub chapters: i32,

    pub authors: Vec<Author>,
    pub tags: Vec<Tag>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Story {
    pub fn get(pool: Pool, id: &str) -> Result<Self, Error> {
        let conn = pool.get()?;

        let authors = Author::story(pool.clone(), id)?;
        let tags = Tag::story(pool.clone(), id)?;

        let story = conn.query_row(
            "SELECT Id, Name, Summary, (SELECT ChapterId FROM StoryChapter WHERE StoryId = ?) AS Chapters, Created, Updated FROM Story WHERE Id = ?;",
            rusqlite::params![],
            |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    summary: row.get("Summary")?,
                    chapters: row.get("Chapters")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                    authors,
                    tags,
                })
            }
        )?;

        Ok(story)
    }
}

impl Schema for Story {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;

        Ok(())
    }
}
