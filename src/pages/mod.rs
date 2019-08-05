use {
    crate::{Chapter, Error, Pool, Story, CSS, GIT, VERSION},
    actix_web::{
        http,
        web::{self, Data, Path, Query},
        HttpResponse,
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
};

macro_rules! hum_num {
    ($num:expr) => {{
        use num_format::ToFormattedString;
        $num.to_formatted_string(&num_format::Locale::en)
    }};
}

macro_rules! hum_time {
    ($time:expr) => {{
        $time.format("%b %e, %Y")
    }};
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub css: &'static str,
    pub git: &'static str,
    pub version: &'static str,
    pub stories: Vec<Story>,
    pub title: &'static str,
    pub search: Option<String>,
    pub duration: Duration,
}

impl Index {
    fn render(
        time: DateTime<Utc>,
        title: &'static str,
        stories: Vec<Story>,
        search: Option<String>,
    ) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Self {
                css: CSS,
                git: GIT,
                version: VERSION,
                title,
                stories,
                search,
                duration: Utc::now().signed_duration_since(time),
            }
            .render()?,
        ))
    }

    pub fn home(pool: Data<Pool>) -> Result<HttpResponse, Error> {
        let now = Utc::now();

        let stories = Story::all(pool.get_ref().clone())?;

        Index::render(now, "Home", stories, None)
    }
}

#[derive(Template)]
#[template(path = "chapter.html")]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ChapterPage {
    pub css: &'static str,
    pub git: &'static str,
    pub version: &'static str,
    pub story: Story,
    pub chapter: Chapter,
    pub page: u32,
    pub next: u32,
    pub prev: u32,
    pub title: String,
    pub search: Option<String>,
    pub duration: Duration,
}

impl ChapterPage {
    fn render(
        time: DateTime<Utc>,
        story: Story,
        chapter: Chapter,
        page: u32,
        next: u32,
        prev: u32,
    ) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Self {
                css: CSS,
                git: GIT,
                version: VERSION,
                title: format!("Chapter {} | {}", page, story.name),
                story,
                chapter,
                page,
                next,
                prev,
                search: None,
                duration: Utc::now().signed_duration_since(time),
            }
            .render()?,
        ))
    }

    pub fn index((pool, path): (Data<Pool>, Path<(String, u32)>)) -> Result<HttpResponse, Error> {
        let (story_id, chapter_number) = path.into_inner();
        let time = Utc::now();

        let story = Story::get(pool.get_ref().clone(), &story_id)?;

        if chapter_number.lt(&story.chapters) && chapter_number.gt(&0) {
            let chapter = Chapter::story(pool.get_ref().clone(), &story_id, chapter_number)?;

            Self::render(
                time,
                story,
                chapter,
                chapter_number,
                chapter_number + 1,
                chapter_number - 1,
            )
        } else {
            Ok(HttpResponse::MovedPermanently()
                .header(http::header::LOCATION, format!("/story/{}/1", story_id))
                .finish())
        }
    }
}
