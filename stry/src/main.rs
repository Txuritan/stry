#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use {
    anyhow::Context,
    clap::{App, Arg},
    stry_config::Config,
    stry_generated_version::GIT_VERSION,
};

fn main() -> anyhow::Result<()> {
    let version = &*format!("{}-{}", env!("CARGO_PKG_VERSION"), GIT_VERSION);
    let matches = app(version).get_matches();

    let path = if let Some(path) = matches.value_of("config").map(String::from) {
        path
    } else {
        String::from("stry.ron")
    };

    let cfg =
        Config::new_from_sources(path, matches).context("Failure to create config instance")?;

    stry::start(cfg)?;

    Ok(())
}

#[rustfmt::skip]
#[inline]
fn app<'b>(version: &'b str) -> App<'static, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(version)
        .arg(value("config", "c", "Use a specified config instead of the default", "CONFIG"))
        .arg(value("workers", "w", "Sets the amount of task workers", "COUNT"))
        .arg(value("backend-database", "d", "Database name for remote backend database", "DATABASE"))
        .arg(value("backend-host", "H", "Host address for remote backend database", "HOST"))
        .arg(value("backend-port", "P", "Port number for remote backend database", "PORT"))
        .arg(value("backend-file", "f", "The file of the SQLite backend database", "FILE"))
        .arg(value("backend-password", "s", "Password for remote backend database user", "PASSWORD"))
        .arg(value("backend-type", "t", "The type of the backend database", "TYPE")
            .possible_values(&["postgres", "sqlite"]))
        .arg(value("backend-username", "u", "Username for remote backend database user", "USERNAME"))
        .arg(value("server-ip", "i", "IP that the server will listen for requests on", "IP"))
        .arg(value("server-port", "p", "Port used by the server", "PORT"))
        .arg(value("tracing-directory", "o", "Directory to write tracing files to", "DIRECTORY"))
        .arg(value("tracing-flame", "F", "Enables and writes tracing flame graph to the given file", "FILE"))
        .arg(value("tracing-level", "l", "Lowest level for tracing output", "LEVEL")
            .possible_values(&["error", "warn", "info", "debug", "trace"]))
        .arg(value("tracing-prefix", "x", "Logging file name prefix", "PREFIX"))
        .arg(flag("tracing-ansi", "a", "Enables ANSI coloring of tracing output"))
        .arg(flag("tracing-json", "j", "Output tracing in JSON format"))
        .arg(flag("tracing-output", "O", "Output tracing in JSON format")
            .possible_values(&["both", "file", "stdout"]))
        .arg(flag("tracing-thread-ids", "D", "Logging output contains the ID of its source thread"))
        .arg(flag("tracing-thread-names", "n", "Logging output contains the name of its source thread"))
        .version_short("v")
}

fn flag<'b>(name: &'b str, short: &'b str, help: &'b str) -> Arg<'b, 'b> {
    Arg::with_name(name).long(name).short(short).help(help)
}

fn value<'b>(name: &'b str, short: &'b str, help: &'b str, value: &'b str) -> Arg<'b, 'b> {
    flag(name, short, help).takes_value(true).value_name(value)
}
