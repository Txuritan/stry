use {
    std::{borrow::Cow, sync::Arc},
    stry_common::{
        backend::{BackendType, StorageType},
        models::{
            Author, Chapter, Character, List, Origin, Pairing, Story, Tag, Warning, WorkerTask,
        },
        version::LibVersion,
    },
};

pub fn version() -> Vec<LibVersion> {
    let mut version = if cfg!(feature = "postgres") {
        stry_backend_postgres::version()
    } else {
        vec![]
    };

    if cfg!(feature = "sqlite") {
        version.append(&mut stry_backend_sqlite::version());
    }

    version
}

#[derive(Clone)]
pub struct DataBackend {
    inner: DataBackendInner,
    pub versions: Arc<Vec<LibVersion>>,
}

// impl juniper::Context for DataBackend {}

#[derive(Clone)]
enum DataBackendInner {
    #[cfg(feature = "postgres")]
    Postgres(stry_backend_postgres::PostgresBackend),
    #[cfg(feature = "sqlite")]
    Sqlite(stry_backend_sqlite::SqliteBackend),
}

impl DataBackend {
    #[tracing::instrument(skip(storage, versions), err)]
    pub async fn init(
        backend: BackendType,
        storage: StorageType,
        versions: Arc<Vec<LibVersion>>,
    ) -> anyhow::Result<Self> {
        match backend {
            #[cfg(feature = "postgres")]
            BackendType::Postgres => {
                let back = stry_backend_postgres::PostgresBackend::init(
                    backend,
                    storage,
                    versions.clone(),
                )
                .await?;

                Ok(DataBackend {
                    inner: DataBackendInner::Postgres(back),
                    versions,
                })
            }
            #[cfg(feature = "sqlite")]
            BackendType::Sqlite => {
                let back =
                    stry_backend_sqlite::SqliteBackend::init(backend, storage, versions.clone())
                        .await?;

                Ok(DataBackend {
                    inner: DataBackendInner::Sqlite(back),
                    versions,
                })
            }
        }
    }

    pub async fn all_authors(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Author>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_authors(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_authors(offset, limit).await,
        }
    }

    pub async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_author(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_author(id).await,
        }
    }

    pub async fn author_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.author_stories(id, offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.author_stories(id, offset, limit).await,
        }
    }

    pub async fn get_chapter(
        &self,
        story_id: Cow<'static, str>,
        chapter_number: i32,
    ) -> anyhow::Result<Option<Chapter>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => {
                backend.get_chapter(story_id, chapter_number).await
            }
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => {
                backend.get_chapter(story_id, chapter_number).await
            }
        }
    }

    pub async fn all_characters(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Character>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_characters(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_characters(offset, limit).await,
        }
    }

    pub async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_character(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_character(id).await,
        }
    }

    pub async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => {
                backend.character_stories(id, offset, limit).await
            }
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.character_stories(id, offset, limit).await,
        }
    }

    pub async fn all_origins(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Origin>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_origins(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_origins(offset, limit).await,
        }
    }

    pub async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_origin(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_origin(id).await,
        }
    }

    pub async fn origin_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.origin_stories(id, offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.origin_stories(id, offset, limit).await,
        }
    }

    pub async fn all_pairings(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Pairing>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_pairings(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_pairings(offset, limit).await,
        }
    }

    pub async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_pairing(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_pairing(id).await,
        }
    }

    pub async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.pairing_stories(id, offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.pairing_stories(id, offset, limit).await,
        }
    }

    pub async fn all_stories(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_stories(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_stories(offset, limit).await,
        }
    }

    pub async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_story(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_story(id).await,
        }
    }

    pub async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => {
                backend.search_stories(input, offset, limit).await
            }
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.search_stories(input, offset, limit).await,
        }
    }

    pub async fn all_tags(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Tag>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_tags(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_tags(offset, limit).await,
        }
    }

    pub async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_tag(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_tag(id).await,
        }
    }

    pub async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.tag_stories(id, offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.tag_stories(id, offset, limit).await,
        }
    }

    pub async fn all_warnings(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Warning>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.all_warnings(offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.all_warnings(offset, limit).await,
        }
    }

    pub async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_warning(id).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_warning(id).await,
        }
    }

    pub async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.warning_stories(id, offset, limit).await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.warning_stories(id, offset, limit).await,
        }
    }

    pub async fn get_new_task(&self) -> anyhow::Result<Option<WorkerTask>> {
        match &self.inner {
            #[cfg(feature = "postgres")]
            DataBackendInner::Postgres(backend) => backend.get_new_task().await,
            #[cfg(feature = "sqlite")]
            DataBackendInner::Sqlite(backend) => backend.get_new_task().await,
        }
    }
}
