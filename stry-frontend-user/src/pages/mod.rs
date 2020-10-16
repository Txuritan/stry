pub mod dashboard;
pub mod edit;
// pub mod explore;
pub mod story;

use {
    crate::{
        pagination::Pagination,
        readable::Readable,
        utils::{Resource, WebError},
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_generated_version::{GIT_VERSION, VERSION},
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
    duration: Duration,

    error: WebError,
}

impl<T> ErrorPage<T>
where
    T: std::fmt::Display,
{
    pub fn new(
        title: T,
        time: DateTime<Utc>,
        code: u32,
        name: &'static str,
        icon: &'static str,
        help: &'static str,
    ) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,

            title,
            duration: Utc::now().signed_duration_since(time),

            error: WebError {
                code,
                name,
                icon,
                help,
            },

            search: None,
        }
    }

    pub fn not_found(title: T, time: DateTime<Utc>) -> Self {
        Self::new(title, time, 404, "Not Found", "danger", "")
    }

    pub fn server_error(title: T, time: DateTime<Utc>) -> Self {
        Self::new(
            title,
            time,
            503,
            "Server Error",
            "danger",
            "Check the log for more information",
        )
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "explore.html")]
pub struct ResourceList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
    duration: Duration,

    pagination: String,

    resources: Vec<Resource>,
}

impl ResourceList {
    pub fn new(
        title: impl Into<String>,
        time: DateTime<Utc>,
        url: String,
        page: i32,
        pages: i32,
        resources: Vec<Resource>,
    ) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: title.into(),
            duration: Utc::now().signed_duration_since(time),
            search: None,
            pagination: Pagination::new(url, None, None, pages as u32, page as u32).to_string(),
            resources,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "story_list.html")]
pub struct StoryList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
    duration: Duration,

    pagination: String,

    stories: Vec<stry_models::Story>,
}

impl StoryList {
    pub fn new(
        title: impl Into<String>,
        time: DateTime<Utc>,
        url: impl Into<String>,
        page: i32,
        pages: i32,
        stories: Vec<stry_models::Story>,
    ) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: title.into(),
            duration: Utc::now().signed_duration_since(time),
            search: None,
            pagination: Pagination::new(url, None, None, pages as u32, page as u32).to_string(),
            stories,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct Search {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
    duration: Duration,

    pagination: String,

    stories: Vec<stry_models::Story>,
}

impl Search {
    pub fn new(
        title: impl Into<String>,
        search: String,
        time: DateTime<Utc>,
        page: i32,
        pages: i32,
        stories: Vec<stry_models::Story>,
    ) -> anyhow::Result<Self> {
        #[derive(serde::Serialize)]
        struct SearchUrl<'s> {
            search: &'s str,
        }

        Ok(Self {
            version: VERSION,
            git: GIT_VERSION,
            title: title.into(),
            duration: Utc::now().signed_duration_since(time),
            pagination: Pagination::new(
                format!(
                    "/search/{}",
                    serde_urlencoded::to_string(SearchUrl { search: &search })?
                ),
                None,
                None,
                pages as u32,
                page as u32,
            )
            .to_string(),
            search: Some(search),
            stories,
        })
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}
