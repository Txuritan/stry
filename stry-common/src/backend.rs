use crate::models::{Author, List};

#[async_trait::async_trait]
pub trait Backend {
    type Connection: BackendConnection;

    async fn conn(&self) -> anyhow::Result<Self::Connection>;
}

pub trait BackendConnection:
    BackendAuthor + BackendChapter + BackendOrigin + BackendStory + BackendTag
{
}

#[async_trait::async_trait]
pub trait BackendAuthor {
    async fn all_authors(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Author>>;
    async fn get_author(&mut self, id: &str);
    async fn author_stories(&mut self, id: &str, offset: u32, limit: u32);
}

#[async_trait::async_trait]
pub trait BackendChapter {
    async fn get_chapter(&mut self, story_id: &str, chapter_number: u32);
}

#[async_trait::async_trait]
pub trait BackendOrigin {
    async fn all_origins(&mut self, offset: u32, limit: u32);
    async fn get_origin(&mut self, id: &str);
    async fn origin_stories(&mut self, id: &str, offset: u32, limit: u32);
}

#[async_trait::async_trait]
pub trait BackendStory {
    async fn all_stories(&mut self, offset: u32, limit: u32);
    async fn get_story(&mut self, id: &str);
}

#[async_trait::async_trait]
pub trait BackendTag {
    async fn all_tags(&mut self, offset: u32, limit: u32);
    async fn get_tag(&mut self, id: &str);
    async fn tag_stories(&mut self, id: &str, offset: u32, limit: u32);
}
