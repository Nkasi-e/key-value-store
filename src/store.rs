//! Key-value store implementation
//! 
//! This module contains the concrete implementation of the Database trait using HashMap.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::database::Database;

/// Key-value store that works with String keys and values
/// 
/// This is a concrete implementation of the Database trait using HashMap for storage.
/// It provides persistence capabilities by saving/loading data to/from JSON files.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::store::KeyValueStore;
/// use mini_db::database::Database;
/// 
/// let mut store = KeyValueStore::new();
/// store.set("key1".to_string(), "value1".to_string());
/// 
/// if let Some(value) = store.get("key1") {
///     println!("Found: {}", value);
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValueStore {
    /// The actual data storage
    data: HashMap<String, String>,
    /// Timestamp when the store was created
    created_at: u64,
    /// Timestamp when the store was last updated
    updated_at: u64,
}

impl KeyValueStore {
    /// Create a new empty key-value store
    /// 
    /// Initializes timestamps to the current time.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::store::KeyValueStore;
    /// 
    /// let store = KeyValueStore::new();
    /// assert_eq!(store.len(), 0);
    /// ```
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            data: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Load a key-value store from a file, or create a new one if the file doesn't exist
    /// 
    /// # Arguments
    /// * `path` - The file path to load from
    /// 
    /// # Returns
    /// * `Result<Self>` - The loaded store or a new empty store if file doesn't exist
    /// 
    /// # Errors
    /// * File read errors
    /// * JSON parsing errors
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::store::KeyValueStore;
    /// 
    /// let store = KeyValueStore::load_from_file("data.json")?;
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Ok(Self::new());
        }

        let contents = fs::read_to_string(path)
            .context("Failed to read file")?;
        let store: Self = serde_json::from_str(&contents)
            .context("Failed to parse JSON")?;
        Ok(store)
    }

    /// Save the key-value store to a file
    /// 
    /// # Arguments
    /// * `path` - The file path to save to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Errors
    /// * JSON serialization errors
    /// * File write errors
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::store::KeyValueStore;
    /// 
    /// let mut store = KeyValueStore::new();
    /// store.set("key".to_string(), "value".to_string());
    /// store.save_to_file("data.json")?;
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize to JSON")?;
        fs::write(path, contents)
            .context("Failed to write file")?;
        Ok(())
    }

    /// Update the timestamp to current time
    /// 
    /// This is called internally whenever the store is modified.
    fn update_timestamp(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Get the creation timestamp
    /// 
    /// # Returns
    /// * `u64` - Unix timestamp when the store was created
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Get the last update timestamp
    /// 
    /// # Returns
    /// * `u64` - Unix timestamp when the store was last updated
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }
}

impl Database<String, String> for KeyValueStore {
    fn get(&self, key: &String) -> Option<String> {
        self.data.get(key).cloned()
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.update_timestamp();
        self.data.insert(key, value)
    }

    fn delete(&mut self, key: &String) -> Option<String> {
        self.update_timestamp();
        self.data.remove(key)
    }

    fn exists(&self, key: &String) -> bool {
        self.data.contains_key(key)
    }

    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.update_timestamp();
        self.data.clear();
    }
}

impl Default for KeyValueStore {
    fn default() -> Self {
        Self::new()
    }
}