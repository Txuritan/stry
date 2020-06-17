use {
    std::borrow::Cow,
    stry_backend_postgres::PostgresBackend,
    stry_backend_sqlite::SqliteBackend,
    stry_common::{
        backend::{
            Backend, BackendAuthor, BackendChapter, BackendCharacter, BackendOrigin,
            BackendPairing, BackendStory, BackendTag, BackendType, BackendWarning, StorageType,
        },
        models::{Author, Chapter, Character, List, Origin, Pairing, Story, Tag, Warning},
    },
};

#[derive(Clone)]
pub enum DataBackend {
    Postgres(PostgresBackend),
    Sqlite(SqliteBackend),
}

#[async_trait::async_trait]
impl Backend for DataBackend {
    async fn init(backend: BackendType, storage: StorageType) -> anyhow::Result<Self> {
        match backend {
            BackendType::Postgres => {
                let back = PostgresBackend::init(backend, storage).await?;

                Ok(DataBackend::Postgres(back))
            }
            BackendType::Sqlite => {
                let back = SqliteBackend::init(backend, storage).await?;

                Ok(DataBackend::Sqlite(back))
            }
        }
    }
}

#[async_trait::async_trait]
impl BackendAuthor for DataBackend {
    async fn all_authors(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Author>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_authors(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_authors(offset, limit).await,
        }
    }

    async fn get_author(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Author>> {
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
    ) -> anyhow::Result<Option<List<Story>>> {
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
    ) -> anyhow::Result<Option<Chapter>> {
        match self {
            DataBackend::Postgres(backend) => backend.get_chapter(story_id, chapter_number).await,
            DataBackend::Sqlite(backend) => backend.get_chapter(story_id, chapter_number).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendCharacter for DataBackend {
    async fn all_characters(
        &self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Character>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_characters(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_characters(offset, limit).await,
        }
    }

    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        match self {
            DataBackend::Postgres(backend) => backend.get_character(id).await,
            DataBackend::Sqlite(backend) => backend.get_character(id).await,
        }
    }

    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.character_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.character_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendOrigin for DataBackend {
    async fn all_origins(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Origin>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_origins(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_origins(offset, limit).await,
        }
    }

    async fn get_origin(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Origin>> {
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
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.origin_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.origin_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendPairing for DataBackend {
    async fn all_pairings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Pairing>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_pairings(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_pairings(offset, limit).await,
        }
    }

    async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        match self {
            DataBackend::Postgres(backend) => backend.get_pairing(id).await,
            DataBackend::Sqlite(backend) => backend.get_pairing(id).await,
        }
    }

    async fn pairing_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.pairing_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.pairing_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendStory for DataBackend {
    async fn all_stories(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_stories(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_stories(offset, limit).await,
        }
    }

    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        match self {
            DataBackend::Postgres(backend) => backend.get_story(id).await,
            DataBackend::Sqlite(backend) => backend.get_story(id).await,
        }
    }

    async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.search_stories(input, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.search_stories(input, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendTag for DataBackend {
    async fn all_tags(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Tag>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_tags(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_tags(offset, limit).await,
        }
    }

    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>> {
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
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.tag_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.tag_stories(id, offset, limit).await,
        }
    }
}

#[async_trait::async_trait]
impl BackendWarning for DataBackend {
    async fn all_warnings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Warning>>> {
        match self {
            DataBackend::Postgres(backend) => backend.all_warnings(offset, limit).await,
            DataBackend::Sqlite(backend) => backend.all_warnings(offset, limit).await,
        }
    }

    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        match self {
            DataBackend::Postgres(backend) => backend.get_warning(id).await,
            DataBackend::Sqlite(backend) => backend.get_warning(id).await,
        }
    }

    async fn warning_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        match self {
            DataBackend::Postgres(backend) => backend.warning_stories(id, offset, limit).await,
            DataBackend::Sqlite(backend) => backend.warning_stories(id, offset, limit).await,
        }
    }
}
