use {crate::SqlitePoolConnection, stry_common::BackendStory};

#[async_trait::async_trait]
impl BackendStory for SqlitePoolConnection {
    async fn all_stories(&mut self, offset: u32, limit: u32) {}
    async fn get_story(&mut self, id: &str) {}
}
