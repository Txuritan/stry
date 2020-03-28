use {
    crate::{models, Pagination, Readable},
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
    page: u32,

    story: models::Story,
    chapter: models::Chapter,
}

impl Chapter {
    pub fn new(
        title: impl Into<String>,
        page: u32,
        story: models::Story,
        chapter: models::Chapter,
    ) -> Self {
        Self {
            version: crate::VERSION,
            git: crate::GIT_VERSION,
            title: title.into(),
            search: None,
            pagination: Pagination::new(
                format!("/story/{}", story.id),
                Some("/"),
                story.chapters,
                page,
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
