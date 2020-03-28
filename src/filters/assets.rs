use {
    resource::{resource, Resource},
    std::borrow::Cow,
    warp::{
        filters::BoxedFilter,
        hyper::Body,
        http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE},
        reply::Response,
        Filter, Reply,
    },
};

macro_rules! embed {
    (@internal_bytes => $s:ident, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).map(|| $s {
            body: {
                static CONTENTS: &'static [u8] =  include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "assets/", $file, ".", $t));

                CONTENTS
            },
        })
    };
    (@internal_str => $s:ident, $t:tt, $file:tt) => {
        warp::path(concat!($file, ".", $t)).map(|| $s {
            body: {
                static CONTENTS: &'static str =  include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "assets/", $file, ".", $t));

                CONTENTS
            },
        })
    };
    (ico => $file:tt) => {{
        embed!(@internal_bytes => Ico, "ico", $file)
    }};
    (png => $file:tt) => {{
        embed!(@internal_bytes => Png, "png", $file)
    }};
    (svg => $file:tt) => {{
        embed!(@internal_str => Xml, "svg", $file)
    }};
    (webmanifest => $file:tt) => {{
        embed!(@internal_str => WebManifest, "webmanifest", $file)
    }};
    (xml => $file:tt) => {{
        embed!(@internal_str => Xml, "xml", $file)
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
                .or(js()),
        )
        .boxed()
}

pub fn js() -> BoxedFilter<(impl Reply,)> {
    warp::path("js")
        .and(warp::path::param::<String>())
        .and_then(|file: String| async move {
            match file.as_str() {
                "markdown-it.js" => Ok(Js::new(resource!("js/markdown-it.js"))),
                "mousetrap.js" => Ok(Js::new(resource!("js/mousetrap.js"))),
                "stry.js" => Ok(Js::new(resource!("js/stry.js"))),
                "stry-dashboard.js" => Ok(Js::new(resource!("js/stry-dashboard.js"))),
                _ => Err(warp::reject::not_found()),
            }
        })
        .boxed()
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

struct Ico<T> {
    body: T,
}

impl<T> Reply for Ico<T>
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
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/x-icon"));

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

struct Png<T> {
    body: T,
}

impl<T> Reply for Png<T>
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
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));

        res
    }
}

struct Svg<T> {
    body: T,
}

impl<T> Reply for Svg<T>
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
            .insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));

        res
    }
}

struct WebManifest<T> {
    body: T,
}

impl<T> Reply for WebManifest<T>
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

        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/manifest+json"),
        );

        res
    }
}

struct Xml<T> {
    body: T,
}

impl<T> Reply for Xml<T>
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
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/xml"));

        res
    }
}
