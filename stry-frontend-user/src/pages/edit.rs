use {
    crate::{readable::Readable, utils::filters},
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_generated_version::{GIT_VERSION, VERSION},
};

#[derive(Template)]
#[template(path = "edit/chapter.html")]
pub struct Chapter {
    version: &'static str,
    git: &'static str,

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
    ) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
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
