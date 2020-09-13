use {
    std::{
        borrow::Borrow, cmp::Eq, collections::HashMap, fmt::Debug, hash::Hash, slice, sync::Arc,
    },
    tokio::sync::RwLock,
};

#[async_trait::async_trait]
pub trait Fetcher<Key, Value> {
    async fn fetch(&self, keys: &[Key], values: Cache<Key, Value>) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct Cache<Key, Value> {
    inner: Arc<RwLock<HashMap<Key, Value>>>,
}

impl<Key, Value> Cache<Key, Value> {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, key: Key, value: Value) -> Option<Value>
    where
        Key: Eq + Hash,
    {
        let mut map = self.inner.write().await;

        map.insert(key, value)
    }

    async fn exists<Q>(&self, key: &Q) -> bool
    where
        Key: Borrow<Q> + Eq + Hash,
        Q: Eq + Hash,
    {
        let map = self.inner.read().await;

        map.contains_key(key)
    }
}

impl<Key, Value> Clone for Cache<Key, Value> {
    fn clone(&self) -> Self {
        Cache {
            inner: self.inner.clone(),
        }
    }
}

impl<Key, Value> Default for Cache<Key, Value> {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

pub struct Loader<F, Key, Value>
where
    F: Fetcher<Key, Value>,
{
    cache: Cache<Key, Value>,
    fetcher: F,
}

impl<F, Key, Value> Loader<F, Key, Value>
where
    F: Fetcher<Key, Value>,
    Key: Debug + Eq + Hash,
    Value: Clone,
{
    #[tracing::instrument(level = "debug", skip(self), err)]
    pub async fn load(&self, key: &Key) -> anyhow::Result<Value> {
        let mut values = self.load_many(slice::from_ref(key)).await?;

        debug_assert!(
            values.len() <= 1,
            "There should only be 1 or 0 values returned for load"
        );

        values.pop()
            .ok_or_else(|| anyhow::anyhow!("Loader did not return any value, either the cache broken, or it does not exist in the data base"))
    }

    #[tracing::instrument(level = "debug", skip(self), err)]
    pub async fn load_many(&self, keys: &[Key]) -> anyhow::Result<Vec<Value>> {
        for key in keys {
            let exists = self.cache.exists(key).await;

            if !exists {
                self.fetcher
                    .fetch(slice::from_ref(key), self.cache.clone())
                    .await?;
            }
        }

        let mut values = Vec::with_capacity(keys.len());

        let map = self.cache.inner.read().await;

        for key in keys {
            if let Some(value) = map.get(key) {
                values.push(value.clone());
            }
        }

        debug_assert!(
            keys.len() == values.len(),
            "Could not get every value, the cache is not working properly"
        );

        Ok(values)
    }
}
