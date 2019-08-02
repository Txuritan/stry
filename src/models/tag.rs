use {
    crate::{Error, Pool, Schema},
    chrono::{DateTime, Utc},
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        Result as RusqliteResult,
    },
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Tag (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Type        TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

const TABLE_BRIDGE: &str = "CREATE TABLE
IF NOT EXISTS
    StoryTag (
        StoryId     TEXT    REFERENCES Story(Id)                NOT NULL,
        TagId       TEXT    REFERENCES Tag(Id)                  NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TagType {
    Warning,
    Pairing,
    Character,
    General,
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TagType::Warning => "red",
                TagType::Pairing => "orange",
                TagType::Character => "purple",
                TagType::General => "black-light",
            }
        )
    }
}

impl FromSql for TagType {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "warning" => TagType::Warning,
            "paring" => TagType::Pairing,
            "character" => TagType::Character,
            "general" => TagType::General,
            _ => unreachable!(),
        })
    }
}

impl ToSql for TagType {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            TagType::Warning => "warning",
            TagType::Pairing => "paring",
            TagType::Character => "character",
            TagType::General => "general",
        }
        .into())
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Tag {
    pub id: String,

    pub name: String,
    pub typ: TagType,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Tag {
    pub fn story(pool: Pool, story: &str) -> Result<Vec<Self>, Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT T.Id, T.Name, T.Type, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = ? ORDER BY T.Name;"
        )?;

        let tag_rows = stmt.query_map(rusqlite::params![&story], |row| Self::row(row))?;

        let mut tags = Vec::new();

        for tag in tag_rows {
            tags.push(tag?);
        }

        tags.sort_by(|a, b| a.typ.cmp(&b.typ));

        Ok(tags)
    }

    fn row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("Id")?,
            name: row.get("Name")?,
            typ: row.get("Type")?,
            created: row.get("Created")?,
            updated: row.get("Updated")?,
        })
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<a label=\"{}\" href=\"/tag/{}\">{}</a>",
            self.typ, self.id, self.name
        )
    }
}

impl Schema for Tag {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result {
        writeln!(m, "{}", TABLE)?;
        writeln!(m, "{}", TABLE_BRIDGE)?;

        Ok(())
    }
}
