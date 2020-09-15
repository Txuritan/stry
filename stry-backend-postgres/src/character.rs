use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::models::{Character, List, Story},
};

#[stry_macros::box_async]
impl PostgresBackend {
    #[tracing::instrument(skip(self), err)]
    pub async fn all_characters(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Character>>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn get_character(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Character>> {
        todo!()
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn character_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
