//! TTL-based in-memory cache for API responses.
//!
//! Prevents hammering the Aula API on every FUSE readdir/getattr/read.
//! Stores deserialized data; file content is derived on demand.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Default TTL for resource lists (readdir).
pub const LIST_TTL: Duration = Duration::from_secs(5 * 60);

/// Default TTL for individual resources (read).
#[allow(dead_code)]
pub const ITEM_TTL: Duration = Duration::from_secs(10 * 60);

/// Default TTL for presence data (more dynamic).
pub const PRESENCE_TTL: Duration = Duration::from_secs(2 * 60);

/// Cache key: identifies a cached API response.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    /// List of resources of a given type and page.
    ResourceList { resource: String, page: i32 },
    /// Single resource detail.
    ResourceDetail { resource: String, id: String },
}

struct CacheEntry {
    data: serde_json::Value,
    fetched_at: Instant,
    ttl: Duration,
}

impl CacheEntry {
    fn is_expired(&self) -> bool {
        self.fetched_at.elapsed() > self.ttl
    }
}

/// Thread-safe TTL cache for API responses.
pub struct Cache {
    entries: HashMap<CacheKey, CacheEntry>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Get a cached value if it exists and is not expired.
    pub fn get(&self, key: &CacheKey) -> Option<&serde_json::Value> {
        let entry = self.entries.get(key)?;
        if entry.is_expired() {
            None
        } else {
            Some(&entry.data)
        }
    }

    /// Store a value in the cache with the given TTL.
    pub fn put(&mut self, key: CacheKey, data: serde_json::Value, ttl: Duration) {
        self.entries.insert(
            key,
            CacheEntry {
                data,
                fetched_at: Instant::now(),
                ttl,
            },
        );
    }

    /// Remove expired entries.
    #[allow(dead_code)]
    pub fn evict_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.is_expired());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get() {
        let mut cache = Cache::new();
        let key = CacheKey::ResourceList {
            resource: "posts".into(),
            page: 0,
        };
        cache.put(key.clone(), serde_json::json!({"test": true}), LIST_TTL);
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn expired_returns_none() {
        let mut cache = Cache::new();
        let key = CacheKey::ResourceList {
            resource: "posts".into(),
            page: 0,
        };
        // Use a zero TTL to immediately expire.
        cache.put(key.clone(), serde_json::json!({}), Duration::ZERO);
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn evict_expired_removes_stale() {
        let mut cache = Cache::new();
        let key = CacheKey::ResourceList {
            resource: "posts".into(),
            page: 0,
        };
        cache.put(key.clone(), serde_json::json!({}), Duration::ZERO);
        cache.evict_expired();
        assert_eq!(cache.entries.len(), 0);
    }
}
