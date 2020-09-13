use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        backend::BackendCharacter,
        models::{Character, List, Story},
    },
};

#[async_trait::async_trait]
impl BackendCharacter for PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    async fn all_characters(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Character>>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
