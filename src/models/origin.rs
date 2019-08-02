use {
    crate::{Error, Pool, Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Origin (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryOrigin (
        StoryId     TEXT    REFERENCES Story(Id)                NOT NULL,
        OriginId    TEXT    REFERENCES Origin(Id)               NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Origin {
    pub id: String,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Origin {
    pub fn story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = ? ORDER BY O.Name;"
        )?;

        let origins =
            stmt.query_map(rusqlite::params![&story], |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                })
            })?;

        origins.map(|a| a.map_err(Error::from)).collect()
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/origin/{}\">{}</a>", self.id, self.name)
    }
}

impl Schema for Origin {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
