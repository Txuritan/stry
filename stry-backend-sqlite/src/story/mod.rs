use {
    crate::{
        utils::{Total, Wrapper},
        SqliteBackend,
    },
    anyhow::Context,
    rewryte::sqlite::{FromRow, SqliteExt, SqliteStmtExt},
    std::borrow::Cow,
    stry_common::models::{
        story::{StoryPart, StoryRow},
        Author, Character, Entity, List, Origin, Pairing, PairingRow, Square, Story, Tag, Warning,
    },
    stry_common::{
        backend::BackendStory,
        search::{SearchParser, SearchValue},
    },
    tracing_futures::Instrument,
};

enum Wrap {
    Story(StoryRow),
    Author(Author),
    Origin(Origin),
    Warning(Warning),
    Character(Character),
    Tag(Tag),
}

#[async_trait::async_trait]
impl BackendStory for SqliteBackend {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn all_stories(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = tracing::trace_span!("prepare")
                    .in_scope(|| conn.prepare(include_str!("all-items.sql")))?;

                let rows = tracing::trace_span!("get_rows")
                    .in_scope(|| {
                        stmt.query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                            Ok(Entity {
                                id: row
                                    .get(0)
                                    .context("Attempting to get row index 0 for story id")?,
                            })
                        })
                    })?
                    .map(|items| items.collect::<Result<_, _>>());

                let items = match rows {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match tracing::trace_span!("get_count").in_scope(|| {
                    conn.query_row_anyhow(
                        include_str!("all-count.sql"),
                        rusqlite::params![],
                        |row| {
                            Ok(row
                                .get(0)
                                .context("Attempting to get row index 0 for story count")?)
                        },
                    )
                })? {
                    Some(total) => total,
                    None => return Ok(None),
                };

                Ok(Some(List { total, items }))
            }
        })
        .await?
        .context("Unable to get stories")?
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

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        let story_part = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<StoryPart>> {
                let conn = inner.0.get()?;

                let (mut stmt, mut story_pairings_stmt, mut pairing_stmt) =
                    tracing::trace_span!("prepare").in_scope(|| -> anyhow::Result<(_, _, _)> {
                        Ok((
                            conn.prepare(include_str!("get-item.sql"))?,
                            conn.prepare(include_str!("get-story-pairing.sql"))?,
                            conn.prepare(include_str!("get-pairing-character.sql"))?,
                        ))
                    })?;

                let rows = tracing::trace_span!("get_rows").in_scope(|| {
                    stmt.query_map_anyhow(rusqlite::params![id, id, id, id, id, id], |row| {
                        let typ: String = row
                            .get(9)
                            .context("Attempting to get row index 8 for story")?;

                        match typ.as_str() {
                            "story" => Ok(Wrap::Story(
                                StoryRow::from_row(row)
                                    .context("Attempting to get story row for story (row)")?,
                            )),
                            "author" => Ok(Wrap::Author(
                                Author::from_row(row)
                                    .context("Attempting to get author for story (row)")?,
                            )),
                            "origin" => Ok(Wrap::Origin(
                                Origin::from_row(row)
                                    .context("Attempting to get origin for story (row)")?,
                            )),
                            "warning" => Ok(Wrap::Warning(
                                Warning::from_row(row)
                                    .context("Attempting to get warning for story (row)")?,
                            )),
                            "character" => Ok(Wrap::Character(
                                Character::from_row(row)
                                    .context("Attempting to get character for story (row)")?,
                            )),
                            "tag" => Ok(Wrap::Tag(
                                Tag::from_row(row)
                                    .context("Attempting to get tag for story (row)")?,
                            )),
                            other => anyhow::bail!("Unknown row group type `{}`", other),
                        }
                    })
                })?;

                let parts = match rows {
                    Some(parts) => parts,
                    None => return Ok(None),
                };

                let pairings = match tracing::trace_span!("get_pairings").in_scope(
                    || -> anyhow::Result<_> {
                        let pairing_parts: Vec<PairingRow> = match story_pairings_stmt
                            .type_query_map_anyhow(rusqlite::params![id])?
                            .map(|items| items.collect::<Result<_, _>>())
                        {
                            Some(items) => items?,
                            None => return Ok(None),
                        };

                        let mut pairings = Vec::with_capacity(pairing_parts.len());

                        for part in pairing_parts {
                            let characters = match pairing_stmt
                                .type_query_map_anyhow(rusqlite::params![part.id])?
                                .map(|items| items.collect::<Result<_, _>>())
                            {
                                Some(items) => items?,
                                None => return Ok(None),
                            };

                            pairings.push(Pairing {
                                id: part.id,

                                characters,

                                platonic: part.platonic,

                                created: part.created,
                                updated: part.updated,
                            });
                        }

                        Ok(Some(pairings))
                    },
                )? {
                    Some(pairings) => pairings,
                    None => return Ok(None),
                };

                let mut story_row = None;

                let mut authors = Vec::new();
                let mut origins = Vec::new();

                let mut warnings = Vec::new();
                let mut characters = Vec::new();
                let mut tags = Vec::new();

                for row in parts {
                    match row? {
                        Wrap::Story(item) => {
                            story_row = Some(item);
                        }
                        Wrap::Author(item) => authors.push(item),
                        Wrap::Origin(item) => origins.push(item),
                        Wrap::Warning(item) => warnings.push(item),
                        Wrap::Character(item) => characters.push(item),
                        Wrap::Tag(item) => tags.push(item),
                    }
                }

                let story_row = story_row
                    .ok_or_else(|| anyhow::anyhow!("Story get did not return story group type"))?;

                Ok(Some(StoryPart {
                    id: story_row.id,

                    name: story_row.name,
                    summary: story_row.summary,

                    rating: story_row.rating,
                    state: story_row.state,

                    chapters: story_row.chapters,
                    words: story_row.words,

                    authors,
                    origins,

                    warnings,
                    characters,
                    pairings,
                    tags,

                    created: story_row.created,
                    updated: story_row.updated,
                }))
            }
        })
        .await?
        .context("Unable to get story")?
        {
            Some(story_part) => story_part,
            None => return Ok(None),
        };

        let story = Story {
            id: story_part.id,

            name: story_part.name,
            summary: story_part.summary,

            square: Square {
                rating: story_part.rating,
                warnings: !story_part.warnings.is_empty(),
                state: story_part.state,
            },

            chapters: story_part.chapters,
            words: story_part.words,

            authors: story_part.authors,
            origins: story_part.origins,

            warnings: story_part.warnings,
            pairings: story_part.pairings,
            characters: story_part.characters,
            tags: story_part.tags,

            // TODO
            series: None,

            created: story_part.created,
            updated: story_part.updated,
        };

        Ok(Some(story))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: i32,
        limit: i32,
    ) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();
            let input = input.to_owned();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let search = SearchParser::parse_to_structure(&input)?;

                let (and, not): (Vec<SearchValue<'_>>, Vec<SearchValue<'_>>) =
                    search.into_iter().partition(|value| value.is_included());
                let (mut query, mut params) = query_from_parts(and, not);

                let conn = inner.0.get()?;

                let row: Option<Total> = tracing::trace_span!("get_count")
                    .in_scope(|| conn.type_query_row_anyhow(&query, &params))?;

                let total: Total = match row {
                    Some(total) => total,
                    None => return Ok(None),
                };

                query.push_str(" LIMIT ? OFFSET ? ");

                let mut stmt = tracing::trace_span!("prepare").in_scope(|| conn.prepare(&query))?;

                params.push(Wrapper::Num(limit));
                params.push(Wrapper::Num(offset));

                let rows = tracing::trace_span!("get_ids").in_scope(|| {
                    stmt.query_map_anyhow(&params, |row| {
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

#[tracing::instrument(level = "debug")]
fn query_from_parts<'p>(
    and: Vec<SearchValue<'p>>,
    not: Vec<SearchValue<'p>>,
) -> (String, Vec<Wrapper<'p>>) {
    let (and_empty, and_len) = (and.is_empty(), and.len());
    let (not_empty, not_len) = (not.is_empty(), not.len());

    let mut query_buff = String::with_capacity((and.len() + not.len()) * 175);
    let mut param_buff = Vec::new();

    if !and_empty {
        for (i, value) in and.into_iter().enumerate() {
            query_from_value(value, &mut query_buff, &mut param_buff, true);

            if i != and_len - 1 {
                query_buff.push_str("INTERSECT\n");
            }
        }
    }

    if !not_empty {
        if !and_empty {
            query_buff.push_str("EXCEPT\n");
        }

        for (i, value) in not.into_iter().enumerate() {
            query_from_value(value, &mut query_buff, &mut param_buff, false);

            if i != not_len - 1 {
                query_buff.push_str("EXCEPT\n");
            }
        }
    }

    query_buff.push_str("ORDER BY 2");

    (query_buff, param_buff)
}

#[tracing::instrument(level = "debug")]
fn query_from_value<'p>(
    value: SearchValue<'p>,
    query_buff: &mut String,
    param_buff: &mut Vec<Wrapper<'p>>,
    _is_and: bool,
) {
    match value {
        SearchValue::Friends(_, _characters) => {}
        SearchValue::Pairing(_, _characters) => {}
        SearchValue::Character(_, name) => {
            query_buff.push_str("SELECT S.Id, S.Updated FROM Story S, StoryCharacter SC WHERE S.Id = SC.StoryId AND SC.CharacterId = (SELECT Id FROM Character WHERE LOWER(Name) LIKE LOWER(?))\n");
            param_buff.push(Wrapper::Cow(name));
        }
        SearchValue::Fandom(_, name) => {
            query_buff.push_str("SELECT S.Id, S.Updated FROM Story S, StoryOrigin SO WHERE S.Id = SO.StoryId AND SO.OriginId = (SELECT Id FROM Origin WHERE LOWER(Name) LIKE LOWER(?))\n");
            param_buff.push(Wrapper::Cow(name));
        }
        SearchValue::General(_, name) => {
            query_buff.push_str("SELECT S.Id, S.Updated FROM Story S, StoryTag ST WHERE S.Id = ST.StoryId AND ST.TagId = (SELECT Id FROM Tag WHERE LOWER(Name) LIKE LOWER(?))\n");
            param_buff.push(Wrapper::Cow(name));
        }
        SearchValue::Rating(_, rating) => {
            query_buff.push_str("SELECT Id, Updated AS StoryId FROM Story WHERE Rating = ?\n");
            param_buff.push(Wrapper::Rating(rating));
        }
    }
}
