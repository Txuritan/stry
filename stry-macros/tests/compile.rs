#[derive(Clone)]
struct Data {}

#[stry_macros::get("/")]
fn index() -> impl warp::Reply {
    "Hello World!"
}

#[stry_macros::get("/hello/{name}")]
fn hello(name: String) -> impl warp::Reply {
    format!("Hello, {}!", name)
}

#[stry_macros::get("/hello/{name}")]
fn hello_header_param(
    name: String,
    #[header("Content-Type")] _content_type: String,
) -> impl warp::Reply {
    format!("Hello, {}!", name)
}

#[stry_macros::get("/hello/{name}")]
#[header("Content-Type": "application/json")]
fn hello_header_attr(name: String) -> impl warp::Reply {
    format!("Hello, {}!", name)
}

#[stry_macros::get("/hello/{name}")]
async fn hello_data(
    #[data] _data: Data,
    name: String,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("Hello, {}!", name))
}
