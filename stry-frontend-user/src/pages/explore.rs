use {
    crate::pagination::Pagination,
    askama::Template,
    stry_common::{models, utils::Readable},
};

#[derive(Template)]
#[template(path = "explore.html")]
pub struct AuthorList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    pagination: String,

    authors: Vec<models::Author>,
}

impl AuthorList {
    pub fn new(
        title: impl Into<String>,
        page: u32,
        pages: u32,
        authors: Vec<models::Author>,
    ) -> Self {
        Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
            title: title.into(),
            search: None,
            pagination: Pagination::new("/explore/author", None, pages, page).to_string(),
            authors,
        }
    }
}
