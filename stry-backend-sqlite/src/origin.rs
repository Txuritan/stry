use {crate::SqlitePoolConnection, stry_common::BackendOrigin};

#[async_trait::async_trait]
impl BackendOrigin for SqlitePoolConnection {
    async fn all_origins(&mut self, offset: u32, limit: u32) {}
    async fn get_origin(&mut self, id: &str) {}
    async fn origin_stories(&mut self, id: &str, offset: u32, limit: u32) {}
}
