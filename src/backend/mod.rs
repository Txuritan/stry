pub mod postgres;
pub mod sqlite;

pub mod nanoid;
pub mod spec;

#[cfg(test)]
pub mod test_helpers;

use {
    crate::{
        backend::{postgres::PostgresBackend, sqlite::SqliteBackend},
        models::{
            Author, Chapter, Character, List, Origin, Pairing, Story, Tag, Warning, WorkerTask,
        },
        version::LibVersion,
    },
    fenn::VecExt,
    std::{borrow::Cow, collections::HashMap, sync::Arc},
};

pub use self::spec::{
    Backend, BackendAuthor, BackendChapter, BackendCharacter, BackendOrigin, BackendPairing,
    BackendStory, BackendTag, BackendWarning, BackendWorker,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum StorageType {
    File {
        location: String,
    },
    Parts {
        username: Option<String>,
        password: Option<String>,
        host: String,
        port: Option<String>,
        database: Option<String>,
        params: Option<HashMap<String, String>>,
    },
}

impl StorageType {
    pub fn is_file(&self) -> bool {
        match self {
            StorageType::File { .. } => true,
            _ => false,
        }
    }

    pub fn is_parts(&self) -> bool {
        match self {
            StorageType::Parts { .. } => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum BackendType {
    Postgres,
    Sqlite,
}

#[derive(Clone)]
pub struct DataBackend {
    inner: DataBackendInner,
    pub versions: Arc<Vec<LibVersion>>,
}

impl juniper::Context for DataBackend {}

#[derive(Clone)]
enum DataBackendInner {
    Postgres(PostgresBackend),
    Sqlite(SqliteBackend),
}

#[async_trait::async_trait]
impl Backend for DataBackend {
    #[tracing::instrument(skip(storage, versions))]
    async fn init(
        backend: BackendType,
        storage: StorageType,
        versions: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        match backend {
            BackendType::Postgres => {
                let back = PostgresBackend::init(backend, storage, versions.clone()).await?;

                Ok(DataBackend {
                    inner: DataBackendInner::Postgres(back),
                    versions,
                })
            }
            BackendType::Sqlite => {
                let back = SqliteBackend::init(backend, storage, versions.clone()).await?;

                Ok(DataBackend {
                    inner: DataBackendInner::Sqlite(back),
                    versions,
                })
            }
        }
    }
}

#[async_trait::async_trait]
impl BackendAuthor for DataBackend {
    async fn all_authors(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Author>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_authors(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_authors(offset, limit).await,
        }
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_author(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_author(id).await,
        }
    }

    async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.author_stories(id, offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.author_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendChapter for DataBackend {
    async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
    ) -> anyhow::Result<Option<Chapter>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => {
                backend.get_chapter(story_id, chapter_number).await
            }
            DataBackendInner::Sqlite(backend) => {
                backend.get_chapter(story_id, chapter_number).await
            }
        }
    }
}

#[async_trait::async_trait]
impl BackendCharacter for DataBackend {
    async fn all_characters(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Character>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_characters(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_characters(offset, limit).await,
        }
    }

    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_character(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_character(id).await,
        }
    }

    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => {
                backend.character_stories(id, offset, limit).await
            }
            DataBackendInner::Sqlite(backend) => backend.character_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendOrigin for DataBackend {
    async fn all_origins(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Origin>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_origins(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_origins(offset, limit).await,
        }
    }

    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_origin(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_origin(id).await,
        }
    }

    async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.origin_stories(id, offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.origin_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendPairing for DataBackend {
    async fn all_pairings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Pairing>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_pairings(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_pairings(offset, limit).await,
        }
    }

    async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_pairing(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_pairing(id).await,
        }
    }

    async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.pairing_stories(id, offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.pairing_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendStory for DataBackend {
    async fn all_stories(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_stories(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_stories(offset, limit).await,
        }
    }

    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_story(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_story(id).await,
        }
    }

    async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => {
                backend.search_stories(input, offset, limit).await
            }
            DataBackendInner::Sqlite(backend) => backend.search_stories(input, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendTag for DataBackend {
    async fn all_tags(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Tag>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_tags(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_tags(offset, limit).await,
        }
    }

    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_tag(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_tag(id).await,
        }
    }

    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.tag_stories(id, offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.tag_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendWarning for DataBackend {
    async fn all_warnings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Warning>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.all_warnings(offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.all_warnings(offset, limit).await,
        }
    }

    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_warning(id).await,
            DataBackendInner::Sqlite(backend) => backend.get_warning(id).await,
        }
    }

    async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.warning_stories(id, offset, limit).await,
            DataBackendInner::Sqlite(backend) => backend.warning_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendWorker for DataBackend {
    async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        match &self.inner {
            DataBackendInner::Postgres(backend) => backend.get_new_task().await,
            DataBackendInner::Sqlite(backend) => backend.get_new_task().await,
        }
    }
}

pub fn version() -> Vec<LibVersion> {
    postgres::version().appended(&mut sqlite::version())
}
