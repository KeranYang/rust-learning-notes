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
        let shard_index = self.get_shard_index(key);
        let mut shard = self.shards[shard_index].lock().unwrap();
        shard.insert(key.to_string(), value);
    }

    /// Retrieves a value by key from the appropriate shard.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let shard_index = self.get_shard_index(key);
        let shard = self.shards[shard_index].lock().unwrap();
        shard.get(key).cloned()
    }

    /// Computes which shard a key belongs to using a hash function.
    fn get_shard_index(&self, key: &str) -> usize {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shards.len()
    }
}

impl Clone for ShardedDatabase {
    fn clone(&self) -> Self {
        Self { shards: Arc::clone(&self.shards) }
    }
}
