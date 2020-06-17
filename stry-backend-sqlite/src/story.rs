use {
    crate::{
        utils::{SqliteExt, SqliteStmtExt},
        SqliteBackend,
    },
    anyhow::Context,
    std::borrow::Cow,
    stry_common::{
        backend::BackendStory,
        models::{
            pairing::PairingPart,
            story::{StoryPart, StoryRow},
            Author, Character, Entity, List, Origin, Pairing, Square, Story, Tag, Warning,
        },
        search::{SearchParser, SearchTypes, SearchValue, VecSearchExt},
    },
};

#[async_trait::async_trait]
impl BackendStory for SqliteBackend {
    async fn all_stories(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Story>>> {
        let ids = match tokio::task::spawn_blocking({
            let inner = self.clone();

            move || -> anyhow::Result<Option<List<Entity>>> {
                let conn = inner.0.get()?;

                let mut stmt =
                    conn.prepare("SELECT Id FROM Story ORDER BY Updated DESC LIMIT ? OFFSET ?;")?;

                let items = match stmt
                    .query_map_anyhow(rusqlite::params![limit, offset * limit], |row| {
                        Ok(Entity {
                            id: row
                                .get(0)
                                .context("Attempting to get row index 0 for story id")?,
                        })
                    })?
                    .map(|items| items.collect::<Result<_, _>>())
                {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let total = match conn.query_row_anyhow(
                    "SELECT COUNT(Id) as Count FROM Story;",
                    rusqlite::params![],
                    |row| {
                        Ok(row
                            .get(0)
                            .context("Attempting to get row index 0 for story count")?)
                    },
                )? {
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
            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        Ok(Some(List { total, items }))
    }

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

                let chapters: u32 = match conn.query_row_anyhow(
                    "SELECT COUNT(StoryId) as Count FROM StoryChapter WHERE StoryId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for chapter count")?),
                )? {
                    Some(chapters) => chapters,
                    None => return Ok(None),
                };

                let words: u32 = match conn.query_row_anyhow(
                    "SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Id = SC.ChapterId WHERE SC.StoryId = ?;",
                    rusqlite::params![id],
                    |row| Ok(row.get(0).context("Attempting to get row index 0 for word count")?),
                )? {
                    Some(words) => words,
                    None => return Ok(None),
                };

                let authors: Vec<Author> = match author_stmt.query_map_anyhow(rusqlite::params![id], |row| Ok(Author {
                    id: row.get(0).context("Attempting to get row index 0 for author")?,

                    name: row.get(1).context("Attempting to get row index 1 for author")?,

                    created: row.get(2).context("Attempting to get row index 2 for author")?,
                    updated: row.get(3).context("Attempting to get row index 3 for author")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let origins: Vec<Origin> = match origin_stmt.query_map_anyhow(rusqlite::params![id], |row| Ok(Origin {
                    id: row.get(0).context("Attempting to get row index 0 for origin")?,

                    name: row.get(1).context("Attempting to get row index 1 for origin")?,

                    created: row.get(2).context("Attempting to get row index 2 for origin")?,
                    updated: row.get(3).context("Attempting to get row index 3 for origin")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let warnings: Vec<Warning> = match warning_stmt.query_map_anyhow(rusqlite::params![id], |row| Ok(Warning {
                    id: row.get(0).context("Attempting to get row index 0 for warning")?,

                    name: row.get(1).context("Attempting to get row index 1 for warning")?,

                    created: row.get(2).context("Attempting to get row index 2 for warning")?,
                    updated: row.get(3).context("Attempting to get row index 3 for warning")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let characters: Vec<Character> = match character_stmt.query_map_anyhow(rusqlite::params![id], |row| Ok(Character {
                    id: row.get(0).context("Attempting to get row index 0 for character")?,

                    name: row.get(1).context("Attempting to get row index 1 for character")?,

                    created: row.get(2).context("Attempting to get row index 2 for character")?,
                    updated: row.get(3).context("Attempting to get row index 3 for character")?,
                }))?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let pairing_parts: Vec<PairingPart> = match story_pairings_stmt.query_map_anyhow(rusqlite::params![id], |row| {
                    Ok(PairingPart {
                        id: row.get(0).context("Attempting to get row index 0 for pairing")?,

                        platonic: row.get(1).context("Attempting to get row index 1 for pairing")?,

                        created: row.get(2).context("Attempting to get row index 2 for pairing")?,
                        updated: row.get(3).context("Attempting to get row index 3 for pairing")?,
                    })
                })?.map(|items| {
                    items.collect::<Result<_, _>>()
                }) {
                    Some(items) => items?,
                    None => return Ok(None),
                };

                let mut pairings = Vec::with_capacity(pairing_parts.len());

                for part in pairing_parts {
                    let characters = match pairing_stmt.query_map_anyhow(rusqlite::params![part.id], |row| Ok(Character {
                        id: row.get(0).context("Attempting to get row index 0 for pairing character")?,

                        name: row.get(1).context("Attempting to get row index 1 for pairing character")?,

                        created: row.get(2).context("Attempting to get row index 2 for pairing character")?,
                        updated: row.get(3).context("Attempting to get row index 3 for pairing character")?,
                    }))?.map(|items| {
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

                let tags: Vec<Tag> = match tag_stmt.query_map_anyhow(rusqlite::params![id], |row| Ok(Tag {
                    id: row.get(0).context("Attempting to get row index 0 for tag")?,

                    name: row.get(1).context("Attempting to get row index 1 for tag")?,

                    created: row.get(2).context("Attempting to get row index 2 for tag")?,
                    updated: row.get(3).context("Attempting to get row index 3 for tag")?,
                }))?.map(|items| {
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

    async fn search_stories(
        &self,
        _input: Cow<'static, str>,
        _offset: u32,
        _limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        // let search = SearchParser::parse_to_structure(input.as_ref())?;

        // let count = search.type_count();

        // // TODO: maybe wrap this in a blocking call
        // let (and, not): (Vec<SearchValue<'_>>, Vec<SearchValue<'_>>) =
        //     search.into_iter().partition(|sv| sv.is_included());

        // let split_and = and.split_on_type();
        // let split_not = not.split_on_type();

        // let mut buff = String::with_capacity((200 * count) + 60);

        // buff.push_str("SELECT id FROM story WHERE ");

        // for (search_type, values) in split_and {
        //     if values.is_none() {
        //         continue;
        //     }

        //     let values = values.unwrap();

        //     match search_type {
        //         SearchTypes::Characters => buff.push_str(
        //             "id IN (SELECT s.id FROM story s, story_characters sc, character c WHERE s.id = sc.story_id AND c.id = sc.character_id AND (lower(c.name) IN ("
        //         ),
        //         SearchTypes::Fandoms => buff.push_str(
        //             "id IN (SELECT s.id FROM story s, story_origin so, origin o WHERE s.id = so.story_id AND o.id = so.origin_id AND (lower(o.name) IN ("
        //         ),
        //         SearchTypes::Friends => {}
        //         SearchTypes::General => buff.push_str(
        //             "id IN (SELECT s.id FROM story s, story_tag st, tag t WHERE s.id = st.story_id AND t.id = st.tag_id AND (lower(t.name) IN ("
        //         ),
        //         SearchTypes::Pairings => buff.push_str(
        //             ""
        //         ),
        //         SearchTypes::Rating => {}
        //     }

        //     buff.push_str(")))");
        // }

        // for (search_type, values) in split_not {
        //     if values.is_none() {
        //         continue;
        //     }

        //     let values = values.unwrap();
        // }

        // buff.push_str(" ORDER BY updated DESC;");

        todo!()
    }
}
