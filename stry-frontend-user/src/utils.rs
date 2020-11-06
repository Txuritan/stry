use {
    askama::Template,
    chrono::Utc,
    fluent::{concurrent::FluentBundle, FluentResource},
    once_cell::sync::OnceCell,
    std::{collections::HashMap, fmt, future::Future, str::FromStr},
    stry_models::{Author, Character, Origin, Pairing, Tag, Warning},
    unic_langid::LanguageIdentifier,
    warp::{
        http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode},
        hyper::Body,
        Rejection, Reply,
    },
};

#[macro_export]
macro_rules! i18n {
    (@inner , $lang:expr , $message:expr , $args:expr ) => {{
        $crate::utils::FLUENT.get().map(|fluent| {
            if $lang.is_empty() {
                panic!("BUG: User Accept-Language header is empty, there should be at least `en-US`");
            }

            for lang in &$lang {
                if !fluent.contains_key(lang) {
                    continue;
                }

                let bundle = fluent.get(lang).unwrap_or_else(|| panic!("BUG: Fluent map doesn't contain a bundle but its ID exists, `{}`", lang));

                #[cfg(debug_assertions)]
                {
                    if !bundle.has_message($message) {
                        panic!("Fluent bundle for `{}` does not contain the message `{}`", lang, $message);
                    }
                }

                let msg = bundle.get_message($message).unwrap_or_else(|| panic!("BUG: Fluent bundle `{}`, does not contain the message `{}`", lang, $message));
                let pattern = msg.value.unwrap_or_else(|| panic!("BUG: Fluent message `{}` has no value", $message));

                let mut errors = Vec::new();

                let value = bundle.format_pattern(&pattern, $args, &mut errors);

                if !errors.is_empty() {
                    let mut error = ::anyhow::anyhow!("Unable to format message pattern");

                    for err in errors {
                        error = error.context(format!("{:?}", err));
                    }

                    panic!("{:#?}", error);
                }

                return value.to_string();
            }

            unreachable!("BUG: User Accept-Language header is empty, there should be at least `en-US`");
        }).unwrap()
    }};
    ( $lang:expr , $message:expr ) => {{
        $crate::i18n!(@inner , $lang , $message , None)
    }};
    ( $lang:expr , $message:expr , { $( $key:expr => $value:expr ),* } ) => {{
        let mut args: ::fluent::FluentArgs = ::fluent::FluentArgs::new();

        $(
            args.add($key, $value.into());
        )*

        $crate::i18n!(@inner , $lang , $message , Some(&args))
    }};
}

pub(crate) static FLUENT: OnceCell<HashMap<LanguageIdentifier, FluentBundle<FluentResource>>> =
    OnceCell::new();

pub(crate) fn init_fluent() -> anyhow::Result<()> {
    static LANGS: [(LanguageIdentifier, &str); 1] = [(
        unic_langid::langid!("en-US"),
        include_str!("../localization/en-US/main.ftl"),
    )];

    let mut fluent_map = HashMap::new();

    for (id, data) in LANGS.iter() {
        let resource = FluentResource::try_new(data.to_string()).map_err(|(_, errs)| {
            let mut error = anyhow::anyhow!("Failed to make Fluent resource");

            for err in errs {
                error = error.context(err);
            }

            error
        })?;

        let mut bundle = FluentBundle::new(&[id.clone()]);

        bundle.add_resource(resource).map_err(|errs| {
            let mut error = anyhow::anyhow!("Failed to add Fluent resource to bundle");

            for err in errs {
                error = error.context(format!("{:?}", err));
            }

            error
        })?;

        fluent_map.insert(id.clone(), bundle);
    }

    FLUENT
        .set(fluent_map)
        .map_err(|_| anyhow::anyhow!("Unable to set global Fluent instance"))?;

    Ok(())
}

pub mod filters {
    use pulldown_cmark::{html, Options, Parser};

    pub fn markdown(input: &str) -> ::askama::Result<String> {
        let parser = Parser::new_ext(input, Options::empty());

        let mut output = String::with_capacity(input.len() + (input.len() / 5));

        html::push_html(&mut output, parser);

        Ok(output)
    }
}

pub fn get_languages(languages: &str) -> Vec<LanguageIdentifier> {
    let mut user_lang = accept_language::parse(&languages)
        .into_iter()
        .map(|lang| lang.parse::<LanguageIdentifier>())
        .collect::<Result<Vec<LanguageIdentifier>, _>>()
        .unwrap_or_else(|_| vec![unic_langid::langid!("en-US")]);

    if user_lang.is_empty() {
        user_lang.push(unic_langid::langid!("en-US"));
    }

    user_lang
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

            if let Ok(rendered) = crate::pages::ErrorPage::server_error(
                "503 server error",
                time,
                vec![unic_langid::langid!("en-US")],
            )
            .render()
            {
                *res.body_mut() = Body::from(rendered);
            }

            Ok(res)
        }
    }
}
