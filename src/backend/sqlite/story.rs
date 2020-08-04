use {
    crate::{
        backend::{
            sqlite::utils::{SqliteExt, SqliteStmtExt, Total, Wrapper},
            BackendStory, SqliteBackend,
        },
        models::{
            pairing::PairingPart,
            story::{StoryPart, StoryRow},
            Author, Character, Entity, List, Origin, Pairing, Square, Story, Tag, Warning,
        },
        search::{SearchParser, SearchValue},
    },
    anyhow::Context,
    std::borrow::Cow,
    tracing_futures::Instrument,
};

#[async_trait::async_trait]
impl BackendStory for SqliteBackend {
    #[tracing::instrument(level = "debug", skip(self))]
    async fn all_stories(&self, offset: i32, limit: i32) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt = tracing::trace_span!("prepare").in_scope(|| {
                    conn.prepare("SELECT Id FROM Story ORDER BY Updated DESC LIMIT ? OFFSET ?;")
                })?;

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
                        "SELECT COUNT(Id) as Count FROM Story;",
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

    #[tracing::instrument(level = "debug", skip(self))]
    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        let story_part = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<StoryPart>> {
                let conn = inner.0.get()?;

                let mut author_stmt = conn.prepare("SELECT A.Id, A.Name, A.Created, A.Updated FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = ? ORDER BY A.Name;")?;
                let mut origin_stmt = conn.prepare("SELECT O.Id, O.Name, O.Created, O.Updated FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = ? ORDER BY O.Name;")?;

                let mut tag_stmt = conn.prepare("SELECT T.Id, T.Name, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = ? ORDER BY T.Name;")?;
                let mut character_stmt = conn.prepare("SELECT C.Id, C.Name, C.Created, C.Updated FROM StoryCharacter SC LEFT JOIN Character C ON SC.CharacterId = C.Id WHERE SC.StoryId = ? ORDER BY C.Name;")?;
                let mut warning_stmt = conn.prepare("SELECT W.Id, w.Name, w.Created, w.Updated FROM StoryWarning SW LEFT JOIN Warning W ON SW.WarningId = W.Id WHERE SW.StoryId = ? ORDER BY W.Name;")?;

                let mut story_pairings_stmt = conn.prepare("SELECT P.Id, P.Platonic, P.Created, P.Updated FROM StoryPairing SP LEFT JOIN Pairing P ON P.Id = SP.PairingId WHERE SP.StoryId = ? ORDER BY (SELECT GROUP_CONCAT(C.Name, '/') FROM PairingCharacter PC LEFT JOIN Character C ON C.Id = PC.CharacterId WHERE PC.PairingId = P.Id);")?;
                let mut pairing_stmt = conn.prepare("SELECT C.Id, C.Name, C.Created, C.Updated FROM Pairing P LEFT JOIN PairingCharacter PC ON PC.PairingId = P.Id LEFT JOIN Character C ON PC.CharacterId = C.Id WHERE P.Id = ? ORDER BY C.Name ASC;")?;

                let story_row = match conn.query_row_anyhow(
                    "SELECT Id, Name, Summary, Rating, State, Created, Updated FROM Story WHERE Id = ?;",
                    rusqlite::params![id],
                    |row| {
                        Ok(StoryRow {
                            id: row.get(0).context("Attempting to get row index 0 for story (row)")?,

                            name: row.get(1).context("Attempting to get row index 1 for story (row)")?,
                            summary: row.get(2).context("Attempting to get row index 2 for story (row)")?,

                            rating: row.get(3).context("Attempting to get row index 3 for story (row)")?,
                            state: row.get(4).context("Attempting to get row index 4 for story (row)")?,

                            created: row.get(5).context("Attempting to get row index 5 for story (row)")?,
                            updated: row.get(6).context("Attempting to get row index 6 for story (row)")?,
                        })
                    },
                )? {
                    Some(story_row) => story_row,
                    None => return Ok(None),
                };

                let chapters: i32 = match conn.query_row_anyhow(
                    "SELECT COUNT(StoryId) as Count FROM StoryChapter WHERE StoryId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for chapter count")?),
                )? {
                    Some(chapters) => chapters,
                    None => return Ok(None),
                };

                let words: i32 = match conn.query_row_anyhow(
                    "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for word count")?),
                )? {
                    Some(words) => words,
                    None => return Ok(None),
                };

                let authors: Vec<Author> = match author_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let origins: Vec<Origin> = match origin_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let warnings: Vec<Warning> = match warning_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let characters: Vec<Character> = match character_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let pairing_parts: Vec<PairingPart> = match story_pairings_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let mut pairings = Vec::with_capacity(pairing_parts.len());

                for part in pairing_parts {
                    let characters = match pairing_stmt.type_query_map_anyhow(rusqlite::params![part.id])?.map(|items| {
                        items.collect::<Result<_, _>>()
                    }) {
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

                let tags: Vec<Tag> = match tag_stmt.type_query_map_anyhow(rusqlite::params![id])?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                Ok(Some(StoryPart {
                    id: story_row.id,

                    name: story_row.name,
                    summary: story_row.summary,

                    rating: story_row.rating,
                    state: story_row.state,

                    chapters,
                    words,

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
        .await?.context("Unable to get story")? {
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

    #[tracing::instrument(skip(self))]
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
        .await?? {
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
