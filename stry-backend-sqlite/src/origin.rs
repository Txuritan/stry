use {
    crate::SqlitePoolConnection,
    std::borrow::Cow,
    stry_common::{
        models::{Entity, List, Origin, Story},
        BackendOrigin, BackendStory,
    },
};

#[async_trait::async_trait]
impl BackendOrigin for SqlitePoolConnection {
    async fn all_origins(&mut self, offset: u32, limit: u32) -> anyhow::Result<List<Origin>> {
        let mut inner = self.clone();

        let ids = tokio::task::spawn_blocking({
            let inner = inner.clone();

            move || -> anyhow::Result<List<Entity>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT id FROM origin ORDER BY name DESC LIMIT ? OFFSET ?;")?;

                let items = stmt
                    .query_map(rusqlite::params![limit, offset], |row| {
                        Ok(Entity { id: row.get(0)? })
                    })?
                    .collect::<Result<_, _>>()?;

                let total = conn.query_row(
                    "SELECT COUNT(id) as count FROM origin;",
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
            let story = inner.get_origin(id.into()).await?;

            items.push(story);
        }

        Ok(List { total, items })
    }

    async fn get_origin(&mut self, id: Cow<'static, str>) -> anyhow::Result<Origin> {
        let inner = self.clone();

        let res = tokio::task::spawn_blocking(move || -> anyhow::Result<Origin> {
            let conn = inner.0.get()?;

            let row = conn.query_row(
                "SELECT id, name, created, updated FROM origin WHERE id = ?;",
                rusqlite::params![id],
                |row| {
                    Ok(Origin {
                        id: row.get(0)?,

                        name: row.get(1)?,

                        created: row.get(2)?,
                        updated: row.get(3)?,
                    })
                },
            )?;

            Ok(row)
        })
        .await??;

        Ok(res)
    }

    async fn origin_stories(
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

                let mut stmt = conn.prepare("SELECT SO.story_id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = ? ORDER BY S.updated DESC LIMIT ? OFFSET ?;")?;

                let items: Vec<Entity> = stmt.query_map(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0)?,
                }))?.collect::<Result<_, _>>()?;

                let total = conn.query_row("SELECT COUNT(SO.story_id) as id FROM story_origin SO LEFT JOIN story S ON S.id = SO.story_id WHERE SO.origin_id = ?;", rusqlite::params![id], |row| Ok(row.get(0)?))?;

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
