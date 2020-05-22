use {
    std::borrow::Cow,
    stry_backend_postgres::PostgresBackend,
    stry_backend_sqlite::SqliteBackend,
    stry_common::{
        models::{Author, Chapter, List, Origin, Story, Tag},
        Backend, BackendAuthor, BackendChapter, BackendOrigin, BackendStory, BackendTag,
    },
};

#[derive(Clone)]
pub enum DataBackend {
    Postgres(PostgresBackend),
    Sqlite(SqliteBackend),
}

impl Backend for DataBackend {}

#[async_trait::async_trait]
impl BackendAuthor for DataBackend {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<List<Author>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_authors(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_authors(offset, limit).await,
        }
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Author> {
        match self {
            DataBackend::Postgres(backend) => backend.get_author(id).await,
            DataBackend::Sqlite(backend) => backend.get_author(id).await,
        }
    }

    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        match self {
            DataBackend::Postgres(backend) => backend.author_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.author_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendChapter for DataBackend {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter> {
        match self {
            DataBackend::Postgres(backend) => backend.get_chapter(story_id, chapter_number).await,
            DataBackend::Sqlite(backend) => backend.get_chapter(story_id, chapter_number).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendOrigin for DataBackend {
    async fn all_origins(&self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_origins(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_origins(offset, limit).await,
        }
    }

    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Origin> {
        match self {
            DataBackend::Postgres(backend) => backend.get_origin(id).await,
            DataBackend::Sqlite(backend) => backend.get_origin(id).await,
        }
    }

    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        match self {
            DataBackend::Postgres(backend) => backend.origin_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.origin_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendStory for DataBackend {
    async fn all_stories(&self, offset: u32, limit: u32) -> anyhow::Result<List<Story>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_stories(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_stories(offset, limit).await,
        }
    }

    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Story> {
        match self {
            DataBackend::Postgres(backend) => backend.get_story(id).await,
            DataBackend::Sqlite(backend) => backend.get_story(id).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendTag for DataBackend {
    async fn all_tags(&self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_tags(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_tags(offset, limit).await,
        }
    }

    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Tag> {
        match self {
            DataBackend::Postgres(backend) => backend.get_tag(id).await,
            DataBackend::Sqlite(backend) => backend.get_tag(id).await,
        }
    }

    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        match self {
            DataBackend::Postgres(backend) => backend.tag_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.tag_stories(id, offset, limit).await,
        }
    }
}
