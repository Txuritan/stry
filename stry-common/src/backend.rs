use {
    crate::models::{Author, Chapter, List, Origin, Story, Tag},
    std::borrow::Cow,
};

#[async_trait::async_trait]
pub trait Backend:
    BackendAuthor + BackendChapter + BackendOrigin + BackendStory + BackendTag
{
}

#[async_trait::async_trait]
pub trait BackendAuthor {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<List<Author>>;
    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Author>;
    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}

#[async_trait::async_trait]
pub trait BackendChapter {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter>;
}

#[async_trait::async_trait]
pub trait BackendOrigin {
    async fn all_origins(&self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>>;
    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Origin>;
    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}

#[async_trait::async_trait]
pub trait BackendStory {
    async fn all_stories(&self, offset: u32, limit: u32) -> anyhow::Result<List<Story>>;
    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Story>;
}

#[async_trait::async_trait]
pub trait BackendTag {
    async fn all_tags(&self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>>;
    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Tag>;
    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}
