use {
    crate::{
        controllers::{story, Counter, Id},
        models::{Story, Tag, TagType},
    },
    db_derive::{prelude::*, OptionalExtension, Pool},
    std::convert::TryFrom,
};

pub fn all(pool: &Pool, offset: u32, limit: u32) -> anyhow::Result<(u32, Vec<Tag>)> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT Id, Name, Type, Created, Updated FROM Tag ORDER BY Name LIMIT {limit} OFFSET {offset};"
    )]
    struct All {
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(Id) as Count FROM Tag;")]
    struct Count {}

    let origins = All {
        limit,
        offset: offset * limit,
    }
    .query_rows(pool)?;

    let count = u32::try_from(Count {}.query_row::<_, Counter>(pool).map(|c| c.count)?)?;

    Ok((count, origins))
}

pub fn all_of_type(
    pool: &Pool,
    typ: TagType,
    offset: u32,
    limit: u32,
) -> anyhow::Result<(u32, Vec<Tag>)> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT Id, Name, Type, Created, Updated FROM Tag WHERE Type = {typ} ORDER BY Name LIMIT {limit} OFFSET {offset};"
    )]
    struct AllOfType {
        typ: TagType,
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(sql = "SELECT COUNT(Id) as Count FROM Tag WHERE Type = {typ};")]
    struct Count {
        typ: TagType,
    }

    let tags = AllOfType {
        typ,
        limit,
        offset: offset * limit,
    }
    .query_rows(pool)?;

    let count = u32::try_from(
        Count { typ }
            .query_row::<_, Counter>(pool)
            .map(|c| c.count)?,
    )?;

    Ok((count, tags))
}

pub fn get(pool: &Pool, id: &str) -> anyhow::Result<Tag> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id, Name, Type, Created, Updated FROM Tag WHERE Id = {id};")]
    struct Get<'q> {
        id: &'q str,
    }

    Get { id }.query_row(pool).map_err(Into::into)
}

pub fn of_story(pool: &Pool, story: &str) -> anyhow::Result<Vec<Tag>> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT T.Id, T.Name, T.Type, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = {story} ORDER BY T.Name;"
    )]
    struct OfStory<'q> {
        story: &'q str,
    }

    let mut tags = OfStory { story }.query_rows::<_, Tag>(pool)?;

    tags.sort_by(|a, b| a.typ.cmp(&b.typ));

    Ok(tags)
}

pub fn stories(
    pool: &Pool,
    id: &str,
    offset: u32,
    limit: u32,
) -> anyhow::Result<(u32, Vec<Story>)> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT ST.StoryId AS Id FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = {id} ORDER BY S.Updated DESC LIMIT {limit} OFFSET {offset};"
    )]
    struct Stories<'q> {
        id: &'q str,
        limit: u32,
        offset: u32,
    }

    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT COUNT(ST.StoryId) as Id FROM StoryTag ST LEFT JOIN Story S ON S.Id = StoryId WHERE ST.TagId = {id};"
    )]
    struct Count<'q> {
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
        Count { id }
            .query_row::<_, Counter>(pool)
            .map(|c| c.count)?,
    )?;

    Ok((count, stories))
}

pub fn find_or_create(pool: &Pool, name: &str, typ: TagType) -> anyhow::Result<String> {
    #[derive(db_derive::Query)]
    #[query(sql = "SELECT Id FROM Tag WHERE Name = {name} AND Type = {typ};")]
    struct Find<'q> {
        name: &'q str,
        typ: TagType,
    }

    #[derive(db_derive::Execute)]
    #[execute(sql = "INSERT INTO Tag(Id, Name, Type) VALUES ({id}, {name}, {typ});")]
    struct Create<'q> {
        id: &'q str,
        name: &'q str,
        typ: TagType,
    }

    let tag = Find { name, typ }.query_row::<_, Id>(pool).optional()?;

    if let Some(tag) = tag {
        Ok(tag.id)
    } else {
        let id = crate::nanoid();

        pool.transaction(|trans| {
            Create {
                id: &id,
                name: &name,
                typ,
            }
            .execute(trans)?;

            Ok(())
        })?;

        Ok(id)
    }
}
