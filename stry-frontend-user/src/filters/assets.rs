use {
    resource::{resource, Resource},
    std::borrow::Cow,
    warp::{
        filters::BoxedFilter,
        http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE},
        hyper::Body,
        reply::Response,
        Filter, Reply,
    },
};

macro_rules! embed {
    (@internal_bytes => $mime:expr, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).map(|| Mime {
            body: {
                static CONTENTS: &'static [u8] =  include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file, ".", $t));

                CONTENTS
            },
            mime: $mime,
        })
    };
    (@internal_str => $mime:expr, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).map(|| Mime {
            body: {
                static CONTENTS: &'static str =  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $file, ".", $t));

                CONTENTS
            },
            mime: $mime,
        })
    };
    (ico => $file:tt) => {{
        embed!(@internal_bytes => "image/x-icon", "ico", $file)
    }};
    (png => $file:tt) => {{
        embed!(@internal_bytes => "image/png", "png", $file)
    }};
    (svg => $file:tt) => {{
        embed!(@internal_str => "image/svg+xml", "svg", $file)
    }};
    (webmanifest => $file:tt) => {{
        embed!(@internal_str => "application/manifest+json", "webmanifest", $file)
    }};
    (xml => $file:tt) => {{
        embed!(@internal_str => "application/xml", "xml", $file)
    }};
}

pub fn assets() -> BoxedFilter<(impl Reply,)> {
    warp::get()
        .and(
            embed!(png => "android-chrome-192x192")
                .or(embed!(png => "android-chrome-256x256"))
                .or(embed!(png => "apple-touch-icon-114x114-precomposed"))
                .or(embed!(png => "apple-touch-icon-114x114"))
                .or(embed!(png => "apple-touch-icon-120x120-precomposed"))
                .or(embed!(png => "apple-touch-icon-120x120"))
                .or(embed!(png => "apple-touch-icon-144x144-precomposed"))
                .or(embed!(png => "apple-touch-icon-144x144"))
                .or(embed!(png => "apple-touch-icon-152x152-precomposed"))
                .or(embed!(png => "apple-touch-icon-152x152"))
                .or(embed!(png => "apple-touch-icon-180x180-precomposed"))
                .or(embed!(png => "apple-touch-icon-180x180"))
                .or(embed!(png => "apple-touch-icon-57x57-precomposed"))
                .or(embed!(png => "apple-touch-icon-57x57"))
                .or(embed!(png => "apple-touch-icon-60x60-precomposed"))
                .or(embed!(png => "apple-touch-icon-60x60"))
                .or(embed!(png => "apple-touch-icon-72x72-precomposed"))
                .or(embed!(png => "apple-touch-icon-72x72"))
                .or(embed!(png => "apple-touch-icon-76x76-precomposed"))
                .or(embed!(png => "apple-touch-icon-76x76"))
                .or(embed!(png => "apple-touch-icon-precomposed"))
                .or(embed!(png => "apple-touch-icon"))
                .or(embed!(xml => "browserconfig"))
                .or(embed!(png => "favicon-16x16"))
                .or(embed!(png => "favicon-32x32"))
                .or(embed!(ico => "favicon"))
                .or(embed!(png => "mstile-144x144"))
                .or(embed!(png => "mstile-150x150"))
                .or(embed!(svg => "safari-pinned-tab"))
                .or(embed!(webmanifest => "site"))
                .or(css())
                .or(js()),
        )
        .boxed()
}

pub fn css() -> BoxedFilter<(impl Reply,)> {
    warp::path("css")
        .and(warp::path::param::<String>())
        .and_then(|file: String| async move {
            match file.as_str() {
                "easymde.css" => Ok(Css::new(resource!("assets/css/easymde.css"))),
                "stry.css" => Ok(Css::new(resource!("assets/css/stry.css"))),
                "tagify.css" => Ok(Css::new(resource!("assets/css/tagify.css"))),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

pub fn js() -> BoxedFilter<(impl Reply,)> {
    warp::path("js")
        .and(warp::path::param::<String>())
        .and_then(|file: String| async move {
            match file.as_str() {
                "easymde.js" => Ok(Js::new(resource!("assets/js/easymde.js"))),
                "marked.js" => Ok(Js::new(resource!("assets/js/marked.js"))),
                "mousetrap.js" => Ok(Js::new(resource!("assets/js/mousetrap.js"))),
                "stry.js" => Ok(Js::new(resource!("assets/js/stry.js"))),
                "stry-dashboard.js" => Ok(Js::new(resource!("assets/js/stry-dashboard.js"))),
                "tagify.js" => Ok(Js::new(resource!("assets/js/tagify.js"))),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
}

struct Mime<T> {
    body: T,
    mime: &'static str,
}

impl<T> Reply for Mime<T>
where
    Body: From<T>,
    T: Send,
{
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::from(self.body));

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(self.mime));

        res
    }
}

struct Css {
    inner: Resource<[u8]>,
}

impl Css {
    fn new(inner: Resource<[u8]>) -> Self {
        Self { inner }
    }
}

impl Reply for Css {
    #[inline]
    fn into_response(self) -> Response {
        let borrow: Cow<'static, [u8]> = self.inner.into();
        let mut res = Response::new(borrow.into());

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("text/css"));

        res
    }
}

struct Js {
    inner: Resource<[u8]>,
}

impl Js {
    fn new(inner: Resource<[u8]>) -> Self {
        Self { inner }
    }
}

impl Reply for Js {
    #[inline]
    fn into_response(self) -> Response {
        let borrow: Cow<'static, [u8]> = self.inner.into();
        let mut res = Response::new(borrow.into());

        res.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=37260"),
        );

        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/javascript"),
        );

        res
    }
}
