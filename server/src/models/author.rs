use {
    crate::{Error, Pool, Schema, Story},
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Author {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Author {
    pub fn all(pool: Pool, page: u32) -> Result<(u32, Vec<Self>), Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT 100 OFFSET ?;",
        )?;

        let author_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
            Ok(Author {
                id: row.get("Id")?,
                name: row.get("Name")?,
                created: row.get("Created")?,
                updated: row.get("Updated")?,
            })
        })?;

        let mut authors = Vec::new();

        for author in author_rows {
            authors.push(author?);
        }

        let count = conn.query_row(
            "SELECT COUNT(Id) as Count FROM Author;",
            rusqlite::NO_PARAMS,
            |row| row.get("Count"),
        )?;

        Ok((count, authors))
    }

    pub fn for_stories(pool: Pool, id: &str, page: u32) -> Result<(u32, Vec<Story>), Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare("SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT 10 OFFSET ?;")?;

        let story_rows = stmt.query_map(rusqlite::params![id, 10 * page], |row| {
            row.get::<_, String>("StoryId")
        })?;

        let mut stories = Vec::new();

        for story in story_rows {
            stories.push(Story::get(pool.clone(), &story?)?);
        }

        let count = conn.query_row("SELECT COUNT(SA.StoryId) as Count FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ?;", rusqlite::params![id], |row| row.get("Count"))?;

        Ok((count, stories))
    }

    pub fn of_story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
        )?;

        let authors =
            stmt.query_map(rusqlite::params![&story], |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                })
            })?;

        authors.map(|a| a.map_err(Error::from)).collect()
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/author/{}\">{}</a>", self.id, self.name)
    }
}

impl Into<json::JsonValue> for Author {
    fn into(self) -> json::JsonValue {
        json::object! {
            "id" => self.id,

            "name" => self.name,

            "created" => self.created.to_rfc3339(),
            "updated" => self.updated.to_rfc3339(),
        }
    }
}

impl Schema for Author {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
