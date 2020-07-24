use {
    std::{
        fs,
        io::{self, prelude::*},
        path::Path,
        sync::Arc,
    },
    stry_common::{
        backend::{BackendType, StorageType},
        config::{Config, FourCount, LogLevel},
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
