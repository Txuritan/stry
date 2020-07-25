use {
    crate::{
        models::Worker,
        version::{LibVersion, BOM, GIT_VERSION, VERSION},
    },
    askama::Template,
};

#[derive(Template)]
#[template(path = "dashboard/about.html")]
pub struct About<'l> {
    version: &'static str,
    git: &'static str,

    title: &'static str,

    licenses: &'static str,
    versions: &'l [LibVersion],
}

impl<'l> About<'l> {
    pub fn new(versions: &'l [LibVersion]) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: "about | dashboard",
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
}

#[derive(Template)]
#[template(path = "dashboard/settings.html")]
pub struct Settings {
    version: &'static str,
    git: &'static str,

    title: String,
}

#[derive(Template)]
#[template(path = "dashboard/stats.html")]
pub struct Stats {
    version: &'static str,
    git: &'static str,

    title: String,
}

#[derive(Template)]
#[template(path = "dashboard/tasks.html")]
pub struct Tasks<'w> {
    version: &'static str,
    git: &'static str,

    title: &'static str,

    workers: &'w [Worker],
}

impl<'w> Tasks<'w> {
    pub fn new(workers: &'w [Worker]) -> Self {
        Self {
            version: VERSION,
            git: GIT_VERSION,
            title: "tasks | dashboard",
            workers,
        }
    }
}
