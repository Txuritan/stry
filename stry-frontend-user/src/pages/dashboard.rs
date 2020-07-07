use askama::Template;

#[derive(Template)]
#[template(path = "dashboard/about.html")]
pub struct About {
    version: &'static str,
    git: &'static str,

    title: String,
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
#[template(path = "dashboard/Tasks.html")]
pub struct Tasks {
    version: &'static str,
    git: &'static str,

    title: String,
}
