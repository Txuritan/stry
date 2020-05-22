use {
    crate::PostgresPoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{Chapter, Entity, List, Story},
        BackendChapter,
    },
};

#[async_trait::async_trait]
impl BackendChapter for PostgresPoolConnection {
    async fn get_chapter(
        &mut self,
        story_id: Cow<'static, str>,
        chapter_number: u32,
    ) -> anyhow::Result<Chapter> {
        todo!()
    }
}
