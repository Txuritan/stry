#[cfg(test)]
pub mod test;

use {
    crate::{utils::Total, SqliteBackend},
    anyhow::Context,
    rewryte::sqlite::{SqliteExt, SqliteStmtExt},
    std::borrow::Cow,
    stry_common::backend::{BackendStory, BackendTag},
    stry_common::models::{Entity, List, Story, Tag},
    tracing_futures::Instrument,
};

#[async_trait::async_trait]
impl BackendTag for SqliteBackend {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn all_tags(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Tag>>> {
        let tags = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Tag>>> {
                let conn = inner.0.get()?;

                let mut stmt = tracing::trace_span!("prepare")
                    .in_scope(|| conn.prepare(include_str!("all-items.sql")))?;

                let row = tracing::trace_span!("get_rows").in_scope(|| {
                    stmt.type_query_map_anyhow(rusqlite::params![limit, offset * limit])
                })?;

                let items: Vec<Tag> = match row.map(|tags| tags.collect::<Result<Vec<Tag>, _>>()) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let row: Option<Total> = tracing::trace_span!("get_count").in_scope(|| {
                    conn.type_query_row_anyhow(include_str!("all-count.sql"), rusqlite::params![])
                })?;

                let total: Total = match row {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List {
                    total: total.total,
                    items,
                }))
            }
        })
        .await??;

        Ok(tags)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_tag(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Tag>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Tag>> {
                let conn = inner.0.get()?;

                let row: Option<Tag> = tracing::trace_span!("get").in_scope(|| {
                    conn.type_query_row_anyhow(include_str!("get-item.sql"), rusqlite::params![id])
                })?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn tag_stories(
        &self,
        id: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = tracing::trace_span!("prepare")
                    .in_scope(|| conn.prepare(include_str!("stories-items.sql")))?;

                let rows = tracing::trace_span!("get_ids").in_scope(|| {
                    stmt.query_map_anyhow(rusqlite::params![id, limit, offset], |row| {
                        Ok(Entity {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for tag story id")?,
                        })
                    })
                })?;

                let items: Vec<Entity> =
                    match rows.map(|items| items.collect::<Result<Vec<Entity>, _>>()) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let row: Option<Total> = tracing::trace_span!("get_count").in_scope(|| {
                    conn.type_query_row_anyhow(
                        include_str!("stories-count.sql"),
                        rusqlite::params![id],
                    )
                })?;

                let total: Total = match row {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List {
                    total: total.total,
                    items,
                }))
            }
        })
        .await??
        {
            Some(ids) => ids,
            None => return Ok(None),
        };

        let (total, entities) = ids.into_parts();

        let mut items: Vec<Story> = Vec::with_capacity(limit as usize);

        for Entity { id } in entities {
            let story = match self
                .get_story(id.into())
                .instrument(tracing::trace_span!("get_story"))
                .await?
            {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        Ok(Some(List { total, items }))
    }
}
