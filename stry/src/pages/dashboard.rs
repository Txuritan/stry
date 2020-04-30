use askama::Template;

#[derive(Template)]
#[template(path = "dashboard/downloads.html")]
struct Downloads {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,
}
