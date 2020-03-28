use {
    crate::models::Chapter,
    db_derive::{prelude::*, Pool},
};

pub fn get(pool: &Pool, story: &str, number: u32) -> anyhow::Result<Chapter> {
    #[derive(db_derive::Query)]
    #[query(
        sql = "SELECT C.Id, C.Name, C.Pre, C.Main, C.Post, C.Words, C.Created, C.Updated FROM StoryChapter SC LEFT JOIN Chapter C ON SC.ChapterId = C.Id WHERE SC.StoryId = {story} AND SC.Place = {number};"
    )]
    struct Get<'q> {
        story: &'q str,
        number: u32,
    }

    Get { story, number }.query_row(pool).map_err(Into::into)
}
