pub mod search;
pub mod story;

use {
    crate::{
        models::{Author, Origin, Paging, Resource, Story, Tag, TagType},
        schema::Backend,
        Error, Readable,
    },
    askama::Template,
    futures::{
        future::{self, err, ok},
        Async, Future,
    },
    std::fmt,
    warp::{
        reject::{custom, not_found},
        reply, Rejection, Reply,
    },
};

#[derive(Template)]
#[template(path = "story_list.html")]
struct StoryList {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    page: u32,
    pages: u32,
    prev: u32,
    next: u32,

    stories: Vec<Story>,
}

impl StoryList {
    fn new(title: impl Into<String>, page: u32, pages: u32, stories: Vec<Story>) -> Self {
        Self {
            version: crate::VERSION,
            git: crate::GIT_VERSION,
            title: title.into(),
            search: None,
            prev: if page >= 1 { page - 1 } else { page },
            next: if page >= pages { page } else { page + 1 },
            page,
            pages,
            stories,
        }
    }
}

pub fn index(
    paging: Paging,
    backend: Backend,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    future::lazy(move || {
        future::poll_fn(move || {
            match tokio_threadpool::blocking(|| {
                let norm = paging.normalize();

                let (count, stories) = Story::all(backend.clone(), norm.page, paging.page_size)
                    .map_err(|err| custom(Error::new(err)))?;

                let rendered = StoryList::new(
                    "home",
                    paging.page,
                    (count + (paging.page_size - 1)) / paging.page_size,
                    stories,
                )
                .render()
                .map_err(|err| custom(Error::new(err)))?;

                Ok(reply::html(rendered))
            }) {
                Ok(Async::Ready(Ok(v))) => Ok(Async::Ready(v)),
                Ok(Async::Ready(Err(err))) => Err(err),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                _ => panic!("the threadpool shut down"),
            }
        })
    })
}

#[derive(Clone, Copy)]
enum RouteType {
    Authors,
    Characters,
    Origins,
    Pairings,
    Tags,
    Warnings,
}

impl RouteType {
    fn parse(typ: &str) -> impl Future<Item = Self, Error = Rejection> {
        match typ {
            "authors" => ok(RouteType::Authors),
            "characters" => ok(RouteType::Characters),
            "origins" => ok(RouteType::Origins),
            "pairings" => ok(RouteType::Pairings),
            "tags" => ok(RouteType::Tags),
            "warnings" => ok(RouteType::Warnings),
            _ => err(not_found()),
        }
    }
}

impl fmt::Display for RouteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RouteType::Authors => "authors",
                RouteType::Characters => "characters",
                RouteType::Origins => "origins",
                RouteType::Pairings => "pairings",
                RouteType::Tags => "tags",
                RouteType::Warnings => "warnings",
            }
        )
    }
}

// TODO: make these two routes cleaner
pub fn item(
    typ: String,
    id: String,
    paging: Paging,
    backend: Backend,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    RouteType::parse(&typ).and_then(move |rt| {
        future::lazy(move || {
            future::poll_fn(move || {
                match tokio_threadpool::blocking(|| {
                    let norm = paging.normalize();

                    match rt {
                        RouteType::Authors => {
                            let (count, stories) = Author::for_stories(backend.clone(), &id, norm)
                                .map_err(|err| custom(Error::new(err)))?;

                            let author = Author::get(backend.clone(), &id)
                                .map_err(|err| custom(Error::new(err)))?;

                            let rendered: String = StoryList::new(
                                format!("{} | {} | authors", paging.page, author.name),
                                paging.page,
                                (count + (paging.page_size - 1)) / paging.page_size,
                                stories,
                            )
                            .render()
                            .map_err(|err| custom(Error::new(err)))?;

                            Ok(reply::html(rendered))
                        }
                        RouteType::Origins => {
                            let (count, stories) = Origin::for_stories(backend.clone(), &id, norm)
                                .map_err(|err| custom(Error::new(err)))?;

                            let origin = Origin::get(backend.clone(), &id)
                                .map_err(|err| custom(Error::new(err)))?;

                            let rendered: String = StoryList::new(
                                format!("{} | {} | origins", paging.page, origin.name),
                                paging.page,
                                (count + (paging.page_size - 1)) / paging.page_size,
                                stories,
                            )
                            .render()
                            .map_err(|err| custom(Error::new(err)))?;

                            Ok(reply::html(rendered))
                        }
                        RouteType::Characters
                        | RouteType::Pairings
                        | RouteType::Tags
                        | RouteType::Warnings => {
                            let (count, stories) = Tag::for_stories(backend.clone(), &id, norm)
                                .map_err(|err| custom(Error::new(err)))?;

                            let tag = Tag::get(backend.clone(), &id)
                                .map_err(|err| custom(Error::new(err)))?;

                            let rendered: String = StoryList::new(
                                format!("{} | {} | {}", paging.page, tag.name, rt),
                                paging.page,
                                (count + (paging.page_size - 1)) / paging.page_size,
                                stories,
                            )
                            .render()
                            .map_err(|err| custom(Error::new(err)))?;

                            Ok(reply::html(rendered))
                        }
                    }
                }) {
                    Ok(Async::Ready(Ok(v))) => Ok(Async::Ready(v)),
                    Ok(Async::Ready(Err(err))) => Err(err),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    _ => panic!("the threadpool shut down"),
                }
            })
        })
    })
}

#[derive(Template)]
#[template(path = "explore.html")]
struct Explore<'a> {
    version: &'static str,
    git: &'static str,

    title: String,
    search: Option<String>,

    typ: String,

    page: u32,
    pages: u32,
    prev: u32,
    next: u32,

    resources: Vec<&'a dyn Resource>,
}

impl<'a> Explore<'a> {
    fn new(
        title: impl Into<String>,
        typ: impl Into<String>,
        page: u32,
        pages: u32,
        resources: Vec<&'a dyn Resource>,
    ) -> Self {
        Self {
            version: crate::VERSION,
            git: crate::GIT_VERSION,
            title: title.into(),
            search: None,
            typ: typ.into(),
            prev: if page >= 1 { page - 1 } else { page },
            next: if page >= pages { page } else { page + 1 },
            page,
            pages,
            resources,
        }
    }
}

pub fn explore(
    typ: String,
    paging: Paging,
    backend: Backend,
) -> impl Future<Item = impl Reply, Error = Rejection> {
    RouteType::parse(&typ).and_then(move |rt| {
        future::lazy(move || {
            future::poll_fn(move || {
                match tokio_threadpool::blocking({
                    let backend = backend.clone();

                    move || {
                        let mut norm = paging.normalize();

                        if norm.page_size == Paging::default().page_size {
                            norm.page_size = 50;
                        }

                        match rt {
                            RouteType::Authors => {
                                let (count, authors) = Author::all(backend.clone(), norm)
                                    .map_err(|err| custom(Error::new(err)))?;

                                let rendered: String = Explore::new(
                                    format!("{} | authors | explore", paging.page),
                                    rt.to_string(),
                                    paging.page,
                                    (count + (paging.page_size - 1)) / paging.page_size,
                                    authors.iter().map(|a| a as &dyn Resource).collect(),
                                )
                                .render()
                                .map_err(|err| custom(Error::new(err)))?;

                                Ok(reply::html(rendered))
                            }
                            RouteType::Origins => {
                                let (count, origins) = Origin::all(backend.clone(), norm)
                                    .map_err(|err| custom(Error::new(err)))?;

                                let rendered: String = Explore::new(
                                    format!("{} | origins | explore", paging.page),
                                    rt.to_string(),
                                    paging.page,
                                    (count + (paging.page_size - 1)) / paging.page_size,
                                    origins.iter().map(|a| a as &dyn Resource).collect(),
                                )
                                .render()
                                .map_err(|err| custom(Error::new(err)))?;

                                Ok(reply::html(rendered))
                            }
                            RouteType::Characters
                            | RouteType::Pairings
                            | RouteType::Tags
                            | RouteType::Warnings => {
                                let (count, tags) = Tag::all_of_type(
                                    backend.clone(),
                                    match rt {
                                        RouteType::Characters => TagType::Character,
                                        RouteType::Pairings => TagType::Pairing,
                                        RouteType::Tags => TagType::General,
                                        RouteType::Warnings => TagType::Warning,
                                        _ => unreachable!(),
                                    },
                                    norm,
                                )
                                .map_err(|err| custom(Error::new(err)))?;

                                let rendered: String = Explore::new(
                                    format!("{} | {} | explore", paging.page, rt),
                                    rt.to_string(),
                                    paging.page,
                                    (count + (paging.page_size - 1)) / paging.page_size,
                                    tags.iter().map(|a| a as &dyn Resource).collect(),
                                )
                                .render()
                                .map_err(|err| custom(Error::new(err)))?;

                                Ok(reply::html(rendered))
                            }
                        }
                    }
                }) {
                    Ok(Async::Ready(Ok(v))) => Ok(Async::Ready(v)),
                    Ok(Async::Ready(Err(err))) => Err(err),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    _ => panic!("the threadpool shut down"),
                }
            })
        })
    })
}
