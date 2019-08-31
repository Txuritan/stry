use {
    crate::{Error, Pool, Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Series (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StorySeries (
        StoryId     TEXT        REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        SeriesId    TEXT        REFERENCES Series(Id)               ON UPDATE CASCADE   NOT NULL,
        Place       INTEGER     REFERENCES Series(Id)                                   NOT NULL,
        Created     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Series {
    pub id: String,

    pub name: String,

    pub summary: String,

    pub place: Option<i32>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Series {
    pub fn story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT A.Id, A.Name, A.Summary, A.Created, A.Updated FROM StorySeries SA LEFT JOIN Series A ON SA.SeriesId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
        )?;

        let series =
            stmt.query_map(rusqlite::params![&story], |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    summary: row.get("Summary")?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                    place: None,
                })
            })?;

        series.map(|a| a.map_err(Error::from)).collect()
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<a class=\"title\" href=\"/series/{}\">{}</a>",
            self.id, self.name
        )
    }
}

impl Into<json::JsonValue> for Series {
    fn into(self) -> json::JsonValue {
        json::object! {
            "id" => self.id,

            "name" => self.name,

            "summary" => self.summary,

            "place" => self.place,

            "created" => self.created.to_rfc3339(),
            "updated" => self.updated.to_rfc3339(),
        }
    }
}

impl Schema for Series {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
