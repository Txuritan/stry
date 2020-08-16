#[cfg(test)]
pub mod test;

use {
    crate::{
        backend::{
            sqlite::utils::{FromRow, SqliteExt, SqliteStmtExt, Total},
            BackendStory, BackendWarning, SqliteBackend,
        },
        models::{Entity, List, Story, Warning},
    },
    anyhow::Context,
    std::borrow::Cow,
    tracing_futures::Instrument,
};

impl FromRow for Warning {
    fn from_row(row: &rusqlite::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Warning {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for warning")?,

            name: row
                .get(1)
                .context("Attempting to get row index 1 for warning")?,

            created: row
                .get(2)
                .context("Attempting to get row index 2 for warning")?,
            updated: row
                .get(3)
                .context("Attempting to get row index 3 for warning")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendWarning for SqliteBackend {
    #[tracing::instrument(level = "debug", skip(self))]
    async fn all_warnings(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Warning>>> {
        let warnings = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Warning>>> {
                let conn = inner.0.get()?;

                let mut stmt = tracing::trace_span!("prepare")
                    .in_scope(|| conn.prepare(include_str!("all-items.sql")))?;

                let rows = tracing::trace_span!("get_rows").in_scope(|| {
                    stmt.type_query_map_anyhow(rusqlite::params![limit, offset * limit])
                })?;

                let items: Vec<Warning> =
                    match rows.map(|items| items.collect::<Result<Vec<Warning>, _>>()) {
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

        Ok(warnings)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn get_warning(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Warning>> {
        let res = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Warning>> {
                let conn = inner.0.get()?;

                let row: Option<Warning> = tracing::trace_span!("get").in_scope(|| {
                    conn.type_query_row_anyhow(include_str!("get-item.sql"), rusqlite::params![id])
                })?;

                Ok(row)
            }
        })
        .await??;

        Ok(res)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn warning_stories(
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
                                .context("Attempting to get row index 0 for warning story id")?,
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

        let mut items = Vec::with_capacity(limit as usize);

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
