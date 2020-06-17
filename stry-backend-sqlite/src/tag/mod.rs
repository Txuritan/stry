use {
    crate::{
        utils::{SqliteExt, SqliteStmtExt},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::{BackendStory, BackendTag},
        models::{Entity, List, Story, Tag},
    },
};

#[async_trait::async_trait]
impl BackendTag for SqliteBackend {
    async fn all_tags(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Tag>>> {
        let tags = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Tag>>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare(include_str!("all-items.sql"))?;

                let items = match stmt
                    .query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                        Ok(Tag {
                            id: row.get(0).context("Attempting to get row index 0 for tag")?,

                            name: row.get(1).context("Attempting to get row index 1 for tag")?,

                            created: row.get(2).context("Attempting to get row index 2 for tag")?,
                            updated: row.get(3).context("Attempting to get row index 3 for tag")?,
                        })
                    })?
                    .map(|tags| {
                        tags.collect::<Result<_, _>>()
                    }) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let total = match conn.query_row_anyhow(
                    include_str!("all-count.sql"),
                    rusqlite::params![],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for tag count")?),
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await??;

        Ok(tags)
    }

    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Tag>> {
                let conn = inner.0.get()?;

                let row = conn.query_row_anyhow(
                    include_str!("get-item.sql"),
                    rusqlite::params![id],
                    |row| {
                        Ok(Tag {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for tag")?,

                            name: row
                                .get(1)
                                .context("Attempting to get row index 1 for tag")?,

                            created: row
                                .get(2)
                                .context("Attempting to get row index 2 for tag")?,
                            updated: row
                                .get(3)
                                .context("Attempting to get row index 3 for tag")?,
                        })
                    },
                )?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = conn.prepare(include_str!("stories-items.sql"))?;

                let items: Vec<Entity> = match stmt.query_map_anyhow(rusqlite::params![id, limit, offset], |row| Ok(Entity {
                    id: row.get(0).context("Attempting to get row index 0 for tag story id")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match conn.query_row_anyhow(
                    include_str!("stories-count.sql"),
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for tag story count")?),
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List {
                    total,
                    items,
                }))
            }
        }).await?? {
            Some(ids) => ids,
            None => return Ok(None),
        };

        let (total, entities) = ids.into_parts();

        let mut items = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        Ok(Some(List { total, items }))
    }
}
