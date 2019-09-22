use {
    crate::{
        models::{author, origin, tag},
        Error, Pool,
    },
    chrono::{DateTime, Utc},
    common::models::{
        story::{Language, Rating, Square, State, Story, Warning},
        tag::TagType,
        Series,
    },
};

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

pub fn all(pool: Pool, page: u32) -> Result<(u32, Vec<Story>), Error> {
    let conn = pool.get()?;

    let mut stmt = conn.prepare(
            "SELECT Id, Name, Summary, Language, Rating, State, Created, Updated FROM Story ORDER BY Updated DESC LIMIT 10 OFFSET ?;",
        )?;

    let story_rows = stmt.query_map(rusqlite::params![10 * page], |row| {
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

        let authors = author::of_story(pool.clone(), &story.id)?;
        let origins = origin::of_story(pool.clone(), &story.id)?;
        let tags = tag::of_story(pool.clone(), &story.id)?;

        let warn = tags.iter().any(|t| t.typ == TagType::Warning);

        stories.push(Story {
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

    let count = conn.query_row(
        "SELECT COUNT(Id) as Count FROM Story;",
        rusqlite::NO_PARAMS,
        |row| row.get("Count"),
    )?;

    Ok((count, stories))
}

pub fn get(pool: Pool, id: &str) -> Result<Story, Error> {
    let conn = pool.get()?;

    let authors = author::of_story(pool.clone(), id)?;
    let origins = origin::of_story(pool.clone(), id)?;
    let tags = tag::of_story(pool.clone(), id)?;

    let warn = tags.iter().any(|t| t.typ == TagType::Warning);

    let story = conn.query_row(
            "SELECT Id, Name, Summary, Language, Rating, State, Created, Updated FROM Story WHERE Id = ?;",
            rusqlite::params![id],
            |row| -> rusqlite::Result<Story> {
                Ok(Story {
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
