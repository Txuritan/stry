use {
    crate::backend::{BackendType, StorageType},
    std::{
        fs,
        io::{self, prelude::*},
        path::Path,
        str::FromStr,
        sync::Arc,
    },
};

macro_rules! over {
    (s, $cfg:ident, $over:ident) => {
        if let Some(v) = $over.take() {
            *$cfg = Some(v);
        }
    };
    (s, $cfg:ident, $over:ident, $( $pp:tt )*) => {
        if let Some(v) = $over.$( $pp )*.take() {
            $cfg.$( $pp )* = Some(v);
        }
    };
    ($cfg:ident, $over:ident, $( $pp:tt )*) => {
        if let Some(v) = $over.$( $pp )*.take() {
            $cfg.$( $pp )* = v;
        }
    };
    ($cfg:ident, $over:ident) => {
        if let Some(v) = $over.take() {
            *$cfg = v;
        }
    };
}

pub fn load_config(
    path: Option<String>,
    mut cfg_override: ConfigOverride,
) -> anyhow::Result<Arc<Config>> {
    let path = if let Some(path) = path {
        path
    } else {
        String::from("stry.ron")
    };

    let cfg_path = Path::new(&path);

    let mut cfg = if cfg_path.exists() {
        let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
        let mut reader = io::BufReader::new(file);

        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        ron::de::from_str(&contents)?
    } else {
        Config::default()
    };

    if let Some(host) = cfg_override.host.take() {
        let mut parts = host
            .split('.')
            .map(str::parse)
            .collect::<Vec<Result<u8, _>>>();

        let four = parts.pop().expect("Missing part of the host address")?;
        let three = parts.pop().expect("Missing part of the host address")?;
        let two = parts.pop().expect("Missing part of the host address")?;
        let one = parts.pop().expect("Missing part of the host address")?;

        cfg.host = [one, two, three, four];
    }

    if let Some(port) = cfg_override.port.take() {
        cfg.port = port.parse()?;
    }

    over!(cfg, cfg_override, workers);
    over!(cfg, cfg_override, database.typ);

    if let Some(mut storage) = cfg_override.database.storage.take() {
        match (&mut cfg.database.storage, &mut storage) {
            (StorageType::File { location: old }, StorageTypeOverride::File { location: new }) => {
                over!(old, new);
            }
            (
                StorageType::Parts {
                    username: username_old,
                    password: password_old,
                    host: host_old,
                    port: port_old,
                    database: database_old,
                    ..
                },
                StorageTypeOverride::Parts {
                    username: username_new,
                    password: password_new,
                    host: host_new,
                    port: port_new,
                    database: database_new,
                },
            ) => {
                over!(s, username_old, username_new);
                over!(s, password_old, password_new);
                over!(host_old, host_new);
                over!(s, port_old, port_new);
                over!(s, database_old, database_new);
            }
            _ => {}
        }
    }

    over!(s, cfg, cfg_override, executor.core_threads);
    over!(s, cfg, cfg_override, executor.max_threads);

    over!(cfg, cfg_override, logging.ansi);
    over!(cfg, cfg_override, logging.directory);
    over!(cfg, cfg_override, logging.level);
    over!(cfg, cfg_override, logging.json);
    over!(cfg, cfg_override, logging.prefix);
    over!(cfg, cfg_override, logging.thread_ids);
    over!(cfg, cfg_override, logging.thread_names);

    Ok(Arc::new(cfg))
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: [u8; 4],
    pub port: u16,
    pub workers: FourCount,
    pub database: Database,
    pub executor: Executor,
    pub logging: Logging,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: [0, 0, 0, 0],
            port: 8901,
            workers: FourCount::Four,
            database: Database::default(),
            executor: Executor::default(),
            logging: Logging::default(),
        }
    }
}

pub struct ConfigOverride {
    pub host: Option<String>,
    pub port: Option<String>,
    pub workers: Option<FourCount>,
    pub database: DatabaseOverride,
    pub executor: ExecutorOverride,
    pub logging: LoggingOverride,
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
    type Err = anyhow::Error;

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
            num => anyhow::bail!(
                "Worker count can only be a multiple of 4 (up to 32), given: {}",
                num
            ),
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

pub struct DatabaseOverride {
    pub typ: Option<BackendType>,
    pub storage: Option<StorageTypeOverride>,
}

pub enum StorageTypeOverride {
    File {
        location: Option<String>,
    },
    Parts {
        username: Option<String>,
        password: Option<String>,
        host: Option<String>,
        port: Option<String>,
        database: Option<String>,
    },
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Executor {
    pub core_threads: Option<usize>,
    pub max_threads: Option<usize>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            core_threads: None,
            max_threads: None,
        }
    }
}

pub struct ExecutorOverride {
    pub core_threads: Option<usize>,
    pub max_threads: Option<usize>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Logging {
    pub ansi: bool,
    pub directory: String,
    pub level: LogLevel,
    pub json: bool,
    pub prefix: String,
    pub thread_ids: bool,
    pub thread_names: bool,
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            ansi: true,
            directory: String::from("./logs"),
            level: LogLevel::Debug,
            json: false,
            prefix: String::from("log"),
            thread_ids: true,
            thread_names: true,
        }
    }
}

pub struct LoggingOverride {
    pub ansi: Option<bool>,
    pub directory: Option<String>,
    pub level: Option<LogLevel>,
    pub json: Option<bool>,
    pub prefix: Option<String>,
    pub thread_ids: Option<bool>,
    pub thread_names: Option<bool>,
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            level => anyhow::bail!(
                "Unknown log level, allowed: [error, warn, info, debug, trace], given: {}",
                level
            ),
        }
    }
}
