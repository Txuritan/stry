pub mod dashboard;
pub mod edit;
// pub mod explore;
pub mod story;

use {
    crate::{
        i18n,
        pagination::Pagination,
        readable::Readable,
        utils::{Resource, WebError},
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_generated_version::{GIT_VERSION, VERSION},
    unic_langid::LanguageIdentifier,
};

pub struct Meta {
    pub version: &'static str,
    pub git: &'static str,

    pub user_lang: Vec<LanguageIdentifier>,
}

impl Meta {
    pub fn new(user_lang: Vec<LanguageIdentifier>) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            user_lang,
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage<T>
where
    T: std::fmt::Display,
{
    meta: Meta,

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
        user_lang: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),

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

    pub fn not_found(title: T, time: DateTime<Utc>, user_lang: Vec<LanguageIdentifier>) -> Self {
        Self::new(title, time, 404, "Not Found", "danger", "", user_lang)
    }

    pub fn server_error(title: T, time: DateTime<Utc>, user_lang: Vec<LanguageIdentifier>) -> Self {
        Self::new(
            title,
            time,
            503,
            "Server Error",
            "danger",
            "Check the log for more information",
            user_lang,
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
    meta: Meta,

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
        user_lang: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),
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
    meta: Meta,

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
        user_lang: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),
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
    meta: Meta,

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
        user_lang: Vec<LanguageIdentifier>,
    ) -> anyhow::Result<Self> {
        #[derive(serde::Serialize)]
        struct SearchUrl<'s> {
            search: &'s str,
        }

        Ok(Self {
            meta: Meta::new(user_lang),
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
