use {
    crate::{
        backend::{BackendStory, PostgresBackend},
        models::{Author, Character, List, Origin, Pairing, Square, Story, Tag, Warning},
        search::SearchParser,
    },
    futures::try_join,
    std::borrow::Cow,
};

#[async_trait::async_trait]
impl BackendStory for PostgresBackend {
    #[tracing::instrument(skip(self))]
    async fn all_stories(&self, offset: u32, limit: u32) -> anyhow::Result<Option<List<Story>>> {
        let conn = self.0.get().await?;

        let stmt = conn
            .prepare("SELECT Id FROM Story ORDER BY Name DESC LIMIT $1 OFFSET $2;")
            .await?;

        let id_rows = conn.query(&stmt, &[&limit, &offset]).await?;

        let mut items = Vec::with_capacity(id_rows.len());

        for id_row in id_rows {
            let id: String = id_row.try_get(0)?;

            let story = match self.get_story(id.into()).await? {
                Some(story) => story,
                None => return Ok(None),
            };

            items.push(story);
        }

        let total = conn
            .query_one("SELECT COUNT(Id) as Count FROM Story;", &[])
            .await?;

        let list = List {
            total: total.try_get(0)?,
            items,
        };

        Ok(Some(list))
    }

    #[tracing::instrument(skip(self))]
    async fn get_story(&self, id: Cow<'static, str>) -> anyhow::Result<Option<Story>> {
        let conn = self.0.get().await?;

        let id_params = &[&id as &(dyn tokio_postgres::types::ToSql + Sync)]
            as &[&(dyn tokio_postgres::types::ToSql + Sync)];

        let (
            author_stmt, origin_stmt,
            warning_stmt, character_stmt, tag_stmt,
            _story_pairings_stmt, pairing_stmt,
            story_row,
            chapter_row, word_row,
        ) = try_join!(
            // author_stmt
            conn.prepare("SELECT A.Id FROM StoryAuthor SA LEFT JOIN Author A ON SA.AuthorId = A.Id WHERE SA.StoryId = $1 ORDER BY A.Name;"),
            // origin_stmt
            conn.prepare("SELECT O.Id FROM StoryOrigin SO LEFT JOIN Origin O ON SO.OriginId = O.Id WHERE SO.StoryId = $1 ORDER BY O.Name;"),
            // warning_stmt
            conn.prepare("SELECT W.Id, W.Name, W.Created, W.Updated FROM StoryWarning SW LEFT JOIN Warning W ON SW.WarningId = W.Id WHERE SW.StoryId = $1 ORDER BY W.Name;"),
            // character_stmt
            conn.prepare("SELECT C.Id, C.Name, C.Created, C.Updated FROM StoryCharacter SC LEFT JOIN Character C ON SC.CharacterId = C.Id WHERE SC.StoryId = $1 ORDER BY C.Name;"),
            // tag_stmt
            conn.prepare("SELECT T.Id, T.Name, T.Created, T.Updated FROM StoryTag ST LEFT JOIN Tag T ON ST.TagId = T.Id WHERE ST.StoryId = $1 ORDER BY T.Name;"),
            // story_pairings_stmt
            conn.prepare("SELECT PairingId FROM StoryPairing WHERE StoryId = $1;"),
            // pairing_stmt
            conn.prepare("SELECT C.Id, C.Platonic, C.Created, C.Updated FROM Pairing P LEFT JOIN PairingCharacter PC ON PC.PairingId = P.Id LEFT JOIN Character C ON PC.CharacterId = C.Id WHERE P.Id = $1 ORDER BY C.Name ASC;"),
            // story_row
            conn.query_one("SELECT Id, Name, Summary, Rating, State, Created, Updated FROM Story WHERE Id = $1;", id_params),
            // chapter_row
            conn.query_one(
                "SELECT COUNT(StoryId) as Count FROM StoryChapter WHERE StoryId = $1;",
                id_params,
            ),
            // word_row
            conn.query_one("SELECT SUM(C.Words) as Words FROM StoryChapter SC LEFT JOIN Chapter C ON C.Cd = SC.ChapterId WHERE SC.StoryId = $1;", id_params),
        )?;

        let (author_rows, origin_rows, warning_rows, pairing_rows, character_rows, tag_rows) = try_join!(
            // author_rows
            conn.query(&author_stmt, id_params),
            // origin_rows
            conn.query(&origin_stmt, id_params),
            // warning_rows
            conn.query(&warning_stmt, id_params),
            // pairing_rows
            conn.query(&pairing_stmt, id_params),
            // character_rows
            conn.query(&character_stmt, id_params),
            // tag_rows
            conn.query(&tag_stmt, id_params),
        )?;

        let authors = author_rows
            .into_iter()
            .map(|row| -> Result<Author, tokio_postgres::Error> {
                Ok(Author {
                    id: row.try_get(0)?,

                    name: row.try_get(1)?,

                    created: row.try_get(2)?,
                    updated: row.try_get(3)?,
                })
            })
            .collect::<Result<_, _>>()?;

        let origins = origin_rows
            .into_iter()
            .map(|row| -> Result<Origin, tokio_postgres::Error> {
                Ok(Origin {
                    id: row.try_get(0)?,

                    name: row.try_get(1)?,

                    created: row.try_get(2)?,
                    updated: row.try_get(3)?,
                })
            })
            .collect::<Result<_, _>>()?;

        let warnings = warning_rows
            .into_iter()
            .map(|row| -> Result<Warning, tokio_postgres::Error> {
                Ok(Warning {
                    id: row.try_get(0)?,

                    name: row.try_get(1)?,

                    created: row.try_get(2)?,
                    updated: row.try_get(3)?,
                })
            })
            .collect::<Result<Vec<Warning>, _>>()?;

        let mut pairings = Vec::with_capacity(pairing_rows.len());

        for row in pairing_rows {
            let pairing_id: String = row.try_get(0)?;

            let characters = conn
                .query(&pairing_stmt, &[&pairing_id])
                .await?
                .into_iter()
                .map(|row| -> Result<Character, tokio_postgres::Error> {
                    Ok(Character {
                        id: row.try_get(0)?,

                        name: row.try_get(1)?,

                        created: row.try_get(2)?,
                        updated: row.try_get(3)?,
                    })
                })
                .collect::<Result<_, _>>()?;

            pairings.push(Pairing {
                id: pairing_id,

                characters,

                platonic: row.try_get(1)?,

                created: row.try_get(2)?,
                updated: row.try_get(3)?,
            });
        }

        let characters = character_rows
            .into_iter()
            .map(|row| -> Result<Character, tokio_postgres::Error> {
                Ok(Character {
                    id: row.try_get(0)?,

                    name: row.try_get(1)?,

                    created: row.try_get(2)?,
                    updated: row.try_get(3)?,
                })
            })
            .collect::<Result<_, _>>()?;

        let tags = tag_rows
            .into_iter()
            .map(|row| -> Result<Tag, tokio_postgres::Error> {
                Ok(Tag {
                    id: row.try_get(0)?,

                    name: row.try_get(1)?,

                    created: row.try_get(2)?,
                    updated: row.try_get(3)?,
                })
            })
            .collect::<Result<_, _>>()?;

        let story = Story {
            id: story_row.try_get(0)?,

            name: story_row.try_get(1)?,
            summary: story_row.try_get(2)?,

            square: Square {
                rating: story_row.try_get(3)?,
                warnings: !warnings.is_empty(),
                state: story_row.try_get(4)?,
            },

            chapters: chapter_row.try_get(0)?,
            words: word_row.try_get(0)?,

            authors,
            origins,

            warnings,
            pairings,
            characters,
            tags,

            // TODO
            series: None,

            created: story_row.try_get(5)?,
            updated: story_row.try_get(6)?,
        };

        Ok(Some(story))
    }

    #[tracing::instrument(skip(self))]
    async fn search_stories(
        &self,
        input: Cow<'static, str>,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Option<List<Story>>> {
        // TODO: maybe wrap this in a blocking call
        let _values = SearchParser::parse_to_structure(input.as_ref())?;

        let _conn = self.0.get().await?;

        todo!()
    }
}
