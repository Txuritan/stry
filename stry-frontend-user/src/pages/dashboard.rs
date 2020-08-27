use {
    askama::Template,
    chrono::{DateTime, Duration, Utc},
    stry_common::{
        models::{Worker, WorkerTask},
        version::{LibVersion, BOM, GIT_VERSION, VERSION},
    },
};

#[derive(Template)]
#[template(path = "dashboard/about.html")]
pub struct About<'l> {
    version: &'static str,
    git: &'static str,

    title: &'static str,
    duration: Duration,

    licenses: &'static str,
    versions: &'l [LibVersion],
}

impl<'l> About<'l> {
    pub fn new(time: DateTime<Utc>, versions: &'l [LibVersion]) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: "about | dashboard",
            duration: Utc::now().signed_duration_since(time),
            licenses: BOM,
            versions,
        }
    }
}

#[derive(Template)]
#[template(path = "dashboard/database.html")]
pub struct Database {
    version: &'static str,
    git: &'static str,

    title: String,
    duration: Duration,
}

#[derive(Template)]
#[template(path = "dashboard/settings.html")]
pub struct Settings {
    version: &'static str,
    git: &'static str,

    title: String,
    duration: Duration,
}

#[derive(Template)]
#[template(path = "dashboard/stats.html")]
pub struct Stats {
    version: &'static str,
    git: &'static str,

    title: String,
    duration: Duration,
}

#[derive(Template)]
#[template(path = "dashboard/tasks.html")]
pub struct Tasks<'w> {
    version: &'static str,
    git: &'static str,

    title: &'static str,
    duration: Duration,

    workers: &'w [Worker],
    tasks: &'w [WorkerTask],
}

impl<'w> Tasks<'w> {
    pub fn new(time: DateTime<Utc>, workers: &'w [Worker], tasks: &'w [WorkerTask]) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: "tasks | dashboard",
            duration: Utc::now().signed_duration_since(time),
            workers,
            tasks,
        }
    }
}
