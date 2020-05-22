use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{Entity, List, Story},
        BackendStory,
    },
};

#[async_trait::async_trait]
impl BackendStory for PostgresPoolConnection {
    async fn all_stories(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Story>> {
        todo!()
    }

    async fn get_story(&mut self, id: Cow<'static, str>) -> anyhow::Result<Story> {
        todo!()
    }
}
