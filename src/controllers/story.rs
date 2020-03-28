use {
    crate::{
        controllers::{author, origin, tag, Counter, Id},
        models::{Square, Story, StoryRow, TagType, Warning},
    },
    db_derive::{prelude::*, Pool},
    std::convert::TryFrom,
};

pub fn all(pool: &Pool, offset: u32, limit: u32) -> anyhow::Result<(u32, Vec<Story>)> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id FROM Story ORDER BY Updated DESC LIMIT {limit} OFFSET {offset};")]
    struct All {
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(Id) as Count FROM Story;")]
    struct Count {}

    let stories = All {
        limit,
        offset: offset * limit,
    }
    .query_rows::<_, Id>(pool)?
    .into_iter()
    .map(|s| get(pool, &s.id))
    .collect::<Result<Vec<_>, _>>()?;

    let count = u32::try_from(Count {}.query_row::<_, Counter>(pool).map(|c| c.count)?)?;

    Ok((count, stories))
}

pub fn get(pool: &Pool, id: &str) -> anyhow::Result<Story> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT Id, Url, Name, Summary, Language, Rating, State, Created, Updated FROM Story WHERE Id = {id};"
    )]
    struct Get<'q> {
        id: &'q str,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(StoryId) as Count FROM StoryChapter WHERE StoryId = {id};")]
    struct Chapters<'q> {
        id: &'q str,
    }

    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = {id};"
    )]
    struct Words<'q> {
        id: &'q str,
    }

    #[derive(Debug, db_derive::Table)]
    #[table(exists, schema)]
    pub struct WordCount {
        #[table(rename = "Words")]
        words: i64,
    }

    let row = Get { id }.query_row::<_, StoryRow>(pool)?;

    let chapters = u32::try_from(
        Chapters { id }
            .query_row::<_, Counter>(pool)
            .map(|c| c.count)?,
    )?;

    let words = u32::try_from(
        Words { id }
            .query_row::<_, WordCount>(pool)
            .map(|c| c.words)?,
    )?;

    let authors = author::of_story(pool, id)?;
    let origins = origin::of_story(pool, id)?;
    let tags = tag::of_story(pool, id)?;

    let warn = tags.iter().any(|t| t.typ == TagType::Warning);

    let story = Story {
        id: row.id,
        url: row.url,
        name: row.name,
        summary: row.summary,
        language: row.language,
        square: Square {
            rating: row.rating,
            warnings: if warn { Warning::Using } else { Warning::None },
            state: row.state,
        },
        chapters,
        words,
        authors,
        origins,
        tags,
        series: None,
        created: row.created,
        updated: row.updated,
    };

    Ok(story)
}
