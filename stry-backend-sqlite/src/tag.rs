use {crate::SqlitePoolConnection, stry_common::BackendTag};

#[async_trait::async_trait]
impl BackendTag for SqlitePoolConnection {
    async fn all_tags(&mut self, offset: u32, limit: u32) {}
    async fn get_tag(&mut self, id: &str) {}
    async fn tag_stories(&mut self, id: &str, offset: u32, limit: u32) {}
}
