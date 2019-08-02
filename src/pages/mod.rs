use {
    crate::{models::Story, Error, Pool, CSS, GIT, VERSION},
    actix_web::{
        web::{self, Data, Path, Query},
        HttpResponse,
    },
    askama::Template,
    chrono::{DateTime, Duration, Utc},
};

macro_rules! hum_num {
    ($num:expr) => {{
        use num_format::ToFormattedString;
        $num.to_formatted_string(&num_format::Locale::en)
    }};
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub css: &'static str,
    pub git: &'static str,
    pub version: &'static str,
    pub stories: Vec<Story>,
    pub title: &'static str,
    pub search: Option<String>,
    pub duration: Duration,
}

impl Index {
    fn render(
        time: DateTime<Utc>,
        title: &'static str,
        stories: Vec<Story>,
        search: Option<String>,
    ) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().content_type("text/html").body(
            Self {
                css: CSS,
                git: GIT,
                version: VERSION,
                title,
                stories,
                search,
                duration: Utc::now().signed_duration_since(time),
            }
            .render()?,
        ))
    }

    pub fn home(pool: Data<Pool>) -> Result<HttpResponse, Error> {
        let now = Utc::now();

        let stories = Story::all(pool.get_ref().clone())?;

        Index::render(now, "Home", stories, None)
    }
}
