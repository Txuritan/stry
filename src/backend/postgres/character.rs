use {
    crate::{
        backend::{BackendCharacter, PostgresBackend},
        models::{Character, List, Story},
    },
    std::borrow::Cow,
};

#[async_trait::async_trait]
impl BackendCharacter for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_characters(
        &self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Character>>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        todo!()
    }

    #[tracing::instrument(skip(self))]
    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
