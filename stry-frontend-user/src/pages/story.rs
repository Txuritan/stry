use {
    crate::pagination::Pagination,
    askama::Template,
    stry_common::{models, utils::Readable},
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
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
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
