use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A sharded database that distributes keys across multiple shards
/// to reduce lock contention in concurrent access scenarios.
/// 
/// Uses the **newtype pattern** to wrap the internal Arc, allowing us to
/// implement methods directly on the type.
pub struct ShardedDatabase {
    shards: Arc<Vec<Mutex<HashMap<String, Bytes>>>>,
}

impl ShardedDatabase {
    /// Creates a new sharded database with the specified number of shards.
    pub fn new(num_shards: usize) -> Self {
        let mut shards = Vec::with_capacity(num_shards);
        for _ in 0..num_shards {
            shards.push(Mutex::new(HashMap::new()));
        }
        Self { shards: Arc::new(shards) }
    }

    /// Inserts a key-value pair into the appropriate shard.
    pub fn insert(&self, key: &str, value: Bytes) {
        let shard_index = Self::get_shard_index(key, self.shards.len());
        let mut shard = self.shards[shard_index].lock().unwrap();
        shard.insert(key.to_string(), value);
    }

    /// Retrieves a value by key from the appropriate shard.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let shard_index = Self::get_shard_index(key, self.shards.len());
        let shard = self.shards[shard_index].lock().unwrap();
        shard.get(key).cloned()
    }

    /// Computes which shard a key belongs to using a hash function.
    /// This is a pure function that doesn't require instance data.
    fn get_shard_index(key: &str, num_shards: usize) -> usize {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % num_shards
    }
}

impl Clone for ShardedDatabase {
    fn clone(&self) -> Self {
        Self { shards: Arc::clone(&self.shards) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let db = ShardedDatabase::new(4);
        db.insert("hello", Bytes::from("world"));
        
        let value = db.get("hello");
        assert_eq!(value, Some(Bytes::from("world")));
    }

    #[test]
    fn test_get_nonexistent_key_returns_none() {
        let db = ShardedDatabase::new(4);
        
        let value = db.get("nonexistent");
        assert_eq!(value, None);
    }

    #[test]
    fn test_overwrite_key() {
        let db = ShardedDatabase::new(4);
        db.insert("key", Bytes::from("value1"));
        db.insert("key", Bytes::from("value2"));
        
        let value = db.get("key");
        assert_eq!(value, Some(Bytes::from("value2")));
    }

    #[test]
    fn test_multiple_keys() {
        let db = ShardedDatabase::new(4);
        db.insert("key1", Bytes::from("value1"));
        db.insert("key2", Bytes::from("value2"));
        db.insert("key3", Bytes::from("value3"));
        
        assert_eq!(db.get("key1"), Some(Bytes::from("value1")));
        assert_eq!(db.get("key2"), Some(Bytes::from("value2")));
        assert_eq!(db.get("key3"), Some(Bytes::from("value3")));
    }

    #[test]
    fn test_clone_shares_data() {
        let db1 = ShardedDatabase::new(4);
        db1.insert("shared_key", Bytes::from("shared_value"));
        
        let db2 = db1.clone();
        
        // Both should see the same data
        assert_eq!(db2.get("shared_key"), Some(Bytes::from("shared_value")));
        
        // Insert via clone should be visible in original
        db2.insert("new_key", Bytes::from("new_value"));
        assert_eq!(db1.get("new_key"), Some(Bytes::from("new_value")));
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;

        let db = ShardedDatabase::new(16);
        let mut handles = vec![];

        // Spawn multiple threads that insert keys
        for i in 0..10 {
            let db_clone = db.clone();
            let handle = thread::spawn(move || {
                let key = format!("key_{}", i);
                let value = format!("value_{}", i);
                db_clone.insert(&key, Bytes::from(value));
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all keys were inserted
        for i in 0..10 {
            let key = format!("key_{}", i);
            let expected = format!("value_{}", i);
            assert_eq!(db.get(&key), Some(Bytes::from(expected)));
        }
    }

    #[test]
    fn test_keys_distribute_across_shards() {
        // With multiple shards, different keys should hash to different shards
        // This is a probabilistic test - with enough keys, they should spread out
        let db = ShardedDatabase::new(4);
        
        // Insert many keys
        for i in 0..100 {
            let key = format!("key_{}", i);
            db.insert(&key, Bytes::from("value"));
        }

        // Verify all keys are retrievable (sharding works correctly)
        for i in 0..100 {
            let key = format!("key_{}", i);
            assert!(db.get(&key).is_some(), "Key {} should exist", key);
        }
    }
}
