use {
    crate::{models::Resource, schema::Schema},
    chrono::{DateTime, Utc},
    std::fmt,
};

const SQLITE_TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Author (
        Id          TEXT    PRIMARY KEY                         NOT NULL    UNIQUE,
        Name        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const SQLITE_TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryAuthor (
        StoryId     TEXT    REFERENCES Story(Id)                ON UPDATE CASCADE   NOT NULL,
        AuthorId    TEXT    REFERENCES Author(Id)               ON UPDATE CASCADE   NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))                        NOT NULL
    );";

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(db_derive::Table)]
#[table(schema)]
pub struct Author {
    #[table(rename = "Id")]
    pub id: String,

    #[table(rename = "Name")]
    pub name: String,

    #[table(rename = "Created")]
    pub created: DateTime<Utc>,

    #[table(rename = "Updated")]
    pub updated: DateTime<Utc>,
}

impl Resource for Author {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    fn updated(&self) -> &DateTime<Utc> {
        &self.updated
    }

    fn color(&self) -> (&str, &str) {
        ("blue-700", "blue-500")
    }
}

impl Schema for Author {
    fn postgres_schema(_buff: &mut impl fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn sqlite_schema(buff: &mut impl fmt::Write) -> fmt::Result {
        writeln!(buff, "{}", SQLITE_TABLE)?;
        writeln!(buff, "{}", SQLITE_TABLE_BRIDGE)?;

        Ok(())
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<a href=\"/author/{}\">{}</a>", self.id, self.name)
    }
}
