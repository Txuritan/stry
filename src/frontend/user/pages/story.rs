use {
    crate::{
        frontend::user::{pagination::Pagination, readable::Readable},
        models,
        version::{GIT_VERSION, VERSION},
    },
    askama::Template,
};

#[derive(Template)]
#[template(path = "story/chapter.html")]
pub struct Chapter {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    pagination: String,
    page: i32,

    story: models::Story,
    chapter: models::Chapter,
}

impl Chapter {
    pub fn new(
        title: impl Into<String>,
        page: i32,
        story: models::Story,
        chapter: models::Chapter,
    ) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: title.into(),
            search: None,
            pagination: Pagination::new(
                format!("/story/{}", story.id),
                Some("/"),
                story.chapters as u32,
                page as u32,
            )
            .to_string(),
            page,
            story,
            chapter,
        }
    }
}

// TODO: list of chapter tiles
#[derive(Template)]
#[template(path = "story/index.html")]
pub struct Index {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    story: models::Story,
}
