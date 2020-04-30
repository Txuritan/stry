use {crate::SqlitePoolConnection, stry_common::BackendChapter};

#[async_trait::async_trait]
impl BackendChapter for SqlitePoolConnection {
    async fn get_chapter(&mut self, story_id: &str, chapter_number: u32) {}
}
