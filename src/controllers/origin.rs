use {
    crate::{
        controllers::{story, Counter, Id},
        models::{Origin, Story},
    },
    db_derive::{prelude::*, OptionalExtension, Pool},
    std::convert::TryFrom,
};

pub fn all(pool: &Pool, offset: u32, limit: u32) -> anyhow::Result<(u32, Vec<Origin>)> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT Id, Name, Created, Updated FROM Origin ORDER BY Name DESC LIMIT {limit} OFFSET {offset};"
    )]
    struct All {
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(Id) as Count FROM Origin;")]
    struct Count {}

    let origins = All {
        limit,
        offset: offset * limit,
    }
    .query_rows(pool)?;

    let count = u32::try_from(Count {}.query_row::<_, Counter>(pool).map(|c| c.count)?)?;

    Ok((count, origins))
}

pub fn get(pool: &Pool, id: &str) -> anyhow::Result<Origin> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id, Name, Created, Updated FROM Origin WHERE Id = {id};")]
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
        sql = "SELECT SO.StoryId AS Id FROM StoryOrigin SO LEFT JOIN Story S ON S.Id = SO.StoryId WHERE SO.OriginId = {id} ORDER BY S.Updated DESC LIMIT {limit} OFFSET {offset};"
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

pub fn of_story(pool: &Pool, story: &str) -> anyhow::Result<Vec<Origin>> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = {story} ORDER BY O.Name;"
    )]
    struct OfStory<'q> {
        story: &'q str,
    }

    OfStory { story }.query_rows(pool).map_err(Into::into)
}

pub fn find_or_create(pool: &Pool, name: &str) -> anyhow::Result<String> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id FROM Origin WHERE Name = {name};")]
    struct Find<'q> {
        name: &'q str,
    }

    #[derive(db_derive::Execute)]
    #[execute(sql = "INSERT INTO Origin(Id, Name) VALUES ({id}, {name});")]
    struct Create<'q> {
        id: &'q str,
        name: &'q str,
    }

    let origin = Find { name }.query_row::<_, Id>(pool).optional()?;

    if let Some(origin) = origin {
        Ok(origin.id)
    } else {
        let id = crate::nanoid();

        pool.transaction(|trans| {
            Create {
                id: &id,
                name: &name,
            }
            .execute(trans)?;

            Ok(())
        })?;

        Ok(id)
    }
}
