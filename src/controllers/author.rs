use {
    crate::{
        controllers::{story, Counter, Id},
        models::{Author, Story},
    },
    db_derive::{prelude::*, Pool},
    std::convert::TryFrom,
};

pub fn all(pool: &Pool, offset: u32, limit: u32) -> anyhow::Result<(u32, Vec<Author>)> {
    #[derive(Debug, db_derive::Query)]
    #[query(
        sql = "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT {limit} OFFSET {offset};"
    )]
    struct All {
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(Id) as Count FROM Author;")]
    struct Count {}

    let authors = All {
        limit,
        offset: offset * limit,
    }
    .query_rows(pool)?;

    let count = u32::try_from(Count {}.query_row::<_, Counter>(pool).map(|c| c.count)?)?;

    Ok((count, authors))
}

pub fn get(pool: &Pool, id: &str) -> anyhow::Result<Author> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id, Name, Created, Updated FROM Author WHERE Id = {id};")]
    struct Get<'q> {
        id: &'q str,
    }

    Get { id }.query_row(pool).map_err(Into::into)
}

pub fn stories(
    pool: &Pool,
    id: &str,
    offset: u32,
    limit: u32,
) -> anyhow::Result<(u32, Vec<Story>)> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = {id} ORDER BY S.Updated DESC LIMIT {limit} OFFSET {offset};"
    )]
    struct Stories<'q> {
        id: &'q str,
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT COUNT(SA.StoryId) as Id FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = {id};"
    )]
    struct StoryCount<'q> {
        id: &'q str,
    }

    let stories = Stories {
        id,
        limit,
        offset: offset * limit,
    }
    .query_rows::<_, Id>(pool)?
    .into_iter()
    .map(|s| story::get(pool, &s.id))
    .collect::<Result<Vec<_>, _>>()?;

    let count = u32::try_from(
        StoryCount { id }
            .query_row::<_, Counter>(pool)
            .map(|c| c.count)?,
    )?;

    Ok((count, stories))
}

pub fn of_story(pool: &Pool, story: &str) -> anyhow::Result<Vec<Author>> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = {story} ORDER BY A.Name;"
    )]
    struct OfStory<'q> {
        story: &'q str,
    }

    OfStory { story }.query_rows(pool).map_err(Into::into)
}
