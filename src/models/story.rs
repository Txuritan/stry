use {
    crate::{
        models::{
            tag::{Tag, TagType},
            Author, Origin, Series,
        },
        Error, Pool, Schema,
    },
    chrono::{DateTime, Utc},
    rusqlite::{
        types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
        Result as RusqliteResult,
    },
    std::fmt,
};

const TABLE: &str = "CREATE TABLE
IF NOT EXISTS
    Story (
        Id          TEXT    PRIMARY KEY                         NOT NULL,
        Name        TEXT                                        NOT NULL,
        Summary     TEXT                                        NOT NULL,
        Language    TEXT                                        NOT NULL,
        Rating      TEXT                                        NOT NULL,
        State       TEXT                                        NOT NULL,
        Created     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL,
        Updated     TEXT    DEFAULT (DATETIME('now', 'utc'))    NOT NULL
    );";

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Story {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub language: Language,
    pub square: Square,

    pub chapters: u32,
    pub words: u32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,
    pub tags: Vec<Tag>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
struct StoryRow {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub language: Language,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,

    pub rating: Rating,
    pub state: State,
}

impl Story {
    pub fn all(pool: Pool) -> Result<Vec<Self>, Error> {
        let conn = pool.get()?;

        let mut stmt = conn.prepare(
            "SELECT Id, Name, Summary, Language, Rating, State, Created, Updated FROM Story ORDER BY Updated DESC;",
        )?;

        let story_rows = stmt.query_map(rusqlite::params![], |row| {
            Ok(StoryRow {
                id: row.get("Id")?,
                name: row.get("Name")?,
                summary: row.get("Summary")?,
                language: row.get("Language")?,
                created: row.get("Created")?,
                updated: row.get("Updated")?,
                rating: row.get("Rating")?,
                state: row.get("State")?,
                series: None,
            })
        })?;

        let mut stories = Vec::new();

        for story in story_rows {
            let story = story?;

            let authors = Author::story(pool.clone(), &story.id)?;
            let origins = Origin::story(pool.clone(), &story.id)?;
            let tags = Tag::story(pool.clone(), &story.id)?;

            let warn = tags.iter().any(|t| t.typ == TagType::Warning);

            stories.push(Self {
                name: story.name,
                summary: story.summary,
                language: story.language,
                chapters: conn.query_row(
                    "SELECT COUNT(StoryId) as Chapters FROM StoryChapter WHERE StoryId = ?;",
                    rusqlite::params![story.id],
                    |row| row.get("Chapters")
                )?,
                words: conn.query_row(
                    "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = ?;",
                    rusqlite::params![story.id],
                    |row| row.get("Words")
                )?,
                id: story.id,
                created: story.created,
                updated: story.updated,
                square: Square {
                    rating: story.rating,
                    warnings: if warn { Warning::Using } else { Warning::None },
                    state: story.state,
                },
                series: None,
                authors,
                origins,
                tags,
            });
        }

        Ok(stories)
    }

    pub fn get(pool: Pool, id: &str) -> Result<Self, Error> {
        let conn = pool.get()?;

        let authors = Author::story(pool.clone(), id)?;
        let origins = Origin::story(pool.clone(), id)?;
        let tags = Tag::story(pool.clone(), id)?;

        let warn = tags.iter().any(|t| t.typ == TagType::Warning);

        let story = conn.query_row(
            "SELECT Id, Name, Summary, Language, Rating, State, Created, Updated FROM Story WHERE Id = ?;",
            rusqlite::params![id],
            |row| -> rusqlite::Result<Self> {
                Ok(Self {
                    id: row.get("Id")?,
                    name: row.get("Name")?,
                    summary: row.get("Summary")?,
                    language: row.get("Language")?,
                    chapters: conn.query_row(
                        "SELECT COUNT(StoryId) as Chapters FROM StoryChapter WHERE StoryId = ?;",
                        rusqlite::params![id],
                        |row| row.get("Chapters")
                    )?,
                    words: conn.query_row(
                        "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = ?;",
                        rusqlite::params![id],
                        |row| row.get("Words")
                    )?,
                    created: row.get("Created")?,
                    updated: row.get("Updated")?,
                    square: Square {
                        rating: row.get("Rating")?,
                        warnings: if warn { Warning::Using } else { Warning::None },
                        state: row.get("State")?,
                    },
                    series: None,
                    authors,
                    origins,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Language {
    English,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::English => "english",
            }
        )
    }
}

impl FromSql for Language {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "english" => Language::English,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Language {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(self.to_string().into())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Rating {
    Explicit,
    Mature,
    Teen,
    General,
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rating::Explicit => "black",
                Rating::Mature => "red",
                Rating::Teen => "green-dark",
                Rating::General => "blue",
            }
        )
    }
}

impl FromSql for Rating {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "explicit" => Rating::Explicit,
            "mature" => Rating::Mature,
            "teen" => Rating::Teen,
            "general" => Rating::General,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Rating {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Rating::Explicit => "explicit",
            Rating::Mature => "mature",
            Rating::Teen => "teen",
            Rating::General => "general",
        }
        .into())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Warning {
    Using,
    None,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Warning::Using => "orange",
                Warning::None => "gray",
            }
        )
    }
}

impl FromSql for Warning {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "using" => Warning::Using,
            "none" => Warning::None,
            _ => unreachable!(),
        })
    }
}

impl ToSql for Warning {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            Warning::Using => "using",
            Warning::None => "none",
        }
        .into())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum State {
    Completed,
    InProgress,
    Hiatus,
    Abandoned,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Completed => "green-dark",
                State::InProgress => "blue",
                State::Hiatus => "purple",
                State::Abandoned => "red",
            }
        )
    }
}

impl FromSql for State {
    fn column_result(value: ValueRef) -> FromSqlResult<Self> {
        String::column_result(value).map(|as_str| match as_str.as_str() {
            "completed" => State::Completed,
            "in-progress" => State::InProgress,
            "hiatus" => State::Hiatus,
            "abandoned" => State::Abandoned,
            _ => unreachable!(),
        })
    }
}

impl ToSql for State {
    fn to_sql(&self) -> RusqliteResult<ToSqlOutput> {
        Ok(match self {
            State::Completed => "completed",
            State::InProgress => "in-progress",
            State::Hiatus => "hiatus",
            State::Abandoned => "abandoned",
        }
        .into())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Square {
    pub rating: Rating,
    pub warnings: Warning,
    pub state: State,
}
