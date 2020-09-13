use {
    crate::{readable::Readable, utils::filters},
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_common::{
        models,
        version::{GIT_VERSION, VERSION},
    },
};

#[derive(Template)]
#[template(path = "edit/chapter.html")]
pub struct Chapter {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
    duration: Duration,

    story: models::Story,
    chapter: models::Chapter,
}

impl Chapter {
    pub fn new(
        title: impl Into<String>,
        time: DateTime<Utc>,
        story: models::Story,
        chapter: models::Chapter,
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
}
