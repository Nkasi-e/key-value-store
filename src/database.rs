//! Database trait definition
//! 
//! This module defines the core Database trait that all storage backends must implement.

/// Database trait - defines the interface for different storage backends
/// 
/// This trait provides a common interface for key-value storage operations.
/// Different implementations can provide different storage mechanisms:
/// - In-memory storage
/// - File-based storage  
/// - Network-based storage
/// - Database-backed storage
/// 
/// # Type Parameters
/// - `K`: The type of keys stored in the database
/// - `V`: The type of values stored in the database
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::Database;
/// use std::collections::HashMap;
/// 
/// struct MemoryDatabase<K, V> {
///     data: HashMap<K, V>,
/// }
/// 
/// impl<K, V> Database<K, V> for MemoryDatabase<K, V> 
/// where 
///     K: std::hash::Hash + Eq + Clone,
///     V: Clone,
/// {
///     fn get(&self, key: &K) -> Option<V> {
///         self.data.get(key).cloned()
///     }
///     
///     fn set(&mut self, key: K, value: V) -> Option<V> {
///         self.data.insert(key, value)
///     }
///     
///     fn delete(&mut self, key: &K) -> Option<V> {
///         self.data.remove(key)
///     }
///     
///     fn exists(&self, key: &K) -> bool {
///         self.data.contains_key(key)
///     }
///     
///     fn keys(&self) -> Vec<K> {
///         self.data.keys().cloned().collect()
///     }
///     
///     fn len(&self) -> usize {
///         self.data.len()
///     }
///     
///     fn clear(&mut self) {
///         self.data.clear();
///     }
/// }
/// ```
pub trait Database<K, V> {
    /// Get a value by key
    /// 
    /// Returns `Some(value)` if the key exists, `None` otherwise.
    fn get(&self, key: &K) -> Option<V>;
    
    /// Set a key-value pair
    /// 
    /// Returns the previous value if the key existed, `None` otherwise.
    fn set(&mut self, key: K, value: V) -> Option<V>;
    
    /// Delete a key-value pair
    /// 
    /// Returns the value if the key existed, `None` otherwise.
    fn delete(&mut self, key: &K) -> Option<V>;
    
    /// Check if a key exists
    /// 
    /// Returns `true` if the key exists, `false` otherwise.
    fn exists(&self, key: &K) -> bool;
    
    /// Get all keys in the database
    /// 
    /// Returns a vector of all keys currently stored.
    fn keys(&self) -> Vec<K>;
    
    /// Get the number of key-value pairs
    /// 
    /// Returns the current size of the database.
    fn len(&self) -> usize;
    
    /// Clear all data from the database
    /// 
    /// Removes all key-value pairs from the database.
    fn clear(&mut self);
}
