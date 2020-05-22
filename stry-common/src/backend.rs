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
    async fn all_authors(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Author>>;
    async fn get_author(&mut self, id: Cow<'static, str>) -> anyhow::Result<Author>;
    async fn author_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}

#[async_trait::async_trait]
pub trait BackendChapter {
    async fn get_chapter(
        &mut self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter>;
}

#[async_trait::async_trait]
pub trait BackendOrigin {
    async fn all_origins(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>>;
    async fn get_origin(&mut self, id: Cow<'static, str>) -> anyhow::Result<Origin>;
    async fn origin_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}

#[async_trait::async_trait]
pub trait BackendStory {
    async fn all_stories(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Story>>;
    async fn get_story(&mut self, id: Cow<'static, str>) -> anyhow::Result<Story>;
}

#[async_trait::async_trait]
pub trait BackendTag {
    async fn all_tags(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>>;
    async fn get_tag(&mut self, id: Cow<'static, str>) -> anyhow::Result<Tag>;
    async fn tag_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>>;
}
