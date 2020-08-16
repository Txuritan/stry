use {
    crate::models::{Author, Character, Origin, Pairing, Tag, Warning},
    askama::Template,
    chrono::Utc,
    std::{fmt, future::Future, str::FromStr},
    warp::{
        http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode},
        hyper::Body,
        Rejection, Reply,
    },
};

pub mod filters {
    use pulldown_cmark::{html, Options, Parser};

    pub fn markdown(input: &str) -> ::askama::Result<String> {
        let parser = Parser::new_ext(input, Options::empty());

        let mut output = String::with_capacity(input.len() + (input.len() / 5));

        html::push_html(&mut output, parser);

        Ok(output)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Items {
    Authors,
    Characters,
    Origins,
    Pairings,
    Tags,
    Warnings,
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Items::Authors => write!(f, "authors"),
            Items::Characters => write!(f, "characters"),
            Items::Origins => write!(f, "origins"),
            Items::Pairings => write!(f, "pairings"),
            Items::Tags => write!(f, "tags"),
            Items::Warnings => write!(f, "warnings"),
        }
    }
}

impl FromStr for Items {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "authors" => Ok(Items::Authors),
            "characters" => Ok(Items::Characters),
            "origins" => Ok(Items::Origins),
            "pairings" => Ok(Items::Pairings),
            "tags" => Ok(Items::Tags),
            "warnings" => Ok(Items::Warnings),
            _ => anyhow::bail!("Unknown path"),
        }
    }
}

pub enum Resource {
    Author(Author),
    Character(Character),
    Origin(Origin),
    Pairing(Pairing),
    Tag(Tag),
    Warning(Warning),
}

impl Resource {
    pub fn id(&self) -> &str {
        match self {
            Resource::Author(entity) => &entity.id,
            Resource::Character(entity) => &entity.id,
            Resource::Origin(entity) => &entity.id,
            Resource::Pairing(entity) => &entity.id,
            Resource::Tag(entity) => &entity.id,
            Resource::Warning(entity) => &entity.id,
        }
    }

    pub fn color(&self) -> &str {
        match self {
            Resource::Author(_) => "color__blue",
            Resource::Character(_) => "color__purple",
            Resource::Origin(_) => "color__green",
            Resource::Pairing(_) => "color__yellow",
            Resource::Tag(_) => "color__silver",
            Resource::Warning(_) => "color__red",
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<li><a class=\"label {}\" href=\"/", self.color())?;

        match self {
            Resource::Author(_) => write!(f, "authors")?,
            Resource::Character(_) => write!(f, "characters")?,
            Resource::Origin(_) => write!(f, "origins")?,
            Resource::Pairing(_) => write!(f, "pairings")?,
            Resource::Tag(_) => write!(f, "tags")?,
            Resource::Warning(_) => write!(f, "warnings")?,
        }

        write!(f, "/{}\">", self.id())?;

        match self {
            Resource::Author(entity) => write!(f, "{}", entity.name)?,
            Resource::Character(entity) => write!(f, "{}", entity.name)?,
            Resource::Origin(entity) => write!(f, "{}", entity.name)?,
            Resource::Pairing(entity) => write!(
                f,
                "{}",
                entity
                    .characters
                    .iter()
                    .map(|c| &*c.name)
                    .collect::<Vec<&str>>()
                    .join(if entity.platonic { "&" } else { "/" })
            )?,
            Resource::Tag(entity) => write!(f, "{}", entity.name)?,
            Resource::Warning(entity) => write!(f, "{}", entity.name)?,
        }

        write!(f, "</a></li>")?;

        Ok(())
    }
}

pub struct WebError {
    pub code: u32,
    pub name: &'static str,
    pub icon: &'static str,
    pub help: &'static str,
}

#[inline(always)]
pub async fn wrap<Run, Fut, Res>(run: Run) -> Result<impl Reply, Rejection>
where
    Run: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<Res>>,
    Res: Reply,
{
    match run().await {
        Ok(res) => {
            let mut response = res.into_response();

            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));

            Ok(response)
        }
        Err(err) => {
            let time = Utc::now();

            {
                let span = tracing::error_span!("Response error");
                let _enter = span.enter();

                for chain in err.chain() {
                    tracing::error!("{}", chain);
                }
            }

            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            if let Ok(rendered) =
                crate::frontend::user::pages::ErrorPage::server_error("503 server error", time)
                    .render()
            {
                *res.body_mut() = Body::from(rendered);
            }

            Ok(res)
        }
    }
}
