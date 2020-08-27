use {
    crate::{BackendType, StorageType},
    std::{borrow::Cow, sync::Arc},
    stry_common::{
        models::{
            Author, Chapter, Character, List, Origin, Pairing, Story, Tag, Warning, WorkerTask,
        },
        version::LibVersion,
    },
};

#[async_trait::async_trait]
pub trait Backend:
    BackendAuthor
    + BackendChapter
    + BackendCharacter
    + BackendOrigin
    + BackendPairing
    + BackendStory
    + BackendTag
    + BackendWarning
    + BackendWorker
    + Clone
{
    async fn init(
        backend: BackendType,
        storage: StorageType,
        versions: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self>;
}

#[async_trait::async_trait]
pub trait BackendAuthor {
    async fn all_authors(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Author>>>;
    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>>;
    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendChapter {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
    ) -> anyhow::Result<Option<Chapter>>;
}

#[async_trait::async_trait]
pub trait BackendCharacter {
    async fn all_characters(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Character>>>;
    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>>;
    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendOrigin {
    async fn all_origins(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Origin>>>;
    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>>;
    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendPairing {
    async fn all_pairings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Pairing>>>;
    async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>>;
    async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendStory {
    async fn all_stories(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Story>>>;
    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>>;
    async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendTag {
    async fn all_tags(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Tag>>>;
    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>>;
    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendWarning {
    async fn all_warnings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Warning>>>;
    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>>;
    async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>>;
}

#[async_trait::async_trait]
pub trait BackendWorker {
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>>;
}
