use {
    crate::{
        i18n,
        pages::Meta,
        pagination::Pagination,
        utils::{filters, Identifiers, BoolAsNum},
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
};

#[derive(Template)]
#[template(path = "story/chapter.html")]
pub struct Chapter {
    meta: Meta,

    title: String,
    search: Option<String>,
    duration: Duration,

    pagination: String,
    page: i32,

    story: stry_models::Story,
    chapter: stry_models::Chapter,
}

impl Chapter {
    pub fn new(
        title: impl Into<String>,
        time: DateTime<Utc>,
        page: i32,
        story: stry_models::Story,
        chapter: stry_models::Chapter,
        user_lang: Identifiers,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang.clone()),
            title: title.into(),
            duration: Utc::now().signed_duration_since(time),
            search: None,
            pagination: Pagination::new(
                Meta::new(user_lang),
                format!("/story/{}", story.id),
                Some("/"),
                Some("#chapter-name"),
                story.chapters as u32,
                page as u32,
            )
            .to_string(),
            page,
            story,
            chapter,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

// TODO: list of chapter tiles
#[derive(Template)]
#[template(path = "story/index.html")]
pub struct Index {
    meta: Meta,

    title: String,
    search: Option<String>,
    duration: Duration,

    story: stry_models::Story,
}
