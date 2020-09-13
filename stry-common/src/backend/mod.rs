pub mod nanoid;
// pub mod spec;

#[cfg(test)]
pub mod test_helpers;

// pub use self::spec::{
//     Backend, BackendAuthor, BackendChapter, BackendCharacter, BackendOrigin, BackendPairing,
//     BackendStory, BackendTag, BackendWarning, BackendWorker,
// };

use std::collections::HashMap;

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
