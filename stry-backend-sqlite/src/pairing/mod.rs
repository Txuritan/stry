use {
    crate::{utils::Total, SqliteBackend},
    rewryte::sqlite::{SqliteExt, SqliteStmtExt},
    std::borrow::Cow,
    stry_common::models::{Character, List, Pairing, PairingRow, Story},
};

impl SqliteBackend {
    #[tracing::instrument(level = "trace", skip(self), err)]
    pub async fn all_pairings(
        &self,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Pairing>>> {
        let pairings = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Pairing>>> {
                let conn = inner.0.get()?;

                let (mut pairing_stmt, mut character_stmt) = tracing::trace_span!("prepare")
                    .in_scope(|| -> anyhow::Result<_> {
                        let pairing_stmt = conn.prepare(include_str!("all-items.sql"))?;
                        let character_stmt = conn.prepare(include_str!("item-characters.sql"))?;

                        Ok((pairing_stmt, character_stmt))
                    })?;

                let rows = tracing::trace_span!("get_parts").in_scope(|| {
                    pairing_stmt.type_query_map_anyhow(rusqlite::params![limit, offset * limit])
                });

                let item_parts: Vec<PairingRow> =
                    match rows?.map(|items| items.collect::<Result<Vec<PairingRow>, _>>()) {
                        Some(items) => items?,
                        None => return Ok(None),
                    };

                let filled: Option<Vec<Pairing>> =
                    tracing::trace_span!("fill_parts").in_scope(|| -> anyhow::Result<_> {
                        let mut items = Vec::with_capacity(item_parts.len());

                        for part in item_parts {
                            let rows = tracing::trace_span!("get_characters").in_scope(|| {
                                character_stmt.type_query_map_anyhow(rusqlite::params![part.id])
                            })?;

                            let characters: Vec<Character> = match rows
                                .map(|items| items.collect::<Result<Vec<Character>, _>>())
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

                        Ok(Some(items))
                    })?;

                let items = match filled {
                    Some(items) => items,
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

        Ok(pairings)
    }

    #[tracing::instrument(level = "trace", skip(self), err)]
    pub async fn get_pairing(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Pairing>> {
        let pairing = tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<Pairing>> {
                let conn = inner.0.get()?;

                let mut character_stmt = tracing::trace_span!("prepare")
                    .in_scope(|| conn.prepare(include_str!("item-characters.sql")))?;

                let row: Option<PairingRow> = tracing::trace_span!("get_part").in_scope(|| {
                    conn.type_query_row_anyhow(include_str!("get-item.sql"), rusqlite::params![id])
                })?;

                let part: PairingRow = match row {
                    Some(part) => part,
                    None => return Ok(None),
                };

                let rows = tracing::trace_span!("get_characters").in_scope(|| {
                    character_stmt.type_query_map_anyhow(rusqlite::params![part.id])
                })?;

                let characters: Vec<Character> =
                    match rows.map(|items| items.collect::<Result<Vec<Character>, _>>()) {
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

    #[tracing::instrument(level = "trace", skip(self), err)]
    pub async fn pairing_stories(
        &self,
        _id: Cow<'static, str>,
        _offset: i32,
        _limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        todo!()
    }
}
