use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{Entity, List, Story, Tag},
        BackendTag,
    },
};

#[async_trait::async_trait]
impl BackendTag for PostgresPoolConnection {
    async fn all_tags(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>> {
        todo!()
    }

    async fn get_tag(&mut self, id: Cow<'static, str>) -> anyhow::Result<Tag> {
        todo!()
    }

    async fn tag_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        todo!()
    }
}
