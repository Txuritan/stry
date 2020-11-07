use {
    crate::{i18n, pages::Meta, readable::Readable, utils::Identifiers},
    askama::Template,
    chrono::{DateTime, Duration, Utc},
};

#[derive(Template)]
#[template(path = "edit/chapter.html")]
pub struct Chapter {
    meta: Meta,

    title: String,
    search: Option<String>,
    duration: Duration,

    story: stry_models::Story,
    chapter: stry_models::Chapter,
}

impl Chapter {
    pub fn new(
        title: impl Into<String>,
        time: DateTime<Utc>,
        story: stry_models::Story,
        chapter: stry_models::Chapter,
        user_lang: Identifiers,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),
            title: title.into(),
            search: None,
            duration: Utc::now().signed_duration_since(time),
            story,
            chapter,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}
