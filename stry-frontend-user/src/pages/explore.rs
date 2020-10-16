use {
    crate::pagination::Pagination,
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_common::{models, utils::Readable},
};

#[derive(Template)]
#[template(path = "explore.html")]
pub struct AuthorList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
    duration: Duration,

    pagination: String,

    authors: Vec<models::Author>,
}

impl AuthorList {
    pub fn new(
        title: impl Into<String>,time: DateTime<Utc>,
        page: u32,
        pages: u32,
        authors: Vec<models::Author>,
    ) -> Self {
        Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
            title: title.into(),
            duration: Utc::now().signed_duration_since(time),
            search: None,
            pagination: Pagination::new("/explore/author", None, pages, page).to_string(),
            authors,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}
