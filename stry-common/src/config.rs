use {
    crate::backend::{BackendType, StorageType},
    std::str::FromStr,
};

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
