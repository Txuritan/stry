use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendCharacter, BackendStory},
        models::{Character, List, Story},
    },
};

#[async_trait::async_trait]
impl BackendCharacter for PostgresBackend {
    async fn all_characters(
        &self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Character>>> {
        todo!()
    }

    async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        todo!()
    }

    async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
