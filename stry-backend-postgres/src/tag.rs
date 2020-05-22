use {
    crate::PostgresBackend,
    std::borrow::Cow,
    stry_common::{
        models::{List, Story, Tag},
        BackendStory, BackendTag,
    },
};

#[async_trait::async_trait]
impl BackendTag for PostgresBackend {
    async fn all_tags(&self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT id FROM tag ORDER BY name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let tag = self.get_tag(id.into()).await?;

            items.push(tag);
        }

        let total = conn
            .query_one("SELECT COUNT(id) as count FROM tag;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }

    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Tag> {
        let conn = self.0.get().await?;

        let row = conn
            .query_one(
                "SELECT id, name, typ, created, updated FROM origin WHERE id = $1;",
                &[&id],
            )
            .await?;

        let tag = Tag {
            id: row.try_get(0)?,

            name: row.try_get(1)?,

            typ: row.try_get(2)?,

            created: row.try_get(3)?,
            updated: row.try_get(4)?,
        };

        Ok(tag)
    }

    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT ST.story_id FROM story_tag ST LEFT JOIN story S ON S.id = ST.story_id WHERE ST.tag_id = $1 ORDER BY S.updated DESC LIMIT $2 OFFSET $3;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = self.get_story(id.into()).await?;

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(ST.story_id) as id FROM story_tag ST LEFT JOIN story S ON S.id = ST.story_id WHERE ST.tag_id = $1;", &[&id])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(list)
    }
}
