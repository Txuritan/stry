use {
    crate::SqliteBackend,
    std::borrow::Cow,
    stry_common::{
        models::{Entity, List, Story, Tag},
        BackendStory, BackendTag,
    },
};

#[async_trait::async_trait]
impl BackendTag for SqliteBackend {
    async fn all_tags(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Tag>> {
        let mut inner = self.clone();

        let ids = tokio::task::spawn_blocking({
            let inner = inner.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT id FROM tag ORDER BY name DESC LIMIT ? OFFSET ?;")?;

                let items = stmt
                    .query_map(rusqlite::params![limit, offset], |row| {
                        Ok(Entity { id: row.get(0)? })
                    })?
                    .collect::<Result<_, _>>()?;

                let total = conn.query_row(
                    "SELECT COUNT(id) as count FROM tag;",
                    rusqlite::params![],
                    |row| Ok(row.get(0)?),
                )?;

                Ok(List { total, items })
            }
        })
        .await??;

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = inner.get_tag(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }

    async fn get_tag(&mut self, id: Cow<'static, str>) -> anyhow::Result<Tag> {
        let inner = self.clone();

        let res = tokio::task::spawn_blocking(move || -> anyhow::Result<Tag> {
            let conn = inner.0.get()?;

            let row = conn.query_row(
                "SELECT id, name, typ, created, updated FROM tag WHERE id = ?;",
                rusqlite::params![id],
                |row| {
                    Ok(Tag {
                        id: row.get(0)?,
                        name: row.get(1)?,

                        typ: row.get(2)?,

                        created: row.get(3)?,
                        updated: row.get(4)?,
                    })
                },
            )?;

            Ok(row)
        })
        .await??;

        Ok(res)
    }

    async fn tag_stories(
        &mut self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<List<Story>> {
        let mut inner = self.clone();

        let ids = tokio::task::spawn_blocking({
            let inner = inner.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare("SELECT ST.story_id FROM story_tag ST LEFT JOIN story S ON S.id = ST.story_id WHERE ST.tag_id = ? ORDER BY S.updated DESC LIMIT ? OFFSET ?;")?;

                let items: Vec<Entity> = stmt.query_map(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                let total = conn.query_row("SELECT COUNT(ST.story_id) as id FROM story_tag ST LEFT JOIN story S ON S.id = ST.story_id WHERE ST.tag_id = ?;", rusqlite::params![id], |row| Ok(row.get(0)?))?;

                Ok(List {
                    total,
                    items,
                })
            }
        }).await??;

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = inner.get_story(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }
}
