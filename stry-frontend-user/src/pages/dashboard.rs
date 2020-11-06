use {
    crate::{i18n, pages::Meta},
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_common::LibraryDetails,
    stry_generated_version::BOM,
    stry_models::{Worker, WorkerTask},
    unic_langid::LanguageIdentifier,
};

#[derive(Template)]
#[template(path = "dashboard/about.html")]
pub struct About<'l> {
    meta: Meta,

    title: &'static str,
    duration: Duration,

    licenses: &'static str,
    details: &'l [LibraryDetails],
}

impl<'l> About<'l> {
    pub fn new(
        time: DateTime<Utc>,
        details: &'l [LibraryDetails],
        user_lang: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),
            title: "about | dashboard",
            duration: Utc::now().signed_duration_since(time),
            licenses: BOM,
            details,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "dashboard/database.html")]
pub struct Database {
    meta: Meta,

    title: String,
    duration: Duration,
}

impl Database {
    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "dashboard/settings.html")]
pub struct Settings {
    meta: Meta,

    title: String,
    duration: Duration,
}

impl Settings {
    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "dashboard/stats.html")]
pub struct Stats {
    meta: Meta,

    title: String,
    duration: Duration,
}

impl Stats {
    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}

#[derive(Template)]
#[template(path = "dashboard/tasks.html")]
pub struct Tasks<'w> {
    meta: Meta,

    title: &'static str,
    duration: Duration,

    workers: &'w [Worker],
    tasks: &'w [WorkerTask],
}

impl<'w> Tasks<'w> {
    pub fn new(
        time: DateTime<Utc>,
        workers: &'w [Worker],
        tasks: &'w [WorkerTask],
        user_lang: Vec<LanguageIdentifier>,
    ) -> Self {
        Self {
            meta: Meta::new(user_lang),
            title: "tasks | dashboard",
            duration: Utc::now().signed_duration_since(time),
            workers,
            tasks,
        }
    }

    #[tracing::instrument(level = "trace", name = "render", skip(self), err)]
    pub fn into_string(self) -> anyhow::Result<String> {
        Ok(self.render()?)
    }
}
