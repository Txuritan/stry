use {
    crate::workers::scraper::{
        converter,
        models::{Chapter, Details, Language, Rating, State, Story},
        query::{Document, Element},
        utils::{self, req, word_count},
        Uri,
    },
    chrono::{DateTime, NaiveDate, Utc},
    std::{fmt::Write, sync::Arc},
};

const NAME: &str = "archive of our own";

const MULTIPLE_CHAPTER_NAME: &str = r#"#chapters > .chapter > div[role="complementary"] > h3"#;
const SINGLE_CHAPTER_NAME: &str = r#"#workskin > .preface > .title"#;
const CHAPTER_SINGLE: &str = r#"#chapters .userstuff > p"#;
const CHAPTER_MULTI_START: &str = r#"#chapters > #chapter-"#;
const CHAPTER_MULTI_END: &str = r#" .userstuff > p"#;

const STORY_AUTHOR: &str = r#"#workskin > .preface > .byline.heading > a[rel="author"]"#;
const STORY_SUMMARY: &str = "#workskin > .preface > .summary > blockquote";
const STORY_NAME: &str = "#workskin > .preface > .title";

const STORY_RATING: &str = ".work > .rating.tags > ul > li > .tag";
const STORY_ORIGINS: &str = ".work > .fandom.tags > ul > li > .tag";

const STORY_STATS_CHAPTERS: &str = "dl.work > dd.stats > dl.stats > dd.chapters";
const STORY_STATS_LANGUAGE: &str = "dl.work > dd.language";
const STORY_STATS_CREATED: &str = "dl.work > dd.stats > dl.stats > dd.published";
const STORY_STATS_UPDATED: &str = "dl.work > dd.stats > dl.stats > dd.status";

pub fn id_from_url(url: &Uri) -> anyhow::Result<String> {
    url.path()
        .split('/')
        .filter(|s| !s.is_empty())
        .nth(1)
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("Unable to find story ID in: {}", url))
}

pub async fn scrape(url: &Uri) -> anyhow::Result<Story> {
    let id = url
        .path()
        .split('/')
        .filter(|s| !s.is_empty())
        .nth(1)
        .expect("No story ID found in URL");

    tracing::info!("[{}] Scraping initial details", url);

    let url = format!(
        "https://archiveofourown.org/works/{}?view_full_work=true",
        id
    )
    .parse::<Uri>()?;

    let body = req(&url).await?;
    let html = Arc::new(body);

    let details = tokio::task::spawn_blocking({
        let html = html.clone();
        || get_details(html)
    })
    .await
    .expect("Thread pool closed")?;

    let chapters = details.chapters;

    let mut story = Story::new(details);

    tracing::info!("[{}] Beginning chapter scraping", url);

    for chapter_number in 1..=chapters {
        let chapter = tokio::task::spawn_blocking({
            let html = html.clone();

            move || get_chapter(html, chapter_number)
        })
        .await
        .expect("Thread pool closed")?;

        story.chapters.push(chapter);
    }

    story.words = story.chapters.iter().map(|c| word_count(&c.main)).sum();

    Ok(story)
}

pub fn get_details(html: impl Into<Document>) -> anyhow::Result<Details> {
    let html = html.into();

    let authors = utils::string_vec(&html, STORY_AUTHOR, NAME)?;
    let origins = utils::string_vec(&html, STORY_ORIGINS, NAME)?;

    let name = utils::string(&html, STORY_NAME, NAME)?.trim().to_string();
    let summary = converter::parse(utils::inner_html(&html, STORY_SUMMARY, NAME)?)?;

    let chapter_expected = utils::string(&html, STORY_STATS_CHAPTERS, NAME)?;

    let chapters: u32 = chapter_expected
        .split('/')
        .next()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap();

    let language: Language = match utils::string(&html, STORY_STATS_LANGUAGE, NAME)?.trim() {
        "English" => Language::English,
        _ => unreachable!(),
    };

    let rating: Rating = match utils::string(&html, STORY_RATING, NAME)?.trim() {
        "Explicit" => Rating::Explicit,
        "Mature" => Rating::Mature,
        "Teen And Up Audiences" => Rating::Teen,
        "General Audiences" => Rating::General,
        _ => unreachable!(),
    };

    let state = {
        let mut split = chapter_expected.split('/');

        let current: &str = split.next().unwrap();
        let expected: &str = split.next().unwrap();

        if current == expected {
            State::Completed
        } else {
            State::InProgress
        }
    };

    let created: Option<DateTime<Utc>> = NaiveDate::parse_from_str(
        &utils::string(&html, STORY_STATS_CREATED, NAME)?,
        "%Y-%m-%d",
    )
    .map(|date| date.and_hms(0, 0, 0))
    .map(|dt| DateTime::from_utc(dt, Utc))
    .ok();

    let updated: Option<DateTime<Utc>> = if state != State::Completed || chapters != 1 {
        NaiveDate::parse_from_str(
            &utils::string(&html, STORY_STATS_UPDATED, NAME)?,
            "%Y-%m-%d",
        )
        .map(|date| date.and_hms(0, 0, 0))
        .map(|dt| DateTime::from_utc(dt, Utc))
        .ok()
    } else if created.is_some() {
        created
    } else {
        None
    };

    Ok(Details {
        name,
        summary,

        chapters,
        language,
        rating,
        state,

        authors,
        origins,
        tags: Vec::new(),

        created: created
            .ok_or_else(|| anyhow::anyhow!("Unparsable date time for site {}", NAME))?,
        updated: updated
            .ok_or_else(|| anyhow::anyhow!("Unparsable date time for site {}", NAME))?,
    })
}

pub fn get_chapter(html: impl Into<Document>, chapter: u32) -> anyhow::Result<Chapter> {
    let html = html.into();

    let multi = html.select(format!(
        "{}{}{}",
        CHAPTER_MULTI_START, chapter, CHAPTER_MULTI_END
    ));

    let elements: Vec<Element> = if multi.is_empty() {
        html.select(CHAPTER_SINGLE)
    } else {
        multi
    };

    let content = converter::parse(
        elements
            .into_iter()
            .map(|n| n.html())
            .fold(Some(String::new()), |mut buffer, html| {
                if let Some(mut buff) = buffer.take() {
                    if let Some(raw) = html {
                        write!(buff, "{}", raw).unwrap();

                        buffer = Some(buff);
                    }
                }

                buffer
            })
            .expect("[chapter_text] HTML is missing the chapter text node, did the html change?"),
    )?
    .trim()
    .replace('“', "\"")
    .replace('”', "\"");

    let name: String = utils::string(&html, MULTIPLE_CHAPTER_NAME, NAME)
        .or_else(|_| utils::string(&html, SINGLE_CHAPTER_NAME, NAME))
        .map(|title| {
            let mut title = title.trim().to_string();

            if title.starts_with(':') {
                title.remove(0);

                title.trim().to_string()
            } else {
                title
            }
        })?;

    Ok(Chapter {
        name,
        words: word_count(&content),
        pre: String::new(),
        post: String::new(),
        main: content,
    })
}
