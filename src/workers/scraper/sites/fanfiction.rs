use {
    crate::workers::scraper::{
        converter,
        models::{Chapter, Details, Language, Rating, State, Story},
        query::Document,
        utils::{self, req, sleep, word_count},
        Uri,
    },
    chrono::{TimeZone, Utc},
    std::str,
};

const NAME: &str = "fanfiction";

const CHAPTER_NAME: &str = "select#chap_select > option[selected]";
const CHAPTER_TEXT: &str = "#storytext";

const STORY_AUTHOR: &str = "#profile_top > a.xcontrast_txt";
const STORY_DETAILS: &str = "#profile_top > span.xgray.xcontrast_txt";
const STORY_DETAILS_RATING: &str =
    r#"#profile_top > span.xgray.xcontrast_txt > a[target="rating"]"#;
const STORY_DETAILS_DATES: &str = "#profile_top > span.xgray.xcontrast_txt > span[data-xutime]";
const STORY_SUMMARY: &str = "#profile_top > div.xcontrast_txt";
const STORY_NAME: &str = "#profile_top > b.xcontrast_txt";
const STORY_ORIGINS: &str = "#pre_story_links > span.lc-left > a.xcontrast_txt";

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

    let body = req(&url).await?;

    let details = tokio::task::spawn_blocking(|| get_details(body))
        .await
        .expect("Thread pool closed")?;

    let chapters = details.chapters;

    let mut story = Story::new(details);

    tracing::info!("[{}] Beginning chapter scraping", url);

    if chapters != 1 {
        for page in 1..=chapters {
            tracing::info!("[{}] Scraping chapter {}", url, page);

            sleep().await?;

            let url = format!("https://www.fanfiction.net/s/{}/{}", id, page)
                .as_str()
                .parse()?;

            let body = req(&url).await?;

            story.chapters.push(
                tokio::task::spawn_blocking(|| get_chapter(body))
                    .await
                    .expect("Thread pool closed")?,
            );
        }
    } else {
        tracing::info!("[{}] Scraping chapter {}", url, 1);

        sleep().await?;

        let url = format!("https://www.fanfiction.net/s/{}/{}", id, 1)
            .as_str()
            .parse()?;

        let body = req(&url).await?;

        story.chapters.push(
            tokio::task::spawn_blocking(|| get_chapter(body))
                .await
                .expect("Thread pool closed")?,
        );
    }

    story.words = story.chapters.iter().map(|c| word_count(&c.main)).sum();

    Ok(story)
}

pub fn get_details(body: impl Into<Document>) -> anyhow::Result<Details> {
    let html = body.into();

    let name = utils::string(&html, STORY_NAME, NAME)?;
    let summary = utils::string(&html, STORY_SUMMARY, NAME)?;
    let details = utils::string(&html, STORY_DETAILS, NAME)?;

    let author: String = html
        .select(STORY_AUTHOR)
        .into_iter()
        .next()
        .map(|ele| ele.text())
        .flatten()
        .ok_or_else(|| anyhow::anyhow!(
            "Sector element for site {} not found: {}",
            NAME,
            STORY_AUTHOR
        ))?;

    let origins: Vec<String> = html
        .select(STORY_ORIGINS)
        .into_iter()
        .last()
        .map(|ele| {
            ele.text()
                .map(|mut text| {
                    if text.ends_with("Crossover") {
                        let len = text.len();
                        let new_len = len.saturating_sub("Crossover".len());

                        text.truncate(new_len);
                    }

                    text
                })
                .map(|text| {
                    text.split(" + ")
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .collect::<Vec<String>>()
                })
        })
        .flatten()
        .ok_or_else(|| anyhow::anyhow!(
            "Sector element for site {} not found: {}",
            NAME,
            STORY_ORIGINS
        ))?;

    let mut chapters = 1u32;
    let mut language = Language::English;
    let rating = match utils::string(&html, STORY_DETAILS_RATING, NAME)?
        .as_str()
        .split(' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .nth(1)
    {
        Some("MA") => Rating::Explicit,
        Some("M") => Rating::Mature,
        Some("T") => Rating::Teen,
        Some("K") | Some("K+") => Rating::General,
        rating => unreachable!("Unknown rating found, please report this: {:?}", rating),
    };
    let mut state = State::InProgress;

    let (created, mut updated) = match html.select(STORY_DETAILS_DATES).as_slice() {
        [first] => {
            let published = first.attr("data-xutime").ok_or_else(|| {
                anyhow::anyhow!(
                    "Element attribute for site {} not found: {}",
                    NAME,
                    "data-xutime"
                )
            })?;

            (Utc.datetime_from_str(&published, "%s").ok(), None)
        }
        [first, second] => {
            let updated = first.attr("data-xutime").ok_or_else(|| {
                anyhow::anyhow!(
                    "Element attribute for site {} not found: {}",
                    NAME,
                    "data-xutime"
                )
            })?;
            let published = second.attr("data-xutime").ok_or_else(|| {
                anyhow::anyhow!(
                    "Element attribute for site {} not found: {}",
                    NAME,
                    "data-xutime"
                )
            })?;

            (
                Utc.datetime_from_str(&published, "%s").ok(),
                Utc.datetime_from_str(&updated, "%s").ok(),
            )
        }
        _ => {
            return Err(anyhow::anyhow!("Unparsable date time for site {}", NAME));
        }
    };

    let words = details.split('-').count();

    for (i, s) in details.split('-').map(str::trim).rev().enumerate() {
        if s.starts_with("Chapters: ") {
            if let Some(ch) = s
                .split(':')
                .map(str::trim)
                .nth(1)
                .and_then(|s| s.parse::<u32>().ok())
            {
                chapters = ch;
            }
        }

        if i == words - 2 {
            language = match s {
                "English" => Language::English,
                _ => unreachable!(),
            };
        }

        if s.starts_with("Status: ") {
            if let Some(st) = s.split(':').map(str::trim).nth(1).map(|s| match s {
                "Complete" => State::Completed,
                _ => unreachable!(),
            }) {
                state = st;
            }
        }
    }

    if updated.is_none() && created.is_some() {
        updated = created;
    }

    Ok(Details {
        name,
        summary,

        chapters,
        language,
        rating,
        state,

        authors: vec![author],
        origins,
        tags: Vec::new(),

        created: created
            .ok_or_else(|| anyhow::anyhow!("Unparsable date time for site {}", NAME))?,
        updated: updated
            .ok_or_else(|| anyhow::anyhow!("Unparsable date time for site {}", NAME))?,
    })
}

pub fn get_chapter(body: impl Into<Document>) -> anyhow::Result<Chapter> {
    let html = body.into();

    let mut main = converter::parse(
        html.select(CHAPTER_TEXT)
            .first()
            .and_then(|node| node.inner_html())
            .expect("[chapter_text] HTML is missing the chapter text node, did the html change?"),
    )?;

    while main.ends_with(' ') {
        let len = main.len();
        let new_len = len.saturating_sub(" ".len());

        main.truncate(new_len);
    }

    Ok(Chapter {
        name: html
            .select(CHAPTER_NAME)
            .first()
            .and_then(|cn| cn.text())
            .map(|cn| cn.split(' ').skip(1).collect::<Vec<_>>().join(" "))
            .or_else(|| {
                Some(
                    utils::string(&html, STORY_NAME, NAME).expect(
                        "[chapter_name] Unable to use story title as fallback chapter title",
                    ),
                )
            })
            .expect("[chapter_name] No text in selected element"),
        words: word_count(&main),
        pre: String::new(),
        post: String::new(),
        main,
    })
}
