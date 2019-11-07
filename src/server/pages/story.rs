use {
    crate::{
        models::{Chapter, Story},
        schema::Backend,
        Error, Readable,
    },
    askama::Template,
    warp::{reject::custom, reply, Rejection, Reply},
};

pub async fn index(_story_id: String, _backend: Backend) -> Result<impl Reply, Rejection> {
    Ok(reply::html("story"))
}

#[derive(Template)]
#[template(path = "chapter.html")]
struct ChapterPage {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    page: u32,
    prev: u32,
    next: u32,

    story: Story,
    chapter: Chapter,
}

impl ChapterPage {
    fn new(title: impl Into<String>, page: u32, story: Story, chapter: Chapter) -> Self {
        Self {
            version: crate::VERSION,
            git: crate::GIT_VERSION,
            title: title.into(),
            search: None,
            prev: if page == 1 { page } else { page - 1 },
            next: if page >= story.chapters {
                page
            } else {
                page + 1
            },
            page,
            story,
            chapter,
        }
    }
}

pub async fn chapter(
    story_id: String,
    mut chapter_page: u32,
    backend: Backend,
) -> Result<impl Reply, Rejection> {
    tokio_executor::blocking::run(move || {
        if chapter_page == 0 {
            chapter_page = 1;
        }

        let story =
            Story::get(backend.clone(), &story_id).map_err(|err| custom(Error::new(err)))?;

        if chapter_page <= story.chapters && chapter_page != 0 {
            let chapter = Chapter::of_story(backend.clone(), &story.id, chapter_page)
                .map_err(|err| custom(Error::new(err)))?;

            let rendered: String = ChapterPage::new(
                format!(
                    "chapter {}: {} | {}",
                    chapter_page, chapter.name, story.name
                ),
                chapter_page,
                story,
                chapter,
            )
            .render()
            .map_err(|err| custom(Error::new(err)))?;

            Ok(reply::html(rendered))
        } else {
            Err(custom(Error::moved(format!("/story/{}/1", story_id))))
        }
    })
    .await
}
