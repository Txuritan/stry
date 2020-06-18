use {
    crate::{
        utils::{FromRow, SqliteExt, SqliteStmtExt},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::BackendPairing,
        models::{pairing::PairingPart, Character, List, Pairing, Story},
    },
};

impl FromRow for PairingPart {
    fn from_row(row: &rusqlite::Row) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(PairingPart {
            id: row
                .get(0)
                .context("Attempting to get row index 0 for pairing")?,

            platonic: row
                .get(1)
                .context("Attempting to get row index 0 for pairing")?,

            created: row
                .get(2)
                .context("Attempting to get row index 0 for pairing")?,
            updated: row
                .get(3)
                .context("Attempting to get row index 0 for pairing")?,
        })
    }
}

#[async_trait::async_trait]
impl BackendPairing for SqliteBackend {
    async fn all_pairings(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Pairing>>> {
        let pairings = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Pairing>>> {
                let conn = inner.0.get()?;

                let mut pairing_stmt = conn.prepare(include_str!("all-items.sql"))?;
                let mut character_stmt = conn.prepare(include_str!("item-characters.sql"))?;

                let item_parts: Vec<PairingPart> = match pairing_stmt
                    .type_query_map_anyhow::<PairingPart, _>(rusqlite::params![
                        limit,
                        offset * limit
                    ])?
                    .map(|items| items.collect::<Result<_, _>>())
                {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let mut items = Vec::with_capacity(item_parts.len());

                for part in item_parts {
                    let characters = match character_stmt
                        .type_query_map_anyhow::<Character, _>(rusqlite::params![part.id])?
                        .map(|items| items.collect::<Result<_, _>>())
                    {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                    items.push(Pairing {
                        id: part.id,

                        characters,

                        platonic: part.platonic,

                        created: part.created,
                        updated: part.updated,
                    });
                }

                let total = match conn.query_row_anyhow(
                    include_str!("all-count.sql"),
                    rusqlite::params![],
                    |row| {
                        Ok(row
                            .get(0)
                            .context("Attempting to get row index 0 for pairing count")?)
                    },
                )? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await??;

        Ok(pairings)
    }

    async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        let pairing = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Pairing>> {
                let conn = inner.0.get()?;

                let mut character_stmt = conn.prepare(include_str!("item-characters.sql"))?;

                let part = match conn.type_query_row_anyhow::<PairingPart, _>(
                    include_str!("get-item.sql"),
                    rusqlite::params![id],
                )? {
                    Some(part) => part,
                    None => return Ok(None),
                };

                let characters = match character_stmt
                    .type_query_map_anyhow::<Character, _>(rusqlite::params![part.id])?
                    .map(|items| items.collect::<Result<_, _>>())
                {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                Ok(Some(Pairing {
                    id: part.id,

                    characters,

                    platonic: part.platonic,

                    created: part.created,
                    updated: part.updated,
                }))
            }
        })
        .await??;

        Ok(pairing)
    }

    async fn pairing_stories(
        &self,
        _id: Cow<'static, str>,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
