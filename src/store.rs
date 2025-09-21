//! Key-value store implementation

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::database::Database;

/// Key-value store with JSON persistence
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValueStore {
    data: HashMap<String, String>,
    created_at: u64,
    updated_at: u64,
}

impl KeyValueStore {
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

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize to JSON")?;
        fs::write(path, contents)
            .context("Failed to write file")?;
        Ok(())
    }

    fn update_timestamp(&mut self) {
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

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