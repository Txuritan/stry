use {
    crate::SqlitePoolConnection,
    stry_common::{
        models::{Author, List},
        BackendAuthor,
    },
};

#[async_trait::async_trait]
impl BackendAuthor for SqlitePoolConnection {
    async fn all_authors(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Author>> {
        // let query = sqlx::query!(
        //     "SELECT Id, Name, Created, Updated FROM Author ORDER BY Name DESC LIMIT ? OFFSET ?;",
        //     limit,
        //     offset
        // );

        todo!()
    }

    async fn get_author(&mut self, id: &str) {}

    async fn author_stories(&mut self, id: &str, offset: u32, limit: u32) {}
}
