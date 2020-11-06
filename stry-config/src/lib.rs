use {
    anyhow::Context,
    std::{collections::HashMap, str::FromStr},
};

#[cfg(feature = "sources")]
use std::{
    env, fs,
    io::{self, prelude::*},
    path::Path,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Frontend can only be `both`, `api` or `user`, found '{value}'")]
    InvalidFrontendValue { value: String },
    #[error("Log level can only be `error`, `warn`, `info`, `debug` or `trace`, found '{value}'")]
    InvalidLogLevel { value: String },
    #[error("Worker count can only be a multiple of 4 (up to 32), found '{value}'")]
    InvalidWorkerCountValue { value: String },
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub ip: [u8; 4],
    pub port: u16,
    pub tls: Tls,
    pub frontend: Frontend,
    pub workers: FourCount,
    pub database: Database,
    pub executor: Executor,
    pub logging: Logging,
}

impl Config {
    #[cfg(feature = "sources")]
    pub fn new_from_sources<P>(path: P, args: clap::ArgMatches<'_>) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let cfg_path = path.as_ref();

        let Config {
            ip,
            port,
            tls,
            frontend,
            workers,
            database,
            executor,
            logging,
        } = if cfg_path.exists() {
            let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
            let mut reader = io::BufReader::new(file);

            let mut contents = String::new();

            reader.read_to_string(&mut contents)?;

            ron::de::from_str(&contents)?
        } else {
            Config::default()
        };

        Ok(Self {
            ip: env::var("STRY_SERVER_IP")
                .context("Unable to get value of environmental variable `STRY_SERVER_IP`")
                .or_else(|_| {
                    args.value_of("server-ip").map(String::from).ok_or_else(|| {
                        anyhow::anyhow!("No argument named 'server-ip' found in provided args")
                    })
                })
                .and_then(|value| {
                    let mut parts = value
                        .split('.')
                        .map(str::parse)
                        .collect::<Vec<Result<u8, _>>>();

                    let four = parts.pop().expect("Missing part of the host address")?;
                    let three = parts.pop().expect("Missing part of the host address")?;
                    let two = parts.pop().expect("Missing part of the host address")?;
                    let one = parts.pop().expect("Missing part of the host address")?;

                    Ok([one, two, three, four])
                })
                .or_else::<anyhow::Error, _>(|_| Ok(ip))?,
            port: env::var("STRY_SERVER_PORT")
                .context("Unable to get value of environmental variable `STRY_SERVER_PORT`")
                .or_else(|_| {
                    args.value_of("server-port")
                        .map(String::from)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "No argument named 'server-port' found in provided args"
                            )
                        })
                })
                .and_then(|value| {
                    let port = value.parse()?;

                    Ok(port)
                })
                .or_else::<anyhow::Error, _>(|_| Ok(port))?,
            tls: env::var("STRY_TLS")
                .context("Unable to get value of environmental variable `STRY_TLS`")
                .and_then(|value| {
                    let cert = env::var("STRY_TLS_CERT")?;
                    let key = env::var("STRY_TLS_KEY")?;

                    match &*value.to_lowercase() {
                        "file" => Ok(Tls::File { cert, key }),
                        "text" => Ok(Tls::Text { cert, key }),
                        _ => Err(anyhow::anyhow!("'{}' is not a valid TLS type")),
                    }
                })
                .or_else::<anyhow::Error, _>(|_| Ok(tls))?,
            frontend: env::var("STRY_FRONTEND")
                .context("Unable to get value of environmental variable `STRY_FRONTEND`")
                .and_then(|value| {
                    let frontend = Frontend::from_str(&value)?;

                    Ok(frontend)
                })
                .or_else::<anyhow::Error, _>(|_| Ok(frontend))?,
            workers: env::var("STRY_WORKERS")
                .context("Unable to get value of environmental variable `STRY_WORKERS`")
                .and_then(|value| {
                    let workers = FourCount::from_str(&value)?;

                    Ok(workers)
                })
                .or_else::<anyhow::Error, _>(|_| Ok(workers))?,
            database: Database::new_from_sources(database, args.clone())?,
            executor: Executor::new_from_sources(executor, args.clone())?,
            logging: Logging::new_from_sources(logging, args.clone())?,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: [0, 0, 0, 0],
            port: 8901,
            tls: Tls::None,
            frontend: Frontend::Both,
            workers: FourCount::Four,
            database: Database::default(),
            executor: Executor::default(),
            logging: Logging::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub enum Tls {
    File { cert: String, key: String },
    Text { cert: String, key: String },
    None,
}

impl Default for Tls {
    fn default() -> Self {
        Tls::None
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Frontend {
    Both,
    Api,
    User,
}

impl Frontend {
    pub fn as_bool(self) -> (bool, bool) {
        match self {
            Frontend::Both => (true, true),
            Frontend::Api => (true, false),
            Frontend::User => (false, true),
        }
    }
}

impl FromStr for Frontend {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "both" => Ok(Frontend::Both),
            "api" => Ok(Frontend::Api),
            "user" => Ok(Frontend::User),
            _ => Err(Error::InvalidFrontendValue {
                value: s.to_string(),
            }),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum FourCount {
    Four,
    Eight,
    Twelve,
    Sixteen,
    Twenty,
    TwentyFour,
    TwentyEight,
    ThirtyTwo,
}

impl FourCount {
    pub fn as_count(self) -> usize {
        match self {
            FourCount::Four => 4,
            FourCount::Eight => 8,
            FourCount::Twelve => 12,
            FourCount::Sixteen => 16,
            FourCount::Twenty => 20,
            FourCount::TwentyFour => 24,
            FourCount::TwentyEight => 28,
            FourCount::ThirtyTwo => 32,
        }
    }
}

impl FromStr for FourCount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "4" => Ok(FourCount::Four),
            "8" => Ok(FourCount::Eight),
            "12" => Ok(FourCount::Twelve),
            "16" => Ok(FourCount::Sixteen),
            "20" => Ok(FourCount::Twenty),
            "24" => Ok(FourCount::TwentyFour),
            "28" => Ok(FourCount::TwentyEight),
            "32" => Ok(FourCount::ThirtyTwo),
            _ => Err(Error::InvalidWorkerCountValue {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Database {
    #[serde(rename = "type")]
    pub typ: BackendType,
    pub storage: StorageType,
}

impl Database {
    #[cfg(feature = "sources")]
    pub fn new_from_sources(
        database: Database,
        args: clap::ArgMatches<'_>,
    ) -> anyhow::Result<Self> {
        let Database { typ, storage } = database;

        let typ = env::var("STRY_BACKEND_TYPE")
            .context("Unable to get value of environmental variable `STRY_BACKEND_TYPE`")
            .or_else(|_| {
                args.value_of("backend-type")
                    .map(String::from)
                    .ok_or_else(|| {
                        anyhow::anyhow!("No argument named 'backend-type' found in provided args")
                    })
            })
            .and_then(|value| match &*value.to_lowercase() {
                "postgres" => Ok(BackendType::Postgres),
                "sqlite" => Ok(BackendType::Sqlite),
                _ => Err(anyhow::anyhow!("'{}' is not a valid database type")),
            })
            .or_else::<anyhow::Error, _>(|_| Ok(typ))?;

        Ok(Self {
            storage: match typ {
                BackendType::Postgres => {
                    env::var("STRY_BACKEND_HOST")
                        .context(
                            "Unable to get value of environmental variable `STRY_BACKEND_HOST`",
                        )
                        .map(|host| {
                            let port = env::var("STRY_BACKEND_PORT");
                            let database = env::var("STRY_BACKEND_DATABASE");
                            let username = env::var("STRY_BACKEND_USERNAME");
                            let password = env::var("STRY_BACKEND_PASSWORD");

                            (username.ok(), password.ok(), host, port.ok(), database.ok())
                        })
                        .or_else(|_| {
                            args.value_of("backend-host")
                                .map(String::from)
                                .ok_or_else(|| {
                                    anyhow::anyhow!(
                                        "No argument named 'backend-host' found in provided args"
                                    )
                                })
                                .map(|host| {
                                    let port = args.value_of("backend-port").map(String::from);
                                    let database =
                                        args.value_of("backend-database").map(String::from);
                                    let username =
                                        args.value_of("backend-username").map(String::from);
                                    let password =
                                        args.value_of("backend-password").map(String::from);

                                    (username, password, host, port, database)
                                })
                        })
                        .map(|(username, password, host, port, database)| {
                            StorageType::Parts {
                                username,
                                password,
                                host,
                                port,
                                database,
                                // TODO: maybe serde will help with this
                                params: None,
                            }
                        })
                        .or_else::<anyhow::Error, _>(|_| Ok(storage))?
                }
                BackendType::Sqlite => env::var("STRY_BACKEND_FILE")
                    .context("Unable to get value of environmental variable `STRY_BACKEND_FILE`")
                    .or_else(|_| {
                        args.value_of("backend-file")
                            .map(String::from)
                            .ok_or_else(|| {
                                anyhow::anyhow!(
                                    "No argument named 'backend-file' found in provided args"
                                )
                            })
                    })
                    .map(|location| StorageType::File { location })
                    .or_else::<anyhow::Error, _>(|_| Ok(storage))?,
            },
            typ,
        })
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            typ: BackendType::Sqlite,
            storage: StorageType::File {
                location: String::from("stry.db"),
            },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum StorageType {
    File {
        location: String,
    },
    Parts {
        username: Option<String>,
        password: Option<String>,
        host: String,
        port: Option<String>,
        database: Option<String>,
        params: Option<HashMap<String, String>>,
    },
}

impl StorageType {
    pub fn is_file(&self) -> bool {
        matches!(self, StorageType::File { .. })
    }

    pub fn is_parts(&self) -> bool {
        matches!(self, StorageType::Parts { .. })
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum BackendType {
    Postgres,
    Sqlite,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Executor {
    pub core_threads: Option<usize>,
    pub max_threads: Option<usize>,
}

impl Executor {
    #[cfg(feature = "sources")]
    pub fn new_from_sources(
        executor: Executor,
        _args: clap::ArgMatches<'_>,
    ) -> anyhow::Result<Self> {
        let Executor {
            core_threads,
            max_threads,
        } = executor;

        Ok(Self {
            core_threads: env::var("STRY_EXECUTOR_CORE_THREADS")
                .context(
                    "Unable to get value of environmental variable `STRY_EXECUTOR_CORE_THREADS`",
                )
                .ok()
                .and_then(|value| value.parse().ok())
                .or_else(|| core_threads),
            max_threads: env::var("STRY_EXECUTOR_MAX_THREADS")
                .context(
                    "Unable to get value of environmental variable `STRY_EXECUTOR_MAX_THREADS`",
                )
                .ok()
                .and_then(|value| value.parse().ok())
                .or_else(|| max_threads),
        })
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            core_threads: None,
            max_threads: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Logging {
    pub ansi: bool,
    pub flame: Option<String>,
    pub level: LogLevel,
    pub out: LoggingOutput,
    pub thread_ids: bool,
    pub thread_names: bool,
}

impl Logging {
    #[cfg(feature = "sources")]
    pub fn new_from_sources(logging: Logging, args: clap::ArgMatches<'_>) -> anyhow::Result<Self> {
        let Logging {
            ansi,
            flame,
            level,
            out,
            thread_ids,
            thread_names,
        } = logging;

        Ok(Self {
            ansi: env::var("STRY_LOGGING_ANSI")
                .context("Unable to get value of environmental variable `STRY_LOGGING_ANSI`")
                .map(|value| match &*value {
                    "0" => false,
                    _ => true,
                })
                .or_else::<anyhow::Error, _>(|_| Ok(ansi))?,
            flame: env::var("STRY_LOGGING_FLAME")
                .context("Unable to get value of environmental variable `STRY_LOGGING_FLAME`")
                .or_else(|_| {
                    args.value_of("tracing-flame")
                        .map(String::from)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "No argument named 'tracing-flame' found in provided args"
                            )
                        })
                })
                .ok()
                .or_else(|| flame),
            level: env::var("STRY_LOGGING_LEVEL")
                .context("Unable to get value of environmental variable `STRY_LOGGING_LEVEL`")
                .or_else(|_| {
                    args.value_of("tracing-level")
                        .map(String::from)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "No argument named 'tracing-level' found in provided args"
                            )
                        })
                })
                .and_then(|value| LogLevel::from_str(&value).map_err(anyhow::Error::from))
                .or_else::<anyhow::Error, _>(|_| Ok(level))?,
            out: env::var("STRY_LOGGING_OUTPUT")
                .context("Unable to get value of environmental variable `STRY_LOGGING_OUTPUT`")
                .map(|output| {
                    let json = env::var("STRY_LOGGING_JSON").map(|value| match &*value {
                        "0" => false,
                        _ => true,
                    });

                    let directory = env::var("STRY_LOGGING_DIRECTORY").ok();
                    let prefix = env::var("STRY_LOGGING_PREFIX").ok();

                    (output, directory, json, prefix)
                })
                .map(|(output, mut directory, json, mut prefix)| {
                    if directory.is_none() && args.is_present("tracing-directory") {
                        directory = args.value_of("tracing-directory").map(String::from);
                    }

                    if prefix.is_none() && args.is_present("tracing-prefix") {
                        prefix = args.value_of("tracing-prefix").map(String::from);
                    }

                    (output, directory, json, prefix)
                })
                .map_err(anyhow::Error::from)
                .and_then(
                    |(output, directory, json, prefix)| match &*output.to_lowercase() {
                        "both" => Ok(LoggingOutput::Both {
                            directory: directory.ok_or_else(|| {
                                anyhow::anyhow!("Unable to get logging directory")
                            })?,
                            json: json.context("Unable to get logging JSON")?,
                            prefix: prefix
                                .ok_or_else(|| anyhow::anyhow!("Unable to get logging prefix"))?,
                        }),
                        "file" => Ok(LoggingOutput::File {
                            directory: directory.ok_or_else(|| {
                                anyhow::anyhow!("Unable to get logging directory")
                            })?,
                            json: json.context("Unable to get logging JSON")?,
                            prefix: prefix
                                .ok_or_else(|| anyhow::anyhow!("Unable to get logging prefix"))?,
                        }),
                        "stdout" => Ok(LoggingOutput::StdOut {
                            json: json.context("Unable to get logging JSON")?,
                        }),
                        _ => anyhow::bail!("`{}` is not a valid logging output type", output),
                    },
                )
                .or_else::<anyhow::Error, _>(|_| Ok(out))?,
            thread_ids: env::var("STRY_LOGGING_THREAD_IDS")
                .context("Unable to get value of environmental variable `STRY_LOGGING_THREAD_IDS`")
                .map(|value| match &*value {
                    "0" => false,
                    _ => true,
                })
                .or_else::<anyhow::Error, _>(|_| Ok(thread_ids))?,
            thread_names: env::var("STRY_LOGGING_THREAD_NAMES")
                .context(
                    "Unable to get value of environmental variable `STRY_LOGGING_THREAD_NAMES`",
                )
                .map(|value| match &*value {
                    "0" => false,
                    _ => true,
                })
                .or_else::<anyhow::Error, _>(|_| Ok(thread_names))?,
        })
    }
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            ansi: true,
            flame: None,
            level: LogLevel::Debug,
            out: LoggingOutput::StdOut { json: false },
            thread_ids: true,
            thread_names: true,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl FromStr for LogLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(Error::InvalidLogLevel {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub enum LoggingOutput {
    Both {
        directory: String,
        json: bool,
        prefix: String,
    },
    File {
        directory: String,
        json: bool,
        prefix: String,
    },
    StdOut {
        json: bool,
    },
}
