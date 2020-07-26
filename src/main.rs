pub mod backend;
pub mod commands;
pub mod frontend;
pub mod models;
pub mod workers;

pub mod config;
pub mod search;
pub mod version;

use {
    crate::{
        backend::BackendType,
        commands::{endpoint, serve},
        config::LogLevel,
    },
    anyhow::Context,
    clap::{App, Arg},
    std::{future::Future, pin::Pin},
    tokio::runtime::Builder,
    tracing::Level,
    tracing_subscriber::{
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

pub type Boxed = Pin<Box<dyn Future<Output = anyhow::Result<()>>>>;

fn main() -> anyhow::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(&*format!("{}-{}", version::VERSION, version::GIT_VERSION))
        .arg(value("config", "c", "Use a specified config instead of the default", "CONFIG"))
        .arg(value("workers", "w", "Sets the amount of task workers", "COUNT"))
        .arg(value("backend-database", "d", "Database name for remote backend database", "DATABASE"))
        .arg(value("backend-host", "H", "Host address for remote backend database", "HOST"))
        .arg(value("backend-port", "P", "Port number for remote backend database", "PORT"))
        .arg(value("backend-file", "f", "The file of the SQLite backend database", "FILE"))
        .arg(value("backend-password", "s", "Password for remote backend database user", "PASSWORD"))
        .arg(value("backend-type", "t", "The type of the backend database", "TYPE").possible_values(&["postgres", "sqlite"]))
        .arg(value("backend-username", "u", "Username for remote backend database user", "USERNAME"))
        .arg(value("log-directory", "o", "Directory to write logging files to", "DIRECTORY"))
        .arg(value("log-level", "l", "Lowest level for logging output", "LEVEL").possible_values(&["error", "warn", "info", "debug", "trace"]))
        .arg(value("log-prefix", "x", "Logging file name prefix", "PREFIX"))
        .arg(value("server-ip", "i", "IP that the server will listen for requests on", "IP"))
        .arg(value("server-port", "p", "Port used by the server", "PORT"))
        .arg(flag("log-ansi", "a", "Enables ANSI coloring of logging output"))
        .arg(flag("log-json", "j", "Output logging in JSON format"))
        .arg(flag("log-thread-ids", "D", "Logging output contains the ID of its source thread"))
        .arg(flag("log-thread-names", "n", "Logging output contains the name of its source thread"))
        .version_short("v")
        .get_matches();

    let mut args = pico_args::Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        help();

        return Ok(());
    }

    if args.contains(["-v", "--version"]) {
        println!("stry v{}-{}", version::VERSION, version::GIT_VERSION);

        return Ok(());
    }

    let cfg = config::load_config(
        args.opt_value_from_str(["-c", "--config"])?,
        get_config_override(&mut args)?,
    )
    .context("Failure to create config instance")?;

    let file_appender =
        tracing_appender::rolling::daily(&cfg.logging.directory, &cfg.logging.prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // TODO: Get JSON output working
    let reg = Registry::default()
        .with(
            fmt::Layer::default()
                .with_ansi(cfg.logging.ansi)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(
            fmt::Layer::default()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE),
        )
        .with(LevelFilter::from(match cfg.logging.level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }));

    tracing::subscriber::set_global_default(reg)
        .context("Failed to set Tracing global subscriber")?;

    tracing_log::LogTracer::init().context("Failed to set Tracing as global Log drain")?;

    let mut builder = Builder::new();

    builder.threaded_scheduler();

    if let Some(count) = &cfg.executor.core_threads {
        builder.core_threads(*count);
    }

    if let Some(count) = &cfg.executor.max_threads {
        builder.max_threads(*count);
    }

    let mut rt = builder
        .enable_all()
        .build()
        .context("Unable to build Tokio runtime")?;

    let fut = match args.subcommand()?.as_deref() {
        Some("endpoint") => endpoint::handle(args, cfg)?,
        Some("serve") | None => serve::handle(args, cfg)?,
        Some(other) => {
            println!("Unknown command: {}", other);
            println!();
            println!("Use `stry --help` to see all allowed commands");

            return Ok(());
        }
    };

    rt.block_on(fut)?;

    tracing::info!("Thank you. I'll say goodbye soon.. Though its the end of the would. Don't blame yourself now.. I'll be okay");

    Ok(())
}

fn flag<'b>(name: &'b str, short: &'b str, help: &'b str)-> Arg<'b, 'b> {
    Arg::with_name(name)
        .long(name)
        .short(short)
        .help(help)
}

fn value<'b>(name: &'b str, short: &'b str, help: &'b str, value: &'b str) -> Arg<'b, 'b> {
    flag(name, short, help)
        .takes_value(true)
        .value_name(value)
}

#[rustfmt::skip]
fn help() {
    println!("stry v{}-{}", version::VERSION, version::GIT_VERSION);
    println!();
    println!("Usage:");
    println!("  stry <COMMAND> [--config <FILE>]");
    println!("  stry <COMMAND> [--host <HOST>] [--port <PORT>]");
    println!("  stry <COMMAND> [--workers <COUNT>]");
    println!("  stry <COMMAND> [--backend-type <TYPE> --backend-file <FILE>]");
    println!("  stry -h | --help");
    println!("  stry -v | --version");
    println!();
    println!("Commands:");
    println!("  stry endpoint                   Start the web server with only the API");
    println!("  stry serve                      Start the web server with user front-end and API");
    println!();
    println!("Options:");
    println!("  -h, --help                      Show this screen");
    println!("  -v, --version                   Show version");
    println!("  -C <FILE>, --config <FILE>      Use a specified config instead of the default");
    println!("  -H <HOST>, --host <HOST>        Sets the server ip [default: 0.0.0.0]");
    println!("  -P <PORT>, --port <PORT>        Sets the server port [default: 8901]");
    println!("  -W <COUNT>, --workers <COUNT>   Sets the amount of task workers [default: 4]");
    println!("  --backend-type <TYPE>           Sets the database type [default: sqlite]");
    println!("  --backend-file <FILE>           Sets the database file path [default: stry.db]");
    println!("  --backend-username <USERNAME>   Sets the database connection username");
    println!("  --backend-password <PASSWORD>   Sets the database connection password");
    println!("  --backend-host <HOST>           Sets the database connection host");
    println!("  --backend-port <PORT>           Sets the database connection port");
    println!("  --backend-database <DATABASE>   Sets the database connection database");
    println!("  --log-ansi                      Enables ANSI coloring if its disabled");
    println!("  --log-no-ansi                   Disables ANSI coloring if its enabled");
    println!("  --log-directory <DIRECTORY>     Sets the directory to write logs to");
    println!("  --log-level <LEVEL>             Sets the lowest logging level");
    println!("  --log-json                      Enabled JSON logging if its disabled");
    println!("  --log-no-json                   Disables JSON logging if its enabled");
    println!("  --log-prefix <PREFIX>           Sets the prefix of log files");
    println!("  --log-thread-ids                Enabled logging of thread IDs if its disabled");
    println!("  --log-no-thread-ids             Disables logging of thread IDs if its enabled");
    println!("  --log-thread-names              Enabled logging of thread names if its disabled");
    println!("  --log-no-thread-names           Disables logging of thread names if its enabled");
}

fn get_config_override(args: &mut pico_args::Arguments) -> anyhow::Result<config::ConfigOverride> {
    let typ = match args
        .opt_value_from_str::<_, String>("--backend-type")?
        .as_deref()
    {
        Some("sqlite") => Some(BackendType::Sqlite),
        Some("postgres") => Some(BackendType::Sqlite),
        _ => None,
    };

    let cfg_override = config::ConfigOverride {
        host: args.opt_value_from_str(["-H", "--host"])?,
        port: args.opt_value_from_str(["-P", "--port"])?,
        workers: args.opt_value_from_str(["-W", "--workers"])?,
        database: config::DatabaseOverride {
            typ,
            storage: if let Some(typ) = typ {
                Some(match typ {
                    BackendType::Sqlite => config::StorageTypeOverride::File {
                        location: args.opt_value_from_str("--backend-file")?,
                    },
                    BackendType::Postgres => config::StorageTypeOverride::Parts {
                        username: args.opt_value_from_str("--backend-username")?,
                        password: args.opt_value_from_str("--backend-password")?,
                        host: args.opt_value_from_str("--backend-host")?,
                        port: args.opt_value_from_str("--backend-port")?,
                        database: args.opt_value_from_str("--backend-database")?,
                    },
                })
            } else {
                None
            },
        },
        executor: config::ExecutorOverride {
            core_threads: None,
            max_threads: None,
        },
        logging: config::LoggingOverride {
            ansi: args
                .opt_value_from_str("--log-ansi")?
                .or(args.opt_value_from_str("--log-no-ansi")?),
            directory: args.opt_value_from_str("--log-directory")?,
            level: args.opt_value_from_str("--log-level")?,
            json: args
                .opt_value_from_str("--log-json")?
                .or(args.opt_value_from_str("--log-no-json")?),
            prefix: args.opt_value_from_str("--log-prefix")?,
            thread_ids: args
                .opt_value_from_str("--log-thread-ids")?
                .or(args.opt_value_from_str("--log-no-thread-ids")?),
            thread_names: args
                .opt_value_from_str("--log-thread-names")?
                .or(args.opt_value_from_str("--log-no-thread-names")?),
        },
    };

    Ok(cfg_override)
}
