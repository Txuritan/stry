use {
    db_derive::Pool,
    std::{
        fs,
        io::{self, prelude::*},
        path::Path,
    },
};

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Config {
    pub stry: Stry,
    pub database: Database,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let cfg_path = Path::new("stry.toml");

        let cfg = if cfg_path.exists() {
            let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
            let mut reader = io::BufReader::new(file);

            let mut contents = String::new();

            reader.read_to_string(&mut contents)?;

            toml::from_str(&contents)?
        } else {
            Config::default()
        };

        Ok(cfg)
    }

    pub fn create_pool(&self) -> anyhow::Result<Pool> {
        let pool = match &self.database {
            Database::SQLite { file } => Pool::sqlite(&file)?,
        };

        Ok(pool)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stry: Stry::default(),
            database: Database::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Stry {
    pub host: String,
    pub port: String,
    pub workers: usize,
}

impl Default for Stry {
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: String::from("8901"),
            workers: 4,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Database {
    #[serde(rename = "sqlite")]
    SQLite { file: String },
}

impl Default for Database {
    fn default() -> Self {
        Database::SQLite {
            file: String::from("stry.db"),
        }
    }
}
