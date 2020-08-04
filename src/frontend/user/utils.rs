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

impl Resource {
    pub fn id(&self) -> &str {
        match self {
            Resource::Author(entity) => &entity.id,
            Resource::Character(entity) => &entity.id,
            Resource::Friend() => todo!(),
            Resource::Origin(entity) => &entity.id,
            Resource::Pairing() => todo!(),
            Resource::Tag(entity) => &entity.id,
            Resource::Warning(entity) => &entity.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Resource::Author(entity) => &entity.name,
            Resource::Character(entity) => &entity.name,
            Resource::Friend() => todo!(),
            Resource::Origin(entity) => &entity.name,
            Resource::Pairing() => todo!(),
            Resource::Tag(entity) => &entity.name,
            Resource::Warning(entity) => &entity.name,
        }
    }

    pub fn color(&self) -> &str {
        match self {
            Resource::Author(_) => "color__blue",
            Resource::Character(_) => "color__purple",
            Resource::Friend() => todo!(),
            Resource::Origin(_) => "color__green",
            Resource::Pairing() => todo!(),
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
            Resource::Friend() => write!(f, "friends")?,
            Resource::Origin(_) => write!(f, "origins")?,
            Resource::Pairing() => write!(f, "pairings")?,
            Resource::Tag(_) => write!(f, "tags")?,
            Resource::Warning(_) => write!(f, "warnings")?,
        }

        write!(f, "/{}\">{}</a></li>", self.id(), self.name())?;

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
