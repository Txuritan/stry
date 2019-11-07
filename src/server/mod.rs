mod pages;

use {
    crate::{
        models::{Paging, Search},
        schema::Backend,
        Error, ErrorKind,
    },
    std::thread,
    warp::{Filter, Reply},
};

pub fn start(backend: Backend) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let css = warp::get2()
            .and(warp::path("css"))
            .and(warp::path::param::<String>())
            .and_then(|file: String| match file.as_str() {
                "polygon.css" => Ok(css(include_bytes!("../../css/polygon.css"))),
                "stry.css" => Ok(css(include_bytes!("../../css/stry.css"))),
                _ => Err(warp::reject::not_found()),
            });

        let js = warp::get2()
            .and(warp::path("js"))
            .and(warp::path::param::<String>())
            .and_then(|file: String| match file.as_str() {
                "js-cookie.js" => Ok(js(include_bytes!("../../js/js-cookie.js"))),
                "markdown-it.js" => Ok(js(include_bytes!("../../js/markdown-it.js"))),
                "mousetrap.js" => Ok(js(include_bytes!("../../js/mousetrap.js"))),
                "stry.js" => Ok(js(include_bytes!("../../js/stry.js"))),
                "stry-dashboard.js" => Ok(js(include_bytes!("../../js/stry-dashboard.js"))),
                _ => Err(warp::reject::not_found()),
            });

        let with_state = warp::any().map(move || backend.clone());

        let index = warp::get2()
            .and(warp::query::<Paging>())
            .and(with_state.clone())
            .and_then(pages::index);

        let search = warp::get2()
            .and(warp::path("search"))
            .and(warp::query::<Paging>())
            .and(warp::query::<Search>())
            .and(with_state.clone())
            .and_then(pages::search::index);

        let story = {
            let chapter = warp::get2()
                .and(warp::path("story"))
                .and(warp::path::param::<String>())
                .and(warp::path::param::<u32>())
                .and(with_state.clone())
                .and_then(pages::story::chapter);

            let story = warp::get2()
                .and(warp::path("story"))
                .and(warp::path::param::<String>())
                .and(with_state.clone())
                .and_then(pages::story::index);

            chapter.or(story)
        };

        let explore = warp::get2()
            .and(warp::path("explore"))
            .and(warp::path::param::<String>())
            .and(warp::query::<Paging>())
            .and(with_state.clone())
            .and_then(pages::explore);

        let item = warp::get2()
            .and(warp::path::param::<String>())
            .and(warp::path::param::<String>())
            .and(warp::query::<Paging>())
            .and(with_state.clone())
            .and_then(pages::item);

        let routes = story
            .or(css)
            .or(js)
            .or(explore)
            .or(item)
            .or(search)
            .or(index)
            .recover(error)
            .with(warp::log("stry"));

        warp::serve(routes).run(([0, 0, 0, 0], 8901));
    })
}

fn error(err: warp::Rejection) -> Result<impl Reply, warp::Rejection> {
    use warp::{
        http::{header, StatusCode},
        reply::{html, with_header, with_status},
    };

    if let Some(err) = &err.find_cause::<Error>() {
        match &err.kind {
            ErrorKind::Moved { location } => Ok(with_status(
                with_header(html(""), header::LOCATION, location).into_response(),
                StatusCode::MOVED_PERMANENTLY,
            )),
            _ => Ok(with_status(
                html("").into_response(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    } else {
        Err(err)
    }
}

pub fn css(val: &'static [u8]) -> impl Reply {
    Css { inner: val }
}

#[allow(missing_debug_implementations)]
struct Css {
    inner: &'static [u8],
}

impl Reply for Css {
    #[inline]
    fn into_response(self) -> warp::reply::Response {
        let mut res = warp::reply::Response::new(self.inner.into());

        res.headers_mut().insert(
            warp::http::header::CACHE_CONTROL,
            warp::http::header::HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut().insert(
            warp::http::header::CONTENT_TYPE,
            warp::http::header::HeaderValue::from_static("text/css"),
        );

        res
    }
}

pub fn js(val: &'static [u8]) -> impl Reply {
    Js { inner: val }
}

#[allow(missing_debug_implementations)]
struct Js {
    inner: &'static [u8],
}

impl Reply for Js {
    #[inline]
    fn into_response(self) -> warp::reply::Response {
        let mut res = warp::reply::Response::new(self.inner.into());

        res.headers_mut().insert(
            warp::http::header::CACHE_CONTROL,
            warp::http::header::HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut().insert(
            warp::http::header::CONTENT_TYPE,
            warp::http::header::HeaderValue::from_static("application/javascript"),
        );

        res
    }
}
