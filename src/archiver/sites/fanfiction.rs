use {
    crate::{
        archiver::sleep,
        story::{Language, Rating, State},
        word_count, Error, Pool,
    },
    chrono::prelude::*,
    comrak::{markdown_to_html, ComrakOptions},
    rusqlite::{OptionalExtension, Transaction},
    scraper::{Html, Selector},
    uuid::Uuid,
};

lazy_static::lazy_static! {
    static ref CHAPTER_NAME_SELECTOR: Selector = Selector::parse("select#chap_select > option[selected]").unwrap();
    static ref CHAPTER_TEXT_SELECTOR: Selector = Selector::parse("#storytext").unwrap();
    static ref STORY_AUTHOR_SELECTOR: Selector = Selector::parse("#profile_top > a.xcontrast_txt:not([title])").unwrap();
    static ref STORY_DETAILS_SELECTOR: Selector = Selector::parse("#profile_top > span.xgray.xcontrast_txt").unwrap();
    static ref STORY_SUMMARY_SELECTOR: Selector = Selector::parse("#profile_top > div.xcontrast_txt").unwrap();
    static ref STORY_NAME_SELECTOR: Selector = Selector::parse("#profile_top > b.xcontrast_txt").unwrap();

    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
    .default_headers({
        use reqwest::header;
        let mut map = header::HeaderMap::new();
        map.insert(header::ACCEPT, header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));
        map.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate"));
        map.insert(header::ACCEPT_LANGUAGE, header::HeaderValue::from_static("en-US,en;q=0.5"));
        map.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:68.0) Gecko/20100101 Firefox/68.0"));
        map
    })
    .build().expect("Unable to create reqwest client");
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct FanFiction {}

impl FanFiction {
    pub fn scrape(
        pool: Pool,
        id: &str,
        origins: &[String],
        tags: &[crate::archiver::list::Tag],
    ) -> Result<(), Error> {
        log::info!("[{}] Importing", id);

        let mut conn = pool.get()?;

        let mut trans = conn.transaction()?;

        let (ch1, de) = Self::chapter(id, 1)?;

        // always some
        if let Some(details) = de {
            let story_id = Self::commit_story(&mut trans, &details, origins, tags)?;
            Self::commit_chapter(&mut trans, &story_id, ch1, 1)?;

            log::info!("[{}] Chapters: {}", id, details.chapters);

            // only run if there are more than one chapter
            if details.chapters != 1 {
                // if there are only two chapters, get the next
                if details.chapters == 2 {
                    sleep();

                    let (ch2, _) = Self::chapter(id, 1)?;
                    Self::commit_chapter(&mut trans, &story_id, ch2, 2)?;
                } else {
                    // more than two, start loop
                    for c in 2..=details.chapters {
                        sleep();

                        let (ch_, _) = Self::chapter(id, c)?;
                        Self::commit_chapter(&mut trans, &story_id, ch_, c)?;
                    }
                }
            }
        }

        trans.commit()?;

        log::info!("[{}] Imported", id);

        Ok(())
    }

    fn commit_story(
        conn: &mut Transaction,
        details: &Details,
        origins: &[String],
        tags: &[crate::archiver::list::Tag],
    ) -> Result<String, Error> {
        log::info!("[{}] Committing", details.name);

        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO Story(Id, Name, Summary, Language, Rating, State, Created, Updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?);",
            rusqlite::params![
                id,
                details.name,
                details.summary,
                details.language,
                details.rating,
                details.state,
                details.created,
                details.updated,
            ],
        )?;

        let author_id = if let Some(existing_id) = conn
            .query_row(
                "SELECT Id FROM Author WHERE Name = ?;",
                rusqlite::params![details.author],
                |row| row.get::<_, String>("Id"),
            )
            .optional()?
        {
            log::info!("[author/{}] Existing", details.author);

            existing_id
        } else {
            log::info!("[author/{}] New", details.author);

            let new_id = Uuid::new_v4().to_string();

            conn.execute(
                "INSERT INTO Author(Id, Name) VALUES (?, ?);",
                rusqlite::params![new_id, details.author],
            )?;

            new_id
        };

        conn.execute(
            "INSERT INTO StoryAuthor(StoryId, AuthorId) VALUES (?, ?);",
            rusqlite::params![id, author_id],
        )?;

        for origin in origins {
            log::info!("[origin/{}] Committing", origin);

            let origin_id = if let Some(existing_id) = conn
                .query_row(
                    "SELECT Id FROM Origin WHERE Name = ?;",
                    rusqlite::params![origin],
                    |row| row.get::<_, String>("Id"),
                )
                .optional()?
            {
                log::info!("[origin/{}] Existing", origin);

                existing_id
            } else {
                log::info!("[origin/{}] New", origin);

                let new_id = Uuid::new_v4().to_string();

                conn.execute(
                    "INSERT INTO Origin(Id, Name) VALUES (?, ?);",
                    rusqlite::params![new_id, origin],
                )?;

                new_id
            };

            conn.execute(
                "INSERT INTO StoryOrigin(StoryId, OriginId) VALUES (?, ?);",
                rusqlite::params![id, origin_id],
            )?;

            log::info!("[origin/{}] Committed", origin);
        }

        for tag in tags {
            log::info!("[tag/{}] Committing", tag.name);

            let tag_id = if let Some(existing_id) = conn
                .query_row(
                    "SELECT Id FROM Tag WHERE Name = ? AND Type = ?;",
                    rusqlite::params![tag.name, tag.tag_type],
                    |row| row.get::<_, String>("Id"),
                )
                .optional()?
            {
                log::info!("[tag/{}] Existing", tag.name);

                existing_id
            } else {
                log::info!("[tag/{}] New", tag.name);

                let new_id = Uuid::new_v4().to_string();

                conn.execute(
                    "INSERT INTO Tag(Id, Name, Type) VALUES (?, ?, ?);",
                    rusqlite::params![new_id, tag.name, tag.tag_type],
                )?;

                new_id
            };

            conn.execute(
                "INSERT INTO StoryTag(StoryId, TagId) VALUES (?, ?);",
                rusqlite::params![id, tag_id],
            )?;

            log::info!("[tag/{}] Committed", tag.name);
        }

        log::info!("[{}] Committed", details.name);

        Ok(id)
    }

    fn commit_chapter(
        conn: &mut Transaction,
        story: &str,
        chapter: Chapter,
        place: u32,
    ) -> Result<(), Error> {
        log::info!("[chapter/{}] Committing", place);

        let chapter_id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO Chapter(Id, Name, Raw, Rendered, Words) VALUES (?, ?, ?, ?, ?);",
            rusqlite::params![
                chapter_id,
                chapter.name,
                chapter.text,
                ammonia::clean(&markdown_to_html(&chapter.text, &ComrakOptions::default())),
                word_count(&chapter.text),
            ],
        )?;

        conn.execute(
            "INSERT INTO StoryChapter(StoryId, ChapterId, Place) VALUES (?, ?, ?);",
            rusqlite::params![story, chapter_id, place],
        )?;

        log::info!("[chapter/{}] Committed", place);

        Ok(())
    }

    fn chapter(id: &str, chapter: u32) -> Result<(Chapter, Option<Details>), Error> {
        log::info!("[chapter/{}] Scraping", chapter);

        let mut res = CLIENT
            .get(&format!("https://www.fanfiction.net/s/{}/{}/", id, chapter))
            .send()?;

        if res.status().is_success() {
            let html = Html::parse_document(&res.text()?);

            let chapter_html = html
                .select(&CHAPTER_TEXT_SELECTOR)
                .next()
                .expect("[CHAPTER_TEXT_SELECTOR] HTML is missing the chapter text node, did the html change?")
                .inner_html();

            let mut chapter_text = CLIENT
                .post("http://localhost:8902/")
                .header("Content-Type", "text/plain")
                .body(chapter_html)
                .send()?;

            if chapter_text.status().is_success() {
                log::info!("[chapter/{}] Scrapped", chapter);

                Ok((
                    Chapter {
                        name: html
                            .select(&CHAPTER_NAME_SELECTOR)
                            .next()
                            .expect("[CHAPTER_NAME_SELECTOR] HTML is missing the chapter name node, did the html change?")
                            .text()
                            .next()
                            .expect("[CHAPTER_NAME_SELECTOR] No text in selected element")
                            .split(' ')
                            .skip(1)
                            .collect::<Vec<_>>()
                            .join(" "),
                        text: chapter_text.text()?,
                    },
                    if chapter == 1 {
                        Some(Details::parse(
                            html
                                .select(&STORY_AUTHOR_SELECTOR)
                                .next()
                                .expect("[STORY_AUTHOR_SELECTOR] HTML is missing the chapter name node, did the html change?")
                                .text()
                                .collect::<Vec<_>>()
                                .join(""),
                            html
                                .select(&STORY_NAME_SELECTOR)
                                .next()
                                .expect("[STORY_NAME_SELECTOR] HTML is missing the chapter name node, did the html change?")
                                .text()
                                .collect::<Vec<_>>()
                                .join(""),
                            html
                                .select(&STORY_SUMMARY_SELECTOR)
                                .next()
                                .expect("[STORY_SUMMARY_SELECTOR] HTML is missing the chapter name node, did the html change?")
                                .text()
                                .collect::<Vec<_>>()
                                .join(""),
                            html
                                .select(&STORY_DETAILS_SELECTOR)
                                .next()
                                .expect(
                                    "[STORY_DETAILS_SELECTOR] HTML is missing the story details node, did the html change?",
                                )
                                .text()
                                .collect::<Vec<_>>()
                                .join(""),
                        ))
                    } else {
                        None
                    },
                ))
            } else {
                log::error!(
                    "[chapter/{}] Non OK response from localhost: {}",
                    chapter,
                    chapter_text.status()
                );
                Err(Error::custom("Non OK response from localhost"))
            }
        } else {
            log::error!(
                "[chapter/{}] Non OK response fanfiction.net: {}",
                chapter,
                res.status()
            );
            Err(Error::custom("Non OK response from fanfiction.net"))
        }
    }
}

pub struct Chapter {
    name: String,
    text: String,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Details {
    author: String,
    name: String,
    summary: String,
    rating: Rating,
    language: Language,
    chapters: u32,
    state: State,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}

impl Details {
    fn parse(author: String, name: String, summary: String, details: String) -> Self {
        log::info!("[details] Parsing");

        let chunks = details.split(" - ").collect::<Vec<&str>>();

        let mut builder = DetailsBuilder::new();

        builder.author(author);
        builder.name(name);
        builder.summary(summary);

        builder.rating(Self::parse_rating(chunks[0].trim()));
        builder.language(Self::parse_language(chunks[1].trim()));

        for chunk in chunks {
            let chunk = chunk.trim();

            if chunk.starts_with("Updated") {
                builder.updated(Self::parse_time(chunk));
            } else if chunk.starts_with("Published") {
                builder.created(Self::parse_time(chunk));
            } else if chunk.starts_with("Status") {
                builder.state(Self::parse_state(chunk));
            } else if chunk.starts_with("Chapters") {
                builder.chapters(Self::parse_chapters(chunk));
            }
        }

        log::info!("[details] Parsed");

        builder.build()
    }

    fn parse_rating(chunk: &str) -> Rating {
        let chunks = chunk.split(": ").collect::<Vec<&str>>();

        match chunks[1].to_lowercase().as_str() {
            "fiction ma" | "fiction  ma" => Rating::Explicit,
            "fiction m" | "fiction  m" => Rating::Mature,
            "fiction t" | "fiction  t" => Rating::Teen,
            "fiction k" | "fiction k+" | "fiction  k" | "fiction  k+" => Rating::General,
            _ => panic!("Unknown rating, Fanfiction.net must be in English"),
        }
    }

    fn parse_language(chunk: &str) -> Language {
        match chunk.to_lowercase().as_str() {
            "english" => Language::English,
            _ => panic!("Unsupported language"),
        }
    }

    fn parse_chapters(chunk: &str) -> u32 {
        let chunks = chunk.split(": ").collect::<Vec<&str>>();

        chunks[1]
            .trim()
            .parse()
            .expect("unable to parse string to u32")
    }

    fn parse_state(chunk: &str) -> State {
        let chunks = chunk.split(": ").collect::<Vec<&str>>();

        match chunks[1].trim().to_lowercase().as_str() {
            "complete" => State::Completed,
            _ => State::InProgress,
        }
    }

    fn parse_time(chunk: &str) -> DateTime<Utc> {
        let chunks = chunk.split(": ").collect::<Vec<&str>>();

        let date = chunks[1].trim();

        if date.len() < 5 {
            DateTime::parse_from_str(
                &format!("{}/2019 0:0:0 +0000", date),
                "%m/%d/%Y %H:%M:%S %z",
            )
            .expect("unable to parse string to datetime")
            .into()
        } else {
            DateTime::parse_from_str(&format!("{} 0:0:0 +0000", date), "%m/%d/%Y %H:%M:%S %z")
                .expect("unable to parse string to datetime")
                .into()
        }
    }
}

struct DetailsBuilder {
    author: Option<String>,
    name: Option<String>,
    summary: Option<String>,
    rating: Option<Rating>,
    language: Option<Language>,
    chapters: Option<u32>,
    state: Option<State>,
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
}

impl DetailsBuilder {
    fn new() -> DetailsBuilder {
        DetailsBuilder {
            author: None,
            name: None,
            summary: None,
            rating: None,
            language: None,
            chapters: None,
            state: None,
            created: None,
            updated: None,
        }
    }

    fn build(self) -> Details {
        Details {
            author: self.author.unwrap(),
            name: self.name.unwrap(),
            summary: self.summary.unwrap(),
            rating: self.rating.unwrap(),
            language: self.language.unwrap(),
            chapters: self.chapters.unwrap_or(1),
            state: self.state.unwrap_or(State::InProgress),
            created: self.created.unwrap(),
            updated: self.updated.unwrap(),
        }
    }

    fn author(&mut self, author: String) {
        self.author = Some(author);
    }

    fn name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn summary(&mut self, summary: String) {
        self.summary = Some(summary);
    }

    fn rating(&mut self, rating: Rating) {
        self.rating = Some(rating);
    }

    fn language(&mut self, language: Language) {
        self.language = Some(language);
    }

    fn chapters(&mut self, chapters: u32) {
        self.chapters = Some(chapters);
    }

    fn state(&mut self, state: State) {
        self.state = Some(state);
    }

    fn created(&mut self, created: DateTime<Utc>) {
        self.created = Some(created);

        if self.updated.is_none() {
            self.updated = Some(created);
        }
    }

    fn updated(&mut self, updated: DateTime<Utc>) {
        self.updated = Some(updated);
    }
}
