pub mod dashboard;
// pub mod explore;
pub mod story;

use {
    crate::{pagination::Pagination, utils::WebError},
    askama::Template,
    stry_common::{models, utils::Readable},
};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage<T>
where
    T: std::fmt::Display,
{
    version: &'static str,
    git: &'static str,

    title: T,
    search: Option<String>,

    error: WebError,
}

impl<T> ErrorPage<T>
where
    T: std::fmt::Display,
{
    pub fn new(
        title: T,
        code: u32,
        name: &'static str,
        icon: &'static str,
        help: &'static str,
    ) -> Self {
        Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,

            title,
            error: WebError {
                code,
                name,
                icon,
                help,
            },

            search: None,
        }
    }

    pub fn not_found(title: T) -> Self {
        Self::new(title, 404, "Not Found", "danger", "")
    }

    pub fn server_error(title: T) -> Self {
        Self::new(
            title,
            503,
            "Server Error",
            "danger",
            "Check the log for more information",
        )
    }
}

#[derive(Template)]
#[template(path = "explore.html")]
pub struct ResourceList<'a> {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    typ: String,

    pagination: String,

    resources: Vec<&'a dyn models::Resource>,
}

impl<'a> ResourceList<'a> {
    pub fn new(
        title: impl Into<String>,
        typ: models::RouteType,
        page: u32,
        pages: u32,
        resources: Vec<&'a dyn models::Resource>,
    ) -> Self {
        Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
            title: title.into(),
            search: None,
            pagination: Pagination::new(format!("/explore/{}", typ), None, pages, page).to_string(),
            typ: typ.to_string(),
            resources,
        }
    }
}

#[derive(Template)]
#[template(path = "story_list.html")]
pub struct StoryList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    pagination: String,

    stories: Vec<models::Story>,
}

impl StoryList {
    pub fn new(
        title: impl Into<String>,
        url: impl Into<String>,
        page: u32,
        pages: u32,
        stories: Vec<models::Story>,
    ) -> Self {
        Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
            title: title.into(),
            search: None,
            pagination: Pagination::new(url, None, pages, page).to_string(),
            stories,
        }
    }
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct Search {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    pagination: String,

    stories: Vec<models::Story>,
}

impl Search {
    pub fn new(
        title: impl Into<String>,
        search: String,
        page: u32,
        pages: u32,
        stories: Vec<models::Story>,
    ) -> anyhow::Result<Self> {
        #[derive(serde::Serialize)]
        struct SearchUrl<'s> {
            search: &'s str,
        }

        Ok(Self {
            version: stry_common::VERSION,
            git: stry_common::GIT_VERSION,
            title: title.into(),
            pagination: Pagination::new(
                format!(
                    "/search/{}",
                    serde_urlencoded::to_string(SearchUrl { search: &search })?
                ),
                None,
                pages,
                page,
            )
            .to_string(),
            search: Some(search),
            stories,
        })
    }
}
