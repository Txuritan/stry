use {
    std::{
        fs,
        io::{self, prelude::*},
        path::Path,
        sync::Arc,
    },
    stry_common::{
        backend::{BackendType, StorageType},
        config::{Config, Database, Executor, FourCount, LogLevel, Logging},
    },
};

pub struct ConfigOverride {
    pub host: Option<String>,
    pub port: Option<String>,
    pub workers: Option<FourCount>,
    pub database: DatabaseOverride,
    pub executor: ExecutorOverride,
    pub logging: LoggingOverride,
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

pub struct ExecutorOverride {
    pub core_threads: Option<usize>,
    pub max_threads: Option<usize>,
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

pub fn load_config(
    path: Option<String>,
    cfg_override: ConfigOverride,
) -> anyhow::Result<Arc<Config>> {
    let path = if let Some(path) = path {
        path
    } else {
        String::from("stry.ron")
    };

    let cfg_path = Path::new(&path);

    let cfg = if cfg_path.exists() {
        let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
        let mut reader = io::BufReader::new(file);

        let mut contents = String::new();

        reader.read_to_string(&mut contents)?;

        ron::de::from_str(&contents)?
    } else {
        Config::default()
    };

    Ok(Arc::new(cfg))
}
