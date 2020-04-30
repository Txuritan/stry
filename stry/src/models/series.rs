use {
    crate::schema::Schema,
    chrono::{DateTime, Utc},
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Series (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StorySeries (
        StoryId     TEXT        REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        SeriesId    TEXT        REFERENCES Series(Id)               ON UPDATE CASCADE   NOT NULL,
        Place       INTEGER     REFERENCES Series(Id)                                   NOT NULL,
        Created     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT        DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub id: String,

    pub name: String,

    pub summary: String,

    pub place: i32,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

// impl Series {
//     #[deprecated]
//     pub fn story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
//         let conn = pool.get()?;
//
//         let mut stmt = conn.prepare(
//             "SELECT A.Id, A.Name, A.Summary, A.Created, A.Updated FROM StorySeries SA LEFT JOIN Series A ON SA.SeriesId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;"
//         )?;
//
//         let series = stmt.query_map(params!(s => [&story]), |row| {
//             Ok(Self {
//                 id: row.get("Id")?,
//                 name: row.get("Name")?,
//                 summary: row.get("Summary")?,
//                 created: row.get("Created")?,
//                 updated: row.get("Updated")?,
//                 place: None,
//             })
//         })?;
//
//         series.map(|a| a.map_err(Error::from)).collect()
//     }
// }

impl Schema for Series {
    fn postgres_schema(_buff: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;
        writeln!(buff, "{}", SQLITE_TABLE_BRIDGE)?;

        Ok(())
    }
}
