use {
    crate::{Error, Pool, Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    [Author] (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryAuthor (
        StoryId     TEXT    REFERENCES Story(Id)                NOT NULL,
        AuthorId    TEXT    REFERENCES Author(Id)               NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Author {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Author {
    pub fn story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
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
        write!(
            f,
            "<a class=\"title\" href=\"/author/{}\">{}</a>",
            self.id, self.name
        )
    }
}

impl Schema for Author {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
