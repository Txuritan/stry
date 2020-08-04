use {
    crate::models::{Author, Character, Origin, Tag, Warning},
    askama::Template,
    std::{fmt, future::Future, str::FromStr},
    warp::{
        http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode},
        hyper::Body,
        Rejection, Reply,
    },
};

#[derive(Clone, Copy, Debug)]
pub enum Items {
    Authors,
    Characters,
    Friends,
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
            Items::Friends => write!(f, "friends"),
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
            "tags" => Ok(Items::Tags),
            "warnings" => Ok(Items::Warnings),
            _ => anyhow::bail!("Unknown path"),
        }
    }
}

pub enum Resource {
    Author(Author),
    Character(Character),
    Friend(),
    Origin(Origin),
    Pairing(),
    Tag(Tag),
    Warning(Warning),
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::Author(entity) => entity.fmt(f),
            Resource::Character(entity) => entity.fmt(f),
            Resource::Friend() => todo!(),
            Resource::Origin(entity) => entity.fmt(f),
            Resource::Pairing() => todo!(),
            Resource::Tag(entity) => entity.fmt(f),
            Resource::Warning(entity) => entity.fmt(f),
        }
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
                crate::frontend::user::pages::ErrorPage::server_error("503 server error").render()
            {
                *res.body_mut() = Body::from(rendered);
            }

            Ok(res)
        }
    }
}
