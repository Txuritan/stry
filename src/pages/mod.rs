use {
    crate::{
        readable,
        server::{Request, Response},
        Author, Chapter, Error, Origin, Pool, Story, Tag, CSS, GIT, VERSION,
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
};

macro_rules! hum_num {
    ($num:expr) => {{
        readable($num)
    }};
}

macro_rules! hum_time {
    ($time:expr) => {{
        $time.format("%b %e, %Y")
    }};
}

fn no_pool() -> Result<Response, Error> {
    Ok(Response::InternalError()
        .header("Content-Type", "text/plain")
        .body(""))
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
    ) -> Result<Response, Error> {
        Ok(Response::Ok().html(
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

    pub fn home(req: Request) -> Result<Response, Error> {
        let now = Utc::now();

        match req.state().get::<Pool>() {
            Some(pool) => {
                let stories = Story::all(pool.clone())?;

                Index::render(now, "Home", stories, None)
            }
            None => no_pool(),
        }
    }

    pub fn author(req: Request) -> Result<Response, Error> {
        let now = Utc::now();

        match req.state().get::<Pool>() {
            Some(pool) => {
                let author_id = &req.params.get("author").unwrap();

                let stories = Author::all(pool.clone(), &author_id)?;

                Index::render(now, "Author", stories, None)
            }
            None => no_pool(),
        }
    }

    pub fn origin(req: Request) -> Result<Response, Error> {
        let now = Utc::now();

        match req.state().get::<Pool>() {
            Some(pool) => {
                let origin_id = &req.params.get("origin").unwrap();

                let stories = Origin::all(pool.clone(), &origin_id)?;

                Index::render(now, "Origin", stories, None)
            }
            None => no_pool(),
        }
    }

    pub fn tag(req: Request) -> Result<Response, Error> {
        let now = Utc::now();

        match req.state().get::<Pool>() {
            Some(pool) => {
                let tag_id = &req.params.get("tag").unwrap();

                let stories = Tag::all(pool.clone(), &tag_id)?;

                Index::render(now, "Tag", stories, None)
            }
            None => no_pool(),
        }
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
    pub is_first: bool,
    pub is_last: bool,
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
    ) -> Result<Response, Error> {
        Ok(Response::Ok().html(
            Self {
                css: CSS,
                git: GIT,
                version: VERSION,
                title: format!("Chapter {} | {}", page, story.name),
                is_first: page == 1,
                is_last: page == story.chapters,
                story,
                chapter,
                page,
                search: None,
                duration: Utc::now().signed_duration_since(time),
            }
            .render()?,
        ))
    }

    pub fn index(req: Request) -> Result<Response, Error> {
        let time = Utc::now();

        let story_id = &req.params.get("story").unwrap();
        let chapter_number = req.params.get("chapter").unwrap().parse::<u32>()?;

        match req.state().get::<Pool>() {
            Some(pool) => {
                let story = Story::get(pool.clone(), &story_id)?;

                if chapter_number.le(&story.chapters) && chapter_number.gt(&0) {
                    let chapter = Chapter::story(pool.clone(), &story_id, chapter_number)?;

                    Self::render(
                        time,
                        story,
                        chapter,
                        chapter_number,
                    )
                } else {
                    Ok(Response::Location(format!("/story/{}/1", story_id)))
                }
            }
            None => no_pool(),
        }
    }
}
