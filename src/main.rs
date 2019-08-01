mod css;
mod models;
mod pages;

mod error;
mod schema;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Conn = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;
pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub use crate::{error::Error, schema::Schema};

use {
    actix_http::KeepAlive,
    actix_web::{middleware, web, App, HttpServer},
    std::net::{IpAddr, Ipv4Addr, SocketAddr},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    let sys = actix_rt::System::new("stry2");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
    })
    .keep_alive(KeepAlive::Timeout(60))
    .bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8901))?
    .start();

    log::info!("Started Stry: 0.0.0.0:8901");

    sys.run()?;

    Ok(())
}
